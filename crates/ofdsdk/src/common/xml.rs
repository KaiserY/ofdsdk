use std::borrow::Cow;
use std::io::BufRead;

use quick_xml::events::attributes::Attribute;
use quick_xml::{Decoder, Reader};

use super::SdkError;

#[inline(always)]
pub(crate) fn from_reader_inner<R: BufRead>(reader: R) -> Result<Reader<R>, SdkError> {
  let mut xml_reader = quick_xml::Reader::from_reader(reader);
  xml_reader.config_mut().check_end_names = false;

  Ok(xml_reader)
}

#[inline(always)]
pub(crate) fn from_str_inner(s: &str) -> Result<Reader<&[u8]>, SdkError> {
  let mut xml_reader = quick_xml::Reader::from_str(s);
  xml_reader.config_mut().check_end_names = false;

  Ok(xml_reader)
}

#[inline(always)]
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

#[inline(always)]
pub(crate) fn attr_raw_value<'a>(attr: &'a Attribute<'a>) -> Option<&'a [u8]> {
  let value = attr.value.as_ref();
  if value.contains(&b'&') {
    None
  } else {
    Some(value)
  }
}
