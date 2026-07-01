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
      CompatibilityAction::OptionalAttribute {} | CompatibilityAction::OptionalChild {} => {
        if rule.field.as_deref().unwrap_or_default().is_empty() {
          let action = match rule.action {
            CompatibilityAction::OptionalAttribute {} => "OptionalAttribute",
            CompatibilityAction::OptionalChild {} => "OptionalChild",
            _ => unreachable!(),
          };
          bail!(
            "compatibility rule {}.{} {} requires Field",
            rule.schema,
            rule.type_name,
            action
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
        CompatibilityAction::TreatAsString {
          default_value: Some(_)
        }
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
    match rule.action {
      CompatibilityAction::OptionalAttribute {} => {
        apply_optional_attribute_rule(sdk_data_schemas, rule)?
      }
      CompatibilityAction::OptionalChild {} => apply_optional_child_rule(sdk_data_schemas, rule)?,
      CompatibilityAction::TreatAsString { .. } => {
        apply_treat_as_string_rule(sdk_data_schemas, rule)?
      }
      CompatibilityAction::EnumValueAlias { .. } => {}
    }
  }

  Ok(())
}

fn find_compat_struct_mut<'a>(
  sdk_data_schemas: &'a mut [SdkDataSchema],
  rule: &CompatibilityRule,
) -> anyhow::Result<&'a mut crate::models::sdk_data::Struct> {
  let schema = sdk_data_schemas
    .iter_mut()
    .find(|schema| schema.module_name == rule.schema)
    .ok_or_else(|| anyhow::anyhow!("compatibility schema {} not found", rule.schema))?;

  schema
    .types
    .iter_mut()
    .find(|item| item.ident == rule.type_name || item.name == rule.type_name)
    .ok_or_else(|| {
      anyhow::anyhow!(
        "compatibility type {}.{} not found",
        rule.schema,
        rule.type_name
      )
    })
}

fn find_compat_attribute_mut<'a>(
  sdk_data_schemas: &'a mut [SdkDataSchema],
  rule: &CompatibilityRule,
) -> anyhow::Result<&'a mut crate::models::sdk_data::Attribute> {
  let field = rule.field.as_deref().unwrap_or_default();

  find_compat_struct_mut(sdk_data_schemas, rule)?
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
    })
}

fn find_compat_child_mut<'a>(
  sdk_data_schemas: &'a mut [SdkDataSchema],
  rule: &CompatibilityRule,
) -> anyhow::Result<&'a mut crate::models::sdk_data::Child> {
  let field = rule.field.as_deref().unwrap_or_default();

  find_compat_struct_mut(sdk_data_schemas, rule)?
    .sequences
    .iter_mut()
    .find(|child| child.name == field || child.ident == field)
    .ok_or_else(|| {
      anyhow::anyhow!(
        "compatibility child {}.{}.{} not found",
        rule.schema,
        rule.type_name,
        field
      )
    })
}

fn apply_optional_attribute_rule(
  sdk_data_schemas: &mut [SdkDataSchema],
  rule: &CompatibilityRule,
) -> anyhow::Result<()> {
  find_compat_attribute_mut(sdk_data_schemas, rule)?.is_option = true;

  Ok(())
}

fn apply_optional_child_rule(
  sdk_data_schemas: &mut [SdkDataSchema],
  rule: &CompatibilityRule,
) -> anyhow::Result<()> {
  find_compat_child_mut(sdk_data_schemas, rule)?.is_option = true;

  Ok(())
}

fn apply_treat_as_string_rule(
  sdk_data_schemas: &mut [SdkDataSchema],
  rule: &CompatibilityRule,
) -> anyhow::Result<()> {
  let attribute = find_compat_attribute_mut(sdk_data_schemas, rule)?;

  attribute.r#type = "String".to_string();
  attribute.resolved_type = "String".to_string();
  attribute.type_kind = CodegenTypeKind::String;
  attribute.is_option = false;

  Ok(())
}
