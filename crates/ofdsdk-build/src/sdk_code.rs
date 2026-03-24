use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use syn::{Ident, ItemMod, parse_str, parse2};

use crate::models::sdk_data::CompatibilityRule as SdkDataCompatibilityRule;
use crate::models::sdk_data::PartBase as SdkDataPartBase;
use crate::models::sdk_data::PartDefinition as SdkDataPartDefinition;
use crate::models::sdk_data::Schema as SdkDataSchema;
use crate::models::sdk_data::SchemaComment as SdkDataSchemaComment;
use crate::sdk_code::deserializer::gen_schema_deserializer;
use crate::sdk_code::parts::{SchemaIndex, gen_package_module, gen_part_module};
use crate::sdk_code::schemas::gen_schema;
use crate::sdk_code::serializer::gen_schema_serializer;
use crate::sdk_data::compatibility::{apply_compatibility, read_compatibility};
use crate::sdk_data::parts::read_parts;
use crate::sdk_data::schema_comments::read_schema_comments;
use crate::sdk_data::schemas::annotate_codegen_types;

pub mod deserializer;
pub mod parts;
pub mod schemas;
pub mod serializer;

const FILE_HEADER: &str = r###"//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//
"###;

pub fn gen_sdk_code<P: AsRef<Path>>(sdk_data_dir: P, out_dir: P) -> anyhow::Result<()> {
  let sdk_data_schemas_dir_path = sdk_data_dir.as_ref().join("schemas");
  let sdk_data_parts_dir_path = sdk_data_dir.as_ref().join("parts");
  let sdk_data_compatibility_path = sdk_data_dir.as_ref().join("compatibility.json");
  let sdk_data_schema_comments_path = sdk_data_dir.as_ref().join("schema_comments");
  let mut sdk_data_schemas = read_schemas(&sdk_data_schemas_dir_path)?;
  let sdk_data_parts = read_parts(&sdk_data_parts_dir_path)?;
  let sdk_data_compatibility = read_compatibility(&sdk_data_compatibility_path)?;
  let sdk_data_schema_comments = read_schema_comments(&sdk_data_schema_comments_path)?;
  apply_compatibility(&mut sdk_data_schemas, &sdk_data_compatibility)?;

  write_schemas(
    &sdk_data_schemas,
    &sdk_data_schema_comments.entries,
    out_dir.as_ref(),
  )?;
  write_deserializers(
    &sdk_data_schemas,
    &sdk_data_compatibility.rules,
    out_dir.as_ref(),
  )?;
  write_serializers(&sdk_data_schemas, out_dir.as_ref())?;
  write_parts(&sdk_data_parts, &sdk_data_schemas, out_dir.as_ref())?;

  Ok(())
}

fn read_schemas(sdk_data_schemas_dir_path: &Path) -> anyhow::Result<Vec<SdkDataSchema>> {
  let mut sdk_data_schemas = vec![];

  for entry in fs::read_dir(sdk_data_schemas_dir_path)? {
    let entry = entry?;
    let path = entry.path();

    if !path.is_file() || path.extension() != Some(OsStr::new("json")) {
      continue;
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let sdk_data_schema: SdkDataSchema = serde_json::from_reader(reader)?;
    sdk_data_schemas.push(sdk_data_schema);
  }

  annotate_codegen_types(&mut sdk_data_schemas);
  sdk_data_schemas.sort_by(|a, b| a.module_name.cmp(&b.module_name));

  Ok(sdk_data_schemas)
}

pub fn write_parts(
  sdk_data_parts: &[SdkDataPartDefinition],
  sdk_data_schemas: &[SdkDataSchema],
  out_dir_path: &Path,
) -> anyhow::Result<()> {
  let out_parts_dir_path = out_dir_path.join("parts");

  fs::create_dir_all(&out_parts_dir_path)?;
  clear_generated_parts_dir(&out_parts_dir_path)?;

  let mut parts_mod_names: BTreeSet<String> = BTreeSet::new();
  let index = SchemaIndex::new(sdk_data_schemas);

  for sdk_data_part in sdk_data_parts
    .iter()
    .filter(|sdk_data_part| sdk_data_part.base == SdkDataPartBase::OfdPackage)
  {
    let package_module_name = sdk_data_part.name.to_snake_case();

    let token_stream: TokenStream =
      gen_package_module(sdk_data_part, sdk_data_parts, sdk_data_schemas)?;
    let syntax_tree = parse2(token_stream)?;
    let formatted = prettyplease::unparse(&syntax_tree);
    let part_path = out_parts_dir_path.join(format!("{package_module_name}.rs"));
    fs::write(part_path, format!("{FILE_HEADER}\n{formatted}"))?;

    for child_part in sdk_data_parts
      .iter()
      .filter(|child_part| child_part.name != sdk_data_part.name)
    {
      let child_module_name = child_part.name.to_snake_case();
      let child_token_stream: TokenStream = gen_part_module(child_part, sdk_data_parts, &index)?;
      let child_syntax_tree = parse2(child_token_stream)?;
      let child_formatted = prettyplease::unparse(&child_syntax_tree);
      let child_part_path = out_parts_dir_path.join(format!("{child_module_name}.rs"));
      fs::write(child_part_path, format!("{FILE_HEADER}\n{child_formatted}"))?;

      parts_mod_names.insert(child_module_name);
    }

    parts_mod_names.insert(package_module_name);
  }

  let mut parts_mod_list: Vec<ItemMod> = vec![];

  for part_mod_name in parts_mod_names {
    let part_mod_ident: Ident = parse_str(&part_mod_name)?;
    parts_mod_list.push(parse2(quote! {
      pub mod #part_mod_ident;
    })?);
  }

  let token_stream: TokenStream = quote! {
    #( #parts_mod_list )*
  };
  let syntax_tree: syn::File = parse2(token_stream)?;
  let formatted = prettyplease::unparse(&syntax_tree);
  let parts_mod_path = out_dir_path.join("parts.rs");
  fs::write(parts_mod_path, format!("{FILE_HEADER}\n{formatted}"))?;

  Ok(())
}

fn clear_generated_parts_dir(out_parts_dir_path: &Path) -> anyhow::Result<()> {
  for entry in fs::read_dir(out_parts_dir_path)? {
    let entry = entry?;
    let path = entry.path();

    if path.is_file() && path.extension() == Some(OsStr::new("rs")) {
      fs::remove_file(path)?;
      continue;
    }

    if path.is_dir() {
      fs::remove_dir_all(path)?;
    }
  }

  Ok(())
}

pub fn write_schemas(
  sdk_data_schemas: &[SdkDataSchema],
  schema_comments: &[SdkDataSchemaComment],
  out_dir_path: &Path,
) -> anyhow::Result<()> {
  let out_schemas_dir_path = out_dir_path.join("schemas");

  fs::create_dir_all(&out_schemas_dir_path)?;

  let mut schemas_mod_list: Vec<ItemMod> = vec![];

  for sdk_data_schema in sdk_data_schemas {
    let token_stream: TokenStream = gen_schema(sdk_data_schema, schema_comments)?;
    let syntax_tree = parse2(token_stream)?;
    let formatted = prettyplease::unparse(&syntax_tree);
    let schema_path = out_schemas_dir_path.join(format!("{}.rs", sdk_data_schema.module_name));
    fs::write(schema_path, format!("{FILE_HEADER}\n{formatted}"))?;

    let schema_mod_ident: Ident = parse_str(&sdk_data_schema.module_name)?;
    schemas_mod_list.push(parse2(quote! {
      pub mod #schema_mod_ident;
    })?);
  }

  let token_stream: TokenStream = quote! {
    #( #schemas_mod_list )*
  };
  let syntax_tree: syn::File = parse2(token_stream)?;
  let formatted = prettyplease::unparse(&syntax_tree);
  let schemas_mod_path = out_dir_path.join("schemas.rs");
  fs::write(schemas_mod_path, format!("{FILE_HEADER}\n{formatted}"))?;

  Ok(())
}

pub fn write_deserializers(
  sdk_data_schemas: &[SdkDataSchema],
  compatibility_rules: &[SdkDataCompatibilityRule],
  out_dir_path: &Path,
) -> anyhow::Result<()> {
  let out_deserializers_dir_path = out_dir_path.join("deserializers");

  fs::create_dir_all(&out_deserializers_dir_path)?;

  let mut deserializers_mod_list: Vec<ItemMod> = vec![];

  for sdk_data_schema in sdk_data_schemas {
    let token_stream: TokenStream =
      gen_schema_deserializer(sdk_data_schema, sdk_data_schemas, compatibility_rules)?;
    let syntax_tree = parse2(token_stream)?;
    let formatted = prettyplease::unparse(&syntax_tree);
    let deserializer_path =
      out_deserializers_dir_path.join(format!("{}.rs", sdk_data_schema.module_name));
    fs::write(deserializer_path, format!("{FILE_HEADER}\n{formatted}"))?;

    let deserializer_mod_ident: Ident = parse_str(&sdk_data_schema.module_name)?;
    deserializers_mod_list.push(parse2(quote! {
      pub mod #deserializer_mod_ident;
    })?);
  }

  let token_stream: TokenStream = quote! {
    #( #deserializers_mod_list )*
  };
  let syntax_tree: syn::File = parse2(token_stream)?;
  let formatted = prettyplease::unparse(&syntax_tree);
  let deserializers_mod_path = out_dir_path.join("deserializers.rs");
  fs::write(
    deserializers_mod_path,
    format!("{FILE_HEADER}\n{formatted}"),
  )?;

  Ok(())
}

pub fn write_serializers(
  sdk_data_schemas: &[SdkDataSchema],
  out_dir_path: &Path,
) -> anyhow::Result<()> {
  let out_serializers_dir_path = out_dir_path.join("serializers");

  fs::create_dir_all(&out_serializers_dir_path)?;

  let mut serializers_mod_list: Vec<ItemMod> = vec![];

  for sdk_data_schema in sdk_data_schemas {
    let token_stream: TokenStream = gen_schema_serializer(sdk_data_schema, sdk_data_schemas)?;
    let syntax_tree = parse2(token_stream)?;
    let formatted = prettyplease::unparse(&syntax_tree);
    let serializer_path =
      out_serializers_dir_path.join(format!("{}.rs", sdk_data_schema.module_name));
    fs::write(serializer_path, format!("{FILE_HEADER}\n{formatted}"))?;

    let serializer_mod_ident: Ident = parse_str(&sdk_data_schema.module_name)?;
    serializers_mod_list.push(parse2(quote! {
      pub mod #serializer_mod_ident;
    })?);
  }

  let token_stream: TokenStream = quote! {
    #( #serializers_mod_list )*
  };
  let syntax_tree: syn::File = parse2(token_stream)?;
  let formatted = prettyplease::unparse(&syntax_tree);
  let serializers_mod_path = out_dir_path.join("serializers.rs");
  fs::write(serializers_mod_path, format!("{FILE_HEADER}\n{formatted}"))?;

  Ok(())
}
