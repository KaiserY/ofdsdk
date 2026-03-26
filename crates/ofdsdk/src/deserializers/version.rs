//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

impl std::str::FromStr for crate::schemas::version::File {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:File", b"File")
  }
}
impl crate::schemas::version::File {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:File", b"File")
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
      "File",
      "File",
      tag_name_prefix,
      tag_name
    );
    let mut id = None;
    let mut xml_value_raw = None;
    let mut xml_value_seen = false;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"ID" => {
          id = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
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
                crate::common::push_xml_general_ref(&mut xml_value_raw, r, "File", "xml_value")?;
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("File"))?,
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
      Err(crate::common::missing_field("File", "xml_value"))?
    };
    let id = id.ok_or_else(|| crate::common::missing_field("File", "id"))?;
    Ok(Self { id, xml_value })
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
      "File",
      "File",
      tag_name_prefix,
      tag_name
    );
    let mut id = None;
    let mut xml_value_raw = None;
    let mut xml_value_seen = false;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"ID" => {
          id = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
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
                crate::common::push_xml_general_ref(&mut xml_value_raw, r, "File", "xml_value")?;
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("File"))?,
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
      Err(crate::common::missing_field("File", "xml_value"))?
    };
    let id = id.ok_or_else(|| crate::common::missing_field("File", "id"))?;
    Ok(Self { id, xml_value })
  }
}
impl std::str::FromStr for crate::schemas::version::FileList {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:FileList", b"FileList")
  }
}
impl crate::schemas::version::FileList {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:FileList",
      b"FileList",
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
      "FileList",
      "FileList",
      tag_name_prefix,
      tag_name
    );
    let mut file = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("FileList"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:File" | b"File" => {
              file.push(crate::schemas::version::File::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:File",
                b"File",
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
    Ok(Self { file })
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
      "FileList",
      "FileList",
      tag_name_prefix,
      tag_name
    );
    let mut file = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("FileList"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:File" | b"File" => {
              file.push(
                crate::schemas::version::File::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:File",
                  b"File",
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
    Ok(Self { file })
  }
}
impl std::str::FromStr for crate::schemas::version::DocVersion {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:DocVersion", b"DocVersion")
  }
}
impl crate::schemas::version::DocVersion {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:DocVersion",
      b"DocVersion",
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
      "DocVersion",
      "DocVersion",
      tag_name_prefix,
      tag_name
    );
    let mut id = None;
    let mut version = None;
    let mut name = None;
    let mut creation_date = None;
    let mut file_list = None;
    let mut doc_root = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"ID" => {
          id = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Version" => {
          version =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"CreationDate" => {
          creation_date =
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("DocVersion"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:FileList" | b"FileList" => {
              file_list = Some(crate::schemas::version::FileList::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:FileList",
                b"FileList",
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
                          &mut value,
                          text,
                          "DocVersion",
                          "doc_root",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:DocRoot" || name == b"DocRoot" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("DocVersion"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              doc_root = Some(parsed_value);
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
    let id = id.ok_or_else(|| crate::common::missing_field("DocVersion", "id"))?;
    let file_list =
      file_list.ok_or_else(|| crate::common::missing_field("DocVersion", "file_list"))?;
    let doc_root =
      doc_root.ok_or_else(|| crate::common::missing_field("DocVersion", "doc_root"))?;
    Ok(Self {
      id,
      version,
      name,
      creation_date,
      file_list,
      doc_root,
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
      "DocVersion",
      "DocVersion",
      tag_name_prefix,
      tag_name
    );
    let mut id = None;
    let mut version = None;
    let mut name = None;
    let mut creation_date = None;
    let mut file_list = None;
    let mut doc_root = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"ID" => {
          id = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Version" => {
          version =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"CreationDate" => {
          creation_date =
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("DocVersion"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:FileList" | b"FileList" => {
              file_list = Some(
                crate::schemas::version::FileList::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:FileList",
                  b"FileList",
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
                      ty: "DocVersion",
                      field: "doc_root",
                      tag_name_prefix: b"ofd:DocRoot",
                      tag_name: b"DocRoot",
                    },
                  )?
                }
              };
              doc_root = Some(parsed_value);
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
    let id = id.ok_or_else(|| crate::common::missing_field("DocVersion", "id"))?;
    let file_list =
      file_list.ok_or_else(|| crate::common::missing_field("DocVersion", "file_list"))?;
    let doc_root =
      doc_root.ok_or_else(|| crate::common::missing_field("DocVersion", "doc_root"))?;
    Ok(Self {
      id,
      version,
      name,
      creation_date,
      file_list,
      doc_root,
    })
  }
}
