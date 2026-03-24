use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, Ident, Type, Variant, parse::Parser, parse_str, parse2};

use crate::models::sdk_data::{
  CodegenTypeKind, ElementApiKind, Schema as SdkDataSchema, SchemaComment, SchemaDocBlockKind,
};

pub fn gen_schema(
  sdk_data_schema: &SdkDataSchema,
  schema_comments: &[SchemaComment],
) -> anyhow::Result<TokenStream> {
  let mut token_stream_list: Vec<TokenStream> = vec![];

  for s in schema_types(sdk_data_schema) {
    let struct_ident: Ident = parse_str(escape_ident(&s.ident))?;
    let struct_doc_attrs = gen_doc_attrs(
      schema_comments,
      &sdk_data_schema.module_name,
      &s.ident,
      None,
    );

    let mut attrs: Vec<Field> = vec![];

    for attr in &s.attributes {
      let attr_ident: Ident = parse_str(escape_ident(&attr.ident))?;
      let attr_doc_attrs = gen_doc_attrs(
        schema_comments,
        &sdk_data_schema.module_name,
        &s.ident,
        Some(&attr.name),
      );

      let attr_ty: Type = if attr.is_option {
        parse_str(&format!("Option<{}>", attr.r#type))?
      } else {
        parse_str(&attr.r#type)?
      };

      attrs.push(Field::parse_named.parse2(quote! {
        #( #attr_doc_attrs )*
        pub #attr_ident: #attr_ty
      })?);
    }

    for item in &s.sequences {
      let item_ident: Ident = parse_str(escape_ident(&item.ident))?;
      let item_doc_attrs = gen_doc_attrs(
        schema_comments,
        &sdk_data_schema.module_name,
        &s.ident,
        Some(&item.name),
      );

      let item_ty: Type = if item.is_option {
        parse_str(&format!("Option<{}>", item.r#type))?
      } else if item.is_vec {
        parse_str(&format!("Vec<{}>", item.r#type))?
      } else {
        parse_str(&item.r#type)?
      };

      attrs.push(Field::parse_named.parse2(quote! {
        #( #item_doc_attrs )*
        pub #item_ident: #item_ty
      })?);
    }

    if !s.children.is_empty() {
      let children_ty: Type = parse_str(&format!("{}ContentChoice", s.ident))?;

      attrs.push(Field::parse_named.parse2(quote! {
        pub xml_children: Vec<#children_ty>
      })?);

      let mut variants: Vec<Variant> = vec![];

      for child in &s.children {
        let variant_ident: Ident = parse_str(escape_ident(&child.ident))?;

        let variant_ty: Type = parse_str(&child.r#type)?;

        variants.push(parse2(quote! {
          #variant_ident(Box<#variant_ty>)
        })?);
      }

      token_stream_list.push(quote! {
        #[derive(Clone, Debug)]
        pub enum #children_ty {
          #( #variants, )*
        }
      });
    }

    token_stream_list.push(quote! {
      #( #struct_doc_attrs )*
      #[derive(Clone, Debug, Default)]
      pub struct #struct_ident {
        #( #attrs, )*
      }
    });
  }

  for a in &sdk_data_schema.aliases {
    let alias_ident: Ident = parse_str(escape_ident(&a.ident))?;
    let alias_ty: Type = parse_str(&a.r#type)?;
    let alias_doc_attrs = gen_doc_attrs(
      schema_comments,
      &sdk_data_schema.module_name,
      &a.ident,
      None,
    );

    token_stream_list.push(quote! {
      #( #alias_doc_attrs )*
      pub type #alias_ident = #alias_ty;
    });
  }

  for element in &sdk_data_schema.elements {
    if element.r#type == element.ident {
      continue;
    }

    if matches!(
      element.type_kind,
      CodegenTypeKind::String
        | CodegenTypeKind::Bool
        | CodegenTypeKind::F64
        | CodegenTypeKind::I32
        | CodegenTypeKind::U32
    ) {
      continue;
    }

    let element_ident: Ident = parse_str(escape_ident(&element.ident))?;
    let element_ty: Type = parse_str(&element.r#type)?;

    match element.api_kind {
      ElementApiKind::EnumAlias => {
        let element_doc_attrs = gen_doc_attrs(
          schema_comments,
          &sdk_data_schema.module_name,
          &element.ident,
          None,
        );
        token_stream_list.push(quote! {
          #( #element_doc_attrs )*
          pub type #element_ident = #element_ty;
        });
      }
      ElementApiKind::StructWrapper => {
        let element_doc_attrs = gen_doc_attrs(
          schema_comments,
          &sdk_data_schema.module_name,
          &element.ident,
          None,
        );
        token_stream_list.push(quote! {
          #( #element_doc_attrs )*
          #[derive(Clone, Debug, Default)]
          pub struct #element_ident(pub #element_ty);

          impl From<#element_ty> for #element_ident {
            fn from(value: #element_ty) -> Self {
              Self(value)
            }
          }

          impl From<#element_ident> for #element_ty {
            fn from(value: #element_ident) -> Self {
              value.0
            }
          }

          impl std::ops::Deref for #element_ident {
            type Target = #element_ty;

            fn deref(&self) -> &Self::Target {
              &self.0
            }
          }

          impl std::ops::DerefMut for #element_ident {
            fn deref_mut(&mut self) -> &mut Self::Target {
              &mut self.0
            }
          }
        });
      }
      ElementApiKind::None => {}
    }
  }

  for e in &sdk_data_schema.enums {
    let enum_ident: Ident = parse_str(escape_ident(&e.ident))?;
    let enum_doc_attrs = gen_doc_attrs(
      schema_comments,
      &sdk_data_schema.module_name,
      &e.ident,
      None,
    );

    let mut variants: Vec<Variant> = vec![];

    for (i, v) in e.variants.iter().enumerate() {
      let variant_ident: Ident = parse_str(escape_ident(&v.ident))?;

      if e.default_index == i {
        variants.push(parse2(quote! {
          #[default]
          #variant_ident
        })?);
      } else {
        variants.push(parse2(quote! {
          #variant_ident
        })?);
      }
    }

    token_stream_list.push(quote! {
      #( #enum_doc_attrs )*
      #[derive(Clone, Debug, Default)]
      pub enum #enum_ident {
        #( #variants, )*
      }
    });
  }

  Ok(quote! {
    #( #token_stream_list )*
  })
}

fn schema_types(sdk_data_schema: &SdkDataSchema) -> &[crate::models::sdk_data::Struct] {
  &sdk_data_schema.types
}

fn gen_doc_attrs(
  schema_comments: &[SchemaComment],
  schema: &str,
  type_name: &str,
  field_name: Option<&str>,
) -> Vec<TokenStream> {
  let Some(entry) = schema_comments.iter().find(|entry| {
    entry.schema == schema && entry.type_name == type_name && entry.field.as_deref() == field_name
  }) else {
    return vec![];
  };

  let mut attrs = vec![];

  for (index, doc) in entry.docs.iter().enumerate() {
    if index > 0 {
      attrs.push(quote! {
        #[doc = ""]
      });
    }

    let prefix = match doc.kind {
      SchemaDocBlockKind::Description => None,
      SchemaDocBlockKind::Example => Some("示例："),
    };

    if let Some(prefix) = prefix {
      attrs.push(quote! {
        #[doc = #prefix]
      });
    }

    for line in doc.text.lines() {
      attrs.push(quote! {
        #[doc = #line]
      });
    }
  }

  attrs
}

pub fn escape_ident(i: &str) -> &str {
  match i {
    "abstract" => "r#abstract",
    "type" => "r#type",
    _ => i,
  }
}
