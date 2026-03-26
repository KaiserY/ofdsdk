use quick_xml::events::attributes::Attribute;
use quick_xml::{Decoder, events::Event};
use std::io::BufRead;

mod error;
mod xml;
mod zip;

pub use error::{
  SdkError, invalid_enum_value, invalid_field_value, missing_field, unexpected_eof, unexpected_tag,
  xml_entity_error,
};
pub(crate) use xml::{attr_raw_value, decode_attr_value, from_reader_inner, from_str_inner};
#[cfg(feature = "parts")]
pub use zip::{
  load_optional_zip_part, load_optional_zip_part_with_context, load_required_zip_part,
  load_required_zip_part_with_context, load_zip_parts, load_zip_parts_with_context, read_zip_data,
  save_optional_zip_part, save_required_zip_part, save_zip_data, save_zip_parts, zip_parent_dirs,
};
pub use zip::{resolve_zip_child_path, resolve_zip_file_path, zip_parent_dir};

pub(crate) struct TextReadSpec<'a> {
  pub ty: &'static str,
  pub field: &'static str,
  pub tag_name_prefix: &'a [u8],
  pub tag_name: &'a [u8],
}

#[inline(always)]
pub(crate) fn parse_bool_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  if let Some(value) = attr_raw_value(attr)
    && let Ok(value) = parse_bool_bytes(value)
  {
    return Ok(value);
  }

  let value = decode_attr_value(attr, decoder)?;
  parse_bool_bytes(value.as_bytes()).map_err(|_| invalid_field_value(ty, field, value.to_string()))
}

#[inline(always)]
pub(crate) fn parse_f64_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<f64, SdkError> {
  if let Some(value) = attr_raw_value(attr)
    && let Ok(value) = std::str::from_utf8(value)
    && let Ok(value) = value.parse::<f64>()
  {
    return Ok(value);
  }

  let value = decode_attr_value(attr, decoder)?;
  value
    .parse::<f64>()
    .map_err(|_| invalid_field_value(ty, field, value.to_string()))
}

#[inline(always)]
pub(crate) fn parse_i32_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<i32, SdkError> {
  if let Some(value) = attr_raw_value(attr)
    && let Ok(value) = std::str::from_utf8(value)
    && let Ok(value) = value.parse::<i32>()
  {
    return Ok(value);
  }

  let value = decode_attr_value(attr, decoder)?;
  value
    .parse::<i32>()
    .map_err(|_| invalid_field_value(ty, field, value.to_string()))
}

#[inline(always)]
pub(crate) fn parse_u32_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<u32, SdkError> {
  if let Some(value) = attr_raw_value(attr)
    && let Ok(value) = std::str::from_utf8(value)
    && let Ok(value) = value.parse::<u32>()
  {
    return Ok(value);
  }

  let value = decode_attr_value(attr, decoder)?;
  value
    .parse::<u32>()
    .map_err(|_| invalid_field_value(ty, field, value.to_string()))
}

#[inline(always)]
pub(crate) fn push_xml_text(
  value: &mut Option<String>,
  text: quick_xml::events::BytesText<'_>,
) -> Result<(), SdkError> {
  let text = text.xml10_content()?;
  if let Some(value) = value {
    value.push_str(text.as_ref());
  } else {
    *value = Some(text.into_owned());
  }

  Ok(())
}

#[inline(always)]
pub(crate) fn push_xml_general_ref(
  value: &mut Option<String>,
  text: quick_xml::events::BytesRef<'_>,
  ty: &'static str,
  field: &'static str,
) -> Result<(), SdkError> {
  let entity = text.xml10_content()?;
  let entity = quick_xml::escape::resolve_predefined_entity(entity.as_ref())
    .ok_or_else(|| invalid_field_value(ty, field, entity.to_string()))?;

  if let Some(value) = value {
    value.push_str(entity);
  } else {
    *value = Some(entity.to_owned());
  }

  Ok(())
}

#[inline(always)]
pub(crate) fn read_text_content_io<R: BufRead>(
  xml_reader: &mut quick_xml::Reader<R>,
  buf: &mut Vec<u8>,
  spec: TextReadSpec<'_>,
) -> Result<String, SdkError> {
  let mut value = None;

  loop {
    buf.clear();
    match xml_reader.read_event_into(buf)? {
      Event::Text(text) => push_xml_text(&mut value, text)?,
      Event::GeneralRef(text) => push_xml_general_ref(&mut value, text, spec.ty, spec.field)?,
      Event::End(end) => match end.name().as_ref() {
        name if name == spec.tag_name_prefix || name == spec.tag_name => break,
        _ => {}
      },
      Event::Eof => Err(unexpected_eof(spec.ty))?,
      _ => {}
    }
  }

  Ok(value.unwrap_or_default())
}

#[inline(always)]
pub(crate) fn read_text_parsed_content_io<R: BufRead, T, FB, FS>(
  xml_reader: &mut quick_xml::Reader<R>,
  buf: &mut Vec<u8>,
  spec: TextReadSpec<'_>,
  parse_bytes: FB,
  parse_str: FS,
) -> Result<T, SdkError>
where
  FB: Fn(&[u8]) -> Result<T, SdkError>,
  FS: Fn(&str) -> Result<T, SdkError>,
{
  let mut first_text = None;
  let mut value = None;

  loop {
    buf.clear();
    match xml_reader.read_event_into(buf)? {
      Event::Text(text) => {
        if let Some(first) = first_text.take() {
          push_xml_text(&mut value, first)?;
          push_xml_text(&mut value, text)?;
        } else if value.is_some() {
          push_xml_text(&mut value, text)?;
        } else {
          first_text = Some(text.into_owned());
        }
      }
      Event::GeneralRef(text) => {
        if let Some(first) = first_text.take() {
          push_xml_text(&mut value, first)?;
        }
        push_xml_general_ref(&mut value, text, spec.ty, spec.field)?;
      }
      Event::End(end) => match end.name().as_ref() {
        name if name == spec.tag_name_prefix || name == spec.tag_name => {
          if let Some(first) = first_text {
            return match parse_bytes(first.as_ref()) {
              Ok(value) => Ok(value),
              Err(_) => {
                let content = first.xml10_content()?;
                parse_str(content.as_ref())
              }
            };
          }

          let value = value.unwrap_or_default();
          return match parse_bytes(value.as_bytes()) {
            Ok(value) => Ok(value),
            Err(_) => parse_str(value.as_str()),
          };
        }
        _ => {}
      },
      Event::Eof => Err(unexpected_eof(spec.ty))?,
      _ => {}
    }
  }
}

#[inline(always)]
pub(crate) fn read_text_enum_content_io<R: BufRead, T, F>(
  xml_reader: &mut quick_xml::Reader<R>,
  buf: &mut Vec<u8>,
  spec: TextReadSpec<'_>,
  parse_bytes: F,
) -> Result<T, SdkError>
where
  T: std::str::FromStr<Err = SdkError>,
  F: Fn(&[u8]) -> Result<T, SdkError>,
{
  read_text_parsed_content_io(xml_reader, buf, spec, parse_bytes, |value| {
    value.parse::<T>()
  })
}

#[inline(always)]
pub fn parse_bool_bytes(b: &[u8]) -> Result<bool, SdkError> {
  match b {
    b"true" | b"1" | b"True" | b"TRUE" | b"t" | b"Yes" | b"YES" | b"yes" | b"y" => Ok(true),
    b"false" | b"0" | b"False" | b"FALSE" | b"f" | b"No" | b"NO" | b"no" | b"n" | b"" => Ok(false),
    other => Err(invalid_field_value(
      "bool",
      "value",
      String::from_utf8_lossy(other).into_owned(),
    )),
  }
}

macro_rules! expect_event_start_slice {
  ($xml_reader:expr, $xml_event:expr, $context:expr, $expected:expr, $tag_prefix:expr, $tag:expr) => {{
    if let Some((e, empty_tag)) = $xml_event {
      (e, empty_tag)
    } else {
      let (e, empty_tag) = loop {
        match $xml_reader.read_event()? {
          quick_xml::events::Event::Start(b) => break (b, false),
          quick_xml::events::Event::Empty(b) => break (b, true),
          quick_xml::events::Event::Eof => {
            return Err(super::super::common::unexpected_eof($context));
          }
          _ => continue,
        }
      };

      match e.name().as_ref() {
        name if name == $tag_prefix || name == $tag => (),
        _ => {
          Err(super::super::common::unexpected_tag(
            $context,
            $expected,
            e.name().as_ref(),
          ))?;
        }
      }

      (e, empty_tag)
    }
  }};
}

macro_rules! expect_event_start_io {
  ($xml_reader:expr, $buf:expr, $xml_event:expr, $context:expr, $expected:expr, $tag_prefix:expr, $tag:expr) => {{
    if let Some((e, empty_tag)) = $xml_event {
      (e, empty_tag)
    } else {
      let (e, empty_tag) = loop {
        $buf.clear();
        match $xml_reader.read_event_into($buf)? {
          quick_xml::events::Event::Start(b) => break (b.into_owned(), false),
          quick_xml::events::Event::Empty(b) => break (b.into_owned(), true),
          quick_xml::events::Event::Eof => {
            return Err(super::super::common::unexpected_eof($context));
          }
          _ => continue,
        }
      };

      match e.name().as_ref() {
        name if name == $tag_prefix || name == $tag => (),
        _ => {
          Err(super::super::common::unexpected_tag(
            $context,
            $expected,
            e.name().as_ref(),
          ))?;
        }
      }

      (e, empty_tag)
    }
  }};
}

pub(crate) use expect_event_start_io;
pub(crate) use expect_event_start_slice;
