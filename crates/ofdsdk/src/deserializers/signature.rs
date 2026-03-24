//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

impl std::str::FromStr for crate::schemas::signature::ReferencesCheckMethod {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "MD5" => Ok(Self::Md5),
      "SHA1" => Ok(Self::Sha1),
      _ => Err(crate::common::invalid_enum_value(
        "ReferencesCheckMethod",
        s,
      )),
    }
  }
}
impl crate::schemas::signature::ReferencesCheckMethod {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"MD5" => Ok(Self::Md5),
      b"SHA1" => Ok(Self::Sha1),
      other => Err(crate::common::invalid_enum_value(
        "ReferencesCheckMethod",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::signature::Provider {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Provider", b"Provider")
  }
}
impl crate::schemas::signature::Provider {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Provider", b"Provider")
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
      "Provider",
      "Provider",
      tag_name_prefix,
      tag_name
    );
    let mut provider_name = None;
    let mut version = None;
    let mut company = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"ProviderName" => {
          provider_name =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Version" => {
          version =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Company" => {
          company =
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Provider"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end(e.to_end().name())?;
        }
      }
    }
    let provider_name =
      provider_name.ok_or_else(|| crate::common::missing_field("Provider", "provider_name"))?;
    Ok(Self {
      provider_name,
      version,
      company,
    })
  }
}
impl std::str::FromStr for crate::schemas::signature::Reference {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Reference", b"Reference")
  }
}
impl crate::schemas::signature::Reference {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Reference", b"Reference")
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
      "Reference",
      "Reference",
      tag_name_prefix,
      tag_name
    );
    let mut file_ref = None;
    let mut check_value = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"FileRef" => {
          file_ref =
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Reference"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:CheckValue" | b"CheckValue" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content(
                    xml_reader,
                    "Reference",
                    "check_value",
                    b"ofd:CheckValue",
                    b"CheckValue",
                  )?
                }
              };
              check_value = Some(parsed_value);
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
    let file_ref = file_ref.ok_or_else(|| crate::common::missing_field("Reference", "file_ref"))?;
    let check_value =
      check_value.ok_or_else(|| crate::common::missing_field("Reference", "check_value"))?;
    Ok(Self {
      file_ref,
      check_value,
    })
  }
}
impl std::str::FromStr for crate::schemas::signature::References {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:References", b"References")
  }
}
impl crate::schemas::signature::References {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:References", b"References")
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
      "References",
      "References",
      tag_name_prefix,
      tag_name
    );
    let mut check_method = None;
    let mut reference = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"CheckMethod" => {
          check_method =
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("References"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Reference" | b"Reference" => {
              reference.push(
                crate::schemas::signature::Reference::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Reference",
                  b"Reference",
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
    let check_method = match check_method {
      Some(value) => value,
      None => "MD5".to_string(),
    };
    Ok(Self {
      check_method,
      reference,
    })
  }
}
impl std::str::FromStr for crate::schemas::signature::StampAnnot {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:StampAnnot", b"StampAnnot")
  }
}
impl crate::schemas::signature::StampAnnot {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:StampAnnot", b"StampAnnot")
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
      "StampAnnot",
      "StampAnnot",
      tag_name_prefix,
      tag_name
    );
    let mut id = None;
    let mut page_ref = None;
    let mut boundary = None;
    let mut clip = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"ID" => {
          id = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"PageRef" => {
          page_ref = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "StampAnnot",
            "page_ref",
          )?);
        }
        b"Boundary" => {
          boundary =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Clip" => {
          clip = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("StampAnnot"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end(e.to_end().name())?;
        }
      }
    }
    let id = id.ok_or_else(|| crate::common::missing_field("StampAnnot", "id"))?;
    let page_ref =
      page_ref.ok_or_else(|| crate::common::missing_field("StampAnnot", "page_ref"))?;
    let boundary =
      boundary.ok_or_else(|| crate::common::missing_field("StampAnnot", "boundary"))?;
    Ok(Self {
      id,
      page_ref,
      boundary,
      clip,
    })
  }
}
impl std::str::FromStr for crate::schemas::signature::Seal {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Seal", b"Seal")
  }
}
impl crate::schemas::signature::Seal {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Seal", b"Seal")
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
      "Seal",
      "Seal",
      tag_name_prefix,
      tag_name
    );
    let mut base_loc = None;
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Seal"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:BaseLoc" | b"BaseLoc" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content(
                    xml_reader,
                    "Seal",
                    "base_loc",
                    b"ofd:BaseLoc",
                    b"BaseLoc",
                  )?
                }
              };
              base_loc = Some(parsed_value);
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
    let base_loc = base_loc.ok_or_else(|| crate::common::missing_field("Seal", "base_loc"))?;
    Ok(Self { base_loc })
  }
}
impl std::str::FromStr for crate::schemas::signature::SignedInfo {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:SignedInfo", b"SignedInfo")
  }
}
impl crate::schemas::signature::SignedInfo {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:SignedInfo", b"SignedInfo")
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
      "SignedInfo",
      "SignedInfo",
      tag_name_prefix,
      tag_name
    );
    let mut provider = None;
    let mut signature_method = None;
    let mut signature_date_time = None;
    let mut references = None;
    let mut stamp_annot = vec![];
    let mut seal = None;
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("SignedInfo"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Provider" | b"Provider" => {
              provider = Some(
                crate::schemas::signature::Provider::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Provider",
                  b"Provider",
                )?,
              );
            }
            b"ofd:SignatureMethod" | b"SignatureMethod" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content(
                    xml_reader,
                    "SignedInfo",
                    "signature_method",
                    b"ofd:SignatureMethod",
                    b"SignatureMethod",
                  )?
                }
              };
              signature_method = Some(parsed_value);
            }
            b"ofd:SignatureDateTime" | b"SignatureDateTime" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content(
                    xml_reader,
                    "SignedInfo",
                    "signature_date_time",
                    b"ofd:SignatureDateTime",
                    b"SignatureDateTime",
                  )?
                }
              };
              signature_date_time = Some(parsed_value);
            }
            b"ofd:References" | b"References" => {
              references = Some(
                crate::schemas::signature::References::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:References",
                  b"References",
                )?,
              );
            }
            b"ofd:StampAnnot" | b"StampAnnot" => {
              stamp_annot.push(
                crate::schemas::signature::StampAnnot::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:StampAnnot",
                  b"StampAnnot",
                )?,
              );
            }
            b"ofd:Seal" | b"Seal" => {
              seal = Some(crate::schemas::signature::Seal::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Seal",
                b"Seal",
              )?);
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
    let provider =
      provider.ok_or_else(|| crate::common::missing_field("SignedInfo", "provider"))?;
    let references =
      references.ok_or_else(|| crate::common::missing_field("SignedInfo", "references"))?;
    Ok(Self {
      provider,
      signature_method,
      signature_date_time,
      references,
      stamp_annot,
      seal,
    })
  }
}
impl std::str::FromStr for crate::schemas::signature::Signature {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Signature", b"Signature")
  }
}
impl crate::schemas::signature::Signature {
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
    let (_e, empty_tag) = crate::common::expect_event_start!(
      xml_reader,
      xml_event,
      "Signature",
      "Signature",
      tag_name_prefix,
      tag_name
    );
    let mut signed_info = None;
    let mut signed_value = None;
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
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:SignedInfo" | b"SignedInfo" => {
              signed_info = Some(
                crate::schemas::signature::SignedInfo::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:SignedInfo",
                  b"SignedInfo",
                )?,
              );
            }
            b"ofd:SignedValue" | b"SignedValue" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content(
                    xml_reader,
                    "Signature",
                    "signed_value",
                    b"ofd:SignedValue",
                    b"SignedValue",
                  )?
                }
              };
              signed_value = Some(parsed_value);
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
    let signed_info =
      signed_info.ok_or_else(|| crate::common::missing_field("Signature", "signed_info"))?;
    let signed_value =
      signed_value.ok_or_else(|| crate::common::missing_field("Signature", "signed_value"))?;
    Ok(Self {
      signed_info,
      signed_value,
    })
  }
}
