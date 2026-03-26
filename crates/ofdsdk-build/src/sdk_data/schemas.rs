use anyhow::Context;
use heck::{ToSnakeCase, ToUpperCamelCase};

use crate::models::{
  sdk_data::{
    Alias, Attribute as SdkDataAttribute, Child, CodegenTypeKind, Element as SdkDataElement,
    ElementApiKind, Enum, Schema as SdkDataSchema, Struct, Variant,
  },
  xsd::{
    Attribute, Choice, ChoiceContentChoice, ComplexContentContentChoice, ComplexType,
    ComplexTypeContentChoice, Element, ElementContentChoice, Extension, ExtensionContentChoice,
    Restriction, RestrictionContentChoice, Schema as XsdSchema, SchemaContentChoice, Sequence,
    SequenceContentChoice, SimpleContentContentChoice, SimpleType, SimpleTypeContentChoice,
  },
};
use crate::sdk_data::context::Context as SdkDataContext;

mod codegen;
mod merge;
mod normalize;
mod traversal;
use codegen::annotate_schema_codegen_types;
use merge::{apply_extension_base, insert_attribute, insert_choice_child, insert_sequence_child};
use normalize::normalize_schema_type_refs;
use traversal::{gen_complex_type_particle, gen_extension_content, gen_simple_content_extension};

#[derive(Clone, Copy)]
enum TypeNameMode {
  PreserveInlineName,
  CamelCaseInlineName,
}

#[derive(Clone, Copy, Default)]
struct OccursState {
  is_option: bool,
  is_vec: bool,
}

#[derive(Clone, Copy)]
struct TraversalOptions {
  type_name_mode: TypeNameMode,
  occurs_state: OccursState,
}

#[derive(Clone)]
struct DeferredInlineElement {
  type_name: String,
  element: Element,
}

pub fn gen_schemas(skd_data_context: &SdkDataContext) -> anyhow::Result<Vec<SdkDataSchema>> {
  let mut schemas = vec![];

  for xsd_schema in &skd_data_context.schemas {
    schemas.push(gen_schema(skd_data_context, xsd_schema)?);
  }

  annotate_codegen_types(&mut schemas);

  Ok(schemas)
}

pub fn annotate_codegen_types(sdk_data_schemas: &mut [SdkDataSchema]) {
  let schemas_snapshot = sdk_data_schemas.to_vec();

  for sdk_data_schema in sdk_data_schemas {
    annotate_schema_codegen_types(&schemas_snapshot, sdk_data_schema);
  }
}

pub fn gen_schema(
  skd_data_context: &SdkDataContext,
  xsd_schema: &XsdSchema,
) -> anyhow::Result<SdkDataSchema> {
  let mut sdk_data_schema = SdkDataSchema {
    target_namespace: xsd_schema
      .target_namespace
      .clone()
      .context("target_namespace")?,
    types: vec![],
    elements: vec![],
    enums: vec![],
    aliases: vec![],
    module_name: xsd_schema.module_name.clone(),
  };

  for content in xsd_schema.contents.iter() {
    match content {
      SchemaContentChoice::Element(e) => {
        gen_element(
          skd_data_context,
          xsd_schema,
          e,
          element_decl_name(e)?.to_string(),
          &mut sdk_data_schema,
        )?;
      }
      SchemaContentChoice::ComplexType(ct) => {
        gen_complex_type(skd_data_context, xsd_schema, ct, &mut sdk_data_schema)?;
      }
      SchemaContentChoice::SimpleType(st) => {
        gen_simple_type(skd_data_context, xsd_schema, st, &mut sdk_data_schema)?;
      }
      _ => (),
    }
  }

  normalize_schema_type_refs(skd_data_context, xsd_schema, &mut sdk_data_schema);

  Ok(sdk_data_schema)
}

fn resolve_inline_type_name(
  skd_data_context: &SdkDataContext,
  xsd_schema: &XsdSchema,
  sdk_data_schema: &SdkDataSchema,
  parent_name: &str,
  child_name: &str,
) -> String {
  let candidate = child_name.to_string();

  if has_type_name_conflict(skd_data_context, xsd_schema, sdk_data_schema, &candidate) {
    format!("{parent_name}{child_name}")
  } else {
    candidate
  }
}

fn new_sdk_data_schema(xsd_schema: &XsdSchema) -> SdkDataSchema {
  SdkDataSchema {
    target_namespace: xsd_schema.target_namespace.clone().unwrap_or_default(),
    types: vec![],
    elements: vec![],
    enums: vec![],
    aliases: vec![],
    module_name: xsd_schema.module_name.clone(),
  }
}

fn push_element_def(
  sdk_data_schema: &mut SdkDataSchema,
  name: String,
  xml_name: String,
  type_name: String,
  is_top_level: bool,
) {
  let ident = name.to_upper_camel_case();

  if sdk_data_schema.elements.iter().any(|element| {
    element.name == name
      && element.xml_name == xml_name
      && element.r#type == type_name
      && element.is_top_level == is_top_level
  }) {
    return;
  }

  sdk_data_schema.elements.push(SdkDataElement {
    name,
    ident,
    xml_name,
    r#type: type_name,
    resolved_type: String::new(),
    type_kind: CodegenTypeKind::String,
    is_struct: false,
    is_public_wrapper: false,
    api_kind: ElementApiKind::None,
    is_top_level,
  });
}

fn has_type_name_conflict(
  skd_data_context: &SdkDataContext,
  xsd_schema: &XsdSchema,
  sdk_data_schema: &SdkDataSchema,
  type_name: &str,
) -> bool {
  matches!(
    skd_data_context.type_name_module_name_map.get(type_name),
    Some(module_name) if module_name == &xsd_schema.module_name
  ) || schema_types(sdk_data_schema)
    .iter()
    .any(|s| s.name == type_name)
    || sdk_data_schema.enums.iter().any(|e| e.name == type_name)
    || sdk_data_schema.aliases.iter().any(|a| a.name == type_name)
}

fn element_decl_name(element: &Element) -> anyhow::Result<&str> {
  element.name.as_deref().context("name")
}

fn resolve_particle_element_name(element: &Element) -> anyhow::Result<String> {
  Ok(element_decl_name(element)?.to_string())
}

fn gen_inline_element(
  skd_data_context: &SdkDataContext,
  xsd_schema: &XsdSchema,
  sdk_data_schema: &mut SdkDataSchema,
  parent_name: &str,
  element: &Element,
) -> anyhow::Result<String> {
  let child_name = element_decl_name(element)?.to_string();
  let candidate = child_name.clone();

  if let Some(existing_struct) = schema_types(sdk_data_schema)
    .iter()
    .find(|s| s.name == candidate)
  {
    let mut trial_schema = new_sdk_data_schema(xsd_schema);
    gen_element(
      skd_data_context,
      xsd_schema,
      element,
      candidate.clone(),
      &mut trial_schema,
    )?;

    if schema_types(&trial_schema).first() == Some(existing_struct) {
      return Ok(candidate);
    }

    let fallback = format!("{parent_name}{child_name}");
    gen_element(
      skd_data_context,
      xsd_schema,
      element,
      fallback.clone(),
      sdk_data_schema,
    )?;

    return Ok(fallback);
  }

  let inline_type_name = resolve_inline_type_name(
    skd_data_context,
    xsd_schema,
    sdk_data_schema,
    parent_name,
    &child_name,
  );

  gen_element(
    skd_data_context,
    xsd_schema,
    element,
    inline_type_name.clone(),
    sdk_data_schema,
  )?;

  Ok(inline_type_name)
}

fn get_element_extension_base(element: &Element) -> Option<&str> {
  if let Some(ElementContentChoice::ComplexType(ct)) = &element.contents {
    for ct_content in &ct.contents {
      if let ComplexTypeContentChoice::ComplexContent(cc) = ct_content
        && let Some(ComplexContentContentChoice::Extension(ext)) = &cc.contents
      {
        return Some(ext.base.as_str());
      }
    }
  }

  None
}

fn resolve_element_type(
  skd_data_context: &SdkDataContext,
  xsd_schema: &XsdSchema,
  sdk_data_schema: &mut SdkDataSchema,
  parent_name: &str,
  element: &Element,
  type_name_mode: TypeNameMode,
  deferred_inline_elements: &mut Vec<DeferredInlineElement>,
) -> anyhow::Result<String> {
  if let Some(t) = &element.r#type {
    Ok(resolve_xsd_type(skd_data_context, xsd_schema, t))
  } else {
    let inline_type_name = if get_element_extension_base(element) == Some(parent_name) {
      let child_name = element_decl_name(element)?.to_string();
      let inline_type_name = resolve_inline_type_name(
        skd_data_context,
        xsd_schema,
        sdk_data_schema,
        parent_name,
        &child_name,
      );

      deferred_inline_elements.push(DeferredInlineElement {
        type_name: inline_type_name.clone(),
        element: element.clone(),
      });

      inline_type_name
    } else {
      gen_inline_element(
        skd_data_context,
        xsd_schema,
        sdk_data_schema,
        parent_name,
        element,
      )?
    };

    match type_name_mode {
      TypeNameMode::PreserveInlineName => Ok(inline_type_name),
      TypeNameMode::CamelCaseInlineName => Ok(inline_type_name.to_upper_camel_case()),
    }
  }
}

fn push_sequence_element(
  sdk_data_schema: &mut SdkDataSchema,
  sdk_data_struct: &mut Struct,
  e_name: String,
  e_type: String,
  is_option: bool,
  is_vec: bool,
) -> anyhow::Result<()> {
  push_element_def(
    sdk_data_schema,
    e_name.clone(),
    e_name.clone(),
    e_type.clone(),
    false,
  );

  insert_sequence_child(
    sdk_data_schema,
    sdk_data_struct,
    Child {
      name: e_name.clone(),
      xml_name: Some(e_name.clone()),
      resolved_xml_name: String::new(),
      ident: e_name.to_snake_case(),
      r#type: e_type,
      resolved_type: String::new(),
      type_kind: CodegenTypeKind::String,
      is_struct: false,
      source_module_name: Some(sdk_data_struct.module_name.clone()),
      is_option,
      is_vec,
    },
  )
}

fn push_choice_element(
  sdk_data_schema: &mut SdkDataSchema,
  sdk_data_struct: &mut Struct,
  e_name: String,
  e_type: String,
) -> anyhow::Result<()> {
  push_element_def(
    sdk_data_schema,
    e_name.clone(),
    e_name.clone(),
    e_type.clone(),
    false,
  );

  insert_choice_child(
    sdk_data_schema,
    sdk_data_struct,
    Child {
      name: e_name.clone(),
      xml_name: Some(e_name.clone()),
      resolved_xml_name: String::new(),
      ident: e_name.to_upper_camel_case(),
      r#type: e_type,
      resolved_type: String::new(),
      type_kind: CodegenTypeKind::String,
      is_struct: false,
      source_module_name: Some(sdk_data_struct.module_name.clone()),
      is_option: false,
      is_vec: false,
    },
  )
}

fn next_occurs_state(
  occurs_state: OccursState,
  min_occurs: Option<&str>,
  max_occurs: Option<&str>,
) -> OccursState {
  OccursState {
    is_option: occurs_state.is_option || (max_occurs.is_none() && min_occurs == Some("0")),
    is_vec: occurs_state.is_vec || max_occurs.is_some(),
  }
}

fn element_occurs_state(occurs_state: OccursState, element: &Element) -> OccursState {
  next_occurs_state(
    occurs_state,
    element.min_occurs.as_deref(),
    element.max_occurs.as_deref(),
  )
}

fn choice_branch_occurs_state(occurs_state: OccursState) -> OccursState {
  OccursState {
    is_option: true,
    is_vec: occurs_state.is_vec,
  }
}

fn gen_extension(
  skd_data_context: &SdkDataContext,
  xsd_schema: &XsdSchema,
  sdk_data_struct: &mut Struct,
  sdk_data_schema: &mut SdkDataSchema,
  ext: &Extension,
  type_name_mode: TypeNameMode,
  deferred_inline_elements: &mut Vec<DeferredInlineElement>,
) -> anyhow::Result<()> {
  apply_extension_base(
    skd_data_context,
    sdk_data_struct,
    sdk_data_schema,
    &ext.base,
  )?;

  for ext_content in &ext.contents {
    gen_extension_content(
      skd_data_context,
      xsd_schema,
      sdk_data_struct,
      sdk_data_schema,
      ext_content,
      type_name_mode,
      deferred_inline_elements,
    )?;
  }

  Ok(())
}

fn fill_struct_from_complex_type(
  skd_data_context: &SdkDataContext,
  xsd_schema: &XsdSchema,
  ct: &ComplexType,
  sdk_data_struct: &mut Struct,
  sdk_data_schema: &mut SdkDataSchema,
  type_name_mode: TypeNameMode,
  deferred_inline_elements: &mut Vec<DeferredInlineElement>,
) -> anyhow::Result<()> {
  for ct_content in &ct.contents {
    if gen_complex_type_particle(
      skd_data_context,
      xsd_schema,
      sdk_data_struct,
      sdk_data_schema,
      ct_content,
      type_name_mode,
      deferred_inline_elements,
    )? {
      continue;
    }

    match ct_content {
      ComplexTypeContentChoice::SimpleContent(sc) => {
        if let Some(SimpleContentContentChoice::Extension(ext)) = &sc.contents {
          gen_simple_content_extension(
            skd_data_context,
            xsd_schema,
            sdk_data_struct,
            sdk_data_schema,
            ext,
          )?;
        }
      }
      ComplexTypeContentChoice::ComplexContent(cc) => {
        if let Some(ComplexContentContentChoice::Extension(ext)) = &cc.contents {
          gen_extension(
            skd_data_context,
            xsd_schema,
            sdk_data_struct,
            sdk_data_schema,
            ext,
            type_name_mode,
            deferred_inline_elements,
          )?;
        }
      }
      _ => {}
    }
  }

  Ok(())
}

pub fn gen_element(
  skd_data_context: &SdkDataContext,
  xsd_schema: &XsdSchema,
  e: &Element,
  e_name: String,
  sdk_data_schema: &mut SdkDataSchema,
) -> anyhow::Result<()> {
  let xml_name = element_decl_name(e)?.to_string();
  let is_top_level = xsd_schema.contents.iter().any(|content| match content {
    SchemaContentChoice::Element(top_level_element) => {
      top_level_element.name.as_deref() == Some(xml_name.as_str())
    }
    _ => false,
  });

  if let Some(ElementContentChoice::SimpleType(st)) = &e.contents
    && let Some(SimpleTypeContentChoice::Restriction(r)) = &st.contents
  {
    if r.contents.is_empty() {
      let alias_ident = e_name.to_upper_camel_case();
      sdk_data_schema.aliases.push(Alias {
        name: e_name.clone(),
        ident: alias_ident.clone(),
        r#type: resolve_xsd_type(skd_data_context, xsd_schema, &r.base),
        module_name: xsd_schema.module_name.clone(),
      });
      push_element_def(sdk_data_schema, e_name, xml_name, alias_ident, is_top_level);
    } else {
      gen_enum(xsd_schema, &e_name, r, sdk_data_schema)?;
      push_element_def(
        sdk_data_schema,
        e_name.clone(),
        xml_name,
        e_name.to_upper_camel_case(),
        is_top_level,
      );
    }

    return Ok(());
  }

  let ident = e_name.to_upper_camel_case();

  let mut sdk_data_struct = Struct {
    name: e_name.clone(),
    ident,
    xml_name: Some(xml_name.clone()),
    resolved_xml_name: String::new(),
    attributes: vec![],
    sequences: vec![],
    children: vec![],
    module_name: xsd_schema.module_name.clone(),
  };
  let mut deferred_inline_elements = vec![];

  if let Some(ElementContentChoice::ComplexType(ct)) = &e.contents {
    fill_struct_from_complex_type(
      skd_data_context,
      xsd_schema,
      ct,
      &mut sdk_data_struct,
      sdk_data_schema,
      TypeNameMode::PreserveInlineName,
      &mut deferred_inline_elements,
    )?;
  }

  sdk_data_schema.types.push(sdk_data_struct);
  push_element_def(
    sdk_data_schema,
    e_name,
    xml_name,
    sdk_data_schema
      .types
      .last()
      .map(|s| s.ident.clone())
      .expect("element struct just pushed"),
    is_top_level,
  );

  for deferred_inline_element in deferred_inline_elements {
    gen_element(
      skd_data_context,
      xsd_schema,
      &deferred_inline_element.element,
      deferred_inline_element.type_name,
      sdk_data_schema,
    )?;
  }

  Ok(())
}

pub fn gen_simple_type(
  skd_data_context: &SdkDataContext,
  xsd_schema: &XsdSchema,
  st: &SimpleType,
  sdk_data_schema: &mut SdkDataSchema,
) -> anyhow::Result<()> {
  let name = st.name.clone().context("name")?;
  let ident = name.to_upper_camel_case();

  if let Some(SimpleTypeContentChoice::Restriction(r)) = &st.contents
    && r.contents.is_empty()
  {
    sdk_data_schema.aliases.push(Alias {
      name,
      ident,
      r#type: resolve_xsd_type(skd_data_context, xsd_schema, &r.base),
      module_name: xsd_schema.module_name.clone(),
    });
  }

  Ok(())
}

pub fn gen_complex_type(
  skd_data_context: &SdkDataContext,
  xsd_schema: &XsdSchema,
  ct: &ComplexType,
  sdk_data_schema: &mut SdkDataSchema,
) -> anyhow::Result<()> {
  let name = ct.name.clone().context("name")?;
  let ident = name.to_upper_camel_case();

  let mut sdk_data_struct = Struct {
    name,
    ident,
    xml_name: None,
    resolved_xml_name: String::new(),
    attributes: vec![],
    sequences: vec![],
    children: vec![],
    module_name: xsd_schema.module_name.clone(),
  };
  let mut deferred_inline_elements = vec![];

  fill_struct_from_complex_type(
    skd_data_context,
    xsd_schema,
    ct,
    &mut sdk_data_struct,
    sdk_data_schema,
    TypeNameMode::CamelCaseInlineName,
    &mut deferred_inline_elements,
  )?;

  sdk_data_schema.types.push(sdk_data_struct);

  for deferred_inline_element in deferred_inline_elements {
    gen_element(
      skd_data_context,
      xsd_schema,
      &deferred_inline_element.element,
      deferred_inline_element.type_name,
      sdk_data_schema,
    )?;
  }

  Ok(())
}

fn schema_types(sdk_data_schema: &SdkDataSchema) -> &[Struct] {
  &sdk_data_schema.types
}

fn schema_types_mut(sdk_data_schema: &mut SdkDataSchema) -> &mut Vec<Struct> {
  &mut sdk_data_schema.types
}

fn split_schema_type_path(type_name: &str) -> Option<(&str, &str)> {
  let prefix = "crate::schemas::";

  if !type_name.starts_with(prefix) {
    return None;
  }

  let rest = &type_name[prefix.len()..];
  let (module_name, ident) = rest.split_once("::")?;

  Some((module_name, ident))
}

pub fn gen_attribute(
  skd_data_context: &SdkDataContext,
  xsd_schema: &XsdSchema,
  attr: &Attribute,
  sdk_data_struct: &mut Struct,
  sdk_data_schema: &mut SdkDataSchema,
) -> anyhow::Result<()> {
  let name = attr.name.clone().context("name")?;
  let ident = name.to_snake_case();

  if let Some(a_t) = &attr.r#type {
    insert_attribute(
      sdk_data_schema,
      sdk_data_struct,
      SdkDataAttribute {
        name,
        ident,
        r#type: resolve_xsd_type(skd_data_context, xsd_schema, a_t),
        resolved_type: String::new(),
        type_kind: CodegenTypeKind::String,
        is_option: !matches!(attr.r#use.as_deref(), Some("required")),
      },
    )?;
  } else if let Some(a_st) = &attr.contents
    && let Some(SimpleTypeContentChoice::Restriction(r)) = &a_st.contents
  {
    let enum_name = format!("{}{}", sdk_data_struct.ident, ident.to_upper_camel_case());

    gen_enum(xsd_schema, &enum_name, r, sdk_data_schema)?;

    insert_attribute(
      sdk_data_schema,
      sdk_data_struct,
      SdkDataAttribute {
        name,
        ident,
        r#type: enum_name,
        resolved_type: String::new(),
        type_kind: CodegenTypeKind::String,
        is_option: !matches!(attr.r#use.as_deref(), Some("required")),
      },
    )?;
  }

  Ok(())
}

pub fn gen_enum(
  xsd_schema: &XsdSchema,
  e_name: &str,
  r: &Restriction,
  sdk_data_schema: &mut SdkDataSchema,
) -> anyhow::Result<()> {
  let mut sdk_data_enum = Enum {
    name: e_name.to_string(),
    ident: e_name.to_upper_camel_case(),
    variants: vec![],
    default_index: 0,
    module_name: xsd_schema.module_name.clone(),
  };

  for c in &r.contents {
    if let RestrictionContentChoice::Enumeration(e) = c {
      let ident = if is_valid_simple_ident(&e.value) {
        e.value.to_upper_camel_case()
      } else {
        format!("_{}", e.value.to_upper_camel_case())
      };

      sdk_data_enum.variants.push(Variant {
        ident,
        value: e.value.clone(),
      });
    }
  }

  sdk_data_schema.enums.push(sdk_data_enum);

  Ok(())
}

pub fn match_xsd_simple_type(xsd_simple_type: &str) -> Option<String> {
  match xsd_simple_type {
    "xs:unsignedInt" => Some("u32".to_string()),
    "xs:int" => Some("i32".to_string()),
    "xs:anyURI" | "xs:anyType" | "xs:string" | "xs:ID" | "xs:date" | "xs:dateTime"
    | "xs:base64Binary" | "xs:IDREF" => Some("String".to_string()),
    "xs:boolean" => Some("bool".to_string()),
    "xs:double" => Some("f64".to_string()),
    _ => None,
  }
}

pub fn resolve_xsd_type(
  skd_data_context: &SdkDataContext,
  xsd_schema: &XsdSchema,
  xsd_type: &str,
) -> String {
  if let Some(rust_type) = match_xsd_simple_type(xsd_type) {
    return rust_type;
  }

  let type_name = xsd_type.split_once(':').map_or_else(
    || xsd_type.to_string(),
    |(_, local_name)| local_name.to_string(),
  );
  let ident = type_name.to_upper_camel_case();

  match skd_data_context
    .type_name_module_name_map
    .get(type_name.as_str())
  {
    Some(module_name) if module_name != &xsd_schema.module_name => {
      format!("crate::schemas::{}::{}", module_name, ident)
    }
    _ => ident,
  }
}

pub fn is_valid_simple_ident(s: &str) -> bool {
  match s.chars().next() {
    None => return false,
    Some(c) if c == '_' || c.is_ascii_alphabetic() => (),
    Some(_) => return false,
  }

  s.chars()
    .skip(1)
    .all(|c| c.is_ascii_alphanumeric() || c == '_')
}
