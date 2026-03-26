use super::*;

fn normalize_type_ref(
  skd_data_context: &SdkDataContext,
  xsd_schema: &XsdSchema,
  sdk_data_schema: &SdkDataSchema,
  type_name: &str,
) -> String {
  if matches!(type_name, "String" | "bool" | "f64" | "i32" | "u32") || type_name.contains("::") {
    return type_name.to_string();
  }

  if sdk_data_schema.aliases.iter().any(|a| a.ident == type_name)
    || sdk_data_schema.enums.iter().any(|e| e.ident == type_name)
    || schema_types(sdk_data_schema)
      .iter()
      .any(|s| s.ident == type_name)
  {
    return type_name.to_string();
  }

  resolve_xsd_type(skd_data_context, xsd_schema, type_name)
}

pub(super) fn normalize_schema_type_refs(
  skd_data_context: &SdkDataContext,
  xsd_schema: &XsdSchema,
  sdk_data_schema: &mut SdkDataSchema,
) {
  let schema_snapshot = sdk_data_schema.clone();

  for alias in &mut sdk_data_schema.aliases {
    alias.r#type = normalize_type_ref(
      skd_data_context,
      xsd_schema,
      &schema_snapshot,
      &alias.r#type,
    );
  }

  for sdk_data_struct in schema_types_mut(sdk_data_schema) {
    for attr in &mut sdk_data_struct.attributes {
      attr.r#type =
        normalize_type_ref(skd_data_context, xsd_schema, &schema_snapshot, &attr.r#type);
    }

    for child in &mut sdk_data_struct.sequences {
      child.r#type = normalize_type_ref(
        skd_data_context,
        xsd_schema,
        &schema_snapshot,
        &child.r#type,
      );
    }

    for child in &mut sdk_data_struct.children {
      child.r#type = normalize_type_ref(
        skd_data_context,
        xsd_schema,
        &schema_snapshot,
        &child.r#type,
      );
    }
  }
}
