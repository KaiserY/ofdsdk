//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

impl std::str::FromStr for crate::schemas::definitions::CtDestType {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "XYZ" => Ok(Self::Xyz),
      "Fit" => Ok(Self::Fit),
      "FitH" => Ok(Self::FitH),
      "FitV" => Ok(Self::FitV),
      "FitR" => Ok(Self::FitR),
      _ => Err(crate::common::invalid_enum_value("CtDestType", s)),
    }
  }
}
impl crate::schemas::definitions::CtDestType {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"XYZ" => Ok(Self::Xyz),
      b"Fit" => Ok(Self::Fit),
      b"FitH" => Ok(Self::FitH),
      b"FitV" => Ok(Self::FitV),
      b"FitR" => Ok(Self::FitR),
      other => Err(crate::common::invalid_enum_value(
        "CtDestType",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::definitions::MovieOperator {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Play" => Ok(Self::Play),
      "Stop" => Ok(Self::Stop),
      "Pause" => Ok(Self::Pause),
      "Resume" => Ok(Self::Resume),
      _ => Err(crate::common::invalid_enum_value("MovieOperator", s)),
    }
  }
}
impl crate::schemas::definitions::MovieOperator {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"Play" => Ok(Self::Play),
      b"Stop" => Ok(Self::Stop),
      b"Pause" => Ok(Self::Pause),
      b"Resume" => Ok(Self::Resume),
      other => Err(crate::common::invalid_enum_value(
        "MovieOperator",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::definitions::CtActionEvent {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "DO" => Ok(Self::Do),
      "PO" => Ok(Self::Po),
      "CLICK" => Ok(Self::Click),
      _ => Err(crate::common::invalid_enum_value("CtActionEvent", s)),
    }
  }
}
impl crate::schemas::definitions::CtActionEvent {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"DO" => Ok(Self::Do),
      b"PO" => Ok(Self::Po),
      b"CLICK" => Ok(Self::Click),
      other => Err(crate::common::invalid_enum_value(
        "CtActionEvent",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::definitions::CtDest {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_Dest", b"CT_Dest")
  }
}
impl crate::schemas::definitions::CtDest {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_Dest", b"CT_Dest")
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
      "CtDest",
      "CT_Dest",
      tag_name_prefix,
      tag_name
    );
    let mut r#type = None;
    let mut page_id = None;
    let mut left = None;
    let mut top = None;
    let mut right = None;
    let mut bottom = None;
    let mut zoom = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Type" => {
          r#type = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::definitions::CtDestType>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::definitions::CtDestType>()?
          });
        }
        b"PageID" => {
          page_id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtDest",
            "page_id",
          )?);
        }
        b"Left" => {
          left = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtDest",
            "left",
          )?);
        }
        b"Top" => {
          top = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtDest",
            "top",
          )?);
        }
        b"Right" => {
          right = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtDest",
            "right",
          )?);
        }
        b"Bottom" => {
          bottom = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtDest",
            "bottom",
          )?);
        }
        b"Zoom" => {
          zoom = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtDest",
            "zoom",
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtDest"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end(e.to_end().name())?;
        }
      }
    }
    let r#type = r#type.ok_or_else(|| crate::common::missing_field("CtDest", "type"))?;
    let page_id = page_id.ok_or_else(|| crate::common::missing_field("CtDest", "page_id"))?;
    Ok(Self {
      r#type,
      page_id,
      left,
      top,
      right,
      bottom,
      zoom,
    })
  }
}
impl std::str::FromStr for crate::schemas::definitions::CtPageArea {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_PageArea", b"CT_PageArea")
  }
}
impl crate::schemas::definitions::CtPageArea {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_PageArea", b"CT_PageArea")
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
      "CtPageArea",
      "CT_PageArea",
      tag_name_prefix,
      tag_name
    );
    let mut physical_box = None;
    let mut application_box = None;
    let mut content_box = None;
    let mut bleed_box = None;
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtPageArea"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:PhysicalBox" | b"PhysicalBox" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content(
                    xml_reader,
                    "CtPageArea",
                    "physical_box",
                    b"ofd:PhysicalBox",
                    b"PhysicalBox",
                  )?
                }
              };
              physical_box = Some(parsed_value);
            }
            b"ofd:ApplicationBox" | b"ApplicationBox" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content(
                    xml_reader,
                    "CtPageArea",
                    "application_box",
                    b"ofd:ApplicationBox",
                    b"ApplicationBox",
                  )?
                }
              };
              application_box = Some(parsed_value);
            }
            b"ofd:ContentBox" | b"ContentBox" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content(
                    xml_reader,
                    "CtPageArea",
                    "content_box",
                    b"ofd:ContentBox",
                    b"ContentBox",
                  )?
                }
              };
              content_box = Some(parsed_value);
            }
            b"ofd:BleedBox" | b"BleedBox" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content(
                    xml_reader,
                    "CtPageArea",
                    "bleed_box",
                    b"ofd:BleedBox",
                    b"BleedBox",
                  )?
                }
              };
              bleed_box = Some(parsed_value);
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
    let physical_box =
      physical_box.ok_or_else(|| crate::common::missing_field("CtPageArea", "physical_box"))?;
    Ok(Self {
      physical_box,
      application_box,
      content_box,
      bleed_box,
    })
  }
}
impl std::str::FromStr for crate::schemas::definitions::Bookmark {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Bookmark", b"Bookmark")
  }
}
impl crate::schemas::definitions::Bookmark {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Bookmark", b"Bookmark")
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
      "Bookmark",
      "Bookmark",
      tag_name_prefix,
      tag_name
    );
    let mut name = None;
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
          quick_xml::events::Event::End(e) => match e.name().as_ref() {
            name if name == tag_name_prefix || name == tag_name => {
              break;
            }
            _ => {}
          },
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Bookmark"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end(e.to_end().name())?;
        }
      }
    }
    let name = name.ok_or_else(|| crate::common::missing_field("Bookmark", "name"))?;
    Ok(Self { name })
  }
}
impl std::str::FromStr for crate::schemas::definitions::Goto {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Goto", b"Goto")
  }
}
impl crate::schemas::definitions::Goto {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Goto", b"Goto")
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
      "Goto",
      "Goto",
      tag_name_prefix,
      tag_name
    );
    let mut xml_children = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Goto"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Dest" | b"Dest" => {
              xml_children.push(crate::schemas::definitions::GotoContentChoice::Dest(
                Box::new(
                  crate::schemas::definitions::CtDest::deserialize_inner_named(
                    xml_reader,
                    Some((e, e_empty)),
                    b"ofd:Dest",
                    b"Dest",
                  )?,
                ),
              ));
            }
            b"ofd:Bookmark" | b"Bookmark" => {
              xml_children.push(crate::schemas::definitions::GotoContentChoice::Bookmark(
                Box::new(
                  crate::schemas::definitions::Bookmark::deserialize_inner_named(
                    xml_reader,
                    Some((e, e_empty)),
                    b"ofd:Bookmark",
                    b"Bookmark",
                  )?,
                ),
              ));
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
    Ok(Self { xml_children })
  }
}
impl std::str::FromStr for crate::schemas::definitions::Uri {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:URI", b"URI")
  }
}
impl crate::schemas::definitions::Uri {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:URI", b"URI")
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
      "Uri",
      "URI",
      tag_name_prefix,
      tag_name
    );
    let mut uri = None;
    let mut base = None;
    let mut target = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"URI" => {
          uri = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Base" => {
          base = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Target" => {
          target =
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Uri"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end(e.to_end().name())?;
        }
      }
    }
    let uri = uri.ok_or_else(|| crate::common::missing_field("Uri", "uri"))?;
    Ok(Self { uri, base, target })
  }
}
impl std::str::FromStr for crate::schemas::definitions::GotoA {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:GotoA", b"GotoA")
  }
}
impl crate::schemas::definitions::GotoA {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:GotoA", b"GotoA")
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
      "GotoA",
      "GotoA",
      tag_name_prefix,
      tag_name
    );
    let mut attach_id = None;
    let mut new_window = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"AttachID" => {
          attach_id =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"NewWindow" => {
          new_window = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "GotoA",
            "new_window",
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("GotoA"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end(e.to_end().name())?;
        }
      }
    }
    let attach_id = attach_id.ok_or_else(|| crate::common::missing_field("GotoA", "attach_id"))?;
    Ok(Self {
      attach_id,
      new_window,
    })
  }
}
impl std::str::FromStr for crate::schemas::definitions::Sound {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Sound", b"Sound")
  }
}
impl crate::schemas::definitions::Sound {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Sound", b"Sound")
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
      "Sound",
      "Sound",
      tag_name_prefix,
      tag_name
    );
    let mut resource_id = None;
    let mut volume = None;
    let mut repeat = None;
    let mut synchronous = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"ResourceID" => {
          resource_id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "Sound",
            "resource_id",
          )?);
        }
        b"Volume" => {
          volume = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "Sound",
            "volume",
          )?);
        }
        b"Repeat" => {
          repeat = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "Sound",
            "repeat",
          )?);
        }
        b"Synchronous" => {
          synchronous = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "Sound",
            "synchronous",
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Sound"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end(e.to_end().name())?;
        }
      }
    }
    let resource_id =
      resource_id.ok_or_else(|| crate::common::missing_field("Sound", "resource_id"))?;
    Ok(Self {
      resource_id,
      volume,
      repeat,
      synchronous,
    })
  }
}
impl std::str::FromStr for crate::schemas::definitions::Movie {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Movie", b"Movie")
  }
}
impl crate::schemas::definitions::Movie {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Movie", b"Movie")
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
      "Movie",
      "Movie",
      tag_name_prefix,
      tag_name
    );
    let mut resource_id = None;
    let mut operator = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"ResourceID" => {
          resource_id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "Movie",
            "resource_id",
          )?);
        }
        b"Operator" => {
          operator = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::definitions::MovieOperator>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::definitions::MovieOperator>()?
          });
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Movie"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end(e.to_end().name())?;
        }
      }
    }
    let resource_id =
      resource_id.ok_or_else(|| crate::common::missing_field("Movie", "resource_id"))?;
    Ok(Self {
      resource_id,
      operator,
    })
  }
}
impl std::str::FromStr for crate::schemas::definitions::CtAction {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_Action", b"CT_Action")
  }
}
impl crate::schemas::definitions::CtAction {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_Action", b"CT_Action")
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
      "CtAction",
      "CT_Action",
      tag_name_prefix,
      tag_name
    );
    let mut event = None;
    let mut region = None;
    let mut xml_children = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Event" => {
          event = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::definitions::CtActionEvent>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::definitions::CtActionEvent>()?
          });
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtAction"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Goto" | b"Goto" => {
              xml_children.push(crate::schemas::definitions::CtActionContentChoice::Goto(
                Box::new(crate::schemas::definitions::Goto::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Goto",
                  b"Goto",
                )?),
              ));
            }
            b"ofd:URI" | b"URI" => {
              xml_children.push(crate::schemas::definitions::CtActionContentChoice::Uri(
                Box::new(crate::schemas::definitions::Uri::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:URI",
                  b"URI",
                )?),
              ));
            }
            b"ofd:GotoA" | b"GotoA" => {
              xml_children.push(crate::schemas::definitions::CtActionContentChoice::GotoA(
                Box::new(crate::schemas::definitions::GotoA::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:GotoA",
                  b"GotoA",
                )?),
              ));
            }
            b"ofd:Sound" | b"Sound" => {
              xml_children.push(crate::schemas::definitions::CtActionContentChoice::Sound(
                Box::new(crate::schemas::definitions::Sound::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Sound",
                  b"Sound",
                )?),
              ));
            }
            b"ofd:Movie" | b"Movie" => {
              xml_children.push(crate::schemas::definitions::CtActionContentChoice::Movie(
                Box::new(crate::schemas::definitions::Movie::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Movie",
                  b"Movie",
                )?),
              ));
            }
            b"ofd:Region" | b"Region" => {
              region = Some(
                crate::schemas::definitions::CtRegion::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Region",
                  b"Region",
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
    let event = event.ok_or_else(|| crate::common::missing_field("CtAction", "event"))?;
    Ok(Self {
      event,
      region,
      xml_children,
    })
  }
}
impl std::str::FromStr for crate::schemas::definitions::Move {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Move", b"Move")
  }
}
impl crate::schemas::definitions::Move {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Move", b"Move")
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
      "Move",
      "Move",
      tag_name_prefix,
      tag_name
    );
    let mut point1 = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Point1" => {
          point1 =
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Move"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end(e.to_end().name())?;
        }
      }
    }
    let point1 = point1.ok_or_else(|| crate::common::missing_field("Move", "point1"))?;
    Ok(Self { point1 })
  }
}
impl std::str::FromStr for crate::schemas::definitions::Line {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Line", b"Line")
  }
}
impl crate::schemas::definitions::Line {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Line", b"Line")
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
      "Line",
      "Line",
      tag_name_prefix,
      tag_name
    );
    let mut point1 = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Point1" => {
          point1 =
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Line"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end(e.to_end().name())?;
        }
      }
    }
    let point1 = point1.ok_or_else(|| crate::common::missing_field("Line", "point1"))?;
    Ok(Self { point1 })
  }
}
impl std::str::FromStr for crate::schemas::definitions::QuadraticBezier {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(
      &mut xml_reader,
      None,
      b"ofd:QuadraticBezier",
      b"QuadraticBezier",
    )
  }
}
impl crate::schemas::definitions::QuadraticBezier {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(
      &mut xml_reader,
      None,
      b"ofd:QuadraticBezier",
      b"QuadraticBezier",
    )
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
      "QuadraticBezier",
      "QuadraticBezier",
      tag_name_prefix,
      tag_name
    );
    let mut point1 = None;
    let mut point2 = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Point1" => {
          point1 =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Point2" => {
          point2 =
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("QuadraticBezier"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end(e.to_end().name())?;
        }
      }
    }
    let point1 = point1.ok_or_else(|| crate::common::missing_field("QuadraticBezier", "point1"))?;
    let point2 = point2.ok_or_else(|| crate::common::missing_field("QuadraticBezier", "point2"))?;
    Ok(Self { point1, point2 })
  }
}
impl std::str::FromStr for crate::schemas::definitions::CubicBezier {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CubicBezier", b"CubicBezier")
  }
}
impl crate::schemas::definitions::CubicBezier {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CubicBezier", b"CubicBezier")
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
      "CubicBezier",
      "CubicBezier",
      tag_name_prefix,
      tag_name
    );
    let mut point1 = None;
    let mut point2 = None;
    let mut point3 = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Point1" => {
          point1 =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Point2" => {
          point2 =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Point3" => {
          point3 =
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CubicBezier"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end(e.to_end().name())?;
        }
      }
    }
    let point3 = point3.ok_or_else(|| crate::common::missing_field("CubicBezier", "point3"))?;
    Ok(Self {
      point1,
      point2,
      point3,
    })
  }
}
impl std::str::FromStr for crate::schemas::definitions::Arc {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Arc", b"Arc")
  }
}
impl crate::schemas::definitions::Arc {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Arc", b"Arc")
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
      "Arc",
      "Arc",
      tag_name_prefix,
      tag_name
    );
    let mut sweep_direction = None;
    let mut large_arc = None;
    let mut rotation_angle = None;
    let mut ellipse_size = None;
    let mut end_point = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"SweepDirection" => {
          sweep_direction = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "Arc",
            "sweep_direction",
          )?);
        }
        b"LargeArc" => {
          large_arc = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "Arc",
            "large_arc",
          )?);
        }
        b"RotationAngle" => {
          rotation_angle = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "Arc",
            "rotation_angle",
          )?);
        }
        b"EllipseSize" => {
          ellipse_size =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"EndPoint" => {
          end_point =
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Arc"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end(e.to_end().name())?;
        }
      }
    }
    let sweep_direction =
      sweep_direction.ok_or_else(|| crate::common::missing_field("Arc", "sweep_direction"))?;
    let large_arc = large_arc.ok_or_else(|| crate::common::missing_field("Arc", "large_arc"))?;
    let rotation_angle =
      rotation_angle.ok_or_else(|| crate::common::missing_field("Arc", "rotation_angle"))?;
    let ellipse_size =
      ellipse_size.ok_or_else(|| crate::common::missing_field("Arc", "ellipse_size"))?;
    let end_point = end_point.ok_or_else(|| crate::common::missing_field("Arc", "end_point"))?;
    Ok(Self {
      sweep_direction,
      large_arc,
      rotation_angle,
      ellipse_size,
      end_point,
    })
  }
}
impl std::str::FromStr for crate::schemas::definitions::Close {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Close", b"Close")
  }
}
impl crate::schemas::definitions::Close {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Close", b"Close")
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
      "Close",
      "Close",
      tag_name_prefix,
      tag_name
    );
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Close"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end(e.to_end().name())?;
        }
      }
    }
    Ok(Self {})
  }
}
impl std::str::FromStr for crate::schemas::definitions::Area {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Area", b"Area")
  }
}
impl crate::schemas::definitions::Area {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Area", b"Area")
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
      "Area",
      "Area",
      tag_name_prefix,
      tag_name
    );
    let mut start = None;
    let mut xml_children = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Start" => {
          start = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Area"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Move" | b"Move" => {
              xml_children.push(crate::schemas::definitions::AreaContentChoice::Move(
                Box::new(crate::schemas::definitions::Move::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Move",
                  b"Move",
                )?),
              ));
            }
            b"ofd:Line" | b"Line" => {
              xml_children.push(crate::schemas::definitions::AreaContentChoice::Line(
                Box::new(crate::schemas::definitions::Line::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Line",
                  b"Line",
                )?),
              ));
            }
            b"ofd:QuadraticBezier" | b"QuadraticBezier" => {
              xml_children.push(
                crate::schemas::definitions::AreaContentChoice::QuadraticBezier(Box::new(
                  crate::schemas::definitions::QuadraticBezier::deserialize_inner_named(
                    xml_reader,
                    Some((e, e_empty)),
                    b"ofd:QuadraticBezier",
                    b"QuadraticBezier",
                  )?,
                )),
              );
            }
            b"ofd:CubicBezier" | b"CubicBezier" => {
              xml_children.push(crate::schemas::definitions::AreaContentChoice::CubicBezier(
                Box::new(
                  crate::schemas::definitions::CubicBezier::deserialize_inner_named(
                    xml_reader,
                    Some((e, e_empty)),
                    b"ofd:CubicBezier",
                    b"CubicBezier",
                  )?,
                ),
              ));
            }
            b"ofd:Arc" | b"Arc" => {
              xml_children.push(crate::schemas::definitions::AreaContentChoice::Arc(
                Box::new(crate::schemas::definitions::Arc::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Arc",
                  b"Arc",
                )?),
              ));
            }
            b"ofd:Close" | b"Close" => {
              xml_children.push(crate::schemas::definitions::AreaContentChoice::Close(
                Box::new(crate::schemas::definitions::Close::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Close",
                  b"Close",
                )?),
              ));
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
    let start = start.ok_or_else(|| crate::common::missing_field("Area", "start"))?;
    Ok(Self {
      start,
      xml_children,
    })
  }
}
impl std::str::FromStr for crate::schemas::definitions::CtRegion {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_Region", b"CT_Region")
  }
}
impl crate::schemas::definitions::CtRegion {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_Region", b"CT_Region")
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
      "CtRegion",
      "CT_Region",
      tag_name_prefix,
      tag_name
    );
    let mut area = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtRegion"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Area" | b"Area" => {
              area.push(crate::schemas::definitions::Area::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Area",
                b"Area",
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
    Ok(Self { area })
  }
}
impl std::str::FromStr for crate::schemas::definitions::Region {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Region", b"Region")
  }
}
impl crate::schemas::definitions::Region {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Region", b"Region")
  }
  pub(crate) fn deserialize_inner_named<'de, R: crate::common::XmlReader<'de>>(
    xml_reader: &mut R,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::definitions::CtRegion>::deserialize_inner_named(
        xml_reader,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
impl std::str::FromStr for crate::schemas::definitions::Dest {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Dest", b"Dest")
  }
}
impl crate::schemas::definitions::Dest {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Dest", b"Dest")
  }
  pub(crate) fn deserialize_inner_named<'de, R: crate::common::XmlReader<'de>>(
    xml_reader: &mut R,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::definitions::CtDest>::deserialize_inner_named(
        xml_reader,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
