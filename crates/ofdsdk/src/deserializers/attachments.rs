//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

impl std::str::FromStr for crate::schemas::attachments::Attachments {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Attachments", b"Attachments")
  }
}
impl crate::schemas::attachments::Attachments {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:Attachments",
      b"Attachments",
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
      "Attachments",
      "Attachments",
      tag_name_prefix,
      tag_name
    );
    let mut attachment = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Attachments"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Attachment" | b"Attachment" => {
              attachment.push(
                crate::schemas::attachments::CtAttachment::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Attachment",
                  b"Attachment",
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
    Ok(Self { attachment })
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
      "Attachments",
      "Attachments",
      tag_name_prefix,
      tag_name
    );
    let mut attachment = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Attachments"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Attachment" | b"Attachment" => {
              attachment.push(
                crate::schemas::attachments::CtAttachment::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Attachment",
                  b"Attachment",
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
    Ok(Self { attachment })
  }
}
impl std::str::FromStr for crate::schemas::attachments::CtAttachment {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(
      &mut xml_reader,
      None,
      b"ofd:CT_Attachment",
      b"CT_Attachment",
    )
  }
}
impl crate::schemas::attachments::CtAttachment {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:CT_Attachment",
      b"CT_Attachment",
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
      "CtAttachment",
      "CT_Attachment",
      tag_name_prefix,
      tag_name
    );
    let mut id = None;
    let mut name = None;
    let mut format = None;
    let mut creation_date = None;
    let mut mod_date = None;
    let mut size = None;
    let mut visible = None;
    let mut usage = None;
    let mut file_loc = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"ID" => {
          id = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Format" => {
          format =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"CreationDate" => {
          creation_date =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"ModDate" => {
          mod_date =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Size" => {
          size = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtAttachment",
            "size",
          )?);
        }
        b"Visible" => {
          visible = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtAttachment",
            "visible",
          )?);
        }
        b"Usage" => {
          usage = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtAttachment"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:FileLoc" | b"FileLoc" => {
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
                          "CtAttachment",
                          "file_loc",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:FileLoc" || name == b"FileLoc" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtAttachment"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              file_loc = Some(parsed_value);
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
    let id = id.ok_or_else(|| crate::common::missing_field("CtAttachment", "id"))?;
    let name = name.ok_or_else(|| crate::common::missing_field("CtAttachment", "name"))?;
    let file_loc =
      file_loc.ok_or_else(|| crate::common::missing_field("CtAttachment", "file_loc"))?;
    Ok(Self {
      id,
      name,
      format,
      creation_date,
      mod_date,
      size,
      visible,
      usage,
      file_loc,
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
      "CtAttachment",
      "CT_Attachment",
      tag_name_prefix,
      tag_name
    );
    let mut id = None;
    let mut name = None;
    let mut format = None;
    let mut creation_date = None;
    let mut mod_date = None;
    let mut size = None;
    let mut visible = None;
    let mut usage = None;
    let mut file_loc = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"ID" => {
          id = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Format" => {
          format =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"CreationDate" => {
          creation_date =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"ModDate" => {
          mod_date =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Size" => {
          size = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtAttachment",
            "size",
          )?);
        }
        b"Visible" => {
          visible = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtAttachment",
            "visible",
          )?);
        }
        b"Usage" => {
          usage = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtAttachment"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:FileLoc" | b"FileLoc" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtAttachment",
                      field: "file_loc",
                      tag_name_prefix: b"ofd:FileLoc",
                      tag_name: b"FileLoc",
                    },
                  )?
                }
              };
              file_loc = Some(parsed_value);
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
    let id = id.ok_or_else(|| crate::common::missing_field("CtAttachment", "id"))?;
    let name = name.ok_or_else(|| crate::common::missing_field("CtAttachment", "name"))?;
    let file_loc =
      file_loc.ok_or_else(|| crate::common::missing_field("CtAttachment", "file_loc"))?;
    Ok(Self {
      id,
      name,
      format,
      creation_date,
      mod_date,
      size,
      visible,
      usage,
      file_loc,
    })
  }
}
impl std::str::FromStr for crate::schemas::attachments::Attachment {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Attachment", b"Attachment")
  }
}
impl crate::schemas::attachments::Attachment {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:Attachment",
      b"Attachment",
    )
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::attachments::CtAttachment>::deserialize_inner_named(
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
      <crate::schemas::attachments::CtAttachment>::deserialize_from_reader_named(
        xml_reader,
        buf,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
