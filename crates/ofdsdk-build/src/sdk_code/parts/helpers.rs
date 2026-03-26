use super::*;
use syn::{Type, parse_str};

use crate::models::sdk_data::{PartChild, PartSource, PartSourceRoot};

pub(super) fn collect_child_parts<'a>(
  part: &'a PartDefinition,
  sdk_data_parts: &'a [PartDefinition],
) -> anyhow::Result<Vec<ResolvedPartChild<'a>>> {
  let mut child_parts = vec![];

  for child in &part.children {
    let child_part = sdk_data_parts
      .iter()
      .find(|child_part| child_part.name == child.name)
      .ok_or_else(|| anyhow!("missing child part {}", child.name))?;
    let source = child
      .from
      .as_ref()
      .ok_or_else(|| anyhow!("child part {} missing Children[].From", child_part.name))?;
    if source.parent != part.name {
      bail!(
        "child part {} is attached under {}, but From.Parent is {}",
        child_part.name,
        part.name,
        source.parent
      );
    }
    if let Some(context_from) = child.context_from.as_ref()
      && context_from.parent != part.name
    {
      bail!(
        "child part {} is attached under {}, but ContextFrom.Parent is {}",
        child_part.name,
        part.name,
        context_from.parent
      );
    }
    child_parts.push(ResolvedPartChild {
      spec: child,
      part: child_part,
      source,
      context_source: child.context_from.as_ref(),
    });
  }

  Ok(child_parts)
}

pub(super) struct ResolvedPartChild<'a> {
  spec: &'a PartChild,
  pub(super) part: &'a PartDefinition,
  pub(super) source: &'a PartSource,
  pub(super) context_source: Option<&'a PartSource>,
}

impl<'a> ResolvedPartChild<'a> {
  pub(super) fn api_name(&self) -> &str {
    &self.spec.api_name
  }

  pub(super) fn part_name(&self) -> &str {
    &self.part.name
  }

  pub(super) fn is_many(&self) -> bool {
    self.spec.max_occurs_great_than_one
  }

  pub(super) fn is_required(&self) -> bool {
    self.spec.min_occurs_is_non_zero
  }
}

pub(super) fn gen_first_path_expr_from_source(
  part: &PartDefinition,
  sdk_data_parts: &[PartDefinition],
  source: &PartSource,
  index: &SchemaIndex<'_>,
) -> anyhow::Result<Option<TokenStream>> {
  let (root_struct, current_expr) = resolve_source_root(part, sdk_data_parts, source, index)?;
  gen_first_path_expr(root_struct, current_expr, &source.field_path, index)
}

fn gen_first_path_expr_from_path(
  part: &PartDefinition,
  sdk_data_parts: &[PartDefinition],
  field_path: &[String],
  index: &SchemaIndex<'_>,
) -> anyhow::Result<Option<TokenStream>> {
  let source = PartSource {
    parent: String::new(),
    field_path: field_path.to_vec(),
    is_vec: false,
    source_root: PartSourceRoot::RootElement,
  };
  gen_first_path_expr_from_source(part, sdk_data_parts, &source, index)
}

fn gen_first_path_expr(
  current_struct: &IndexedStruct<'_>,
  current_expr: TokenStream,
  field_path: &[String],
  index: &SchemaIndex<'_>,
) -> anyhow::Result<Option<TokenStream>> {
  let field_name = match field_path.first() {
    Some(field_name) => field_name,
    None => return Ok(None),
  };
  let field = index.find_field(current_struct, field_name)?;

  if matches!(field.source_kind, FieldSourceKind::ChoiceChild) {
    return Ok(None);
  }

  let field_ident = format_ident!("{}", field.ident);
  let field_expr = quote! { #current_expr.#field_ident };

  if field_path.len() == 1 {
    return Ok(match field.wrapper {
      WrapperKind::Single => Some(quote! { #field_expr.clone() }),
      WrapperKind::Option | WrapperKind::Vec => None,
    });
  }

  if !matches!(field.wrapper, WrapperKind::Single) {
    return Ok(None);
  }

  let next_struct = index.resolve_field_struct(current_struct, &field)?;
  gen_first_path_expr(next_struct, field_expr, &field_path[1..], index)
}

pub(super) fn gen_part_path_expr(
  parent_part: &PartDefinition,
  sdk_data_parts: &[PartDefinition],
  child_part: &PartDefinition,
  source_root: PartSourceRoot,
  child_path_expr: &TokenStream,
  index: &SchemaIndex<'_>,
) -> anyhow::Result<TokenStream> {
  Ok(match &child_part.path {
    PartPath::ReferenceField => match source_root {
      PartSourceRoot::RootElement => quote! {
        crate::common::resolve_zip_child_path(&current_dir, #child_path_expr)
      },
      PartSourceRoot::ContextElement => quote! {
        crate::common::resolve_zip_file_path(#child_path_expr)
      },
    },
    PartPath::ReferenceFieldWithBase { base_field_path } => {
      let child_name = child_part.name.clone();
      if let Some(base_path_expr) =
        gen_first_path_expr_from_path(parent_part, sdk_data_parts, base_field_path, index)?
      {
        quote! {{
          let base_path = #base_path_expr;
          let base_dir = crate::common::resolve_zip_child_path(&current_dir, &base_path);
          crate::common::resolve_zip_child_path(&format!("{base_dir}/"), #child_path_expr)
        }}
      } else {
        let base_paths_ident = format_ident!("{}_base_paths", child_part.name.to_snake_case());
        let base_collect_stmt = gen_collect_values_stmt_from_path(
          parent_part,
          sdk_data_parts,
          PartSourceRoot::RootElement,
          base_field_path,
          &base_paths_ident,
          index,
        )?;
        quote! {{
          let mut #base_paths_ident: Vec<String> = vec![];
          #base_collect_stmt
          let base_path = #base_paths_ident.into_iter().next().ok_or_else(|| {
            crate::common::SdkError::CommonError(format!("missing base path for {}", #child_name))
          })?;
          let base_dir = crate::common::resolve_zip_child_path(&current_dir, &base_path);
          crate::common::resolve_zip_child_path(&format!("{base_dir}/"), #child_path_expr)
        }}
      }
    }
    PartPath::Fixed { path } => {
      quote! {
        crate::common::resolve_zip_file_path(#path)
      }
    }
  })
}

pub(super) fn gen_collect_values_stmt(
  parent_part: &PartDefinition,
  sdk_data_parts: &[PartDefinition],
  child_source: &PartSource,
  target_ident: &syn::Ident,
  index: &SchemaIndex<'_>,
) -> anyhow::Result<TokenStream> {
  gen_collect_values_stmt_from_path(
    parent_part,
    sdk_data_parts,
    child_source.source_root.clone(),
    &child_source.field_path,
    target_ident,
    index,
  )
}

fn gen_collect_values_stmt_from_path(
  parent_part: &PartDefinition,
  sdk_data_parts: &[PartDefinition],
  source_root: PartSourceRoot,
  field_path: &[String],
  target_ident: &syn::Ident,
  index: &SchemaIndex<'_>,
) -> anyhow::Result<TokenStream> {
  let (root_struct, current_expr) =
    resolve_collect_root(parent_part, sdk_data_parts, source_root, index)?;
  let mut counter = 0usize;
  gen_collect_steps(
    root_struct,
    current_expr,
    field_path,
    target_ident,
    &mut counter,
    index,
  )
}

pub(super) fn gen_required_single_value_expr_from_source(
  part: &PartDefinition,
  sdk_data_parts: &[PartDefinition],
  source: &PartSource,
  child_api_name: &str,
  index: &SchemaIndex<'_>,
) -> anyhow::Result<TokenStream> {
  let source_expr = gen_first_path_expr_from_source(part, sdk_data_parts, source, index)?
    .ok_or_else(|| anyhow!("missing single value source for {}", child_api_name))?;
  Ok(source_expr)
}

fn resolve_collect_root<'a>(
  part: &'a PartDefinition,
  sdk_data_parts: &'a [PartDefinition],
  source_root: PartSourceRoot,
  index: &'a SchemaIndex<'a>,
) -> anyhow::Result<(&'a IndexedStruct<'a>, TokenStream)> {
  match source_root {
    PartSourceRoot::RootElement => Ok((
      index.resolve_part_root_struct(part)?,
      quote! { root_element },
    )),
    PartSourceRoot::ContextElement => {
      let context_model = resolve_part_context_model(
        part,
        sdk_data_parts,
        part
          .context_from
          .as_ref()
          .ok_or_else(|| anyhow!("part {} missing ContextFrom", part.name))?,
        index,
      )?;
      let context_field_ident = format_ident!("{}", context_model.field_ident);
      Ok((
        context_model.indexed_struct,
        quote! { #context_field_ident },
      ))
    }
  }
}

fn resolve_source_root<'a>(
  part: &'a PartDefinition,
  sdk_data_parts: &'a [PartDefinition],
  source: &PartSource,
  index: &'a SchemaIndex<'a>,
) -> anyhow::Result<(&'a IndexedStruct<'a>, TokenStream)> {
  resolve_collect_root(part, sdk_data_parts, source.source_root.clone(), index)
}

pub(super) struct PartContextModel<'a> {
  pub(super) field_ident: String,
  pub(super) ty: Type,
  pub(super) indexed_struct: &'a IndexedStruct<'a>,
}

pub(super) fn resolve_part_context_model<'a>(
  part: &'a PartDefinition,
  sdk_data_parts: &'a [PartDefinition],
  context_from: &PartSource,
  index: &'a SchemaIndex<'a>,
) -> anyhow::Result<PartContextModel<'a>> {
  let field_name = context_from
    .field_path
    .last()
    .ok_or_else(|| anyhow!("part {} has empty ContextFrom", part.name))?;
  let parent_part = sdk_data_parts
    .iter()
    .find(|candidate| candidate.name == context_from.parent)
    .ok_or_else(|| anyhow!("missing context parent part {}", context_from.parent))?;
  let indexed_struct = index.resolve_field_path_struct(parent_part, &context_from.field_path)?;
  let qualified_type = format!(
    "crate::schemas::{}::{}",
    indexed_struct.schema.module_name, indexed_struct.sdk_data_struct.ident
  );
  let ty: Type = parse_str(&qualified_type)?;

  Ok(PartContextModel {
    field_ident: field_name.to_snake_case(),
    ty,
    indexed_struct,
  })
}

fn gen_collect_steps(
  current_struct: &IndexedStruct<'_>,
  current_expr: TokenStream,
  field_path: &[String],
  target_ident: &syn::Ident,
  counter: &mut usize,
  index: &SchemaIndex<'_>,
) -> anyhow::Result<TokenStream> {
  let field_name = field_path
    .first()
    .ok_or_else(|| anyhow!("empty field path"))?;
  let field = index.find_field(current_struct, field_name)?;
  let field_ident = format_ident!("{}", field.ident);
  let field_expr = quote! { #current_expr.#field_ident };

  if matches!(field.source_kind, FieldSourceKind::ChoiceChild) {
    let choice_type_ident = format_ident!("{}ContentChoice", current_struct.sdk_data_struct.ident);
    let choice_variant_ident = format_ident!("{}", field.name.to_upper_camel_case());
    let module_ident = format_ident!("{}", current_struct.schema.module_name);

    if field_path.len() == 1 {
      return Ok(quote! {
        for choice in &#current_expr.xml_children {
          if let crate::schemas::#module_ident::#choice_type_ident::#choice_variant_ident(value) = choice {
            #target_ident.push((**value).clone());
          }
        }
      });
    }

    let next_struct = index.resolve_field_struct(current_struct, &field)?;
    let next_ident = format_ident!("value_{}", *counter);
    *counter += 1;
    let inner = gen_collect_steps(
      next_struct,
      quote! { #next_ident },
      &field_path[1..],
      target_ident,
      counter,
      index,
    )?;

    return Ok(quote! {
      for choice in &#current_expr.xml_children {
        if let crate::schemas::#module_ident::#choice_type_ident::#choice_variant_ident(value) = choice {
          let #next_ident = &**value;
          #inner
        }
      }
    });
  }

  if field_path.len() == 1 {
    return Ok(match field.wrapper {
      WrapperKind::Single => {
        quote! {
          #target_ident.push(#field_expr.clone());
        }
      }
      WrapperKind::Option => {
        quote! {
          if let Some(value) = &#field_expr {
            #target_ident.push(value.clone());
          }
        }
      }
      WrapperKind::Vec => {
        quote! {
          for value in &#field_expr {
            #target_ident.push(value.clone());
          }
        }
      }
    });
  }

  let next_struct = index.resolve_field_struct(current_struct, &field)?;
  let next_ident = format_ident!("value_{}", *counter);
  *counter += 1;
  let inner = gen_collect_steps(
    next_struct,
    quote! { #next_ident },
    &field_path[1..],
    target_ident,
    counter,
    index,
  )?;

  Ok(match field.wrapper {
    WrapperKind::Single => {
      quote! {
        let #next_ident = &#field_expr;
        #inner
      }
    }
    WrapperKind::Option => {
      quote! {
        if let Some(#next_ident) = &#field_expr {
          #inner
        }
      }
    }
    WrapperKind::Vec => {
      quote! {
        for #next_ident in &#field_expr {
          #inner
        }
      }
    }
  })
}
