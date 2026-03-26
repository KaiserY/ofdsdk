use anyhow::{anyhow, bail};
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;

use crate::models::sdk_data::{
  Attribute, Child, CodegenTypeKind, PartContent, PartDefinition, PartPath, Schema, Struct,
};

mod helpers;
use helpers::{
  collect_child_parts, gen_collect_values_stmt, gen_first_path_expr_from_source,
  gen_part_path_expr, gen_required_single_value_expr_from_source, resolve_part_context_model,
};

fn child_field_tokens(
  child_field_ident: &syn::Ident,
  child_type: &TokenStream,
  is_many: bool,
  is_required: bool,
) -> TokenStream {
  if is_many {
    quote! {
      pub #child_field_ident: Vec<#child_type>,
    }
  } else if is_required {
    quote! {
      pub #child_field_ident: Box<#child_type>,
    }
  } else {
    quote! {
      pub #child_field_ident: Option<Box<#child_type>>,
    }
  }
}

fn save_child_stmt(
  child_field_ident: &syn::Ident,
  is_many: bool,
  is_required: bool,
) -> TokenStream {
  if is_many {
    quote! {
      crate::common::save_zip_parts(&self.#child_field_ident, zip, entry_set, |child, zip, entry_set| {
        child.save_zip(zip, entry_set)
      })?;
    }
  } else if is_required {
    quote! {
      crate::common::save_required_zip_part(&self.#child_field_ident, zip, entry_set, |child, zip, entry_set| {
        child.save_zip(zip, entry_set)
      })?;
    }
  } else {
    quote! {
      crate::common::save_optional_zip_part(&self.#child_field_ident, zip, entry_set, |child, zip, entry_set| {
        child.save_zip(zip, entry_set)
      })?;
    }
  }
}

struct LoadChildStmt<'a> {
  child_field_ident: &'a syn::Ident,
  path_expr: &'a syn::Ident,
  contexts_expr: Option<&'a syn::Ident>,
  child_api_name: &'a str,
  path_resolve: &'a TokenStream,
  child_module_ident: &'a syn::Ident,
  child_struct_ident: &'a syn::Ident,
  is_many: bool,
  is_required: bool,
}

fn load_child_stmt(spec: LoadChildStmt<'_>) -> TokenStream {
  let LoadChildStmt {
    child_field_ident,
    path_expr,
    contexts_expr,
    child_api_name,
    path_resolve,
    child_module_ident,
    child_struct_ident,
    is_many,
    is_required,
  } = spec;

  match (is_many, is_required, contexts_expr) {
    (true, _, Some(contexts_ident)) => quote! {
      let #child_field_ident = crate::common::load_zip_parts_with_context(
        &current_dir,
        #path_expr,
        #contexts_ident,
        #child_api_name,
        archive,
        |child_path, child_context, archive| {
          let resolved_path = #path_resolve;
          crate::parts::#child_module_ident::#child_struct_ident::new_from_archive(
            &resolved_path,
            child_context,
            archive,
          )
        },
      )?;
    },
    (true, _, None) => quote! {
      let #child_field_ident = crate::common::load_zip_parts(
        &current_dir,
        #path_expr,
        archive,
        |child_path, archive| {
          let resolved_path = #path_resolve;
          crate::parts::#child_module_ident::#child_struct_ident::new_from_archive(
            &resolved_path,
            archive,
          )
        },
      )?;
    },
    (false, true, Some(contexts_ident)) => quote! {
      let #child_field_ident = crate::common::load_required_zip_part_with_context(
        &current_dir,
        #path_expr,
        #contexts_ident,
        #child_api_name,
        archive,
        |child_path, child_context, archive| {
          let resolved_path = #path_resolve;
          crate::parts::#child_module_ident::#child_struct_ident::new_from_archive(
            &resolved_path,
            child_context,
            archive,
          )
        },
      )?;
    },
    (false, true, None) => quote! {
      let #child_field_ident = crate::common::load_required_zip_part(
        &current_dir,
        #path_expr,
        #child_api_name,
        archive,
        |child_path, archive| {
          let resolved_path = #path_resolve;
          crate::parts::#child_module_ident::#child_struct_ident::new_from_archive(
            &resolved_path,
            archive,
          )
        },
      )?;
    },
    (false, false, Some(contexts_ident)) => quote! {
      let #child_field_ident = crate::common::load_optional_zip_part_with_context(
        &current_dir,
        #path_expr,
        #contexts_ident,
        #child_api_name,
        archive,
        |child_path, child_context, archive| {
          let resolved_path = #path_resolve;
          crate::parts::#child_module_ident::#child_struct_ident::new_from_archive(
            &resolved_path,
            child_context,
            archive,
          )
        },
      )?;
    },
    (false, false, None) => quote! {
      let #child_field_ident = crate::common::load_optional_zip_part(
        &current_dir,
        #path_expr,
        archive,
        |child_path, archive| {
          let resolved_path = #path_resolve;
          crate::parts::#child_module_ident::#child_struct_ident::new_from_archive(
            &resolved_path,
            archive,
          )
        },
      )?;
    },
  }
}

pub fn gen_package_module(
  sdk_data_root_part: &PartDefinition,
  sdk_data_parts: &[PartDefinition],
  sdk_data_schemas: &[Schema],
) -> anyhow::Result<TokenStream> {
  let index = SchemaIndex::new(sdk_data_schemas);
  let root_struct_ident = format_ident!("{}", sdk_data_root_part.name.to_upper_camel_case());
  let root_part_path = match &sdk_data_root_part.path {
    PartPath::Fixed { path } => path,
    _ => bail!("root part {} must use Fixed path", sdk_data_root_part.name),
  };

  let root_part_tokens = gen_part_module(sdk_data_root_part, sdk_data_parts, &index)?;

  Ok(quote! {
    #root_part_tokens

    impl #root_struct_ident {
      pub fn new<R: std::io::Read + std::io::Seek>(
        reader: R,
      ) -> Result<Self, crate::common::SdkError> {
        let mut archive = zip::ZipArchive::new(reader)?;
        Self::new_from_archive(#root_part_path, &mut archive)
      }

      pub fn new_from_file<P: AsRef<std::path::Path>>(
        path: P,
      ) -> Result<Self, crate::common::SdkError> {
        Self::new(std::io::BufReader::new(std::fs::File::open(path)?))
      }

      pub fn save<W: std::io::Write + std::io::Seek>(
        &self,
        writer: W,
      ) -> Result<(), crate::common::SdkError> {
        let mut entry_set = std::collections::HashSet::new();
        let mut zip = zip::ZipWriter::new(writer);
        self.save_zip(&mut zip, &mut entry_set)?;
        zip.finish()?;
        Ok(())
      }

      pub fn save_to_file<P: AsRef<std::path::Path>>(
        &self,
        path: P,
      ) -> Result<(), crate::common::SdkError> {
        self.save(std::fs::File::create(path)?)
      }
    }
  })
}

pub(crate) fn gen_part_module(
  part: &PartDefinition,
  sdk_data_parts: &[PartDefinition],
  index: &SchemaIndex<'_>,
) -> anyhow::Result<TokenStream> {
  let struct_ident = format_ident!("{}", part.name.to_upper_camel_case());
  let mut fields = vec![quote! {
    pub inner_path: String,
  }];
  let mut body_stmts = vec![];
  let mut save_stmts = vec![];
  let mut save_child_stmts = vec![];
  let mut self_values = vec![quote! {
    inner_path: path.to_string()
  }];
  let child_parts = collect_child_parts(part, sdk_data_parts)?;
  let context_model = part
    .context_from
    .as_ref()
    .map(|context_from| resolve_part_context_model(part, sdk_data_parts, context_from, index))
    .transpose()?;
  let constructor_args = if let Some(context_model) = &context_model {
    let context_field_ident = format_ident!("{}", context_model.field_ident);
    let context_type = &context_model.ty;
    vec![quote! { #context_field_ident: #context_type, }]
  } else {
    vec![]
  };

  if let Some(context_model) = &context_model {
    let context_field_ident = format_ident!("{}", context_model.field_ident);
    let context_type = &context_model.ty;
    fields.push(quote! {
      pub #context_field_ident: #context_type,
    });
    self_values.push(quote! {
      #context_field_ident
    });
  }

  match part.content.as_ref().unwrap() {
    PartContent::Xml => {
      let schema_module = part
        .schema_module
        .as_deref()
        .ok_or_else(|| anyhow!("xml part {} missing SchemaModule", part.name))?;
      let root_element = part
        .root_element
        .as_deref()
        .ok_or_else(|| anyhow!("xml part {} missing RootElement", part.name))?;
      let root_type = index.resolve_root_type(schema_module, root_element)?;
      fields.push(quote! {
        pub root_element: #root_type,
      });
      body_stmts.push(quote! {
        let root_element = #root_type::from_reader(std::io::BufReader::new(
          std::io::Cursor::new(crate::common::read_zip_data(archive, path)?),
        ))?;
      });
      self_values.push(quote! {
        root_element
      });
      save_stmts.push(quote! {
        crate::common::save_zip_data(
          &self.inner_path,
          self.root_element.to_xml()?.as_bytes(),
          zip,
          entry_set,
        )?;
      });
    }
    PartContent::Blob => {
      fields.push(quote! {
        pub part_content: Vec<u8>,
      });
      body_stmts.push(quote! {
        let part_content = crate::common::read_zip_data(archive, path)?;
      });
      self_values.push(quote! {
        part_content
      });
      save_stmts.push(quote! {
        crate::common::save_zip_data(&self.inner_path, &self.part_content, zip, entry_set)?;
      });
    }
  }

  if !child_parts.is_empty() {
    body_stmts.push(quote! {
      let current_dir = crate::common::zip_parent_dir(path);
    });
  }

  for child in child_parts {
    let child_field_ident = format_ident!("{}", child.api_name().to_snake_case());
    let child_module_ident = format_ident!("{}", child.part_name().to_snake_case());
    let child_struct_ident = format_ident!("{}", child.part_name().to_upper_camel_case());
    let child_type = quote! { crate::parts::#child_module_ident::#child_struct_ident };
    let paths_ident = format_ident!("{}_paths", child.api_name().to_snake_case());
    let single_child_path_expr =
      gen_first_path_expr_from_source(part, sdk_data_parts, child.source, index)?;
    let context_value_model = child
      .context_source
      .map(|context_from| {
        resolve_part_context_model(child.part, sdk_data_parts, context_from, index)
      })
      .transpose()?;
    save_child_stmts.push(save_child_stmt(
      &child_field_ident,
      child.is_many(),
      child.is_required(),
    ));

    if child.is_many() {
      let resolve_child_paths =
        gen_collect_values_stmt(part, sdk_data_parts, child.source, &paths_ident, index)?;
      let path_resolve = gen_part_path_expr(
        part,
        sdk_data_parts,
        child.part,
        child.source.source_root.clone(),
        &quote! { child_path },
        index,
      )?;
      fields.push(child_field_tokens(
        &child_field_ident,
        &child_type,
        true,
        child.is_required(),
      ));
      body_stmts.push(quote! {
        let mut #paths_ident: Vec<String> = vec![];
      });
      body_stmts.push(resolve_child_paths);
      if let Some(context_value_model) = context_value_model {
        let contexts_ident = format_ident!("{}_contexts", child.api_name().to_snake_case());
        let context_type = context_value_model.ty;
        let resolve_child_contexts = gen_collect_values_stmt(
          part,
          sdk_data_parts,
          child.context_source.unwrap(),
          &contexts_ident,
          index,
        )?;
        body_stmts.push(quote! {
          let mut #contexts_ident: Vec<#context_type> = vec![];
        });
        body_stmts.push(resolve_child_contexts);
        body_stmts.push(load_child_stmt(LoadChildStmt {
          child_field_ident: &child_field_ident,
          path_expr: &paths_ident,
          contexts_expr: Some(&contexts_ident),
          child_api_name: child.api_name(),
          path_resolve: &path_resolve,
          child_module_ident: &child_module_ident,
          child_struct_ident: &child_struct_ident,
          is_many: true,
          is_required: child.is_required(),
        }));
      } else {
        body_stmts.push(load_child_stmt(LoadChildStmt {
          child_field_ident: &child_field_ident,
          path_expr: &paths_ident,
          contexts_expr: None,
          child_api_name: child.api_name(),
          path_resolve: &path_resolve,
          child_module_ident: &child_module_ident,
          child_struct_ident: &child_struct_ident,
          is_many: true,
          is_required: child.is_required(),
        }));
      }
      self_values.push(quote! {
        #child_field_ident
      });
    } else {
      let path_resolve = gen_part_path_expr(
        part,
        sdk_data_parts,
        child.part,
        child.source.source_root.clone(),
        &quote! { child_path },
        index,
      )?;
      if let Some(single_child_path_expr) = single_child_path_expr {
        let single_paths_ident = format_ident!("{}_single_paths", child.api_name().to_snake_case());
        let single_child_context_expr = child
          .context_source
          .map(|context_from| {
            gen_required_single_value_expr_from_source(
              part,
              sdk_data_parts,
              context_from,
              child.api_name(),
              index,
            )
          })
          .transpose()?;
        fields.push(child_field_tokens(
          &child_field_ident,
          &child_type,
          false,
          child.is_required(),
        ));
        body_stmts.push(quote! {
          let #single_paths_ident = vec![#single_child_path_expr];
        });
        if let Some(single_child_context_expr) = single_child_context_expr {
          let single_contexts_ident =
            format_ident!("{}_single_contexts", child.api_name().to_snake_case());
          body_stmts.push(quote! {
            let #single_contexts_ident = vec![#single_child_context_expr];
          });
          body_stmts.push(load_child_stmt(LoadChildStmt {
            child_field_ident: &child_field_ident,
            path_expr: &single_paths_ident,
            contexts_expr: Some(&single_contexts_ident),
            child_api_name: child.api_name(),
            path_resolve: &path_resolve,
            child_module_ident: &child_module_ident,
            child_struct_ident: &child_struct_ident,
            is_many: false,
            is_required: child.is_required(),
          }));
        } else {
          body_stmts.push(load_child_stmt(LoadChildStmt {
            child_field_ident: &child_field_ident,
            path_expr: &single_paths_ident,
            contexts_expr: None,
            child_api_name: child.api_name(),
            path_resolve: &path_resolve,
            child_module_ident: &child_module_ident,
            child_struct_ident: &child_struct_ident,
            is_many: false,
            is_required: child.is_required(),
          }));
        }
      } else {
        fields.push(child_field_tokens(
          &child_field_ident,
          &child_type,
          false,
          child.is_required(),
        ));
        let resolve_child_paths =
          gen_collect_values_stmt(part, sdk_data_parts, child.source, &paths_ident, index)?;
        body_stmts.push(quote! {
          let mut #paths_ident: Vec<String> = vec![];
        });
        body_stmts.push(resolve_child_paths);
        if let Some(context_value_model) = context_value_model {
          let contexts_ident = format_ident!("{}_contexts", child.api_name().to_snake_case());
          let context_type = context_value_model.ty;
          let resolve_child_contexts = gen_collect_values_stmt(
            part,
            sdk_data_parts,
            child.context_source.unwrap(),
            &contexts_ident,
            index,
          )?;
          body_stmts.push(quote! {
            let mut #contexts_ident: Vec<#context_type> = vec![];
          });
          body_stmts.push(resolve_child_contexts);
          body_stmts.push(load_child_stmt(LoadChildStmt {
            child_field_ident: &child_field_ident,
            path_expr: &paths_ident,
            contexts_expr: Some(&contexts_ident),
            child_api_name: child.api_name(),
            path_resolve: &path_resolve,
            child_module_ident: &child_module_ident,
            child_struct_ident: &child_struct_ident,
            is_many: false,
            is_required: child.is_required(),
          }));
        } else {
          body_stmts.push(load_child_stmt(LoadChildStmt {
            child_field_ident: &child_field_ident,
            path_expr: &paths_ident,
            contexts_expr: None,
            child_api_name: child.api_name(),
            path_resolve: &path_resolve,
            child_module_ident: &child_module_ident,
            child_struct_ident: &child_struct_ident,
            is_many: false,
            is_required: child.is_required(),
          }));
        }
      }
      self_values.push(quote! {
        #child_field_ident
      });
    }
  }

  let _ = part.base;

  Ok(quote! {
    #[derive(Clone, Debug, Default)]
    pub struct #struct_ident {
      #( #fields )*
    }

    impl #struct_ident {
      pub(crate) fn new_from_archive<R: std::io::Read + std::io::Seek>(
        path: &str,
        #( #constructor_args )*
        archive: &mut zip::ZipArchive<R>,
      ) -> Result<Self, crate::common::SdkError> {
        #( #body_stmts )*

        Ok(Self {
          #( #self_values, )*
        })
      }

      pub(crate) fn save_zip<W: std::io::Write + std::io::Seek>(
        &self,
        zip: &mut zip::ZipWriter<W>,
        entry_set: &mut std::collections::HashSet<String>,
      ) -> Result<(), crate::common::SdkError> {
        #( #save_stmts )*
        #( #save_child_stmts )*
        Ok(())
      }
    }
  })
}

pub(crate) struct SchemaIndex<'a> {
  schemas_by_module: HashMap<&'a str, &'a Schema>,
  structs_by_qualified_type: HashMap<String, IndexedStruct<'a>>,
}

impl<'a> SchemaIndex<'a> {
  pub(crate) fn new(sdk_data_schemas: &'a [Schema]) -> Self {
    let mut schemas_by_module = HashMap::new();
    let mut structs_by_qualified_type = HashMap::new();

    for sdk_data_schema in sdk_data_schemas {
      schemas_by_module.insert(sdk_data_schema.module_name.as_str(), sdk_data_schema);

      for sdk_data_struct in &sdk_data_schema.types {
        let qualified_type = format!(
          "crate::schemas::{}::{}",
          sdk_data_schema.module_name, sdk_data_struct.ident
        );
        structs_by_qualified_type.insert(
          qualified_type,
          IndexedStruct {
            schema: sdk_data_schema,
            sdk_data_struct,
          },
        );
      }
    }

    Self {
      schemas_by_module,
      structs_by_qualified_type,
    }
  }

  fn resolve_root_type(
    &self,
    schema_module: &str,
    root_element: &str,
  ) -> anyhow::Result<TokenStream> {
    let sdk_data_schema = self
      .schemas_by_module
      .get(schema_module)
      .ok_or_else(|| anyhow!("missing schema module {}", schema_module))?;
    let sdk_data_element = sdk_data_schema
      .elements
      .iter()
      .find(|element| element.is_top_level && element.xml_name == root_element)
      .ok_or_else(|| anyhow!("missing root element {} in {}", root_element, schema_module))?;
    let qualified_type = qualify_schema_type(schema_module, &sdk_data_element.resolved_type);
    let ty: syn::Type = syn::parse_str(&qualified_type)?;

    Ok(quote! { #ty })
  }

  fn resolve_part_root_struct(&self, part: &PartDefinition) -> anyhow::Result<&IndexedStruct<'a>> {
    if part.content.as_ref() != Some(&PartContent::Xml) {
      bail!("blob part {} cannot be a parent path source", part.name);
    }

    let schema_module = part
      .schema_module
      .as_deref()
      .ok_or_else(|| anyhow!("xml part {} missing SchemaModule", part.name))?;
    let root_element = part
      .root_element
      .as_deref()
      .ok_or_else(|| anyhow!("xml part {} missing RootElement", part.name))?;
    let sdk_data_schema = self
      .schemas_by_module
      .get(schema_module)
      .ok_or_else(|| anyhow!("missing schema module {}", schema_module))?;
    let sdk_data_element = sdk_data_schema
      .elements
      .iter()
      .find(|element| element.is_top_level && element.xml_name == root_element)
      .ok_or_else(|| anyhow!("missing root element {} in {}", root_element, schema_module))?;
    let qualified_type = qualify_schema_type(schema_module, &sdk_data_element.resolved_type);
    self
      .structs_by_qualified_type
      .get(&qualified_type)
      .ok_or_else(|| anyhow!("missing struct model {}", qualified_type))
  }

  fn find_field(
    &self,
    indexed_struct: &IndexedStruct<'a>,
    field_name: &str,
  ) -> anyhow::Result<FieldModel<'a>> {
    if let Some(attribute) = indexed_struct
      .sdk_data_struct
      .attributes
      .iter()
      .find(|attribute| attribute.name == field_name)
    {
      return Ok(FieldModel::from_attribute(attribute));
    }

    if let Some(child) = indexed_struct
      .sdk_data_struct
      .sequences
      .iter()
      .find(|child| child.name == field_name)
    {
      return Ok(FieldModel::from_child(child));
    }

    if let Some(child) = indexed_struct
      .sdk_data_struct
      .children
      .iter()
      .find(|child| child.name == field_name)
    {
      return Ok(FieldModel::from_choice_child(child));
    }

    bail!(
      "missing field {} in {}::{}",
      field_name,
      indexed_struct.schema.module_name,
      indexed_struct.sdk_data_struct.ident
    )
  }

  fn resolve_field_struct(
    &self,
    indexed_struct: &IndexedStruct<'a>,
    field: &FieldModel<'a>,
  ) -> anyhow::Result<&IndexedStruct<'a>> {
    match field.type_kind {
      CodegenTypeKind::Struct => {
        let qualified_type =
          qualify_schema_type(&indexed_struct.schema.module_name, field.resolved_type);
        self
          .structs_by_qualified_type
          .get(&qualified_type)
          .ok_or_else(|| anyhow!("missing struct model {}", qualified_type))
      }
      _ => bail!(
        "field {} in {}::{} is not a struct",
        field.ident,
        indexed_struct.schema.module_name,
        indexed_struct.sdk_data_struct.ident
      ),
    }
  }

  fn resolve_field_path_struct(
    &self,
    part: &PartDefinition,
    field_path: &[String],
  ) -> anyhow::Result<&IndexedStruct<'a>> {
    let mut current_struct = self.resolve_part_root_struct(part)?;

    for field_name in field_path {
      let field = self.find_field(current_struct, field_name)?;
      current_struct = self.resolve_field_struct(current_struct, &field)?;
    }

    Ok(current_struct)
  }
}

fn qualify_schema_type(schema_module: &str, ty: &str) -> String {
  if ty.starts_with("crate::")
    || ty == "String"
    || ty == "bool"
    || ty == "f64"
    || ty == "i32"
    || ty == "u32"
  {
    ty.to_string()
  } else {
    format!("crate::schemas::{}::{}", schema_module, ty)
  }
}

struct IndexedStruct<'a> {
  schema: &'a Schema,
  sdk_data_struct: &'a Struct,
}

#[derive(Clone, Copy)]
enum WrapperKind {
  Single,
  Option,
  Vec,
}

struct FieldModel<'a> {
  name: &'a str,
  ident: &'a str,
  resolved_type: &'a str,
  type_kind: CodegenTypeKind,
  wrapper: WrapperKind,
  source_kind: FieldSourceKind,
}

impl<'a> FieldModel<'a> {
  fn from_attribute(attribute: &'a Attribute) -> Self {
    Self {
      name: &attribute.name,
      ident: &attribute.ident,
      resolved_type: &attribute.resolved_type,
      type_kind: attribute.type_kind.clone(),
      wrapper: if attribute.is_option {
        WrapperKind::Option
      } else {
        WrapperKind::Single
      },
      source_kind: FieldSourceKind::Attribute,
    }
  }

  fn from_child(child: &'a Child) -> Self {
    Self {
      name: &child.name,
      ident: &child.ident,
      resolved_type: &child.resolved_type,
      type_kind: child.type_kind.clone(),
      wrapper: if child.is_vec {
        WrapperKind::Vec
      } else if child.is_option {
        WrapperKind::Option
      } else {
        WrapperKind::Single
      },
      source_kind: FieldSourceKind::SequenceChild,
    }
  }

  fn from_choice_child(child: &'a Child) -> Self {
    Self {
      name: &child.name,
      ident: &child.ident,
      resolved_type: &child.resolved_type,
      type_kind: child.type_kind.clone(),
      wrapper: WrapperKind::Vec,
      source_kind: FieldSourceKind::ChoiceChild,
    }
  }
}

#[derive(Clone, Copy)]
enum FieldSourceKind {
  Attribute,
  SequenceChild,
  ChoiceChild,
}
