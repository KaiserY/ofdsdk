use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::models::sdk_data::{PartContent, PartDefinition};
use anyhow::{anyhow, bail};

pub fn read_parts(parts_dir_path: &Path) -> anyhow::Result<Vec<PartDefinition>> {
  let mut part_definitions = vec![];

  if !parts_dir_path.exists() {
    return Ok(vec![]);
  }

  for entry in fs::read_dir(parts_dir_path)? {
    let entry = entry?;
    let path = entry.path();

    if !path.is_file() || path.extension() != Some(OsStr::new("json")) {
      continue;
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut part_definition: PartDefinition = serde_json::from_reader(reader)?;
    normalize_part_definition(&mut part_definition);
    part_definitions.push(part_definition);
  }

  part_definitions.sort_by(|a, b| a.name.cmp(&b.name));
  validate_parts(&part_definitions)?;

  Ok(part_definitions)
}

fn validate_parts(part_definitions: &[PartDefinition]) -> anyhow::Result<()> {
  let mut part_names = std::collections::BTreeSet::new();

  for part_definition in part_definitions {
    if !part_names.insert(part_definition.name.as_str()) {
      bail!("duplicate part definition {}", part_definition.name);
    }
  }

  for part_definition in part_definitions {
    match part_definition.content.as_ref().unwrap() {
      PartContent::Xml => {
        if part_definition.schema_module.is_none() || part_definition.root_element.is_none() {
          bail!(
            "xml part {} must define SchemaModule and RootElement",
            part_definition.name
          );
        }
      }
      PartContent::Blob => {
        if part_definition.schema_module.is_some() || part_definition.root_element.is_some() {
          bail!(
            "blob part {} must not define SchemaModule or RootElement",
            part_definition.name
          );
        }
      }
    }

    if let Some(context_from) = &part_definition.context_from {
      validate_parent_ref(
        part_definition,
        &context_from.parent,
        &part_names,
        "ContextFrom",
      )?;
    }

    for child in &part_definition.children {
      if let Some(from) = &child.from {
        validate_parent_ref(
          part_definition,
          &from.parent,
          &part_names,
          "Children[].From",
        )?;
      }

      if let Some(context_from) = &child.context_from {
        validate_parent_ref(
          part_definition,
          &context_from.parent,
          &part_names,
          "Children[].ContextFrom",
        )?;
      }

      if !part_names.contains(child.name.as_str()) {
        return Err(anyhow!(
          "part {} references missing child part {}",
          part_definition.name,
          child.name
        ));
      }
    }
  }

  Ok(())
}

fn normalize_part_definition(part_definition: &mut PartDefinition) {
  if part_definition.content.is_some() {
    return;
  }

  if part_definition.schema_module.is_none() && part_definition.root_element.is_none() {
    part_definition.content = Some(PartContent::Blob);
    return;
  }

  part_definition.content = Some(PartContent::Xml);
}

fn validate_parent_ref(
  part_definition: &PartDefinition,
  parent: &str,
  part_names: &std::collections::BTreeSet<&str>,
  field_name: &str,
) -> anyhow::Result<()> {
  if !part_names.contains(parent) {
    bail!(
      "part {} has unknown {} parent {}",
      part_definition.name,
      field_name,
      parent
    );
  }

  Ok(())
}
