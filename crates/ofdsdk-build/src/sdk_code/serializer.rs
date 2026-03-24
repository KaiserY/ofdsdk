use proc_macro2::TokenStream;
use quote::quote;
use syn::{Arm, Expr, Ident, ItemFn, ItemImpl, Stmt, Type, parse_str, parse2};

use crate::models::sdk_data::{
  Child, CodegenTypeKind, Enum, Schema as SdkDataSchema, Struct as SdkDataStruct,
};
use crate::sdk_code::schemas::escape_ident;

pub fn gen_schema_serializer(
  sdk_data_schema: &SdkDataSchema,
  sdk_data_schemas: &[SdkDataSchema],
) -> anyhow::Result<TokenStream> {
  let mut impls: Vec<ItemImpl> = vec![];

  for e in &sdk_data_schema.enums {
    let (to_xml_impl, display_impl) = gen_enum_serializer(sdk_data_schema, e)?;
    impls.push(to_xml_impl);
    impls.push(display_impl);
  }

  for s in schema_types(sdk_data_schema) {
    let (impl_block, display_impl) = gen_struct_serializer(sdk_data_schema, sdk_data_schemas, s)?;
    impls.push(impl_block);
    impls.push(display_impl);
  }

  for element in &sdk_data_schema.elements {
    if let Some((impl_block, display_impl)) =
      gen_element_wrapper_serializer(sdk_data_schema, sdk_data_schemas, element)?
    {
      impls.push(impl_block);
      impls.push(display_impl);
    }
  }

  Ok(quote! {
    #( #impls )*
  })
}

fn gen_enum_serializer(
  sdk_data_schema: &SdkDataSchema,
  e: &Enum,
) -> anyhow::Result<(ItemImpl, ItemImpl)> {
  let enum_type: Type = parse_str(&format!(
    "crate::schemas::{}::{}",
    sdk_data_schema.module_name, e.ident
  ))?;

  let mut variants: Vec<Arm> = vec![];

  for variant in &e.variants {
    let variant_ident: Ident = parse_str(escape_ident(&variant.ident))?;
    let variant_value = variant.value.clone();

    variants.push(parse2(quote! {
      Self::#variant_ident => #variant_value,
    })?);
  }

  let impl_block = parse2(quote! {
    impl #enum_type {
      pub fn as_xml_str(&self) -> &'static str {
        match self {
          #( #variants )*
        }
      }

      pub fn to_xml(&self) -> String {
        self.as_xml_str().to_string()
      }
    }
  })?;

  let display_impl = parse2(quote! {
    impl std::fmt::Display for #enum_type {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_xml_str())
      }
    }
  })?;

  Ok((impl_block, display_impl))
}

fn gen_element_wrapper_serializer(
  sdk_data_schema: &SdkDataSchema,
  _sdk_data_schemas: &[SdkDataSchema],
  element: &crate::models::sdk_data::Element,
) -> anyhow::Result<Option<(ItemImpl, ItemImpl)>> {
  if element.r#type == element.ident
    || !is_public_struct_wrapper_candidate(sdk_data_schema, element)
  {
    return Ok(None);
  }

  let wrapper_type: Type = parse_str(&format!(
    "crate::schemas::{}::{}",
    sdk_data_schema.module_name, element.ident
  ))?;
  let inner_type: Type = parse_str(&element.resolved_type)?;
  let xml_name = element.xml_name.clone();

  let impl_block = parse2(quote! {
    impl #wrapper_type {
      pub fn to_xml(&self) -> Result<String, std::fmt::Error> {
        let mut writer = String::with_capacity(64);

        self.write_xml(&mut writer, true)?;

        Ok(writer)
      }

      pub(crate) fn write_xml<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        with_xmlns: bool,
      ) -> Result<(), std::fmt::Error> {
        self.write_xml_named(writer, with_xmlns, #xml_name)
      }

      pub(crate) fn write_xml_named<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        with_xmlns: bool,
        tag_name: &str,
      ) -> Result<(), std::fmt::Error> {
        <#inner_type>::write_xml_named(&self.0, writer, with_xmlns, tag_name)
      }
    }
  })?;

  let display_impl = parse2(quote! {
    impl std::fmt::Display for #wrapper_type {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_xml(f, true)
      }
    }
  })?;

  Ok(Some((impl_block, display_impl)))
}

fn gen_struct_serializer(
  sdk_data_schema: &SdkDataSchema,
  sdk_data_schemas: &[SdkDataSchema],
  s: &SdkDataStruct,
) -> anyhow::Result<(ItemImpl, ItemImpl)> {
  let struct_type: Type = parse_str(&format!(
    "crate::schemas::{}::{}",
    sdk_data_schema.module_name, s.ident
  ))?;
  let struct_xml_name = s.resolved_xml_name.as_str();
  let has_content = !s.sequences.is_empty() || !s.children.is_empty();

  let mut attr_stmts: Vec<Stmt> = vec![];
  let mut child_stmts: Vec<Stmt> = vec![];
  let mut child_arms: Vec<Arm> = vec![];
  let mut xml_value_stmt: Option<Stmt> = None;
  let mut xml_children_stmt: Option<Stmt> = None;

  for attr in &s.attributes {
    attr_stmts.push(gen_attr_stmt(
      &attr.ident,
      &attr.type_kind,
      attr.is_option,
      &attr.name,
    )?);
  }

  for child in &s.sequences {
    if child.ident == "xml_value" {
      xml_value_stmt = Some(gen_xml_value_stmt(
        &child.ident,
        &child.type_kind,
        child.is_option,
      )?);
      continue;
    }

    child_stmts.push(gen_sequence_stmt(sdk_data_schema, sdk_data_schemas, child)?);
  }

  for child in &s.children {
    child_arms.push(gen_choice_arm(sdk_data_schema, sdk_data_schemas, s, child)?);
  }

  if !s.children.is_empty() {
    xml_children_stmt = Some(parse2(quote! {
      for child in &self.xml_children {
        match child {
          #( #child_arms )*
        }
      }
    })?);
  }

  let to_xml_fn: ItemFn = parse2(quote! {
    pub fn to_xml(&self) -> Result<String, std::fmt::Error> {
      let mut writer = String::with_capacity(64);

      self.write_xml(&mut writer, true)?;

      Ok(writer)
    }
  })?;

  let write_xml_fn: ItemFn = parse2(quote! {
    pub(crate) fn write_xml<W: std::fmt::Write>(
      &self,
      writer: &mut W,
      with_xmlns: bool,
    ) -> Result<(), std::fmt::Error> {
      self.write_xml_named(writer, with_xmlns, #struct_xml_name)
    }
  })?;

  let write_xml_named_fn: ItemFn = if has_content {
    parse2(quote! {
      pub(crate) fn write_xml_named<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        with_xmlns: bool,
        tag_name: &str,
      ) -> Result<(), std::fmt::Error> {
        writer.write_char('<')?;
        writer.write_str("ofd:")?;
        writer.write_str(tag_name)?;

        if with_xmlns {
          writer.write_str(r#" xmlns:ofd="http://www.ofdspec.org/2016""#)?;
        }

        #( #attr_stmts )*

        writer.write_char('>')?;

        #xml_value_stmt

        #( #child_stmts )*

        #xml_children_stmt

        writer.write_str("</ofd:")?;
        writer.write_str(tag_name)?;
        writer.write_char('>')?;

        Ok(())
      }
    })?
  } else {
    parse2(quote! {
      pub(crate) fn write_xml_named<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        with_xmlns: bool,
        tag_name: &str,
      ) -> Result<(), std::fmt::Error> {
        writer.write_char('<')?;
        writer.write_str("ofd:")?;
        writer.write_str(tag_name)?;

        if with_xmlns {
          writer.write_str(r#" xmlns:ofd="http://www.ofdspec.org/2016""#)?;
        }

        #( #attr_stmts )*

        writer.write_str("/>")?;

        Ok(())
      }
    })?
  };

  let impl_block = parse2(quote! {
    impl #struct_type {
      #to_xml_fn

      #write_xml_fn

      #write_xml_named_fn
    }
  })?;

  let display_impl = parse2(quote! {
    impl std::fmt::Display for #struct_type {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_xml(f, true)
      }
    }
  })?;

  Ok((impl_block, display_impl))
}

fn gen_scalar_value_expr(
  type_kind: &CodegenTypeKind,
  value: Expr,
  is_ref: bool,
) -> anyhow::Result<Expr> {
  if is_ref && matches!(type_kind, CodegenTypeKind::Bool) {
    Ok(parse2(quote! { *#value })?)
  } else {
    Ok(value)
  }
}

fn gen_scalar_write_expr(type_kind: &CodegenTypeKind, value: Expr) -> anyhow::Result<Stmt> {
  Ok(match type_kind {
    CodegenTypeKind::String => parse2(quote! {
      writer.write_str(&quick_xml::escape::escape(#value.as_str()))?;
    })?,
    CodegenTypeKind::Bool => parse2(quote! {
      writer.write_str(if #value { "true" } else { "false" })?;
    })?,
    CodegenTypeKind::F64 | CodegenTypeKind::I32 | CodegenTypeKind::U32 => parse2(quote! {
      write!(writer, "{}", #value)?;
    })?,
    CodegenTypeKind::Enum => parse2(quote! {
      writer.write_str(#value.as_xml_str())?;
    })?,
    CodegenTypeKind::Struct => anyhow::bail!("scalar write type cannot be struct"),
  })
}

fn gen_attr_stmt(
  attr_ident: &str,
  type_kind: &CodegenTypeKind,
  is_option: bool,
  attr_name: &str,
) -> anyhow::Result<Stmt> {
  let attr_ident: Ident = parse_str(escape_ident(attr_ident))?;
  let attr_name_fmt = format!(" {attr_name}=\"");

  if is_option {
    let value = gen_scalar_value_expr(type_kind, parse2(quote! { #attr_ident })?, true)?;
    let write_value = gen_scalar_write_expr(type_kind, value)?;
    Ok(parse2(quote! {
      if let Some(#attr_ident) = &self.#attr_ident {
        writer.write_str(#attr_name_fmt)?;
        #write_value
        writer.write_char('"')?;
      }
    })?)
  } else {
    let value = gen_scalar_value_expr(type_kind, parse2(quote! { self.#attr_ident })?, false)?;
    let write_value = gen_scalar_write_expr(type_kind, value)?;
    Ok(parse2(quote! {
      {
        writer.write_str(#attr_name_fmt)?;
        #write_value
        writer.write_char('"')?;
      }
    })?)
  }
}

fn gen_xml_value_stmt(
  child_ident: &str,
  type_kind: &CodegenTypeKind,
  is_option: bool,
) -> anyhow::Result<Stmt> {
  let child_ident: Ident = parse_str(escape_ident(child_ident))?;

  if is_option {
    let value = gen_scalar_value_expr(type_kind, parse2(quote! { #child_ident })?, true)?;
    let write_value = gen_scalar_write_expr(type_kind, value)?;
    Ok(parse2(quote! {
      if let Some(#child_ident) = &self.#child_ident {
        #write_value
      }
    })?)
  } else {
    let value = gen_scalar_value_expr(type_kind, parse2(quote! { self.#child_ident })?, false)?;
    let write_value = gen_scalar_write_expr(type_kind, value)?;
    Ok(parse2(quote! {
      #write_value
    })?)
  }
}

fn gen_sequence_stmt(
  _sdk_data_schema: &SdkDataSchema,
  _sdk_data_schemas: &[SdkDataSchema],
  child: &Child,
) -> anyhow::Result<Stmt> {
  let child_ident: Ident = parse_str(escape_ident(&child.ident))?;
  let child_xml_name = child.resolved_xml_name.as_str();

  if child.is_struct {
    if child.is_vec {
      Ok(parse2(quote! {
        for child in &self.#child_ident {
          child.write_xml_named(writer, false, #child_xml_name)?;
        }
      })?)
    } else if child.is_option {
      Ok(parse2(quote! {
        if let Some(#child_ident) = &self.#child_ident {
          #child_ident.write_xml_named(writer, false, #child_xml_name)?;
        }
      })?)
    } else {
      Ok(parse2(quote! {
        self.#child_ident.write_xml_named(writer, false, #child_xml_name)?;
      })?)
    }
  } else {
    let child_name_prefix = format!("ofd:{}", child_xml_name);
    let child_end_tag = format!("</ofd:{}>", child_xml_name);

    if child.is_vec {
      let value = gen_scalar_value_expr(&child.type_kind, parse2(quote! { child })?, true)?;
      let write_value = gen_scalar_write_expr(&child.type_kind, value)?;
      Ok(parse2(quote! {
        for child in &self.#child_ident {
          writer.write_char('<')?;
          writer.write_str(#child_name_prefix)?;
          writer.write_char('>')?;
          #write_value
          writer.write_str(#child_end_tag)?;
        }
      })?)
    } else if child.is_option {
      let value = gen_scalar_value_expr(&child.type_kind, parse2(quote! { #child_ident })?, true)?;
      let write_value = gen_scalar_write_expr(&child.type_kind, value)?;
      Ok(parse2(quote! {
        if let Some(#child_ident) = &self.#child_ident {
          writer.write_char('<')?;
          writer.write_str(#child_name_prefix)?;
          writer.write_char('>')?;
          #write_value
          writer.write_str(#child_end_tag)?;
        }
      })?)
    } else {
      let value = gen_scalar_value_expr(
        &child.type_kind,
        parse2(quote! { self.#child_ident })?,
        false,
      )?;
      let write_value = gen_scalar_write_expr(&child.type_kind, value)?;
      Ok(parse2(quote! {
        {
          writer.write_char('<')?;
          writer.write_str(#child_name_prefix)?;
          writer.write_char('>')?;
          #write_value
          writer.write_str(#child_end_tag)?;
        }
      })?)
    }
  }
}

fn gen_choice_arm(
  sdk_data_schema: &SdkDataSchema,
  _sdk_data_schemas: &[SdkDataSchema],
  s: &SdkDataStruct,
  child: &Child,
) -> anyhow::Result<Arm> {
  let child_choice_enum_type: Type = parse_str(&format!(
    "crate::schemas::{}::{}ContentChoice",
    sdk_data_schema.module_name, s.ident
  ))?;
  let child_variant_ident: Ident = parse_str(escape_ident(&child.ident))?;
  let child_xml_name = child.resolved_xml_name.as_str();
  let child_name_prefix = format!("ofd:{}", child_xml_name);
  let child_end_tag = format!("</ofd:{}>", child_xml_name);

  if child.is_struct {
    Ok(parse2(quote! {
      #child_choice_enum_type::#child_variant_ident(child) => child.write_xml_named(writer, false, #child_xml_name)?,
    })?)
  } else {
    let value = gen_scalar_value_expr(&child.type_kind, parse2(quote! { child })?, true)?;
    let write_value = gen_scalar_write_expr(&child.type_kind, value)?;
    Ok(parse2(quote! {
      #child_choice_enum_type::#child_variant_ident(child) => {
        writer.write_char('<')?;
        writer.write_str(#child_name_prefix)?;
        writer.write_char('>')?;
        #write_value
        writer.write_str(#child_end_tag)?;
      },
    })?)
  }
}

fn schema_types(sdk_data_schema: &SdkDataSchema) -> &[SdkDataStruct] {
  &sdk_data_schema.types
}

fn is_public_struct_wrapper_candidate(
  _sdk_data_schema: &SdkDataSchema,
  element: &crate::models::sdk_data::Element,
) -> bool {
  matches!(
    element.api_kind,
    crate::models::sdk_data::ElementApiKind::StructWrapper
  )
}
