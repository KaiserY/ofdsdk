//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

impl std::str::FromStr for crate::schemas::custom_tags::CustomTag {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CustomTag", b"CustomTag")
  }
}
impl crate::schemas::custom_tags::CustomTag {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CustomTag", b"CustomTag")
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
      "CustomTag",
      "CustomTag",
      tag_name_prefix,
      tag_name
    );
    let mut name_space = None;
    let mut schema_loc = None;
    let mut file_loc = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"NameSpace" => {
          name_space =
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CustomTag"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:SchemaLoc" | b"SchemaLoc" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content(
                    xml_reader,
                    "CustomTag",
                    "schema_loc",
                    b"ofd:SchemaLoc",
                    b"SchemaLoc",
                  )?
                }
              };
              schema_loc = Some(parsed_value);
            }
            b"ofd:FileLoc" | b"FileLoc" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content(
                    xml_reader,
                    "CustomTag",
                    "file_loc",
                    b"ofd:FileLoc",
                    b"FileLoc",
                  )?
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
    let name_space = match name_space {
      Some(value) => value,
      None => "".to_string(),
    };
    let file_loc = file_loc.ok_or_else(|| crate::common::missing_field("CustomTag", "file_loc"))?;
    Ok(Self {
      name_space,
      schema_loc,
      file_loc,
    })
  }
}
impl std::str::FromStr for crate::schemas::custom_tags::CustomTags {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CustomTags", b"CustomTags")
  }
}
impl crate::schemas::custom_tags::CustomTags {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CustomTags", b"CustomTags")
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
      "CustomTags",
      "CustomTags",
      tag_name_prefix,
      tag_name
    );
    let mut custom_tag = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CustomTags"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:CustomTag" | b"CustomTag" => {
              custom_tag.push(
                crate::schemas::custom_tags::CustomTag::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:CustomTag",
                  b"CustomTag",
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
    Ok(Self { custom_tag })
  }
}
