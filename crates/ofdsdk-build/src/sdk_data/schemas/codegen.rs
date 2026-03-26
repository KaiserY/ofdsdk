use super::*;

struct CodegenTypeMeta {
  resolved_type: String,
  type_kind: CodegenTypeKind,
  is_struct: bool,
}

pub(super) fn annotate_schema_codegen_types(
  sdk_data_schemas: &[SdkDataSchema],
  sdk_data_schema: &mut SdkDataSchema,
) {
  let schema_snapshot = sdk_data_schema.clone();

  for element in &mut sdk_data_schema.elements {
    let meta = resolve_codegen_type_meta(&schema_snapshot, sdk_data_schemas, &element.r#type);
    element.resolved_type = meta.resolved_type;
    element.type_kind = meta.type_kind;
    element.is_struct = meta.is_struct;
    element.is_public_wrapper = resolve_is_public_wrapper(&schema_snapshot, element);
    element.api_kind = resolve_element_api_kind(element);
  }

  for sdk_data_struct in schema_types_mut(sdk_data_schema) {
    sdk_data_struct.resolved_xml_name =
      resolve_root_struct_xml_name(&schema_snapshot, sdk_data_struct);
  }

  for sdk_data_struct in schema_types_mut(sdk_data_schema) {
    for attr in &mut sdk_data_struct.attributes {
      let meta = resolve_codegen_type_meta(&schema_snapshot, sdk_data_schemas, &attr.r#type);
      attr.resolved_type = meta.resolved_type;
      attr.type_kind = meta.type_kind;
    }
  }

  for sdk_data_struct in schema_types_mut(sdk_data_schema) {
    for child in &mut sdk_data_struct.sequences {
      let meta = resolve_codegen_type_meta(&schema_snapshot, sdk_data_schemas, &child.r#type);
      child.resolved_xml_name = resolve_child_xml_name(&schema_snapshot, child);
      child.resolved_type = meta.resolved_type;
      child.type_kind = meta.type_kind;
      child.is_struct = meta.is_struct;
    }

    for child in &mut sdk_data_struct.children {
      let meta = resolve_codegen_type_meta(&schema_snapshot, sdk_data_schemas, &child.r#type);
      child.resolved_xml_name = resolve_child_xml_name(&schema_snapshot, child);
      child.resolved_type = meta.resolved_type;
      child.type_kind = meta.type_kind;
      child.is_struct = meta.is_struct;
    }
  }
}

fn resolve_root_struct_xml_name(
  sdk_data_schema: &SdkDataSchema,
  sdk_data_struct: &Struct,
) -> String {
  sdk_data_schema
    .elements
    .iter()
    .find(|element| element.is_top_level && element.r#type == sdk_data_struct.ident)
    .map(|element| element.xml_name.clone())
    .or_else(|| sdk_data_struct.xml_name.clone())
    .unwrap_or_else(|| sdk_data_struct.name.clone())
}

fn resolve_is_public_wrapper(sdk_data_schema: &SdkDataSchema, element: &SdkDataElement) -> bool {
  if schema_types(sdk_data_schema)
    .iter()
    .any(|s| s.ident == element.ident)
    || sdk_data_schema
      .enums
      .iter()
      .any(|e| e.ident == element.ident)
    || sdk_data_schema
      .aliases
      .iter()
      .any(|a| a.ident == element.ident)
  {
    return false;
  }

  element.is_struct
}

fn resolve_element_api_kind(element: &SdkDataElement) -> ElementApiKind {
  if element.r#type == element.ident {
    return ElementApiKind::None;
  }

  match element.type_kind {
    CodegenTypeKind::Enum => ElementApiKind::EnumAlias,
    CodegenTypeKind::Struct if element.is_public_wrapper => ElementApiKind::StructWrapper,
    _ => ElementApiKind::None,
  }
}

fn resolve_child_xml_name(sdk_data_schema: &SdkDataSchema, child: &Child) -> String {
  let child_xml_name = child.xml_name.as_deref().unwrap_or(&child.name);
  let child_type_name = if child.r#type.contains("::") {
    child.r#type.clone()
  } else {
    match child.source_module_name.as_deref() {
      Some(module_name) if module_name != sdk_data_schema.module_name => {
        format!("crate::schemas::{}::{}", module_name, child.r#type)
      }
      _ => child.r#type.clone(),
    }
  };

  sdk_data_schema
    .elements
    .iter()
    .find(|element| {
      !element.is_top_level
        && child
          .source_module_name
          .as_deref()
          .unwrap_or(&sdk_data_schema.module_name)
          == sdk_data_schema.module_name
        && element.xml_name == child_xml_name
        && element.r#type == child_type_name
    })
    .map(|element| element.xml_name.clone())
    .unwrap_or_else(|| child_xml_name.to_string())
}

fn resolve_codegen_type_meta(
  sdk_data_schema: &SdkDataSchema,
  sdk_data_schemas: &[SdkDataSchema],
  type_name: &str,
) -> CodegenTypeMeta {
  match type_name {
    "String" => {
      return CodegenTypeMeta {
        resolved_type: type_name.to_string(),
        type_kind: CodegenTypeKind::String,
        is_struct: false,
      };
    }
    "bool" => {
      return CodegenTypeMeta {
        resolved_type: type_name.to_string(),
        type_kind: CodegenTypeKind::Bool,
        is_struct: false,
      };
    }
    "f64" => {
      return CodegenTypeMeta {
        resolved_type: type_name.to_string(),
        type_kind: CodegenTypeKind::F64,
        is_struct: false,
      };
    }
    "i32" => {
      return CodegenTypeMeta {
        resolved_type: type_name.to_string(),
        type_kind: CodegenTypeKind::I32,
        is_struct: false,
      };
    }
    "u32" => {
      return CodegenTypeMeta {
        resolved_type: type_name.to_string(),
        type_kind: CodegenTypeKind::U32,
        is_struct: false,
      };
    }
    _ => {}
  }

  if let Some(alias) = sdk_data_schema
    .aliases
    .iter()
    .find(|alias| alias.ident == type_name)
  {
    return resolve_codegen_type_meta(sdk_data_schema, sdk_data_schemas, &alias.r#type);
  }

  if sdk_data_schema.enums.iter().any(|e| e.ident == type_name) {
    return CodegenTypeMeta {
      resolved_type: format!(
        "crate::schemas::{}::{}",
        sdk_data_schema.module_name, type_name
      ),
      type_kind: CodegenTypeKind::Enum,
      is_struct: false,
    };
  }

  if schema_types(sdk_data_schema)
    .iter()
    .any(|s| s.ident == type_name)
  {
    return CodegenTypeMeta {
      resolved_type: format!(
        "crate::schemas::{}::{}",
        sdk_data_schema.module_name, type_name
      ),
      type_kind: CodegenTypeKind::Struct,
      is_struct: true,
    };
  }

  if let Some((module_name, ident)) = split_schema_type_path(type_name) {
    let referenced_schema = sdk_data_schemas
      .iter()
      .find(|schema| schema.module_name == module_name)
      .unwrap_or_else(|| panic!("schema module `{module_name}` not found"));

    if let Some(alias) = referenced_schema
      .aliases
      .iter()
      .find(|alias| alias.ident == ident)
    {
      return resolve_codegen_type_meta(referenced_schema, sdk_data_schemas, &alias.r#type);
    }

    if referenced_schema.enums.iter().any(|e| e.ident == ident) {
      return CodegenTypeMeta {
        resolved_type: type_name.to_string(),
        type_kind: CodegenTypeKind::Enum,
        is_struct: false,
      };
    }

    if schema_types(referenced_schema)
      .iter()
      .any(|s| s.ident == ident)
    {
      return CodegenTypeMeta {
        resolved_type: type_name.to_string(),
        type_kind: CodegenTypeKind::Struct,
        is_struct: true,
      };
    }
  }

  panic!("unsupported type `{type_name}`")
}
