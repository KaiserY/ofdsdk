//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

impl std::str::FromStr for crate::schemas::annotation::AnnotType {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Link" => Ok(Self::Link),
      "Path" => Ok(Self::Path),
      "Highlight" => Ok(Self::Highlight),
      "Stamp" => Ok(Self::Stamp),
      "Watermark" => Ok(Self::Watermark),
      _ => Err(crate::common::invalid_enum_value("AnnotType", s)),
    }
  }
}
impl crate::schemas::annotation::AnnotType {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"Link" => Ok(Self::Link),
      b"Path" => Ok(Self::Path),
      b"Highlight" => Ok(Self::Highlight),
      b"Stamp" => Ok(Self::Stamp),
      b"Watermark" => Ok(Self::Watermark),
      other => Err(crate::common::invalid_enum_value(
        "AnnotType",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::annotation::Parameter {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Parameter", b"Parameter")
  }
}
impl crate::schemas::annotation::Parameter {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Parameter", b"Parameter")
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
      "Parameter",
      "Parameter",
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
        match xml_reader.next()? {
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
                  "Parameter",
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Parameter"))?,
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
      Err(crate::common::missing_field("Parameter", "xml_value"))?
    };
    let name = name.ok_or_else(|| crate::common::missing_field("Parameter", "name"))?;
    Ok(Self { name, xml_value })
  }
}
impl std::str::FromStr for crate::schemas::annotation::Parameters {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Parameters", b"Parameters")
  }
}
impl crate::schemas::annotation::Parameters {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Parameters", b"Parameters")
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
      "Parameters",
      "Parameters",
      tag_name_prefix,
      tag_name
    );
    let mut parameter = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Parameters"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Parameter" | b"Parameter" => {
              parameter.push(
                crate::schemas::annotation::Parameter::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Parameter",
                  b"Parameter",
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
    Ok(Self { parameter })
  }
}
impl std::str::FromStr for crate::schemas::annotation::Appearance {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Appearance", b"Appearance")
  }
}
impl crate::schemas::annotation::Appearance {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Appearance", b"Appearance")
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
      "Appearance",
      "Appearance",
      tag_name_prefix,
      tag_name
    );
    let mut boundary = None;
    let mut xml_children = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Boundary" => {
          boundary =
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Appearance"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:TextObject" | b"TextObject" => {
              xml_children.push(
                crate::schemas::annotation::AppearanceContentChoice::TextObject(Box::new(
                  crate::schemas::page::TextObject::deserialize_inner_named(
                    xml_reader,
                    Some((e, e_empty)),
                    b"ofd:TextObject",
                    b"TextObject",
                  )?,
                )),
              );
            }
            b"ofd:PathObject" | b"PathObject" => {
              xml_children.push(
                crate::schemas::annotation::AppearanceContentChoice::PathObject(Box::new(
                  crate::schemas::page::PathObject::deserialize_inner_named(
                    xml_reader,
                    Some((e, e_empty)),
                    b"ofd:PathObject",
                    b"PathObject",
                  )?,
                )),
              );
            }
            b"ofd:ImageObject" | b"ImageObject" => {
              xml_children.push(
                crate::schemas::annotation::AppearanceContentChoice::ImageObject(Box::new(
                  crate::schemas::page::ImageObject::deserialize_inner_named(
                    xml_reader,
                    Some((e, e_empty)),
                    b"ofd:ImageObject",
                    b"ImageObject",
                  )?,
                )),
              );
            }
            b"ofd:CompositeObject" | b"CompositeObject" => {
              xml_children.push(
                crate::schemas::annotation::AppearanceContentChoice::CompositeObject(Box::new(
                  crate::schemas::page::CompositeObject::deserialize_inner_named(
                    xml_reader,
                    Some((e, e_empty)),
                    b"ofd:CompositeObject",
                    b"CompositeObject",
                  )?,
                )),
              );
            }
            b"ofd:PageBlock" | b"PageBlock" => {
              xml_children.push(
                crate::schemas::annotation::AppearanceContentChoice::PageBlock(Box::new(
                  crate::schemas::page::PageBlock::deserialize_inner_named(
                    xml_reader,
                    Some((e, e_empty)),
                    b"ofd:PageBlock",
                    b"PageBlock",
                  )?,
                )),
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
      boundary,
      xml_children,
    })
  }
}
impl std::str::FromStr for crate::schemas::annotation::Annot {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Annot", b"Annot")
  }
}
impl crate::schemas::annotation::Annot {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Annot", b"Annot")
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
      "Annot",
      "Annot",
      tag_name_prefix,
      tag_name
    );
    let mut id = None;
    let mut r#type = None;
    let mut creator = None;
    let mut last_mod_date = None;
    let mut visible = None;
    let mut subtype = None;
    let mut print = None;
    let mut no_zoom = None;
    let mut no_rotate = None;
    let mut read_only = None;
    let mut remark = None;
    let mut parameters = None;
    let mut appearance = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"ID" => {
          id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "Annot",
            "id",
          )?);
        }
        b"Type" => {
          r#type = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::annotation::AnnotType>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::annotation::AnnotType>()?
          });
        }
        b"Creator" => {
          creator =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"LastModDate" => {
          last_mod_date =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Visible" => {
          visible = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "Annot",
            "visible",
          )?);
        }
        b"Subtype" => {
          subtype =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Print" => {
          print = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "Annot",
            "print",
          )?);
        }
        b"NoZoom" => {
          no_zoom = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "Annot",
            "no_zoom",
          )?);
        }
        b"NoRotate" => {
          no_rotate = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "Annot",
            "no_rotate",
          )?);
        }
        b"ReadOnly" => {
          read_only = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "Annot",
            "read_only",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Annot"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Remark" | b"Remark" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content(
                    xml_reader,
                    "Annot",
                    "remark",
                    b"ofd:Remark",
                    b"Remark",
                  )?
                }
              };
              remark = Some(parsed_value);
            }
            b"ofd:Parameters" | b"Parameters" => {
              parameters = Some(
                crate::schemas::annotation::Parameters::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Parameters",
                  b"Parameters",
                )?,
              );
            }
            b"ofd:Appearance" | b"Appearance" => {
              appearance = Some(
                crate::schemas::annotation::Appearance::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Appearance",
                  b"Appearance",
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
    let id = id.ok_or_else(|| crate::common::missing_field("Annot", "id"))?;
    let r#type = r#type.ok_or_else(|| crate::common::missing_field("Annot", "type"))?;
    let creator = match creator {
      Some(value) => value,
      None => "".to_string(),
    };
    let last_mod_date = match last_mod_date {
      Some(value) => value,
      None => "".to_string(),
    };
    let appearance =
      appearance.ok_or_else(|| crate::common::missing_field("Annot", "appearance"))?;
    Ok(Self {
      id,
      r#type,
      creator,
      last_mod_date,
      visible,
      subtype,
      print,
      no_zoom,
      no_rotate,
      read_only,
      remark,
      parameters,
      appearance,
    })
  }
}
impl std::str::FromStr for crate::schemas::annotation::PageAnnot {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:PageAnnot", b"PageAnnot")
  }
}
impl crate::schemas::annotation::PageAnnot {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:PageAnnot", b"PageAnnot")
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
      "PageAnnot",
      "PageAnnot",
      tag_name_prefix,
      tag_name
    );
    let mut annot = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("PageAnnot"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Annot" | b"Annot" => {
              annot.push(crate::schemas::annotation::Annot::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Annot",
                b"Annot",
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
    Ok(Self { annot })
  }
}
