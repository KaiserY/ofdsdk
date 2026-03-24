use std::collections::BTreeSet;
use std::fs::File;
use std::fs::read_dir;
use std::io::BufReader;
use std::path::Path;

use anyhow::bail;

use crate::models::sdk_data::{SchemaComment, SchemaCommentsConfig, SchemaDocBlockKind};

pub fn read_schema_comments(path: &Path) -> anyhow::Result<SchemaCommentsConfig> {
  if !path.exists() {
    return Ok(SchemaCommentsConfig::default());
  }

  if path.is_dir() {
    return read_schema_comments_dir(path);
  }

  let file = File::open(path)?;
  let reader = BufReader::new(file);
  let schema_comments: SchemaCommentsConfig = serde_json::from_reader(reader)?;

  validate_schema_comments(&schema_comments)?;

  Ok(schema_comments)
}

fn read_schema_comments_dir(path: &Path) -> anyhow::Result<SchemaCommentsConfig> {
  let mut file_paths = vec![];

  for entry in read_dir(path)? {
    let entry = entry?;
    let file_path = entry.path();

    if !file_path.is_file() || file_path.extension() != Some(std::ffi::OsStr::new("json")) {
      continue;
    }

    file_paths.push(file_path);
  }

  file_paths.sort();

  let mut entries = vec![];

  for file_path in file_paths {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let schema_comments: SchemaCommentsConfig = serde_json::from_reader(reader)?;
    validate_schema_comments(&schema_comments)?;
    entries.extend(schema_comments.entries);
  }

  let schema_comments = SchemaCommentsConfig { entries };
  validate_schema_comments(&schema_comments)?;

  Ok(schema_comments)
}

fn validate_schema_comments(schema_comments: &SchemaCommentsConfig) -> anyhow::Result<()> {
  let mut keys = BTreeSet::new();

  for entry in &schema_comments.entries {
    validate_schema_comment(entry)?;

    let key = (
      entry.schema.as_str(),
      entry.type_name.as_str(),
      entry.field.as_deref().unwrap_or_default(),
    );

    if !keys.insert(key) {
      bail!(
        "duplicate schema comment {}.{}.{}",
        entry.schema,
        entry.type_name,
        entry.field.as_deref().unwrap_or_default()
      );
    }
  }

  Ok(())
}

fn validate_schema_comment(entry: &SchemaComment) -> anyhow::Result<()> {
  if entry.schema.is_empty() {
    bail!("schema comment schema cannot be empty");
  }

  if entry.type_name.is_empty() {
    bail!("schema comment type cannot be empty");
  }

  if entry.docs.is_empty() {
    bail!(
      "schema comment {}.{} must define Docs",
      entry.schema,
      entry.type_name
    );
  }

  for doc in &entry.docs {
    if doc.text.trim().is_empty() {
      bail!(
        "schema comment {}.{} contains empty doc text",
        entry.schema,
        entry.type_name
      );
    }

    match doc.kind {
      SchemaDocBlockKind::Description | SchemaDocBlockKind::Example => {}
    }
  }

  Ok(())
}
