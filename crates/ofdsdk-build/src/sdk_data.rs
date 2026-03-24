use serde_json::to_writer_pretty;
use std::{fs, fs::File, io::BufWriter, path::Path};

use crate::sdk_data::{context::Context, schemas::gen_schemas};

pub mod compatibility;
pub mod context;
pub mod parts;
pub mod schema_comments;
pub mod schemas;

pub fn gen_sdk_data<P: AsRef<Path>>(schemas_dir: P, out_dir: P) -> anyhow::Result<()> {
  let gen_context = Context::new(schemas_dir.as_ref())?;
  let out_schemas_dir_path = out_dir.as_ref().join("schemas");

  fs::create_dir_all(&out_schemas_dir_path)?;

  let sdk_data_schemas = gen_schemas(&gen_context)?;

  for sdk_data_schema in sdk_data_schemas {
    let file =
      File::create(out_schemas_dir_path.join(format!("{}.json", sdk_data_schema.module_name)))?;
    let writer = BufWriter::new(file);

    to_writer_pretty(writer, &sdk_data_schema)?;
  }

  Ok(())
}
