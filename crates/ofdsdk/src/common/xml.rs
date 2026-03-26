use std::borrow::Cow;
use std::io::BufRead;

use quick_xml::events::Event;
use quick_xml::events::attributes::Attribute;
use quick_xml::name::QName;
use quick_xml::{Decoder, Reader};

use super::SdkError;

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
