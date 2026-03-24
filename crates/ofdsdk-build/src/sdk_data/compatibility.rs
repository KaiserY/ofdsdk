use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::bail;

use crate::models::sdk_data::{CodegenTypeKind, Schema as SdkDataSchema};
use crate::models::sdk_data::{CompatibilityAction, CompatibilityConfig, CompatibilityRule};

pub fn read_compatibility(path: &Path) -> anyhow::Result<CompatibilityConfig> {
  if !path.exists() {
    return Ok(CompatibilityConfig::default());
  }

  let file = File::open(path)?;
  let reader = BufReader::new(file);
  let compatibility: CompatibilityConfig = serde_json::from_reader(reader)?;

  validate_compatibility(&compatibility)?;

  Ok(compatibility)
}

fn validate_compatibility(compatibility: &CompatibilityConfig) -> anyhow::Result<()> {
  for rule in &compatibility.rules {
    if rule.schema.is_empty() {
      bail!("compatibility rule schema cannot be empty");
    }

    if rule.type_name.is_empty() {
      bail!("compatibility rule type cannot be empty");
    }

    match &rule.action {
      CompatibilityAction::AllowMissingAttribute { .. } => {
        if rule.field.as_deref().unwrap_or_default().is_empty() {
          bail!(
            "compatibility rule {}.{} AllowMissingAttribute requires Field",
            rule.schema,
            rule.type_name
          );
        }
      }
      CompatibilityAction::EnumValueAlias { input, canonical } => {
        if rule.field.is_some() {
          bail!(
            "compatibility rule {}.{} EnumValueAlias must not set Field",
            rule.schema,
            rule.type_name
          );
        }

        if input.is_empty() || canonical.is_empty() {
          bail!(
            "compatibility rule {}.{} EnumValueAlias requires Input and Canonical",
            rule.schema,
            rule.type_name
          );
        }
      }
      CompatibilityAction::TreatAsString { .. } => {
        if rule.field.as_deref().unwrap_or_default().is_empty() {
          bail!(
            "compatibility rule {}.{} TreatAsString requires Field",
            rule.schema,
            rule.type_name
          );
        }
      }
    }
  }

  Ok(())
}

pub fn find_missing_attribute_rule<'a>(
  compatibility_rules: &'a [CompatibilityRule],
  schema: &str,
  type_name: &str,
  field_name: &str,
  field_ident: &str,
) -> Option<&'a CompatibilityRule> {
  compatibility_rules.iter().find(|rule| {
    rule.schema == schema
      && rule.type_name == type_name
      && matches!(rule.field.as_deref(), Some(field) if field == field_name || field == field_ident)
      && matches!(
        rule.action,
        CompatibilityAction::AllowMissingAttribute { .. }
          | CompatibilityAction::TreatAsString { .. }
      )
  })
}

pub fn enum_value_alias_rules<'a>(
  compatibility_rules: &'a [CompatibilityRule],
  schema: &str,
  type_name: &str,
) -> Vec<&'a CompatibilityRule> {
  compatibility_rules
    .iter()
    .filter(|rule| {
      rule.schema == schema
        && rule.type_name == type_name
        && matches!(rule.action, CompatibilityAction::EnumValueAlias { .. })
    })
    .collect()
}

pub fn apply_compatibility(
  sdk_data_schemas: &mut [SdkDataSchema],
  compatibility: &CompatibilityConfig,
) -> anyhow::Result<()> {
  for rule in &compatibility.rules {
    if let CompatibilityAction::TreatAsString { .. } = rule.action {
      apply_treat_as_string_rule(sdk_data_schemas, rule)?;
    }
  }

  Ok(())
}

fn apply_treat_as_string_rule(
  sdk_data_schemas: &mut [SdkDataSchema],
  rule: &CompatibilityRule,
) -> anyhow::Result<()> {
  let field = rule.field.as_deref().unwrap_or_default();

  let schema = sdk_data_schemas
    .iter_mut()
    .find(|schema| schema.module_name == rule.schema)
    .ok_or_else(|| anyhow::anyhow!("compatibility schema {} not found", rule.schema))?;

  let struct_type = schema
    .types
    .iter_mut()
    .find(|item| item.ident == rule.type_name || item.name == rule.type_name)
    .ok_or_else(|| {
      anyhow::anyhow!(
        "compatibility type {}.{} not found",
        rule.schema,
        rule.type_name
      )
    })?;

  let attribute = struct_type
    .attributes
    .iter_mut()
    .find(|attr| attr.name == field || attr.ident == field)
    .ok_or_else(|| {
      anyhow::anyhow!(
        "compatibility field {}.{}.{} not found",
        rule.schema,
        rule.type_name,
        field
      )
    })?;

  attribute.r#type = "String".to_string();
  attribute.resolved_type = "String".to_string();
  attribute.type_kind = CodegenTypeKind::String;
  attribute.is_option = false;

  Ok(())
}
