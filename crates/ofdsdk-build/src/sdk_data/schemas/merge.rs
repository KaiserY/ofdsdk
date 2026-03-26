use super::*;

fn find_complex_type<'a>(
  skd_data_context: &'a SdkDataContext,
  type_name: &str,
) -> Option<(&'a XsdSchema, &'a ComplexType, usize)> {
  let local_name = type_name
    .split_once(':')
    .map_or(type_name, |(_, local_name)| local_name);
  let (schema_index, content_index) = *skd_data_context.complex_type_name_map.get(local_name)?;
  let xsd_schema = skd_data_context.schemas.get(schema_index)?;

  match xsd_schema.contents.get(content_index)? {
    SchemaContentChoice::ComplexType(ct) => Some((xsd_schema, ct, content_index)),
    _ => None,
  }
}

fn find_struct_by_name<'a>(sdk_data_schema: &'a SdkDataSchema, name: &str) -> Option<&'a Struct> {
  schema_types(sdk_data_schema)
    .iter()
    .find(|s| s.name == name)
}

fn element_has_extension_base(element: &Element, base_name: &str) -> bool {
  if let Some(ElementContentChoice::ComplexType(ct)) = &element.contents {
    for ct_content in &ct.contents {
      if let ComplexTypeContentChoice::ComplexContent(cc) = ct_content
        && let Some(ComplexContentContentChoice::Extension(ext)) = &cc.contents
        && ext.base == base_name
      {
        return true;
      }
    }
  }

  false
}

fn has_recursive_self_extension(ct: &ComplexType, ct_name: &str) -> bool {
  for ct_content in &ct.contents {
    match ct_content {
      ComplexTypeContentChoice::Sequence(seq) => {
        for seq_content in &seq.contents {
          match seq_content {
            SequenceContentChoice::Element(element) => {
              if element_has_extension_base(element, ct_name) {
                return true;
              }
            }
            SequenceContentChoice::Choice(choice) => {
              for choice_content in &choice.contents {
                if let ChoiceContentChoice::Element(element) = choice_content
                  && element_has_extension_base(element, ct_name)
                {
                  return true;
                }
              }
            }
            _ => (),
          }
        }
      }
      ComplexTypeContentChoice::Choice(choice) => {
        for choice_content in &choice.contents {
          if let ChoiceContentChoice::Element(element) = choice_content
            && element_has_extension_base(element, ct_name)
          {
            return true;
          }
        }
      }
      _ => (),
    }
  }

  false
}

fn qualify_merged_type(
  type_name: &str,
  src_schema: &SdkDataSchema,
  dst_module_name: &str,
) -> String {
  if src_schema.module_name == dst_module_name || type_name.contains("::") {
    return type_name.to_string();
  }

  if src_schema.aliases.iter().any(|a| a.ident == type_name)
    || src_schema.enums.iter().any(|e| e.ident == type_name)
    || schema_types(src_schema)
      .iter()
      .any(|s| s.ident == type_name)
  {
    return format!("crate::schemas::{}::{}", src_schema.module_name, type_name);
  }

  type_name.to_string()
}

fn merge_type_name_with_schema(
  src_schema: &SdkDataSchema,
  dst_module_name: &str,
  type_name: &str,
) -> String {
  qualify_merged_type(type_name, src_schema, dst_module_name)
}

fn canonical_compare_type(
  schema: &SdkDataSchema,
  peer_schema: Option<&SdkDataSchema>,
  type_name: &str,
) -> String {
  match type_name {
    "String" | "bool" | "f64" | "i32" | "u32" => return type_name.to_string(),
    _ => {}
  }

  if let Some((module_name, ident)) = split_schema_type_path(type_name) {
    if schema.module_name == module_name {
      return canonical_compare_type(schema, peer_schema, ident);
    }

    if let Some(peer_schema) = peer_schema
      && peer_schema.module_name == module_name
    {
      return canonical_compare_type(peer_schema, Some(schema), ident);
    }

    return type_name.to_string();
  }

  if let Some(alias) = schema.aliases.iter().find(|alias| alias.ident == type_name) {
    return canonical_compare_type(schema, peer_schema, &alias.r#type);
  }

  if schema.enums.iter().any(|e| e.ident == type_name)
    || schema_types(schema).iter().any(|s| s.ident == type_name)
  {
    return format!("crate::schemas::{}::{}", schema.module_name, type_name);
  }

  type_name.to_string()
}

fn same_attribute_identity(
  dst_schema: &SdkDataSchema,
  lhs: &SdkDataAttribute,
  rhs: &SdkDataAttribute,
  src_schema: &SdkDataSchema,
  dst_module_name: &str,
) -> bool {
  lhs.name == rhs.name
    && lhs.ident == rhs.ident
    && canonical_compare_type(dst_schema, Some(src_schema), &lhs.r#type)
      == canonical_compare_type(
        dst_schema,
        Some(src_schema),
        &merge_type_name_with_schema(src_schema, dst_module_name, &rhs.r#type),
      )
}

fn same_child_identity(
  dst_schema: &SdkDataSchema,
  lhs: &Child,
  rhs: &Child,
  src_schema: &SdkDataSchema,
  dst_module_name: &str,
) -> bool {
  let rhs_source_module_name = rhs
    .source_module_name
    .clone()
    .unwrap_or_else(|| src_schema.module_name.clone());

  lhs.name == rhs.name
    && lhs.xml_name == rhs.xml_name
    && lhs.ident == rhs.ident
    && canonical_compare_type(dst_schema, Some(src_schema), &lhs.r#type)
      == canonical_compare_type(
        dst_schema,
        Some(src_schema),
        &merge_type_name_with_schema(src_schema, dst_module_name, &rhs.r#type),
      )
    && lhs.source_module_name.as_deref().unwrap_or(dst_module_name) == rhs_source_module_name
}

fn same_local_attribute_identity(
  sdk_data_schema: &SdkDataSchema,
  lhs: &SdkDataAttribute,
  rhs: &SdkDataAttribute,
) -> bool {
  lhs.name == rhs.name
    && lhs.ident == rhs.ident
    && canonical_compare_type(sdk_data_schema, None, &lhs.r#type)
      == canonical_compare_type(sdk_data_schema, None, &rhs.r#type)
}

fn same_local_child_identity(
  sdk_data_schema: &SdkDataSchema,
  lhs: &Child,
  rhs: &Child,
  owner_module_name: &str,
) -> bool {
  let lhs_source_module_name = lhs
    .source_module_name
    .as_deref()
    .unwrap_or(owner_module_name);
  let rhs_source_module_name = rhs
    .source_module_name
    .as_deref()
    .unwrap_or(owner_module_name);

  lhs.name == rhs.name
    && lhs.xml_name == rhs.xml_name
    && lhs.ident == rhs.ident
    && canonical_compare_type(sdk_data_schema, None, &lhs.r#type)
      == canonical_compare_type(sdk_data_schema, None, &rhs.r#type)
    && lhs_source_module_name == rhs_source_module_name
}

pub(super) fn insert_attribute(
  sdk_data_schema: &SdkDataSchema,
  sdk_data_struct: &mut Struct,
  attr: SdkDataAttribute,
) -> anyhow::Result<()> {
  if let Some(existing_index) = sdk_data_struct
    .attributes
    .iter()
    .position(|existing| existing.ident == attr.ident)
  {
    if !same_local_attribute_identity(
      sdk_data_schema,
      &sdk_data_struct.attributes[existing_index],
      &attr,
    ) {
      anyhow::bail!(
        "conflicting attribute insert for struct `{}` field `{}`",
        sdk_data_struct.ident,
        attr.ident
      );
    }

    sdk_data_struct.attributes[existing_index].is_option |= attr.is_option;
    return Ok(());
  }

  sdk_data_struct.attributes.push(attr);
  Ok(())
}

pub(super) fn insert_sequence_child(
  sdk_data_schema: &SdkDataSchema,
  sdk_data_struct: &mut Struct,
  child: Child,
) -> anyhow::Result<()> {
  if let Some(existing_index) = sdk_data_struct
    .sequences
    .iter()
    .position(|existing| existing.ident == child.ident)
  {
    if !same_local_child_identity(
      sdk_data_schema,
      &sdk_data_struct.sequences[existing_index],
      &child,
      &sdk_data_struct.module_name,
    ) {
      anyhow::bail!(
        "conflicting sequence insert for struct `{}` field `{}`",
        sdk_data_struct.ident,
        child.ident
      );
    }

    sdk_data_struct.sequences[existing_index].is_option |= child.is_option;
    sdk_data_struct.sequences[existing_index].is_vec |= child.is_vec;
    return Ok(());
  }

  sdk_data_struct.sequences.push(child);
  Ok(())
}

pub(super) fn insert_choice_child(
  sdk_data_schema: &SdkDataSchema,
  sdk_data_struct: &mut Struct,
  child: Child,
) -> anyhow::Result<()> {
  if let Some(existing_index) = sdk_data_struct
    .children
    .iter()
    .position(|existing| existing.ident == child.ident)
  {
    if !same_local_child_identity(
      sdk_data_schema,
      &sdk_data_struct.children[existing_index],
      &child,
      &sdk_data_struct.module_name,
    ) {
      anyhow::bail!(
        "conflicting choice insert for struct `{}` field `{}`",
        sdk_data_struct.ident,
        child.ident
      );
    }

    sdk_data_struct.children[existing_index].is_option |= child.is_option;
    sdk_data_struct.children[existing_index].is_vec |= child.is_vec;
    return Ok(());
  }

  sdk_data_struct.children.push(child);
  Ok(())
}

fn merge_struct_fields(
  dst_schema: &SdkDataSchema,
  dst: &mut Struct,
  src: &Struct,
  src_schema: &SdkDataSchema,
  dst_module_name: &str,
) -> anyhow::Result<()> {
  for mut attr in src.attributes.iter().cloned() {
    if let Some(existing_index) = dst
      .attributes
      .iter()
      .position(|existing| existing.ident == attr.ident)
    {
      if !same_attribute_identity(
        dst_schema,
        &dst.attributes[existing_index],
        &attr,
        src_schema,
        dst_module_name,
      ) {
        anyhow::bail!(
          "conflicting attribute merge for struct `{}` field `{}`",
          dst.ident,
          attr.ident
        );
      }

      dst.attributes[existing_index].is_option |= attr.is_option;
      continue;
    }

    attr.r#type = merge_type_name_with_schema(src_schema, dst_module_name, &attr.r#type);
    dst.attributes.push(attr);
  }

  for mut child in src.sequences.iter().cloned() {
    if let Some(existing_index) = dst
      .sequences
      .iter()
      .position(|existing| existing.ident == child.ident)
    {
      if !same_child_identity(
        dst_schema,
        &dst.sequences[existing_index],
        &child,
        src_schema,
        dst_module_name,
      ) {
        anyhow::bail!(
          "conflicting sequence merge for struct `{}` field `{}`",
          dst.ident,
          child.ident
        );
      }

      dst.sequences[existing_index].is_option |= child.is_option;
      dst.sequences[existing_index].is_vec |= child.is_vec;
      continue;
    }

    child.r#type = merge_type_name_with_schema(src_schema, dst_module_name, &child.r#type);
    child.source_module_name = Some(
      child
        .source_module_name
        .clone()
        .unwrap_or_else(|| src_schema.module_name.clone()),
    );
    dst.sequences.push(child);
  }

  for mut child in src.children.iter().cloned() {
    if let Some(existing_index) = dst
      .children
      .iter()
      .position(|existing| existing.ident == child.ident)
    {
      if !same_child_identity(
        dst_schema,
        &dst.children[existing_index],
        &child,
        src_schema,
        dst_module_name,
      ) {
        anyhow::bail!(
          "conflicting choice merge for struct `{}` field `{}`",
          dst.ident,
          child.ident
        );
      }

      dst.children[existing_index].is_option |= child.is_option;
      dst.children[existing_index].is_vec |= child.is_vec;
      continue;
    }

    child.r#type = merge_type_name_with_schema(src_schema, dst_module_name, &child.r#type);
    child.source_module_name = Some(
      child
        .source_module_name
        .clone()
        .unwrap_or_else(|| src_schema.module_name.clone()),
    );
    dst.children.push(child);
  }

  Ok(())
}

fn build_base_sdk_data_schema(
  skd_data_context: &SdkDataContext,
  base_schema: &XsdSchema,
  base_ct: &ComplexType,
  base_content_index: usize,
  base_name: &str,
) -> anyhow::Result<SdkDataSchema> {
  let mut base_sdk_data_schema = new_sdk_data_schema(base_schema);

  if has_recursive_self_extension(base_ct, base_name) {
    for content in base_schema.contents.iter().take(base_content_index) {
      match content {
        SchemaContentChoice::ComplexType(ct) => {
          gen_complex_type(skd_data_context, base_schema, ct, &mut base_sdk_data_schema)?;
        }
        SchemaContentChoice::SimpleType(st) => {
          gen_simple_type(skd_data_context, base_schema, st, &mut base_sdk_data_schema)?;
        }
        _ => (),
      }
    }
  }

  gen_complex_type(
    skd_data_context,
    base_schema,
    base_ct,
    &mut base_sdk_data_schema,
  )?;

  Ok(base_sdk_data_schema)
}

fn resolve_base_struct(
  skd_data_context: &SdkDataContext,
  current_schema: &SdkDataSchema,
  base_type_name: &str,
) -> anyhow::Result<Option<(SdkDataSchema, Struct)>> {
  let base_name = base_type_name
    .split_once(':')
    .map_or(base_type_name, |(_, local_name)| local_name);

  if let Some(base_struct) = find_struct_by_name(current_schema, base_name) {
    return Ok(Some((current_schema.clone(), base_struct.clone())));
  }

  let Some((base_schema, base_ct, base_content_index)) =
    find_complex_type(skd_data_context, base_type_name)
  else {
    return Ok(None);
  };

  let base_sdk_data_schema = build_base_sdk_data_schema(
    skd_data_context,
    base_schema,
    base_ct,
    base_content_index,
    base_name,
  )?;

  Ok(
    find_struct_by_name(&base_sdk_data_schema, base_name)
      .cloned()
      .map(|base_struct| (base_sdk_data_schema, base_struct)),
  )
}

fn merge_base_struct(
  current_schema: &SdkDataSchema,
  sdk_data_struct: &mut Struct,
  base_schema: &SdkDataSchema,
  base_struct: &Struct,
) -> anyhow::Result<()> {
  let dst_module_name = sdk_data_struct.module_name.clone();
  merge_struct_fields(
    current_schema,
    sdk_data_struct,
    base_struct,
    base_schema,
    &dst_module_name,
  )
}

pub(super) fn apply_extension_base(
  skd_data_context: &SdkDataContext,
  sdk_data_struct: &mut Struct,
  sdk_data_schema: &SdkDataSchema,
  base_type_name: &str,
) -> anyhow::Result<()> {
  let base_name = base_type_name
    .split_once(':')
    .map_or(base_type_name, |(_, local_name)| local_name);

  if base_name == sdk_data_struct.name {
    return Ok(());
  }

  if let Some((base_schema, base_struct)) =
    resolve_base_struct(skd_data_context, sdk_data_schema, base_type_name)?
  {
    merge_base_struct(sdk_data_schema, sdk_data_struct, &base_schema, &base_struct)?;
  }

  Ok(())
}
