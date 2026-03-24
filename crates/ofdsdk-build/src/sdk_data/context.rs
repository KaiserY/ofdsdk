use anyhow::anyhow;
use heck::ToSnakeCase;
use quick_xml::de::from_reader;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::models::xsd::{Schema, SchemaContentChoice};

#[derive(Debug, Default)]
pub struct Context {
  pub schemas: Vec<Schema>,
  pub type_name_module_name_map: HashMap<String, String>,
  pub complex_type_name_map: HashMap<String, (usize, usize)>,
}

impl Context {
  pub fn new(schemas_dir: &Path) -> anyhow::Result<Self> {
    let mut schemas = vec![];
    let mut type_name_module_name_map = HashMap::new();
    let mut complex_type_name_map = HashMap::new();

    for entry in fs::read_dir(schemas_dir)? {
      let entry = entry?;
      let path = entry.path();

      if !path.is_file() || path.extension() != Some(OsStr::new("xsd")) {
        continue;
      }

      let file = File::open(path)?;
      let reader = BufReader::new(file);

      let mut schema: Schema = from_reader(reader)?;

      let schema_mod = entry
        .path()
        .file_stem()
        .ok_or_else(|| anyhow!("{entry:?}"))?
        .to_string_lossy()
        .to_snake_case();

      schema.module_name = schema_mod;

      let schema_index = schemas.len();

      for (content_index, content) in schema.contents.iter().enumerate() {
        match content {
          SchemaContentChoice::SimpleType(simple_type) => {
            if let Some(name) = &simple_type.name {
              type_name_module_name_map.insert(name.clone(), schema.module_name.clone());
            }
          }
          SchemaContentChoice::ComplexType(complex_type) => {
            if let Some(name) = &complex_type.name {
              type_name_module_name_map.insert(name.clone(), schema.module_name.clone());
              complex_type_name_map.insert(name.clone(), (schema_index, content_index));
            }
          }
          SchemaContentChoice::Element(element) => {
            if let Some(name) = &element.name {
              type_name_module_name_map.insert(name.clone(), schema.module_name.clone());
            }
          }
          _ => (),
        }
      }

      schemas.push(schema);
    }

    Ok(Self {
      schemas,
      type_name_module_name_map,
      complex_type_name_map,
    })
  }
}
