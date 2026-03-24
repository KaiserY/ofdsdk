//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

impl std::str::FromStr for crate::schemas::signatures::SignatureType {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Seal" => Ok(Self::Seal),
      "Sign" => Ok(Self::Sign),
      _ => Err(crate::common::invalid_enum_value("SignatureType", s)),
    }
  }
}
impl crate::schemas::signatures::SignatureType {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"Seal" => Ok(Self::Seal),
      b"Sign" => Ok(Self::Sign),
      other => Err(crate::common::invalid_enum_value(
        "SignatureType",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::signatures::Signature {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Signature", b"Signature")
  }
}
impl crate::schemas::signatures::Signature {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Signature", b"Signature")
  }
  pub(crate) fn deserialize_inner_named<'de, R: crate::common::XmlReader<'de>>(
    xml_reader: &mut R,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    let (e, empty_tag) = crate::common::expect_event_start!(
      xml_reader,
      xml_event,
      "Signature",
      "Signature",
      tag_name_prefix,
      tag_name
    );
    let mut id = None;
    let mut r#type = None;
    let mut base_loc = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"ID" => {
          id = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Type" => {
          r#type = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::signatures::SignatureType>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::signatures::SignatureType>()?
          });
        }
        b"BaseLoc" => {
          base_loc =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        _ => {}
      }
    }
    if !empty_tag {
      loop {
        let mut e_opt: Option<quick_xml::events::BytesStart<'_>> = None;
        let mut e_empty = false;
        match xml_reader.next()? {
          quick_xml::events::Event::Start(e) => {
            e_opt = Some(e);
          }
          quick_xml::events::Event::Empty(e) => {
            e_empty = true;
            e_opt = Some(e);
          }
          quick_xml::events::Event::End(e) => match e.name().as_ref() {
            name if name == tag_name_prefix || name == tag_name => {
              break;
            }
            _ => {}
          },
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Signature"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end(e.to_end().name())?;
        }
      }
    }
    let id = id.ok_or_else(|| crate::common::missing_field("Signature", "id"))?;
    let base_loc = base_loc.ok_or_else(|| crate::common::missing_field("Signature", "base_loc"))?;
    Ok(Self {
      id,
      r#type,
      base_loc,
    })
  }
}
impl std::str::FromStr for crate::schemas::signatures::Signatures {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Signatures", b"Signatures")
  }
}
impl crate::schemas::signatures::Signatures {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Signatures", b"Signatures")
  }
  pub(crate) fn deserialize_inner_named<'de, R: crate::common::XmlReader<'de>>(
    xml_reader: &mut R,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    let (_e, empty_tag) = crate::common::expect_event_start!(
      xml_reader,
      xml_event,
      "Signatures",
      "Signatures",
      tag_name_prefix,
      tag_name
    );
    let mut max_sign_id = None;
    let mut signature = vec![];
    if !empty_tag {
      loop {
        let mut e_opt: Option<quick_xml::events::BytesStart<'_>> = None;
        let mut e_empty = false;
        match xml_reader.next()? {
          quick_xml::events::Event::Start(e) => {
            e_opt = Some(e);
          }
          quick_xml::events::Event::Empty(e) => {
            e_empty = true;
            e_opt = Some(e);
          }
          quick_xml::events::Event::End(e) => match e.name().as_ref() {
            name if name == tag_name_prefix || name == tag_name => {
              break;
            }
            _ => {}
          },
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Signatures"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:MaxSignId" | b"MaxSignId" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content(
                    xml_reader,
                    "Signatures",
                    "max_sign_id",
                    b"ofd:MaxSignId",
                    b"MaxSignId",
                  )?
                }
              };
              max_sign_id = Some(parsed_value);
            }
            b"ofd:Signature" | b"Signature" => {
              signature.push(
                crate::schemas::signatures::Signature::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Signature",
                  b"Signature",
                )?,
              );
            }
            _ => {
              if !e_empty {
                xml_reader.read_to_end(e.to_end().name())?;
              }
            }
          }
        }
      }
    }
    Ok(Self {
      max_sign_id,
      signature,
    })
  }
}
