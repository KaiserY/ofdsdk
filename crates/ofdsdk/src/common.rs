use quick_xml::{
  Decoder, Reader,
  encoding::EncodingError,
  events::attributes::Attribute,
  events::{Event, attributes::AttrError},
  name::QName,
};
use std::borrow::Cow;
use std::{
  io::BufRead,
  num::{ParseFloatError, ParseIntError},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SdkError {
  #[error("quick_xml error")]
  QuickXmlError(#[from] quick_xml::Error),
  #[error("quick_xml encoding error")]
  QuickEncodingError(#[from] EncodingError),
  #[error("quick_xml attr error")]
  AttrError(#[from] AttrError),
  #[error("ParseIntError")]
  ParseIntError(#[from] ParseIntError),
  #[error("ParseFloatError")]
  ParseFloatError(#[from] ParseFloatError),
  #[error("StdFmtError")]
  StdFmtError(#[from] std::fmt::Error),
  #[error("StdIoError")]
  StdIoError(#[from] std::io::Error),
  #[cfg(feature = "parts")]
  #[error("ZipError")]
  ZipError(#[from] zip::result::ZipError),
  #[error("CommonError")]
  CommonError(String),
  #[error("unexpected tag while parsing {ty} (expected {expected:?}, found {found:?})")]
  UnexpectedTag {
    ty: &'static str,
    expected: &'static str,
    found: String,
  },
  #[error("missing field `{field}` while parsing {ty}")]
  MissingField {
    ty: &'static str,
    field: &'static str,
  },
  #[error("invalid enum value while parsing {ty}: {value:?}")]
  InvalidEnumValue { ty: &'static str, value: String },
  #[error("invalid field `{field}` while parsing {ty}: {value:?}")]
  InvalidFieldValue {
    ty: &'static str,
    field: &'static str,
    value: String,
  },
  #[error("unexpected EOF while parsing {context}")]
  UnexpectedEof { context: &'static str },
  #[error("unsupported xml entity {entity:?}")]
  XmlEntityError { entity: String },
}

#[inline]
pub fn unexpected_tag(ty: &'static str, expected: &'static str, found: &[u8]) -> SdkError {
  SdkError::UnexpectedTag {
    ty,
    expected,
    found: String::from_utf8_lossy(found).into_owned(),
  }
}

#[inline]
pub fn missing_field(ty: &'static str, field: &'static str) -> SdkError {
  SdkError::MissingField { ty, field }
}

#[inline]
pub fn invalid_enum_value(ty: &'static str, value: impl Into<String>) -> SdkError {
  SdkError::InvalidEnumValue {
    ty,
    value: value.into(),
  }
}

#[inline]
pub fn invalid_field_value(
  ty: &'static str,
  field: &'static str,
  value: impl Into<String>,
) -> SdkError {
  SdkError::InvalidFieldValue {
    ty,
    field,
    value: value.into(),
  }
}

#[inline]
pub fn unexpected_eof(context: &'static str) -> SdkError {
  SdkError::UnexpectedEof { context }
}

#[inline]
pub fn xml_entity_error(entity: impl Into<String>) -> SdkError {
  SdkError::XmlEntityError {
    entity: entity.into(),
  }
}

pub trait XmlReader<'de> {
  fn next(&mut self) -> Result<Event<'de>, SdkError>;

  fn read_to_end(&mut self, end: QName) -> Result<(), SdkError>;

  fn decoder(&self) -> Decoder;
}

pub struct IoReader<R: BufRead> {
  reader: Reader<R>,
  buf: Vec<u8>,
}

impl<R: BufRead> IoReader<R> {
  pub fn new(reader: Reader<R>) -> Self {
    Self {
      reader,
      buf: Vec::new(),
    }
  }
}

impl<'de, R: BufRead> XmlReader<'de> for IoReader<R> {
  #[inline]
  fn next(&mut self) -> Result<Event<'de>, SdkError> {
    self.buf.clear();

    Ok(self.reader.read_event_into(&mut self.buf)?.into_owned())
  }

  #[inline]
  fn read_to_end(&mut self, end: QName) -> Result<(), SdkError> {
    self.reader.read_to_end_into(end, &mut self.buf)?;

    Ok(())
  }

  #[inline]
  fn decoder(&self) -> Decoder {
    self.reader.decoder()
  }
}

pub struct SliceReader<'de> {
  reader: Reader<&'de [u8]>,
}

impl<'de> SliceReader<'de> {
  pub fn new(reader: Reader<&'de [u8]>) -> Self {
    Self { reader }
  }
}

impl<'de> XmlReader<'de> for SliceReader<'de> {
  #[inline]
  fn next(&mut self) -> Result<Event<'de>, SdkError> {
    Ok(self.reader.read_event()?)
  }

  #[inline]
  fn read_to_end(&mut self, end: QName) -> Result<(), SdkError> {
    self.reader.read_to_end(end)?;

    Ok(())
  }

  #[inline]
  fn decoder(&self) -> Decoder {
    self.reader.decoder()
  }
}

pub fn resolve_zip_file_path(path: &str) -> String {
  let mut stack = Vec::new();

  for component in path.split('/') {
    match component {
      "" | "." => {}
      ".." => {
        stack.pop();
      }
      _ => {
        stack.push(component);
      }
    }
  }
  stack.join("/")
}

#[cfg(feature = "parts")]
pub fn zip_parent_dirs(path: &str) -> Vec<String> {
  let mut dirs = Vec::new();
  let mut current = String::new();

  for component in path.split('/').filter(|component| !component.is_empty()) {
    if !current.is_empty() {
      current.push('/');
    }
    current.push_str(component);
    current.push('/');
    dirs.push(current.clone());
    current.pop();
  }

  dirs.pop();
  dirs
}

#[inline]
pub fn zip_parent_dir(path: &str) -> String {
  match path.rfind('/') {
    Some(index) => path[..index + 1].to_string(),
    None => String::new(),
  }
}

#[inline]
pub fn resolve_zip_child_path(base_dir: &str, target: &str) -> String {
  if target.starts_with('/') {
    resolve_zip_file_path(target)
  } else {
    resolve_zip_file_path(&format!("{base_dir}{target}"))
  }
}

#[cfg(feature = "parts")]
pub fn read_zip_data<R: std::io::Read + std::io::Seek>(
  archive: &mut zip::ZipArchive<R>,
  path: &str,
) -> Result<Vec<u8>, SdkError> {
  use std::io::Read;

  let normalized_path = resolve_zip_file_path(path);
  let mut zip_entry = archive.by_name(&normalized_path)?;
  let mut data = Vec::with_capacity(zip_entry.size() as usize);
  zip_entry.read_to_end(&mut data)?;
  Ok(data)
}

#[cfg(feature = "parts")]
pub fn load_zip_parts<R, T, F>(
  _current_dir: &str,
  child_paths: Vec<String>,
  archive: &mut zip::ZipArchive<R>,
  mut load: F,
) -> Result<Vec<T>, SdkError>
where
  R: std::io::Read + std::io::Seek,
  F: FnMut(&str, &mut zip::ZipArchive<R>) -> Result<T, SdkError>,
{
  let mut parts = Vec::with_capacity(child_paths.len());

  for child_path in child_paths {
    parts.push(load(&child_path, archive)?);
  }

  Ok(parts)
}

#[cfg(feature = "parts")]
pub fn load_zip_parts_with_context<R, T, C, F>(
  _current_dir: &str,
  child_paths: Vec<String>,
  child_contexts: Vec<C>,
  child_api_name: &str,
  archive: &mut zip::ZipArchive<R>,
  mut load: F,
) -> Result<Vec<T>, SdkError>
where
  R: std::io::Read + std::io::Seek,
  F: FnMut(&str, C, &mut zip::ZipArchive<R>) -> Result<T, SdkError>,
{
  if child_paths.len() != child_contexts.len() {
    return Err(SdkError::CommonError(format!(
      "mismatched child contexts for {}",
      child_api_name
    )));
  }

  let mut parts = Vec::with_capacity(child_paths.len());

  for (child_path, child_context) in child_paths.into_iter().zip(child_contexts.into_iter()) {
    parts.push(load(&child_path, child_context, archive)?);
  }

  Ok(parts)
}

#[cfg(feature = "parts")]
pub fn load_optional_zip_part<R, T, F>(
  _current_dir: &str,
  child_paths: Vec<String>,
  archive: &mut zip::ZipArchive<R>,
  mut load: F,
) -> Result<Option<Box<T>>, SdkError>
where
  R: std::io::Read + std::io::Seek,
  F: FnMut(&str, &mut zip::ZipArchive<R>) -> Result<T, SdkError>,
{
  match child_paths.len() {
    0 => Ok(None),
    1 => Ok(Some(Box::new(load(&child_paths[0], archive)?))),
    _ => Err(SdkError::CommonError(
      "multiple child paths for optional part".to_string(),
    )),
  }
}

#[cfg(feature = "parts")]
pub fn load_optional_zip_part_with_context<R, T, C, F>(
  _current_dir: &str,
  child_paths: Vec<String>,
  child_contexts: Vec<C>,
  child_api_name: &str,
  archive: &mut zip::ZipArchive<R>,
  mut load: F,
) -> Result<Option<Box<T>>, SdkError>
where
  R: std::io::Read + std::io::Seek,
  F: FnMut(&str, C, &mut zip::ZipArchive<R>) -> Result<T, SdkError>,
{
  if child_paths.len() != child_contexts.len() {
    return Err(SdkError::CommonError(format!(
      "mismatched child contexts for {}",
      child_api_name
    )));
  }

  match child_paths.len() {
    0 => Ok(None),
    1 => {
      let child_path = child_paths.into_iter().next().unwrap();
      let child_context = child_contexts.into_iter().next().unwrap();
      Ok(Some(Box::new(load(&child_path, child_context, archive)?)))
    }
    _ => Err(SdkError::CommonError(
      "multiple child paths for optional part".to_string(),
    )),
  }
}

#[cfg(feature = "parts")]
pub fn load_required_zip_part<R, T, F>(
  current_dir: &str,
  child_paths: Vec<String>,
  child_api_name: &str,
  archive: &mut zip::ZipArchive<R>,
  load: F,
) -> Result<Box<T>, SdkError>
where
  R: std::io::Read + std::io::Seek,
  F: FnMut(&str, &mut zip::ZipArchive<R>) -> Result<T, SdkError>,
{
  load_optional_zip_part(current_dir, child_paths, archive, load)?
    .ok_or_else(|| SdkError::CommonError(child_api_name.to_string()))
}

#[cfg(feature = "parts")]
pub fn load_required_zip_part_with_context<R, T, C, F>(
  current_dir: &str,
  child_paths: Vec<String>,
  child_contexts: Vec<C>,
  child_api_name: &str,
  archive: &mut zip::ZipArchive<R>,
  load: F,
) -> Result<Box<T>, SdkError>
where
  R: std::io::Read + std::io::Seek,
  F: FnMut(&str, C, &mut zip::ZipArchive<R>) -> Result<T, SdkError>,
{
  load_optional_zip_part_with_context(
    current_dir,
    child_paths,
    child_contexts,
    child_api_name,
    archive,
    load,
  )?
  .ok_or_else(|| SdkError::CommonError(child_api_name.to_string()))
}

#[cfg(feature = "parts")]
pub fn save_zip_data<W: std::io::Write + std::io::Seek>(
  inner_path: &str,
  data: &[u8],
  zip: &mut zip::ZipWriter<W>,
  entry_set: &mut std::collections::HashSet<String>,
) -> Result<(), SdkError> {
  use std::io::Write;

  let options = zip::write::SimpleFileOptions::default()
    .compression_method(zip::CompressionMethod::Deflated)
    .unix_permissions(0o755);

  if inner_path.is_empty() {
    return Err(SdkError::CommonError("empty inner_path".to_string()));
  }

  for dir_path in zip_parent_dirs(inner_path) {
    if !entry_set.contains(&dir_path) {
      zip.add_directory(&dir_path, options)?;
      entry_set.insert(dir_path);
    }
  }

  if !entry_set.contains(inner_path) {
    zip.start_file(inner_path, options)?;
    zip.write_all(data)?;
    entry_set.insert(inner_path.to_string());
  }

  Ok(())
}

#[cfg(feature = "parts")]
pub fn save_zip_parts<W, T, F>(
  children: &[T],
  zip: &mut zip::ZipWriter<W>,
  entry_set: &mut std::collections::HashSet<String>,
  mut save: F,
) -> Result<(), SdkError>
where
  W: std::io::Write + std::io::Seek,
  F: FnMut(
    &T,
    &mut zip::ZipWriter<W>,
    &mut std::collections::HashSet<String>,
  ) -> Result<(), SdkError>,
{
  for child in children {
    save(child, zip, entry_set)?;
  }

  Ok(())
}

#[cfg(feature = "parts")]
pub fn save_optional_zip_part<W, T, F>(
  child: &Option<Box<T>>,
  zip: &mut zip::ZipWriter<W>,
  entry_set: &mut std::collections::HashSet<String>,
  mut save: F,
) -> Result<(), SdkError>
where
  W: std::io::Write + std::io::Seek,
  F: FnMut(
    &T,
    &mut zip::ZipWriter<W>,
    &mut std::collections::HashSet<String>,
  ) -> Result<(), SdkError>,
{
  if let Some(child) = child {
    save(child, zip, entry_set)?;
  }

  Ok(())
}

#[cfg(feature = "parts")]
pub fn save_required_zip_part<W, T, F>(
  child: &T,
  zip: &mut zip::ZipWriter<W>,
  entry_set: &mut std::collections::HashSet<String>,
  mut save: F,
) -> Result<(), SdkError>
where
  W: std::io::Write + std::io::Seek,
  F: FnMut(
    &T,
    &mut zip::ZipWriter<W>,
    &mut std::collections::HashSet<String>,
  ) -> Result<(), SdkError>,
{
  save(child, zip, entry_set)
}

#[inline]
pub(crate) fn from_reader_inner<R: BufRead>(reader: R) -> Result<IoReader<R>, SdkError> {
  let mut xml_reader = quick_xml::Reader::from_reader(reader);
  xml_reader.config_mut().check_end_names = false;

  Ok(IoReader::new(xml_reader))
}

#[inline]
pub(crate) fn from_str_inner(s: &str) -> Result<SliceReader<'_>, SdkError> {
  let mut xml_reader = quick_xml::Reader::from_str(s);
  xml_reader.config_mut().check_end_names = false;

  Ok(SliceReader::new(xml_reader))
}

#[inline]
pub(crate) fn decode_attr_value<'a>(
  attr: &'a Attribute<'a>,
  decoder: Decoder,
) -> Result<Cow<'a, str>, SdkError> {
  if attr.value.as_ref().contains(&b'&') {
    Ok(attr.decode_and_unescape_value(decoder)?)
  } else {
    Ok(decoder.decode(attr.value.as_ref())?)
  }
}

#[inline]
pub(crate) fn attr_raw_value<'a>(attr: &'a Attribute<'a>) -> Option<&'a [u8]> {
  let value = attr.value.as_ref();
  if value.contains(&b'&') {
    None
  } else {
    Some(value)
  }
}

#[inline]
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

#[inline]
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

#[inline]
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

#[inline]
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

#[inline]
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

#[inline]
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

pub(crate) fn read_text_content<'de, R: XmlReader<'de>>(
  xml_reader: &mut R,
  ty: &'static str,
  field: &'static str,
  tag_name_prefix: &[u8],
  tag_name: &[u8],
) -> Result<String, SdkError> {
  let mut value = None;

  loop {
    match xml_reader.next()? {
      Event::Text(text) => push_xml_text(&mut value, text)?,
      Event::GeneralRef(text) => push_xml_general_ref(&mut value, text, ty, field)?,
      Event::End(end) => match end.name().as_ref() {
        name if name == tag_name_prefix || name == tag_name => break,
        _ => {}
      },
      Event::Eof => Err(unexpected_eof(ty))?,
      _ => {}
    }
  }

  Ok(value.unwrap_or_default())
}

pub(crate) fn read_text_enum_content<'de, R, T, F>(
  xml_reader: &mut R,
  ty: &'static str,
  field: &'static str,
  tag_name_prefix: &[u8],
  tag_name: &[u8],
  parse_bytes: F,
) -> Result<T, SdkError>
where
  R: XmlReader<'de>,
  T: std::str::FromStr<Err = SdkError>,
  F: Fn(&[u8]) -> Result<T, SdkError>,
{
  let mut first_text = None;
  let mut value = None;

  loop {
    match xml_reader.next()? {
      Event::Text(text) => {
        if let Some(first) = first_text.take() {
          push_xml_text(&mut value, first)?;
          push_xml_text(&mut value, text)?;
        } else if value.is_some() {
          push_xml_text(&mut value, text)?;
        } else {
          first_text = Some(text);
        }
      }
      Event::GeneralRef(text) => {
        if let Some(first) = first_text.take() {
          push_xml_text(&mut value, first)?;
        }
        push_xml_general_ref(&mut value, text, ty, field)?;
      }
      Event::End(end) => match end.name().as_ref() {
        name if name == tag_name_prefix || name == tag_name => {
          if let Some(first) = first_text {
            return match parse_bytes(first.as_ref()) {
              Ok(value) => Ok(value),
              Err(_) => first.xml10_content()?.parse::<T>(),
            };
          }

          return value.unwrap_or_default().parse::<T>();
        }
        _ => {}
      },
      Event::Eof => Err(unexpected_eof(ty))?,
      _ => {}
    }
  }
}

#[inline]
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

macro_rules! expect_event_start {
  ($xml_reader:expr, $xml_event:expr, $context:expr, $expected:expr, $tag_prefix:expr, $tag:expr) => {{
    if let Some((e, empty_tag)) = $xml_event {
      (e, empty_tag)
    } else {
      let (e, empty_tag) = loop {
        match $xml_reader.next()? {
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

pub(crate) use expect_event_start;
