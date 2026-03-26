use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Arm, Ident, ItemFn, ItemImpl, LitByteStr, Stmt, Type, parse_str, parse2};

use crate::models::sdk_data::{
  Attribute, Child, CodegenTypeKind, CompatibilityAction, CompatibilityRule, ElementApiKind, Enum,
  Schema as SdkDataSchema, Struct as SdkDataStruct,
};
use crate::sdk_code::schemas::escape_ident;
use crate::sdk_data::compatibility::{enum_value_alias_rules, find_missing_attribute_rule};

#[derive(Clone, Copy)]
enum DeserializeMode {
  Slice,
  Io,
}

pub fn gen_schema_deserializer(
  sdk_data_schema: &SdkDataSchema,
  sdk_data_schemas: &[SdkDataSchema],
  compatibility_rules: &[CompatibilityRule],
) -> anyhow::Result<TokenStream> {
  let mut impls: Vec<ItemImpl> = vec![];

  for e in &sdk_data_schema.enums {
    let (from_str_impl, impl_block) =
      gen_enum_deserializer(sdk_data_schema, e, compatibility_rules)?;
    impls.push(from_str_impl);
    impls.push(impl_block);
  }

  for s in schema_types(sdk_data_schema) {
    let (from_str_impl, impl_block) =
      gen_struct_deserializer(sdk_data_schema, sdk_data_schemas, s, compatibility_rules)?;
    impls.push(from_str_impl);
    impls.push(impl_block);
  }

  for element in &sdk_data_schema.elements {
    if let Some((from_str_impl, impl_block)) =
      gen_element_wrapper_deserializer(sdk_data_schema, sdk_data_schemas, element)?
    {
      impls.push(from_str_impl);
      impls.push(impl_block);
    }
  }

  Ok(quote! {
    #( #impls )*
  })
}

fn gen_enum_deserializer(
  sdk_data_schema: &SdkDataSchema,
  e: &Enum,
  compatibility_rules: &[CompatibilityRule],
) -> anyhow::Result<(ItemImpl, ItemImpl)> {
  let enum_type: Type = parse_str(&format!(
    "crate::schemas::{}::{}",
    sdk_data_schema.module_name, e.ident
  ))?;

  let mut str_variants: Vec<Arm> = vec![];
  let mut byte_variants: Vec<Arm> = vec![];
  let enum_name = syn::LitStr::new(&e.ident, Span::call_site());

  for variant in &e.variants {
    let variant_ident: Ident = parse_str(escape_ident(&variant.ident))?;
    let value_literal = variant.value.clone();
    let value_bytes = LitByteStr::new(variant.value.as_bytes(), Span::call_site());

    str_variants.push(parse2(quote! {
      #value_literal => Ok(Self::#variant_ident),
    })?);

    byte_variants.push(parse2(quote! {
      #value_bytes => Ok(Self::#variant_ident),
    })?);
  }

  for rule in enum_value_alias_rules(compatibility_rules, &sdk_data_schema.module_name, &e.ident) {
    let CompatibilityAction::EnumValueAlias { input, canonical } = &rule.action else {
      continue;
    };
    let input_literal = input.clone();
    let value_bytes = LitByteStr::new(input.as_bytes(), Span::call_site());

    str_variants.push(parse2(quote! {
      #input_literal => Self::from_str(#canonical),
    })?);

    byte_variants.push(parse2(quote! {
      #value_bytes => Self::from_str(#canonical),
    })?);
  }

  let from_str_impl = parse2(quote! {
    impl std::str::FromStr for #enum_type {
      type Err = crate::common::SdkError;

      fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
          #( #str_variants )*
          _ => Err(crate::common::invalid_enum_value(#enum_name, s)),
        }
      }
    }
  })?;

  let impl_block = parse2(quote! {
    impl #enum_type {
      pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
        match b {
          #( #byte_variants )*
          other => Err(crate::common::invalid_enum_value(
            #enum_name,
            String::from_utf8_lossy(other).into_owned(),
          )),
        }
      }
    }
  })?;

  Ok((from_str_impl, impl_block))
}

fn gen_element_wrapper_deserializer(
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
  let element_name_prefix = LitByteStr::new(
    format!("ofd:{}", element.xml_name).as_bytes(),
    Span::call_site(),
  );
  let element_name = LitByteStr::new(element.xml_name.as_bytes(), Span::call_site());

  let from_str_impl = parse2(quote! {
    impl std::str::FromStr for #wrapper_type {
      type Err = crate::common::SdkError;

      fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut xml_reader = crate::common::from_str_inner(s)?;

        Self::deserialize_inner_named(
          &mut xml_reader,
          None,
          #element_name_prefix,
          #element_name,
        )
      }
    }
  })?;

  let impl_block = parse2(quote! {
    impl #wrapper_type {
      pub fn from_reader<R: std::io::BufRead>(
        reader: R,
      ) -> Result<Self, crate::common::SdkError> {
        let mut xml_reader = crate::common::from_reader_inner(reader)?;
        let mut buf = Vec::new();

        Self::deserialize_from_reader_named(
          &mut xml_reader,
          &mut buf,
          None,
          #element_name_prefix,
          #element_name,
        )
      }

      pub(crate) fn deserialize_inner_named<'de>(
        xml_reader: &mut quick_xml::Reader<&'de [u8]>,
        xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
        tag_name_prefix: &[u8],
        tag_name: &[u8],
      ) -> Result<Self, crate::common::SdkError> {
        Ok(Self(<#inner_type>::deserialize_inner_named(
          xml_reader,
          xml_event,
          tag_name_prefix,
          tag_name,
        )?))
      }

      pub(crate) fn deserialize_from_reader_named<R: std::io::BufRead>(
        xml_reader: &mut quick_xml::Reader<R>,
        buf: &mut Vec<u8>,
        xml_event: Option<(quick_xml::events::BytesStart<'static>, bool)>,
        tag_name_prefix: &[u8],
        tag_name: &[u8],
      ) -> Result<Self, crate::common::SdkError> {
        Ok(Self(<#inner_type>::deserialize_from_reader_named(
          xml_reader,
          buf,
          xml_event,
          tag_name_prefix,
          tag_name,
        )?))
      }
    }
  })?;

  Ok(Some((from_str_impl, impl_block)))
}

fn gen_struct_deserializer(
  sdk_data_schema: &SdkDataSchema,
  sdk_data_schemas: &[SdkDataSchema],
  s: &SdkDataStruct,
  compatibility_rules: &[CompatibilityRule],
) -> anyhow::Result<(ItemImpl, ItemImpl)> {
  let struct_type: Type = parse_str(&format!(
    "crate::schemas::{}::{}",
    sdk_data_schema.module_name, s.ident
  ))?;

  let struct_xml_name = s.resolved_xml_name.as_str();
  let struct_name_literal = syn::LitStr::new(&s.ident, Span::call_site());
  let struct_xml_name_literal = syn::LitStr::new(struct_xml_name, Span::call_site());
  let struct_name_prefix = LitByteStr::new(
    format!("ofd:{}", struct_xml_name).as_bytes(),
    Span::call_site(),
  );
  let struct_name = LitByteStr::new(struct_xml_name.as_bytes(), Span::call_site());

  let expect_event_ident = if s.attributes.is_empty() {
    parse_str::<Ident>("_e")?
  } else {
    parse_str::<Ident>("e")?
  };

  let mut field_declarations: Vec<Stmt> = vec![];
  let mut attr_match_arms: Vec<Arm> = vec![];
  let mut field_post_process: Vec<Stmt> = vec![];
  let mut field_unwraps: Vec<Stmt> = vec![];
  let mut field_idents: Vec<Ident> = vec![];
  let mut event_match_arms: Vec<Arm> = vec![];
  let mut child_match_arms: Vec<Arm> = vec![];

  for attr in &s.attributes {
    let attr_ident: Ident = parse_str(escape_ident(&attr.ident))?;
    let attr_name = LitByteStr::new(attr.name.as_bytes(), Span::call_site());
    let missing_attr_rule = find_missing_attribute_rule(
      compatibility_rules,
      &sdk_data_schema.module_name,
      &s.ident,
      &attr.name,
      &attr.ident,
    );

    field_declarations.push(parse2(quote! {
      let mut #attr_ident = None;
    })?);

    attr_match_arms.push(gen_attr_match_arm(
      &struct_name_literal,
      attr,
      &attr_ident,
      &attr_name,
    )?);

    if !attr.is_option {
      let attr_name_literal = syn::LitStr::new(&attr.ident, Span::call_site());
      if let Some(rule) = missing_attr_rule {
        let default_value = match &rule.action {
          CompatibilityAction::AllowMissingAttribute { default_value } => default_value,
          CompatibilityAction::TreatAsString { default_value } => default_value,
          CompatibilityAction::EnumValueAlias { .. } => unreachable!(),
        };
        let default_expr = gen_default_compat_value_expr(
          &struct_name_literal,
          &attr_name_literal,
          &attr.type_kind,
          &attr.resolved_type,
          default_value,
        )?;

        field_unwraps.push(parse2(quote! {
          let #attr_ident = match #attr_ident {
            Some(value) => value,
            None => #default_expr,
          };
        })?);
      } else {
        field_unwraps.push(parse2(quote! {
          let #attr_ident = #attr_ident
            .ok_or_else(|| crate::common::missing_field(#struct_name_literal, #attr_name_literal))?;
        })?);
      }
    }

    field_idents.push(attr_ident);
  }

  let mut has_xml_value = false;

  for child in &s.sequences {
    let child_ident: Ident = parse_str(escape_ident(&child.ident))?;

    if child.ident == "xml_value" {
      has_xml_value = true;
      let child_raw_ident: Ident = parse_str(&format!("{}_raw", escape_ident(&child.ident)))?;
      let child_seen_ident: Ident = parse_str(&format!("{}_seen", escape_ident(&child.ident)))?;
      let parsed_value = if child.type_kind == CodegenTypeKind::String {
        quote! { #child_raw_ident.unwrap_or_default() }
      } else {
        let child_name_literal = syn::LitStr::new(&child.ident, Span::call_site());
        let value_expr = gen_value_from_string_expr(
          &struct_name_literal,
          &child_name_literal,
          &child.type_kind,
          &child.resolved_type,
        )?;
        quote! {{
          let value = #child_raw_ident.unwrap_or_default();
          #value_expr
        }}
      };

      field_declarations.push(parse2(quote! {
        let mut #child_raw_ident = None;
      })?);
      field_declarations.push(parse2(quote! {
        let mut #child_seen_ident = false;
      })?);

      if child.is_option {
        field_post_process.push(parse2(quote! {
          let #child_ident = if #child_seen_ident {
            Some(#parsed_value)
          } else {
            None
          };
        })?);
      } else {
        let child_name_literal = syn::LitStr::new(&child.ident, Span::call_site());
        field_post_process.push(parse2(quote! {
          let #child_ident = if #child_seen_ident {
            #parsed_value
          } else {
            Err(crate::common::missing_field(#struct_name_literal, #child_name_literal))?
          };
        })?);
      }
    } else if child.is_vec {
      field_declarations.push(parse2(quote! {
        let mut #child_ident = vec![];
      })?);
    } else {
      field_declarations.push(parse2(quote! {
        let mut #child_ident = None;
      })?);
    }

    if child.ident != "xml_value" && !child.is_option && !child.is_vec {
      let child_name_literal = syn::LitStr::new(&child.ident, Span::call_site());
      field_unwraps.push(parse2(quote! {
        let #child_ident = #child_ident
          .ok_or_else(|| crate::common::missing_field(#struct_name_literal, #child_name_literal))?;
      })?);
    }

    field_idents.push(child_ident);
  }

  if !s.children.is_empty() {
    field_declarations.push(parse2(quote! {
      let mut xml_children = vec![];
    })?);

    for child in &s.children {
      child_match_arms.push(gen_choice_child_match_arm(
        &struct_name_literal,
        sdk_data_schema,
        sdk_data_schemas,
        s,
        child,
        DeserializeMode::Slice,
      )?);
    }

    field_idents.push(parse_str("xml_children")?);
  }

  if has_xml_value {
    let xml_value_child = s
      .sequences
      .iter()
      .find(|child| child.ident == "xml_value")
      .expect("xml_value sequence must exist");
    let xml_value_raw_ident: Ident =
      parse_str(&format!("{}_raw", escape_ident(&xml_value_child.ident)))?;
    let xml_value_seen_ident: Ident =
      parse_str(&format!("{}_seen", escape_ident(&xml_value_child.ident)))?;

    event_match_arms.push(gen_xml_value_event_match_arm(
      &struct_name_literal,
      &syn::LitStr::new(&xml_value_child.ident, Span::call_site()),
      &xml_value_raw_ident,
      &xml_value_seen_ident,
    )?);
  }

  for child in &s.sequences {
    if child.ident == "xml_value" {
      continue;
    }

    child_match_arms.push(gen_sequence_child_match_arm(
      &struct_name_literal,
      sdk_data_schema,
      sdk_data_schemas,
      child,
      DeserializeMode::Slice,
    )?);
  }

  let attr_loop: Option<Stmt> = if s.attributes.is_empty() {
    None
  } else {
    Some(parse2(quote! {
      for attr in e.attributes().with_checks(false) {
        let attr = attr?;

        #[allow(clippy::single_match)]
        match attr.key.as_ref() {
          #( #attr_match_arms )*
          _ => {}
        }
      }
    })?)
  };

  let child_dispatch_slice: Stmt = if child_match_arms.is_empty() {
    parse2(quote! {
      if let Some(e) = e_opt
        && !e_empty
      {
        xml_reader.read_to_end(e.to_end().name())?;
      }
    })?
  } else {
    parse2(quote! {
      if let Some(e) = e_opt {
        match e.name().as_ref() {
          #( #child_match_arms )*
          _ => {
            if !e_empty {
              xml_reader.read_to_end(e.to_end().name())?;
            }
          }
        }
      }
    })?
  };

  let child_match_arms_io: Vec<Arm> = s
    .children
    .iter()
    .map(|child| {
      gen_choice_child_match_arm(
        &struct_name_literal,
        sdk_data_schema,
        sdk_data_schemas,
        s,
        child,
        DeserializeMode::Io,
      )
    })
    .collect::<anyhow::Result<_>>()?;
  let sequence_match_arms_io: Vec<Arm> = s
    .sequences
    .iter()
    .filter(|child| child.ident != "xml_value")
    .map(|child| {
      gen_sequence_child_match_arm(
        &struct_name_literal,
        sdk_data_schema,
        sdk_data_schemas,
        child,
        DeserializeMode::Io,
      )
    })
    .collect::<anyhow::Result<_>>()?;
  let mut child_match_arms_io_all = vec![];
  child_match_arms_io_all.extend(child_match_arms_io);
  child_match_arms_io_all.extend(sequence_match_arms_io);

  let child_dispatch_io: Stmt = if child_match_arms_io_all.is_empty() {
    parse2(quote! {
      if let Some(e) = e_opt
        && !e_empty
      {
        xml_reader.read_to_end_into(e.to_end().name(), buf)?;
      }
    })?
  } else {
    parse2(quote! {
      if let Some(e) = e_opt {
        match e.name().as_ref() {
          #( #child_match_arms_io_all )*
          _ => {
            if !e_empty {
              xml_reader.read_to_end_into(e.to_end().name(), buf)?;
            }
          }
        }
      }
    })?
  };

  let from_str_impl = parse2(quote! {
    impl std::str::FromStr for #struct_type {
      type Err = crate::common::SdkError;

      fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut xml_reader = crate::common::from_str_inner(s)?;

        Self::deserialize_inner_named(&mut xml_reader, None, #struct_name_prefix, #struct_name)
      }
    }
  })?;

  let from_reader_fn: ItemFn = parse2(quote! {
    pub fn from_reader<R: std::io::BufRead>(
      reader: R,
    ) -> Result<Self, crate::common::SdkError> {
      let mut xml_reader = crate::common::from_reader_inner(reader)?;
      let mut buf = Vec::new();

      Self::deserialize_from_reader_named(
        &mut xml_reader,
        &mut buf,
        None,
        #struct_name_prefix,
        #struct_name,
      )
    }
  })?;

  let deserialize_inner_fn: ItemFn = parse2(quote! {
    pub(crate) fn deserialize_inner_named<'de>(
      xml_reader: &mut quick_xml::Reader<&'de [u8]>,
      xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
      tag_name_prefix: &[u8],
      tag_name: &[u8],
    ) -> Result<Self, crate::common::SdkError> {
      let (#expect_event_ident, empty_tag) = crate::common::expect_event_start_slice!(
        xml_reader,
        xml_event,
        #struct_name_literal,
        #struct_xml_name_literal,
        tag_name_prefix,
        tag_name
      );

      #( #field_declarations )*

      #attr_loop

      if !empty_tag {
        loop {
          let mut e_opt: Option<quick_xml::events::BytesStart<'_>> = None;
          let mut e_empty = false;

          match xml_reader.read_event()? {
            quick_xml::events::Event::Start(e) => {
              e_opt = Some(e);
            }
            quick_xml::events::Event::Empty(e) => {
              e_empty = true;
              e_opt = Some(e);
            }
            #( #event_match_arms )*
            quick_xml::events::Event::End(e) => match e.name().as_ref() {
              name if name == tag_name_prefix || name == tag_name => {
                break;
              }
              _ => (),
            },
            quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof(#struct_name_literal))?,
            _ => (),
          }

          #child_dispatch_slice
        }
      }

      #( #field_post_process )*
      #( #field_unwraps )*

      Ok(Self {
        #( #field_idents, )*
      })
    }
  })?;

  let event_match_arms_io = event_match_arms.clone();

  let deserialize_from_reader_fn: ItemFn = parse2(quote! {
    pub(crate) fn deserialize_from_reader_named<R: std::io::BufRead>(
      xml_reader: &mut quick_xml::Reader<R>,
      buf: &mut Vec<u8>,
      xml_event: Option<(quick_xml::events::BytesStart<'static>, bool)>,
      tag_name_prefix: &[u8],
      tag_name: &[u8],
    ) -> Result<Self, crate::common::SdkError> {
      let (#expect_event_ident, empty_tag) = crate::common::expect_event_start_io!(
        xml_reader,
        buf,
        xml_event,
        #struct_name_literal,
        #struct_xml_name_literal,
        tag_name_prefix,
        tag_name
      );

      #( #field_declarations )*

      #attr_loop

      if !empty_tag {
        loop {
          let mut e_opt: Option<quick_xml::events::BytesStart<'static>> = None;
          let mut e_empty = false;

          buf.clear();
          match xml_reader.read_event_into(buf)? {
            quick_xml::events::Event::Start(e) => {
              e_opt = Some(e.into_owned());
            }
            quick_xml::events::Event::Empty(e) => {
              e_empty = true;
              e_opt = Some(e.into_owned());
            }
            #( #event_match_arms_io )*
            quick_xml::events::Event::End(e) => match e.name().as_ref() {
              name if name == tag_name_prefix || name == tag_name => {
                break;
              }
              _ => (),
            },
            quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof(#struct_name_literal))?,
            _ => (),
          }

          #child_dispatch_io
        }
      }

      #( #field_post_process )*
      #( #field_unwraps )*

      Ok(Self {
        #( #field_idents, )*
      })
    }
  })?;

  let impl_block = parse2(quote! {
    impl #struct_type {
      #from_reader_fn

      #deserialize_inner_fn

      #deserialize_from_reader_fn
    }
  })?;

  Ok((from_str_impl, impl_block))
}

fn gen_attr_match_arm(
  struct_name: &syn::LitStr,
  attr: &Attribute,
  attr_ident: &Ident,
  attr_name: &LitByteStr,
) -> anyhow::Result<Arm> {
  let parsed = gen_value_from_attr_expr(struct_name, attr)?;

  Ok(parse2(quote! {
    #attr_name => {
      #attr_ident = Some(#parsed);
    }
  })?)
}

fn gen_xml_value_event_match_arm(
  struct_name_literal: &syn::LitStr,
  child_name_literal: &syn::LitStr,
  child_raw_ident: &Ident,
  child_seen_ident: &Ident,
) -> anyhow::Result<Arm> {
  Ok(parse2(quote! {
    event @ quick_xml::events::Event::Text(_)
    | event @ quick_xml::events::Event::GeneralRef(_) => {
      #child_seen_ident = true;
      match event {
        quick_xml::events::Event::Text(t) => {
          crate::common::push_xml_text(&mut #child_raw_ident, t)?;
        }
        quick_xml::events::Event::GeneralRef(r) => {
          crate::common::push_xml_general_ref(
            &mut #child_raw_ident,
            r,
            #struct_name_literal,
            #child_name_literal,
          )?;
        }
        _ => unreachable!(),
      }
    }
  })?)
}

fn gen_sequence_child_match_arm(
  struct_name: &syn::LitStr,
  _sdk_data_schema: &SdkDataSchema,
  _sdk_data_schemas: &[SdkDataSchema],
  child: &Child,
  mode: DeserializeMode,
) -> anyhow::Result<Arm> {
  let child_xml_name = child.resolved_xml_name.as_str();
  let child_type_name = child.resolved_type.as_str();
  let child_name_prefix = LitByteStr::new(
    format!("ofd:{}", child_xml_name).as_bytes(),
    Span::call_site(),
  );
  let child_name = LitByteStr::new(child_xml_name.as_bytes(), Span::call_site());
  let child_name_literal = syn::LitStr::new(&child.ident, Span::call_site());
  let child_ident: Ident = parse_str(escape_ident(&child.ident))?;

  if child.is_struct {
    let child_type: Type = parse_str(child_type_name)?;
    let deserialize_fn = match mode {
      DeserializeMode::Slice => quote! { deserialize_inner_named },
      DeserializeMode::Io => quote! { deserialize_from_reader_named },
    };
    let buf_arg = match mode {
      DeserializeMode::Slice => quote! {},
      DeserializeMode::Io => quote! { buf, },
    };

    if child.is_vec {
      Ok(parse2(quote! {
        #child_name_prefix | #child_name => {
          #child_ident.push(#child_type::#deserialize_fn(
            xml_reader,
            #buf_arg
            Some((e, e_empty)),
            #child_name_prefix,
            #child_name,
          )?);
        }
      })?)
    } else {
      Ok(parse2(quote! {
        #child_name_prefix | #child_name => {
          #child_ident = Some(#child_type::#deserialize_fn(
            xml_reader,
            #buf_arg
            Some((e, e_empty)),
            #child_name_prefix,
            #child_name,
          )?);
        }
      })?)
    }
  } else {
    let value_block = gen_simple_child_value_block(
      struct_name,
      &child_name_literal,
      child_name_prefix.clone(),
      child_name.clone(),
      &child.type_kind,
      child_type_name,
      mode,
    )?;

    if child.is_vec {
      Ok(parse2(quote! {
        #child_name_prefix | #child_name => {
          let parsed_value = #value_block;
          #child_ident.push(parsed_value);
        }
      })?)
    } else {
      Ok(parse2(quote! {
        #child_name_prefix | #child_name => {
          let parsed_value = #value_block;
          #child_ident = Some(parsed_value);
        }
      })?)
    }
  }
}

fn gen_choice_child_match_arm(
  struct_name: &syn::LitStr,
  sdk_data_schema: &SdkDataSchema,
  _sdk_data_schemas: &[SdkDataSchema],
  s: &SdkDataStruct,
  child: &Child,
  mode: DeserializeMode,
) -> anyhow::Result<Arm> {
  let child_xml_name = child.resolved_xml_name.as_str();
  let child_type_name = child.resolved_type.as_str();
  let child_name_prefix = LitByteStr::new(
    format!("ofd:{}", child_xml_name).as_bytes(),
    Span::call_site(),
  );
  let child_name = LitByteStr::new(child_xml_name.as_bytes(), Span::call_site());
  let child_name_literal = syn::LitStr::new(&child.ident, Span::call_site());
  let child_choice_type: Type = parse_str(&format!(
    "crate::schemas::{}::{}ContentChoice",
    sdk_data_schema.module_name, s.ident
  ))?;
  let child_variant_ident: Ident = parse_str(escape_ident(&child.ident))?;

  if child.is_struct {
    let child_type: Type = parse_str(child_type_name)?;
    let deserialize_fn = match mode {
      DeserializeMode::Slice => quote! { deserialize_inner_named },
      DeserializeMode::Io => quote! { deserialize_from_reader_named },
    };
    let buf_arg = match mode {
      DeserializeMode::Slice => quote! {},
      DeserializeMode::Io => quote! { buf, },
    };

    Ok(parse2(quote! {
      #child_name_prefix | #child_name => {
        xml_children.push(#child_choice_type::#child_variant_ident(Box::new(
          #child_type::#deserialize_fn(
            xml_reader,
            #buf_arg
            Some((e, e_empty)),
            #child_name_prefix,
            #child_name,
          )?,
        )));
      }
    })?)
  } else {
    let value_block = gen_simple_child_value_block(
      struct_name,
      &child_name_literal,
      child_name_prefix.clone(),
      child_name.clone(),
      &child.type_kind,
      child_type_name,
      mode,
    )?;
    let child_type: Type = parse_str(child_type_name)?;

    Ok(parse2(quote! {
      #child_name_prefix | #child_name => {
        let parsed_value: #child_type = #value_block;
        xml_children.push(#child_choice_type::#child_variant_ident(Box::new(parsed_value)));
      }
    })?)
  }
}

fn gen_simple_child_value_block(
  struct_name: &syn::LitStr,
  field_name: &syn::LitStr,
  child_name_prefix: LitByteStr,
  child_name: LitByteStr,
  type_kind: &CodegenTypeKind,
  resolved_type: &str,
  mode: DeserializeMode,
) -> anyhow::Result<TokenStream> {
  if matches!(type_kind, CodegenTypeKind::Enum) {
    let enum_type: Type = parse_str(resolved_type)?;
    let slice_expr = quote! {{
      if e_empty {
        <#enum_type>::from_bytes(b"")?
      } else {
        let mut first_text = None;
        let mut value = None;

        loop {
          match xml_reader.read_event()? {
            quick_xml::events::Event::Text(text) => {
              if let Some(first) = first_text.take() {
                crate::common::push_xml_text(&mut value, first)?;
                crate::common::push_xml_text(&mut value, text)?;
              } else if value.is_some() {
                crate::common::push_xml_text(&mut value, text)?;
              } else {
                first_text = Some(text);
              }
            }
            quick_xml::events::Event::GeneralRef(text) => {
              if let Some(first) = first_text.take() {
                crate::common::push_xml_text(&mut value, first)?;
              }
              crate::common::push_xml_general_ref(&mut value, text, #struct_name, #field_name)?;
            }
            quick_xml::events::Event::End(end) => match end.name().as_ref() {
              name if name == #child_name_prefix || name == #child_name => {
                break if let Some(first) = first_text {
                  match <#enum_type>::from_bytes(first.as_ref()) {
                    Ok(value) => value,
                    Err(_) => first.xml10_content()?.as_ref().parse::<#enum_type>()?,
                  }
                } else {
                  value.unwrap_or_default().parse::<#enum_type>()?
                };
              }
              _ => {}
            },
            quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof(#struct_name))?,
            _ => {}
          }
        }
      }
    }};

    let io_expr = quote! {{
      if e_empty {
        <#enum_type>::from_bytes(b"")?
      } else {
        crate::common::read_text_enum_content_io(
          xml_reader,
          buf,
          crate::common::TextReadSpec {
            ty: #struct_name,
            field: #field_name,
            tag_name_prefix: #child_name_prefix,
            tag_name: #child_name,
          },
          <#enum_type>::from_bytes,
        )?
      }
    }};

    return Ok(match mode {
      DeserializeMode::Slice => slice_expr,
      DeserializeMode::Io => io_expr,
    });
  }

  if matches!(type_kind, CodegenTypeKind::String) {
    let slice_expr = quote! {{
      if e_empty {
        String::new()
      } else {
        let mut value = None;

        loop {
          match xml_reader.read_event()? {
            quick_xml::events::Event::Text(text) => crate::common::push_xml_text(&mut value, text)?,
            quick_xml::events::Event::GeneralRef(text) => {
              crate::common::push_xml_general_ref(&mut value, text, #struct_name, #field_name)?;
            }
            quick_xml::events::Event::End(end) => match end.name().as_ref() {
              name if name == #child_name_prefix || name == #child_name => {
                break value.unwrap_or_default();
              }
              _ => {}
            },
            quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof(#struct_name))?,
            _ => {}
          }
        }
      }
    }};

    let io_expr = quote! {{
      if e_empty {
        String::new()
      } else {
        crate::common::read_text_content_io(
          xml_reader,
          buf,
          crate::common::TextReadSpec {
            ty: #struct_name,
            field: #field_name,
            tag_name_prefix: #child_name_prefix,
            tag_name: #child_name,
          },
        )?
      }
    }};

    return Ok(match mode {
      DeserializeMode::Slice => slice_expr,
      DeserializeMode::Io => io_expr,
    });
  }

  let text_expr = gen_value_from_string_expr(struct_name, field_name, type_kind, resolved_type)?;

  let parse_bytes_expr = gen_value_from_bytes_expr(struct_name, field_name, type_kind)?;

  let slice_expr = quote! {{
    if e_empty {
      let value = String::new();
      { #text_expr }
    } else {
      let parse_bytes = |value: &[u8]| { #parse_bytes_expr };
      let mut first_text = None;
      let mut value = None;

      loop {
        match xml_reader.read_event()? {
          quick_xml::events::Event::Text(text) => {
            if let Some(first) = first_text.take() {
              crate::common::push_xml_text(&mut value, first)?;
              crate::common::push_xml_text(&mut value, text)?;
            } else if value.is_some() {
              crate::common::push_xml_text(&mut value, text)?;
            } else {
              first_text = Some(text);
            }
          }
          quick_xml::events::Event::GeneralRef(text) => {
            if let Some(first) = first_text.take() {
              crate::common::push_xml_text(&mut value, first)?;
            }
            crate::common::push_xml_general_ref(&mut value, text, #struct_name, #field_name)?;
          }
          quick_xml::events::Event::End(end) => match end.name().as_ref() {
            name if name == #child_name_prefix || name == #child_name => {
              break if let Some(first) = first_text {
                match parse_bytes(first.as_ref()) {
                  Ok(value) => value,
                  Err(_) => {
                    let value = first.xml10_content()?.into_owned();
                    { #text_expr }
                  }
                }
              } else {
                let value = value.unwrap_or_default();
                match parse_bytes(value.as_bytes()) {
                  Ok(value) => value,
                  Err(_) => { #text_expr }
                }
              };
            }
            _ => {}
          },
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof(#struct_name))?,
          _ => {}
        }
      }
    }
  }};

  let io_expr = quote! {{
    if e_empty {
      let value = String::new();
      { #text_expr }
    } else {
      crate::common::read_text_parsed_content_io(
        xml_reader,
        buf,
        crate::common::TextReadSpec {
          ty: #struct_name,
          field: #field_name,
          tag_name_prefix: #child_name_prefix,
          tag_name: #child_name,
        },
        |value| { #parse_bytes_expr },
        |value| {
          let value = value.to_string();
          Ok({ #text_expr })
        },
      )?
    }
  }};

  Ok(match mode {
    DeserializeMode::Slice => slice_expr,
    DeserializeMode::Io => io_expr,
  })
}

fn gen_value_from_attr_expr(
  struct_name: &syn::LitStr,
  attr: &Attribute,
) -> anyhow::Result<TokenStream> {
  let attr_name = syn::LitStr::new(&attr.ident, Span::call_site());

  match attr.type_kind {
    CodegenTypeKind::String => Ok(quote! {
      crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned()
    }),
    CodegenTypeKind::Bool => Ok(quote! {
      crate::common::parse_bool_attr(&attr, xml_reader.decoder(), #struct_name, #attr_name)?
    }),
    CodegenTypeKind::F64 => Ok(quote! {
      crate::common::parse_f64_attr(&attr, xml_reader.decoder(), #struct_name, #attr_name)?
    }),
    CodegenTypeKind::I32 => Ok(quote! {
      crate::common::parse_i32_attr(&attr, xml_reader.decoder(), #struct_name, #attr_name)?
    }),
    CodegenTypeKind::U32 => Ok(quote! {
      crate::common::parse_u32_attr(&attr, xml_reader.decoder(), #struct_name, #attr_name)?
    }),
    CodegenTypeKind::Enum => {
      let enum_type: Type = parse_str(&attr.resolved_type)?;
      Ok(quote! {
        if let Some(value) = crate::common::attr_raw_value(&attr) {
          <#enum_type>::from_bytes(value)?
        } else {
          crate::common::decode_attr_value(&attr, xml_reader.decoder())?
            .parse::<#enum_type>()?
        }
      })
    }
    CodegenTypeKind::Struct => anyhow::bail!("attribute type `{}` cannot be a struct", attr.r#type),
  }
}

fn gen_value_from_string_expr(
  ty_name: &syn::LitStr,
  field_name: &syn::LitStr,
  type_kind: &CodegenTypeKind,
  resolved_type: &str,
) -> anyhow::Result<TokenStream> {
  match type_kind {
    CodegenTypeKind::String => Ok(quote! {
      value
    }),
    CodegenTypeKind::Bool => Ok(quote! {
      crate::common::parse_bool_bytes(value.as_bytes())
        .map_err(|_| crate::common::invalid_field_value(#ty_name, #field_name, value.clone()))?
    }),
    CodegenTypeKind::F64 => Ok(quote! {
      value
        .parse::<f64>()
        .map_err(|_| crate::common::invalid_field_value(#ty_name, #field_name, value.clone()))?
    }),
    CodegenTypeKind::I32 => Ok(quote! {
      value
        .parse::<i32>()
        .map_err(|_| crate::common::invalid_field_value(#ty_name, #field_name, value.clone()))?
    }),
    CodegenTypeKind::U32 => Ok(quote! {
      value
        .parse::<u32>()
        .map_err(|_| crate::common::invalid_field_value(#ty_name, #field_name, value.clone()))?
    }),
    CodegenTypeKind::Enum => {
      let enum_type: Type = parse_str(resolved_type)?;
      Ok(quote! {
        value.parse::<#enum_type>()?
      })
    }
    CodegenTypeKind::Struct => {
      anyhow::bail!("string value type `{resolved_type}` cannot be a struct")
    }
  }
}

fn gen_value_from_bytes_expr(
  ty_name: &syn::LitStr,
  field_name: &syn::LitStr,
  type_kind: &CodegenTypeKind,
) -> anyhow::Result<TokenStream> {
  match type_kind {
    CodegenTypeKind::Bool => Ok(quote! {
      crate::common::parse_bool_bytes(value)
        .map_err(|_| {
          crate::common::invalid_field_value(
            #ty_name,
            #field_name,
            String::from_utf8_lossy(value).into_owned(),
          )
        })
    }),
    CodegenTypeKind::F64 => Ok(quote! {
      std::str::from_utf8(value)
        .ok()
        .and_then(|value| value.parse::<f64>().ok())
        .ok_or_else(|| {
          crate::common::invalid_field_value(
            #ty_name,
            #field_name,
            String::from_utf8_lossy(value).into_owned(),
          )
        })
    }),
    CodegenTypeKind::I32 => Ok(quote! {
      std::str::from_utf8(value)
        .ok()
        .and_then(|value| value.parse::<i32>().ok())
        .ok_or_else(|| {
          crate::common::invalid_field_value(
            #ty_name,
            #field_name,
            String::from_utf8_lossy(value).into_owned(),
          )
        })
    }),
    CodegenTypeKind::U32 => Ok(quote! {
      std::str::from_utf8(value)
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
        .ok_or_else(|| {
          crate::common::invalid_field_value(
            #ty_name,
            #field_name,
            String::from_utf8_lossy(value).into_owned(),
          )
        })
    }),
    CodegenTypeKind::String | CodegenTypeKind::Enum | CodegenTypeKind::Struct => {
      anyhow::bail!("bytes value type is not supported for `{type_kind:?}`")
    }
  }
}

fn gen_default_compat_value_expr(
  ty_name: &syn::LitStr,
  field_name: &syn::LitStr,
  type_kind: &CodegenTypeKind,
  resolved_type: &str,
  default_value: &str,
) -> anyhow::Result<TokenStream> {
  let default_value_literal = syn::LitStr::new(default_value, Span::call_site());

  if matches!(type_kind, CodegenTypeKind::String) {
    return Ok(quote! {
      #default_value_literal.to_string()
    });
  }

  let value_expr = gen_value_from_string_expr(ty_name, field_name, type_kind, resolved_type)?;

  Ok(quote! {{
    let value = #default_value_literal.to_string();
    #value_expr
  }})
}

fn schema_types(sdk_data_schema: &SdkDataSchema) -> &[SdkDataStruct] {
  &sdk_data_schema.types
}

fn is_public_struct_wrapper_candidate(
  _sdk_data_schema: &SdkDataSchema,
  element: &crate::models::sdk_data::Element,
) -> bool {
  matches!(element.api_kind, ElementApiKind::StructWrapper)
}
