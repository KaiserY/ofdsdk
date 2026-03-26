//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

impl std::str::FromStr for crate::schemas::ofd::OfdDocType {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "OFD" => Ok(Self::Ofd),
      _ => Err(crate::common::invalid_enum_value("OfdDocType", s)),
    }
  }
}
impl crate::schemas::ofd::OfdDocType {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"OFD" => Ok(Self::Ofd),
      other => Err(crate::common::invalid_enum_value(
        "OfdDocType",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::ofd::Version {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Version", b"Version")
  }
}
impl crate::schemas::ofd::Version {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:Version", b"Version")
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    let (e, empty_tag) = crate::common::expect_event_start_slice!(
      xml_reader,
      xml_event,
      "Version",
      "Version",
      tag_name_prefix,
      tag_name
    );
    let mut id = None;
    let mut index = None;
    let mut current = None;
    let mut base_loc = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"ID" => {
          id = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Index" => {
          index = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "Version",
            "index",
          )?);
        }
        b"Current" => {
          current = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "Version",
            "current",
          )?);
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
        match xml_reader.read_event()? {
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Version"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end(e.to_end().name())?;
        }
      }
    }
    let id = id.ok_or_else(|| crate::common::missing_field("Version", "id"))?;
    let index = index.ok_or_else(|| crate::common::missing_field("Version", "index"))?;
    let base_loc = base_loc.ok_or_else(|| crate::common::missing_field("Version", "base_loc"))?;
    Ok(Self {
      id,
      index,
      current,
      base_loc,
    })
  }
  pub(crate) fn deserialize_from_reader_named<R: std::io::BufRead>(
    xml_reader: &mut quick_xml::Reader<R>,
    buf: &mut Vec<u8>,
    xml_event: Option<(quick_xml::events::BytesStart<'static>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    let (e, empty_tag) = crate::common::expect_event_start_io!(
      xml_reader,
      buf,
      xml_event,
      "Version",
      "Version",
      tag_name_prefix,
      tag_name
    );
    let mut id = None;
    let mut index = None;
    let mut current = None;
    let mut base_loc = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"ID" => {
          id = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Index" => {
          index = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "Version",
            "index",
          )?);
        }
        b"Current" => {
          current = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "Version",
            "current",
          )?);
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
          quick_xml::events::Event::End(e) => match e.name().as_ref() {
            name if name == tag_name_prefix || name == tag_name => {
              break;
            }
            _ => {}
          },
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Version"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end_into(e.to_end().name(), buf)?;
        }
      }
    }
    let id = id.ok_or_else(|| crate::common::missing_field("Version", "id"))?;
    let index = index.ok_or_else(|| crate::common::missing_field("Version", "index"))?;
    let base_loc = base_loc.ok_or_else(|| crate::common::missing_field("Version", "base_loc"))?;
    Ok(Self {
      id,
      index,
      current,
      base_loc,
    })
  }
}
impl std::str::FromStr for crate::schemas::ofd::Versions {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Versions", b"Versions")
  }
}
impl crate::schemas::ofd::Versions {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:Versions",
      b"Versions",
    )
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    let (_e, empty_tag) = crate::common::expect_event_start_slice!(
      xml_reader,
      xml_event,
      "Versions",
      "Versions",
      tag_name_prefix,
      tag_name
    );
    let mut version = vec![];
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
          quick_xml::events::Event::End(e) => match e.name().as_ref() {
            name if name == tag_name_prefix || name == tag_name => {
              break;
            }
            _ => {}
          },
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Versions"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Version" | b"Version" => {
              version.push(crate::schemas::ofd::Version::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Version",
                b"Version",
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
    Ok(Self { version })
  }
  pub(crate) fn deserialize_from_reader_named<R: std::io::BufRead>(
    xml_reader: &mut quick_xml::Reader<R>,
    buf: &mut Vec<u8>,
    xml_event: Option<(quick_xml::events::BytesStart<'static>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    let (_e, empty_tag) = crate::common::expect_event_start_io!(
      xml_reader,
      buf,
      xml_event,
      "Versions",
      "Versions",
      tag_name_prefix,
      tag_name
    );
    let mut version = vec![];
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
          quick_xml::events::Event::End(e) => match e.name().as_ref() {
            name if name == tag_name_prefix || name == tag_name => {
              break;
            }
            _ => {}
          },
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Versions"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Version" | b"Version" => {
              version.push(crate::schemas::ofd::Version::deserialize_from_reader_named(
                xml_reader,
                buf,
                Some((e, e_empty)),
                b"ofd:Version",
                b"Version",
              )?);
            }
            _ => {
              if !e_empty {
                xml_reader.read_to_end_into(e.to_end().name(), buf)?;
              }
            }
          }
        }
      }
    }
    Ok(Self { version })
  }
}
impl std::str::FromStr for crate::schemas::ofd::DocBody {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:DocBody", b"DocBody")
  }
}
impl crate::schemas::ofd::DocBody {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:DocBody", b"DocBody")
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    let (_e, empty_tag) = crate::common::expect_event_start_slice!(
      xml_reader,
      xml_event,
      "DocBody",
      "DocBody",
      tag_name_prefix,
      tag_name
    );
    let mut doc_info = None;
    let mut doc_root = None;
    let mut versions = None;
    let mut signatures = None;
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
          quick_xml::events::Event::End(e) => match e.name().as_ref() {
            name if name == tag_name_prefix || name == tag_name => {
              break;
            }
            _ => {}
          },
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("DocBody"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:DocInfo" | b"DocInfo" => {
              doc_info = Some(crate::schemas::ofd::CtDocInfo::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:DocInfo",
                b"DocInfo",
              )?);
            }
            b"ofd:DocRoot" | b"DocRoot" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        crate::common::push_xml_text(&mut value, text)?
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        crate::common::push_xml_general_ref(
                          &mut value, text, "DocBody", "doc_root",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:DocRoot" || name == b"DocRoot" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("DocBody"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              doc_root = Some(parsed_value);
            }
            b"ofd:Versions" | b"Versions" => {
              versions = Some(crate::schemas::ofd::Versions::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Versions",
                b"Versions",
              )?);
            }
            b"ofd:Signatures" | b"Signatures" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        crate::common::push_xml_text(&mut value, text)?
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "DocBody",
                          "signatures",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:Signatures" || name == b"Signatures" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("DocBody"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              signatures = Some(parsed_value);
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
    let doc_info = doc_info.ok_or_else(|| crate::common::missing_field("DocBody", "doc_info"))?;
    let doc_root = doc_root.ok_or_else(|| crate::common::missing_field("DocBody", "doc_root"))?;
    Ok(Self {
      doc_info,
      doc_root,
      versions,
      signatures,
    })
  }
  pub(crate) fn deserialize_from_reader_named<R: std::io::BufRead>(
    xml_reader: &mut quick_xml::Reader<R>,
    buf: &mut Vec<u8>,
    xml_event: Option<(quick_xml::events::BytesStart<'static>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    let (_e, empty_tag) = crate::common::expect_event_start_io!(
      xml_reader,
      buf,
      xml_event,
      "DocBody",
      "DocBody",
      tag_name_prefix,
      tag_name
    );
    let mut doc_info = None;
    let mut doc_root = None;
    let mut versions = None;
    let mut signatures = None;
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
          quick_xml::events::Event::End(e) => match e.name().as_ref() {
            name if name == tag_name_prefix || name == tag_name => {
              break;
            }
            _ => {}
          },
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("DocBody"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:DocInfo" | b"DocInfo" => {
              doc_info = Some(
                crate::schemas::ofd::CtDocInfo::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:DocInfo",
                  b"DocInfo",
                )?,
              );
            }
            b"ofd:DocRoot" | b"DocRoot" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "DocBody",
                      field: "doc_root",
                      tag_name_prefix: b"ofd:DocRoot",
                      tag_name: b"DocRoot",
                    },
                  )?
                }
              };
              doc_root = Some(parsed_value);
            }
            b"ofd:Versions" | b"Versions" => {
              versions = Some(
                crate::schemas::ofd::Versions::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Versions",
                  b"Versions",
                )?,
              );
            }
            b"ofd:Signatures" | b"Signatures" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "DocBody",
                      field: "signatures",
                      tag_name_prefix: b"ofd:Signatures",
                      tag_name: b"Signatures",
                    },
                  )?
                }
              };
              signatures = Some(parsed_value);
            }
            _ => {
              if !e_empty {
                xml_reader.read_to_end_into(e.to_end().name(), buf)?;
              }
            }
          }
        }
      }
    }
    let doc_info = doc_info.ok_or_else(|| crate::common::missing_field("DocBody", "doc_info"))?;
    let doc_root = doc_root.ok_or_else(|| crate::common::missing_field("DocBody", "doc_root"))?;
    Ok(Self {
      doc_info,
      doc_root,
      versions,
      signatures,
    })
  }
}
impl std::str::FromStr for crate::schemas::ofd::Ofd {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:OFD", b"OFD")
  }
}
impl crate::schemas::ofd::Ofd {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:OFD", b"OFD")
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    let (e, empty_tag) = crate::common::expect_event_start_slice!(
      xml_reader,
      xml_event,
      "Ofd",
      "OFD",
      tag_name_prefix,
      tag_name
    );
    let mut version = None;
    let mut doc_type = None;
    let mut doc_body = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Version" => {
          version =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"DocType" => {
          doc_type = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::ofd::OfdDocType>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::ofd::OfdDocType>()?
          });
        }
        _ => {}
      }
    }
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
          quick_xml::events::Event::End(e) => match e.name().as_ref() {
            name if name == tag_name_prefix || name == tag_name => {
              break;
            }
            _ => {}
          },
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Ofd"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:DocBody" | b"DocBody" => {
              doc_body.push(crate::schemas::ofd::DocBody::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:DocBody",
                b"DocBody",
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
    let version = version.ok_or_else(|| crate::common::missing_field("Ofd", "version"))?;
    let doc_type = doc_type.ok_or_else(|| crate::common::missing_field("Ofd", "doc_type"))?;
    Ok(Self {
      version,
      doc_type,
      doc_body,
    })
  }
  pub(crate) fn deserialize_from_reader_named<R: std::io::BufRead>(
    xml_reader: &mut quick_xml::Reader<R>,
    buf: &mut Vec<u8>,
    xml_event: Option<(quick_xml::events::BytesStart<'static>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    let (e, empty_tag) = crate::common::expect_event_start_io!(
      xml_reader,
      buf,
      xml_event,
      "Ofd",
      "OFD",
      tag_name_prefix,
      tag_name
    );
    let mut version = None;
    let mut doc_type = None;
    let mut doc_body = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Version" => {
          version =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"DocType" => {
          doc_type = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::ofd::OfdDocType>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::ofd::OfdDocType>()?
          });
        }
        _ => {}
      }
    }
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
          quick_xml::events::Event::End(e) => match e.name().as_ref() {
            name if name == tag_name_prefix || name == tag_name => {
              break;
            }
            _ => {}
          },
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Ofd"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:DocBody" | b"DocBody" => {
              doc_body.push(crate::schemas::ofd::DocBody::deserialize_from_reader_named(
                xml_reader,
                buf,
                Some((e, e_empty)),
                b"ofd:DocBody",
                b"DocBody",
              )?);
            }
            _ => {
              if !e_empty {
                xml_reader.read_to_end_into(e.to_end().name(), buf)?;
              }
            }
          }
        }
      }
    }
    let version = version.ok_or_else(|| crate::common::missing_field("Ofd", "version"))?;
    let doc_type = doc_type.ok_or_else(|| crate::common::missing_field("Ofd", "doc_type"))?;
    Ok(Self {
      version,
      doc_type,
      doc_body,
    })
  }
}
impl std::str::FromStr for crate::schemas::ofd::Keywords {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Keywords", b"Keywords")
  }
}
impl crate::schemas::ofd::Keywords {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:Keywords",
      b"Keywords",
    )
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    let (_e, empty_tag) = crate::common::expect_event_start_slice!(
      xml_reader,
      xml_event,
      "Keywords",
      "Keywords",
      tag_name_prefix,
      tag_name
    );
    let mut keyword = vec![];
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
          quick_xml::events::Event::End(e) => match e.name().as_ref() {
            name if name == tag_name_prefix || name == tag_name => {
              break;
            }
            _ => {}
          },
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Keywords"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Keyword" | b"Keyword" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        crate::common::push_xml_text(&mut value, text)?
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        crate::common::push_xml_general_ref(
                          &mut value, text, "Keywords", "keyword",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:Keyword" || name == b"Keyword" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("Keywords"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              keyword.push(parsed_value);
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
    Ok(Self { keyword })
  }
  pub(crate) fn deserialize_from_reader_named<R: std::io::BufRead>(
    xml_reader: &mut quick_xml::Reader<R>,
    buf: &mut Vec<u8>,
    xml_event: Option<(quick_xml::events::BytesStart<'static>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    let (_e, empty_tag) = crate::common::expect_event_start_io!(
      xml_reader,
      buf,
      xml_event,
      "Keywords",
      "Keywords",
      tag_name_prefix,
      tag_name
    );
    let mut keyword = vec![];
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
          quick_xml::events::Event::End(e) => match e.name().as_ref() {
            name if name == tag_name_prefix || name == tag_name => {
              break;
            }
            _ => {}
          },
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Keywords"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Keyword" | b"Keyword" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "Keywords",
                      field: "keyword",
                      tag_name_prefix: b"ofd:Keyword",
                      tag_name: b"Keyword",
                    },
                  )?
                }
              };
              keyword.push(parsed_value);
            }
            _ => {
              if !e_empty {
                xml_reader.read_to_end_into(e.to_end().name(), buf)?;
              }
            }
          }
        }
      }
    }
    Ok(Self { keyword })
  }
}
impl std::str::FromStr for crate::schemas::ofd::CustomData {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CustomData", b"CustomData")
  }
}
impl crate::schemas::ofd::CustomData {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:CustomData",
      b"CustomData",
    )
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    let (e, empty_tag) = crate::common::expect_event_start_slice!(
      xml_reader,
      xml_event,
      "CustomData",
      "CustomData",
      tag_name_prefix,
      tag_name
    );
    let mut name = None;
    let mut xml_value_raw = None;
    let mut xml_value_seen = false;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        _ => {}
      }
    }
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
          event @ quick_xml::events::Event::Text(_)
          | event @ quick_xml::events::Event::GeneralRef(_) => {
            xml_value_seen = true;
            match event {
              quick_xml::events::Event::Text(t) => {
                crate::common::push_xml_text(&mut xml_value_raw, t)?;
              }
              quick_xml::events::Event::GeneralRef(r) => {
                crate::common::push_xml_general_ref(
                  &mut xml_value_raw,
                  r,
                  "CustomData",
                  "xml_value",
                )?;
              }
              _ => unreachable!(),
            }
          }
          quick_xml::events::Event::End(e) => match e.name().as_ref() {
            name if name == tag_name_prefix || name == tag_name => {
              break;
            }
            _ => {}
          },
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CustomData"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end(e.to_end().name())?;
        }
      }
    }
    let xml_value = if xml_value_seen {
      xml_value_raw.unwrap_or_default()
    } else {
      Err(crate::common::missing_field("CustomData", "xml_value"))?
    };
    let name = name.ok_or_else(|| crate::common::missing_field("CustomData", "name"))?;
    Ok(Self { name, xml_value })
  }
  pub(crate) fn deserialize_from_reader_named<R: std::io::BufRead>(
    xml_reader: &mut quick_xml::Reader<R>,
    buf: &mut Vec<u8>,
    xml_event: Option<(quick_xml::events::BytesStart<'static>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    let (e, empty_tag) = crate::common::expect_event_start_io!(
      xml_reader,
      buf,
      xml_event,
      "CustomData",
      "CustomData",
      tag_name_prefix,
      tag_name
    );
    let mut name = None;
    let mut xml_value_raw = None;
    let mut xml_value_seen = false;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        _ => {}
      }
    }
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
          event @ quick_xml::events::Event::Text(_)
          | event @ quick_xml::events::Event::GeneralRef(_) => {
            xml_value_seen = true;
            match event {
              quick_xml::events::Event::Text(t) => {
                crate::common::push_xml_text(&mut xml_value_raw, t)?;
              }
              quick_xml::events::Event::GeneralRef(r) => {
                crate::common::push_xml_general_ref(
                  &mut xml_value_raw,
                  r,
                  "CustomData",
                  "xml_value",
                )?;
              }
              _ => unreachable!(),
            }
          }
          quick_xml::events::Event::End(e) => match e.name().as_ref() {
            name if name == tag_name_prefix || name == tag_name => {
              break;
            }
            _ => {}
          },
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CustomData"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end_into(e.to_end().name(), buf)?;
        }
      }
    }
    let xml_value = if xml_value_seen {
      xml_value_raw.unwrap_or_default()
    } else {
      Err(crate::common::missing_field("CustomData", "xml_value"))?
    };
    let name = name.ok_or_else(|| crate::common::missing_field("CustomData", "name"))?;
    Ok(Self { name, xml_value })
  }
}
impl std::str::FromStr for crate::schemas::ofd::CustomDatas {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CustomDatas", b"CustomDatas")
  }
}
impl crate::schemas::ofd::CustomDatas {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:CustomDatas",
      b"CustomDatas",
    )
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    let (_e, empty_tag) = crate::common::expect_event_start_slice!(
      xml_reader,
      xml_event,
      "CustomDatas",
      "CustomDatas",
      tag_name_prefix,
      tag_name
    );
    let mut custom_data = vec![];
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
          quick_xml::events::Event::End(e) => match e.name().as_ref() {
            name if name == tag_name_prefix || name == tag_name => {
              break;
            }
            _ => {}
          },
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CustomDatas"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:CustomData" | b"CustomData" => {
              custom_data.push(crate::schemas::ofd::CustomData::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:CustomData",
                b"CustomData",
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
    Ok(Self { custom_data })
  }
  pub(crate) fn deserialize_from_reader_named<R: std::io::BufRead>(
    xml_reader: &mut quick_xml::Reader<R>,
    buf: &mut Vec<u8>,
    xml_event: Option<(quick_xml::events::BytesStart<'static>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    let (_e, empty_tag) = crate::common::expect_event_start_io!(
      xml_reader,
      buf,
      xml_event,
      "CustomDatas",
      "CustomDatas",
      tag_name_prefix,
      tag_name
    );
    let mut custom_data = vec![];
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
          quick_xml::events::Event::End(e) => match e.name().as_ref() {
            name if name == tag_name_prefix || name == tag_name => {
              break;
            }
            _ => {}
          },
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CustomDatas"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:CustomData" | b"CustomData" => {
              custom_data.push(
                crate::schemas::ofd::CustomData::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:CustomData",
                  b"CustomData",
                )?,
              );
            }
            _ => {
              if !e_empty {
                xml_reader.read_to_end_into(e.to_end().name(), buf)?;
              }
            }
          }
        }
      }
    }
    Ok(Self { custom_data })
  }
}
impl std::str::FromStr for crate::schemas::ofd::CtDocInfo {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_DocInfo", b"CT_DocInfo")
  }
}
impl crate::schemas::ofd::CtDocInfo {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:CT_DocInfo",
      b"CT_DocInfo",
    )
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    let (_e, empty_tag) = crate::common::expect_event_start_slice!(
      xml_reader,
      xml_event,
      "CtDocInfo",
      "CT_DocInfo",
      tag_name_prefix,
      tag_name
    );
    let mut doc_id = None;
    let mut title = None;
    let mut author = None;
    let mut subject = None;
    let mut r#abstract = None;
    let mut creation_date = None;
    let mut mod_date = None;
    let mut doc_usage = None;
    let mut cover = None;
    let mut keywords = None;
    let mut creator = None;
    let mut creator_version = None;
    let mut custom_datas = None;
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
          quick_xml::events::Event::End(e) => match e.name().as_ref() {
            name if name == tag_name_prefix || name == tag_name => {
              break;
            }
            _ => {}
          },
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtDocInfo"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:DocID" | b"DocID" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        crate::common::push_xml_text(&mut value, text)?
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CtDocInfo",
                          "doc_id",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:DocID" || name == b"DocID" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtDocInfo"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              doc_id = Some(parsed_value);
            }
            b"ofd:Title" | b"Title" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        crate::common::push_xml_text(&mut value, text)?
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CtDocInfo",
                          "title",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:Title" || name == b"Title" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtDocInfo"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              title = Some(parsed_value);
            }
            b"ofd:Author" | b"Author" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        crate::common::push_xml_text(&mut value, text)?
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CtDocInfo",
                          "author",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:Author" || name == b"Author" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtDocInfo"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              author = Some(parsed_value);
            }
            b"ofd:Subject" | b"Subject" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        crate::common::push_xml_text(&mut value, text)?
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CtDocInfo",
                          "subject",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:Subject" || name == b"Subject" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtDocInfo"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              subject = Some(parsed_value);
            }
            b"ofd:Abstract" | b"Abstract" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        crate::common::push_xml_text(&mut value, text)?
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CtDocInfo",
                          "abstract",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:Abstract" || name == b"Abstract" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtDocInfo"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              r#abstract = Some(parsed_value);
            }
            b"ofd:CreationDate" | b"CreationDate" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        crate::common::push_xml_text(&mut value, text)?
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CtDocInfo",
                          "creation_date",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:CreationDate" || name == b"CreationDate" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtDocInfo"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              creation_date = Some(parsed_value);
            }
            b"ofd:ModDate" | b"ModDate" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        crate::common::push_xml_text(&mut value, text)?
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CtDocInfo",
                          "mod_date",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:ModDate" || name == b"ModDate" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtDocInfo"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              mod_date = Some(parsed_value);
            }
            b"ofd:DocUsage" | b"DocUsage" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        crate::common::push_xml_text(&mut value, text)?
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CtDocInfo",
                          "doc_usage",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:DocUsage" || name == b"DocUsage" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtDocInfo"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              doc_usage = Some(parsed_value);
            }
            b"ofd:Cover" | b"Cover" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        crate::common::push_xml_text(&mut value, text)?
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CtDocInfo",
                          "cover",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:Cover" || name == b"Cover" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtDocInfo"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              cover = Some(parsed_value);
            }
            b"ofd:Keywords" | b"Keywords" => {
              keywords = Some(crate::schemas::ofd::Keywords::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Keywords",
                b"Keywords",
              )?);
            }
            b"ofd:Creator" | b"Creator" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        crate::common::push_xml_text(&mut value, text)?
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CtDocInfo",
                          "creator",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:Creator" || name == b"Creator" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtDocInfo"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              creator = Some(parsed_value);
            }
            b"ofd:CreatorVersion" | b"CreatorVersion" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        crate::common::push_xml_text(&mut value, text)?
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CtDocInfo",
                          "creator_version",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:CreatorVersion" || name == b"CreatorVersion" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtDocInfo"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              creator_version = Some(parsed_value);
            }
            b"ofd:CustomDatas" | b"CustomDatas" => {
              custom_datas = Some(crate::schemas::ofd::CustomDatas::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:CustomDatas",
                b"CustomDatas",
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
    let doc_id = doc_id.ok_or_else(|| crate::common::missing_field("CtDocInfo", "doc_id"))?;
    Ok(Self {
      doc_id,
      title,
      author,
      subject,
      r#abstract,
      creation_date,
      mod_date,
      doc_usage,
      cover,
      keywords,
      creator,
      creator_version,
      custom_datas,
    })
  }
  pub(crate) fn deserialize_from_reader_named<R: std::io::BufRead>(
    xml_reader: &mut quick_xml::Reader<R>,
    buf: &mut Vec<u8>,
    xml_event: Option<(quick_xml::events::BytesStart<'static>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    let (_e, empty_tag) = crate::common::expect_event_start_io!(
      xml_reader,
      buf,
      xml_event,
      "CtDocInfo",
      "CT_DocInfo",
      tag_name_prefix,
      tag_name
    );
    let mut doc_id = None;
    let mut title = None;
    let mut author = None;
    let mut subject = None;
    let mut r#abstract = None;
    let mut creation_date = None;
    let mut mod_date = None;
    let mut doc_usage = None;
    let mut cover = None;
    let mut keywords = None;
    let mut creator = None;
    let mut creator_version = None;
    let mut custom_datas = None;
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
          quick_xml::events::Event::End(e) => match e.name().as_ref() {
            name if name == tag_name_prefix || name == tag_name => {
              break;
            }
            _ => {}
          },
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtDocInfo"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:DocID" | b"DocID" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtDocInfo",
                      field: "doc_id",
                      tag_name_prefix: b"ofd:DocID",
                      tag_name: b"DocID",
                    },
                  )?
                }
              };
              doc_id = Some(parsed_value);
            }
            b"ofd:Title" | b"Title" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtDocInfo",
                      field: "title",
                      tag_name_prefix: b"ofd:Title",
                      tag_name: b"Title",
                    },
                  )?
                }
              };
              title = Some(parsed_value);
            }
            b"ofd:Author" | b"Author" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtDocInfo",
                      field: "author",
                      tag_name_prefix: b"ofd:Author",
                      tag_name: b"Author",
                    },
                  )?
                }
              };
              author = Some(parsed_value);
            }
            b"ofd:Subject" | b"Subject" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtDocInfo",
                      field: "subject",
                      tag_name_prefix: b"ofd:Subject",
                      tag_name: b"Subject",
                    },
                  )?
                }
              };
              subject = Some(parsed_value);
            }
            b"ofd:Abstract" | b"Abstract" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtDocInfo",
                      field: "abstract",
                      tag_name_prefix: b"ofd:Abstract",
                      tag_name: b"Abstract",
                    },
                  )?
                }
              };
              r#abstract = Some(parsed_value);
            }
            b"ofd:CreationDate" | b"CreationDate" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtDocInfo",
                      field: "creation_date",
                      tag_name_prefix: b"ofd:CreationDate",
                      tag_name: b"CreationDate",
                    },
                  )?
                }
              };
              creation_date = Some(parsed_value);
            }
            b"ofd:ModDate" | b"ModDate" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtDocInfo",
                      field: "mod_date",
                      tag_name_prefix: b"ofd:ModDate",
                      tag_name: b"ModDate",
                    },
                  )?
                }
              };
              mod_date = Some(parsed_value);
            }
            b"ofd:DocUsage" | b"DocUsage" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtDocInfo",
                      field: "doc_usage",
                      tag_name_prefix: b"ofd:DocUsage",
                      tag_name: b"DocUsage",
                    },
                  )?
                }
              };
              doc_usage = Some(parsed_value);
            }
            b"ofd:Cover" | b"Cover" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtDocInfo",
                      field: "cover",
                      tag_name_prefix: b"ofd:Cover",
                      tag_name: b"Cover",
                    },
                  )?
                }
              };
              cover = Some(parsed_value);
            }
            b"ofd:Keywords" | b"Keywords" => {
              keywords = Some(
                crate::schemas::ofd::Keywords::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Keywords",
                  b"Keywords",
                )?,
              );
            }
            b"ofd:Creator" | b"Creator" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtDocInfo",
                      field: "creator",
                      tag_name_prefix: b"ofd:Creator",
                      tag_name: b"Creator",
                    },
                  )?
                }
              };
              creator = Some(parsed_value);
            }
            b"ofd:CreatorVersion" | b"CreatorVersion" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtDocInfo",
                      field: "creator_version",
                      tag_name_prefix: b"ofd:CreatorVersion",
                      tag_name: b"CreatorVersion",
                    },
                  )?
                }
              };
              creator_version = Some(parsed_value);
            }
            b"ofd:CustomDatas" | b"CustomDatas" => {
              custom_datas = Some(
                crate::schemas::ofd::CustomDatas::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:CustomDatas",
                  b"CustomDatas",
                )?,
              );
            }
            _ => {
              if !e_empty {
                xml_reader.read_to_end_into(e.to_end().name(), buf)?;
              }
            }
          }
        }
      }
    }
    let doc_id = doc_id.ok_or_else(|| crate::common::missing_field("CtDocInfo", "doc_id"))?;
    Ok(Self {
      doc_id,
      title,
      author,
      subject,
      r#abstract,
      creation_date,
      mod_date,
      doc_usage,
      cover,
      keywords,
      creator,
      creator_version,
      custom_datas,
    })
  }
}
impl std::str::FromStr for crate::schemas::ofd::DocInfo {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:DocInfo", b"DocInfo")
  }
}
impl crate::schemas::ofd::DocInfo {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:DocInfo", b"DocInfo")
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::ofd::CtDocInfo>::deserialize_inner_named(
        xml_reader,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
  pub(crate) fn deserialize_from_reader_named<R: std::io::BufRead>(
    xml_reader: &mut quick_xml::Reader<R>,
    buf: &mut Vec<u8>,
    xml_event: Option<(quick_xml::events::BytesStart<'static>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::ofd::CtDocInfo>::deserialize_from_reader_named(
        xml_reader,
        buf,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
