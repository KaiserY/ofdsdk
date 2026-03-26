//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

impl std::str::FromStr for crate::schemas::page::TemplateZOrder {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Background" => Ok(Self::Background),
      "Foreground" => Ok(Self::Foreground),
      _ => Err(crate::common::invalid_enum_value("TemplateZOrder", s)),
    }
  }
}
impl crate::schemas::page::TemplateZOrder {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"Background" => Ok(Self::Background),
      b"Foreground" => Ok(Self::Foreground),
      other => Err(crate::common::invalid_enum_value(
        "TemplateZOrder",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::page::CtLayerType {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Body" => Ok(Self::Body),
      "Background" => Ok(Self::Background),
      "Foreground" => Ok(Self::Foreground),
      "Custom" => Ok(Self::Custom),
      _ => Err(crate::common::invalid_enum_value("CtLayerType", s)),
    }
  }
}
impl crate::schemas::page::CtLayerType {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"Body" => Ok(Self::Body),
      b"Background" => Ok(Self::Background),
      b"Foreground" => Ok(Self::Foreground),
      b"Custom" => Ok(Self::Custom),
      other => Err(crate::common::invalid_enum_value(
        "CtLayerType",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::page::CtGraphicUnitCap {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Butt" => Ok(Self::Butt),
      "Round" => Ok(Self::Round),
      "Square" => Ok(Self::Square),
      _ => Err(crate::common::invalid_enum_value("CtGraphicUnitCap", s)),
    }
  }
}
impl crate::schemas::page::CtGraphicUnitCap {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"Butt" => Ok(Self::Butt),
      b"Round" => Ok(Self::Round),
      b"Square" => Ok(Self::Square),
      other => Err(crate::common::invalid_enum_value(
        "CtGraphicUnitCap",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::page::CtGraphicUnitJoin {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Miter" => Ok(Self::Miter),
      "Round" => Ok(Self::Round),
      "Bevel" => Ok(Self::Bevel),
      _ => Err(crate::common::invalid_enum_value("CtGraphicUnitJoin", s)),
    }
  }
}
impl crate::schemas::page::CtGraphicUnitJoin {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"Miter" => Ok(Self::Miter),
      b"Round" => Ok(Self::Round),
      b"Bevel" => Ok(Self::Bevel),
      other => Err(crate::common::invalid_enum_value(
        "CtGraphicUnitJoin",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::page::CtTextWeight {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "0" => Ok(Self::_0),
      "100" => Ok(Self::_100),
      "200" => Ok(Self::_200),
      "300" => Ok(Self::_300),
      "400" => Ok(Self::_400),
      "500" => Ok(Self::_500),
      "600" => Ok(Self::_600),
      "700" => Ok(Self::_700),
      "800" => Ok(Self::_800),
      "900" => Ok(Self::_900),
      "1000" => Ok(Self::_1000),
      _ => Err(crate::common::invalid_enum_value("CtTextWeight", s)),
    }
  }
}
impl crate::schemas::page::CtTextWeight {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"0" => Ok(Self::_0),
      b"100" => Ok(Self::_100),
      b"200" => Ok(Self::_200),
      b"300" => Ok(Self::_300),
      b"400" => Ok(Self::_400),
      b"500" => Ok(Self::_500),
      b"600" => Ok(Self::_600),
      b"700" => Ok(Self::_700),
      b"800" => Ok(Self::_800),
      b"900" => Ok(Self::_900),
      b"1000" => Ok(Self::_1000),
      other => Err(crate::common::invalid_enum_value(
        "CtTextWeight",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::page::CtPathRule {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "NonZero" => Ok(Self::NonZero),
      "Even-Odd" => Ok(Self::_EvenOdd),
      _ => Err(crate::common::invalid_enum_value("CtPathRule", s)),
    }
  }
}
impl crate::schemas::page::CtPathRule {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"NonZero" => Ok(Self::NonZero),
      b"Even-Odd" => Ok(Self::_EvenOdd),
      other => Err(crate::common::invalid_enum_value(
        "CtPathRule",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::page::CtPatternReflectMethod {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Normal" => Ok(Self::Normal),
      "Row" => Ok(Self::Row),
      "Column" => Ok(Self::Column),
      "RowAndColumn" => Ok(Self::RowAndColumn),
      _ => Err(crate::common::invalid_enum_value(
        "CtPatternReflectMethod",
        s,
      )),
    }
  }
}
impl crate::schemas::page::CtPatternReflectMethod {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"Normal" => Ok(Self::Normal),
      b"Row" => Ok(Self::Row),
      b"Column" => Ok(Self::Column),
      b"RowAndColumn" => Ok(Self::RowAndColumn),
      other => Err(crate::common::invalid_enum_value(
        "CtPatternReflectMethod",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::page::CtPatternRelativeTo {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Page" => Ok(Self::Page),
      "Object" => Ok(Self::Object),
      _ => Err(crate::common::invalid_enum_value("CtPatternRelativeTo", s)),
    }
  }
}
impl crate::schemas::page::CtPatternRelativeTo {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"Page" => Ok(Self::Page),
      b"Object" => Ok(Self::Object),
      other => Err(crate::common::invalid_enum_value(
        "CtPatternRelativeTo",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::page::CtAxialShdMapType {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Direct" => Ok(Self::Direct),
      "Repeat" => Ok(Self::Repeat),
      "Reflect" => Ok(Self::Reflect),
      _ => Err(crate::common::invalid_enum_value("CtAxialShdMapType", s)),
    }
  }
}
impl crate::schemas::page::CtAxialShdMapType {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"Direct" => Ok(Self::Direct),
      b"Repeat" => Ok(Self::Repeat),
      b"Reflect" => Ok(Self::Reflect),
      other => Err(crate::common::invalid_enum_value(
        "CtAxialShdMapType",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::page::CtAxialShdExtend {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "0" => Ok(Self::_0),
      "1" => Ok(Self::_1),
      "2" => Ok(Self::_2),
      "3" => Ok(Self::_3),
      _ => Err(crate::common::invalid_enum_value("CtAxialShdExtend", s)),
    }
  }
}
impl crate::schemas::page::CtAxialShdExtend {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"0" => Ok(Self::_0),
      b"1" => Ok(Self::_1),
      b"2" => Ok(Self::_2),
      b"3" => Ok(Self::_3),
      other => Err(crate::common::invalid_enum_value(
        "CtAxialShdExtend",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::page::CtRadialShdMapType {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Direct" => Ok(Self::Direct),
      "Repeat" => Ok(Self::Repeat),
      "Reflect" => Ok(Self::Reflect),
      _ => Err(crate::common::invalid_enum_value("CtRadialShdMapType", s)),
    }
  }
}
impl crate::schemas::page::CtRadialShdMapType {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"Direct" => Ok(Self::Direct),
      b"Repeat" => Ok(Self::Repeat),
      b"Reflect" => Ok(Self::Reflect),
      other => Err(crate::common::invalid_enum_value(
        "CtRadialShdMapType",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::page::PointEdgeFlag {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "0" => Ok(Self::_0),
      "1" => Ok(Self::_1),
      "2" => Ok(Self::_2),
      _ => Err(crate::common::invalid_enum_value("PointEdgeFlag", s)),
    }
  }
}
impl crate::schemas::page::PointEdgeFlag {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"0" => Ok(Self::_0),
      b"1" => Ok(Self::_1),
      b"2" => Ok(Self::_2),
      other => Err(crate::common::invalid_enum_value(
        "PointEdgeFlag",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::page::Template {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Template", b"Template")
  }
}
impl crate::schemas::page::Template {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:Template",
      b"Template",
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
      "Template",
      "Template",
      tag_name_prefix,
      tag_name
    );
    let mut template_id = None;
    let mut z_order = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"TemplateID" => {
          template_id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "Template",
            "template_id",
          )?);
        }
        b"ZOrder" => {
          z_order = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::TemplateZOrder>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::TemplateZOrder>()?
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Template"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end(e.to_end().name())?;
        }
      }
    }
    let template_id =
      template_id.ok_or_else(|| crate::common::missing_field("Template", "template_id"))?;
    Ok(Self {
      template_id,
      z_order,
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
      "Template",
      "Template",
      tag_name_prefix,
      tag_name
    );
    let mut template_id = None;
    let mut z_order = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"TemplateID" => {
          template_id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "Template",
            "template_id",
          )?);
        }
        b"ZOrder" => {
          z_order = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::TemplateZOrder>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::TemplateZOrder>()?
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Template"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end_into(e.to_end().name(), buf)?;
        }
      }
    }
    let template_id =
      template_id.ok_or_else(|| crate::common::missing_field("Template", "template_id"))?;
    Ok(Self {
      template_id,
      z_order,
    })
  }
}
impl std::str::FromStr for crate::schemas::page::Layer {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Layer", b"Layer")
  }
}
impl crate::schemas::page::Layer {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:Layer", b"Layer")
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
      "Layer",
      "Layer",
      tag_name_prefix,
      tag_name
    );
    let mut r#type = None;
    let mut draw_param = None;
    let mut id = None;
    let mut xml_children = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Type" => {
          r#type = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtLayerType>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtLayerType>()?
          });
        }
        b"DrawParam" => {
          draw_param = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "Layer",
            "draw_param",
          )?);
        }
        b"ID" => {
          id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "Layer",
            "id",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Layer"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:TextObject" | b"TextObject" => {
              xml_children.push(crate::schemas::page::LayerContentChoice::TextObject(
                Box::new(crate::schemas::page::TextObject::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:TextObject",
                  b"TextObject",
                )?),
              ));
            }
            b"ofd:PathObject" | b"PathObject" => {
              xml_children.push(crate::schemas::page::LayerContentChoice::PathObject(
                Box::new(crate::schemas::page::PathObject::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:PathObject",
                  b"PathObject",
                )?),
              ));
            }
            b"ofd:ImageObject" | b"ImageObject" => {
              xml_children.push(crate::schemas::page::LayerContentChoice::ImageObject(
                Box::new(crate::schemas::page::ImageObject::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:ImageObject",
                  b"ImageObject",
                )?),
              ));
            }
            b"ofd:CompositeObject" | b"CompositeObject" => {
              xml_children.push(crate::schemas::page::LayerContentChoice::CompositeObject(
                Box::new(
                  crate::schemas::page::CompositeObject::deserialize_inner_named(
                    xml_reader,
                    Some((e, e_empty)),
                    b"ofd:CompositeObject",
                    b"CompositeObject",
                  )?,
                ),
              ));
            }
            b"ofd:PageBlock" | b"PageBlock" => {
              xml_children.push(crate::schemas::page::LayerContentChoice::PageBlock(
                Box::new(crate::schemas::page::PageBlock::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:PageBlock",
                  b"PageBlock",
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
    let id = id.ok_or_else(|| crate::common::missing_field("Layer", "id"))?;
    Ok(Self {
      r#type,
      draw_param,
      id,
      xml_children,
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
      "Layer",
      "Layer",
      tag_name_prefix,
      tag_name
    );
    let mut r#type = None;
    let mut draw_param = None;
    let mut id = None;
    let mut xml_children = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Type" => {
          r#type = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtLayerType>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtLayerType>()?
          });
        }
        b"DrawParam" => {
          draw_param = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "Layer",
            "draw_param",
          )?);
        }
        b"ID" => {
          id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "Layer",
            "id",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Layer"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:TextObject" | b"TextObject" => {
              xml_children.push(crate::schemas::page::LayerContentChoice::TextObject(
                Box::new(
                  crate::schemas::page::TextObject::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:TextObject",
                    b"TextObject",
                  )?,
                ),
              ));
            }
            b"ofd:PathObject" | b"PathObject" => {
              xml_children.push(crate::schemas::page::LayerContentChoice::PathObject(
                Box::new(
                  crate::schemas::page::PathObject::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:PathObject",
                    b"PathObject",
                  )?,
                ),
              ));
            }
            b"ofd:ImageObject" | b"ImageObject" => {
              xml_children.push(crate::schemas::page::LayerContentChoice::ImageObject(
                Box::new(
                  crate::schemas::page::ImageObject::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:ImageObject",
                    b"ImageObject",
                  )?,
                ),
              ));
            }
            b"ofd:CompositeObject" | b"CompositeObject" => {
              xml_children.push(crate::schemas::page::LayerContentChoice::CompositeObject(
                Box::new(
                  crate::schemas::page::CompositeObject::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:CompositeObject",
                    b"CompositeObject",
                  )?,
                ),
              ));
            }
            b"ofd:PageBlock" | b"PageBlock" => {
              xml_children.push(crate::schemas::page::LayerContentChoice::PageBlock(
                Box::new(
                  crate::schemas::page::PageBlock::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:PageBlock",
                    b"PageBlock",
                  )?,
                ),
              ));
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
    let id = id.ok_or_else(|| crate::common::missing_field("Layer", "id"))?;
    Ok(Self {
      r#type,
      draw_param,
      id,
      xml_children,
    })
  }
}
impl std::str::FromStr for crate::schemas::page::Content {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Content", b"Content")
  }
}
impl crate::schemas::page::Content {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:Content", b"Content")
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
      "Content",
      "Content",
      tag_name_prefix,
      tag_name
    );
    let mut layer = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Content"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Layer" | b"Layer" => {
              layer.push(crate::schemas::page::Layer::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Layer",
                b"Layer",
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
    Ok(Self { layer })
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
      "Content",
      "Content",
      tag_name_prefix,
      tag_name
    );
    let mut layer = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Content"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Layer" | b"Layer" => {
              layer.push(crate::schemas::page::Layer::deserialize_from_reader_named(
                xml_reader,
                buf,
                Some((e, e_empty)),
                b"ofd:Layer",
                b"Layer",
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
    Ok(Self { layer })
  }
}
impl std::str::FromStr for crate::schemas::page::Actions {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Actions", b"Actions")
  }
}
impl crate::schemas::page::Actions {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:Actions", b"Actions")
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
      "Actions",
      "Actions",
      tag_name_prefix,
      tag_name
    );
    let mut action = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Actions"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Action" | b"Action" => {
              action.push(
                crate::schemas::definitions::CtAction::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Action",
                  b"Action",
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
    Ok(Self { action })
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
      "Actions",
      "Actions",
      tag_name_prefix,
      tag_name
    );
    let mut action = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Actions"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Action" | b"Action" => {
              action.push(
                crate::schemas::definitions::CtAction::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Action",
                  b"Action",
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
    Ok(Self { action })
  }
}
impl std::str::FromStr for crate::schemas::page::Page {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Page", b"Page")
  }
}
impl crate::schemas::page::Page {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:Page", b"Page")
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
      "Page",
      "Page",
      tag_name_prefix,
      tag_name
    );
    let mut template = vec![];
    let mut page_res = vec![];
    let mut area = None;
    let mut content = None;
    let mut actions = None;
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Page"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Template" | b"Template" => {
              template.push(crate::schemas::page::Template::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Template",
                b"Template",
              )?);
            }
            b"ofd:PageRes" | b"PageRes" => {
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
                        crate::common::push_xml_general_ref(&mut value, text, "Page", "page_res")?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:PageRes" || name == b"PageRes" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Page"))?,
                      _ => {}
                    }
                  }
                }
              };
              page_res.push(parsed_value);
            }
            b"ofd:Area" | b"Area" => {
              area = Some(
                crate::schemas::definitions::CtPageArea::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Area",
                  b"Area",
                )?,
              );
            }
            b"ofd:Content" | b"Content" => {
              content = Some(crate::schemas::page::Content::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Content",
                b"Content",
              )?);
            }
            b"ofd:Actions" | b"Actions" => {
              actions = Some(crate::schemas::page::Actions::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Actions",
                b"Actions",
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
    Ok(Self {
      template,
      page_res,
      area,
      content,
      actions,
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
      "Page",
      "Page",
      tag_name_prefix,
      tag_name
    );
    let mut template = vec![];
    let mut page_res = vec![];
    let mut area = None;
    let mut content = None;
    let mut actions = None;
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Page"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Template" | b"Template" => {
              template.push(
                crate::schemas::page::Template::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Template",
                  b"Template",
                )?,
              );
            }
            b"ofd:PageRes" | b"PageRes" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "Page",
                      field: "page_res",
                      tag_name_prefix: b"ofd:PageRes",
                      tag_name: b"PageRes",
                    },
                  )?
                }
              };
              page_res.push(parsed_value);
            }
            b"ofd:Area" | b"Area" => {
              area = Some(
                crate::schemas::definitions::CtPageArea::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Area",
                  b"Area",
                )?,
              );
            }
            b"ofd:Content" | b"Content" => {
              content = Some(
                crate::schemas::page::Content::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Content",
                  b"Content",
                )?,
              );
            }
            b"ofd:Actions" | b"Actions" => {
              actions = Some(
                crate::schemas::page::Actions::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Actions",
                  b"Actions",
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
    Ok(Self {
      template,
      page_res,
      area,
      content,
      actions,
    })
  }
}
impl std::str::FromStr for crate::schemas::page::Area {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Area", b"Area")
  }
}
impl crate::schemas::page::Area {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:Area", b"Area")
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
      "Area",
      "Area",
      tag_name_prefix,
      tag_name
    );
    let mut draw_param = None;
    let mut ctm = None;
    let mut xml_children = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"DrawParam" => {
          draw_param = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "Area",
            "draw_param",
          )?);
        }
        b"CTM" => {
          ctm = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Area"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Path" | b"Path" => {
              xml_children.push(crate::schemas::page::AreaContentChoice::Path(Box::new(
                crate::schemas::page::CtPath::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Path",
                  b"Path",
                )?,
              )));
            }
            b"ofd:Text" | b"Text" => {
              xml_children.push(crate::schemas::page::AreaContentChoice::Text(Box::new(
                crate::schemas::page::CtText::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Text",
                  b"Text",
                )?,
              )));
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
      draw_param,
      ctm,
      xml_children,
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
      "Area",
      "Area",
      tag_name_prefix,
      tag_name
    );
    let mut draw_param = None;
    let mut ctm = None;
    let mut xml_children = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"DrawParam" => {
          draw_param = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "Area",
            "draw_param",
          )?);
        }
        b"CTM" => {
          ctm = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Area"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Path" | b"Path" => {
              xml_children.push(crate::schemas::page::AreaContentChoice::Path(Box::new(
                crate::schemas::page::CtPath::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Path",
                  b"Path",
                )?,
              )));
            }
            b"ofd:Text" | b"Text" => {
              xml_children.push(crate::schemas::page::AreaContentChoice::Text(Box::new(
                crate::schemas::page::CtText::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Text",
                  b"Text",
                )?,
              )));
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
    Ok(Self {
      draw_param,
      ctm,
      xml_children,
    })
  }
}
impl std::str::FromStr for crate::schemas::page::CtClip {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_Clip", b"CT_Clip")
  }
}
impl crate::schemas::page::CtClip {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:CT_Clip", b"CT_Clip")
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
      "CtClip",
      "CT_Clip",
      tag_name_prefix,
      tag_name
    );
    let mut area = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtClip"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Area" | b"Area" => {
              area.push(crate::schemas::page::Area::deserialize_inner_named(
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
      "CtClip",
      "CT_Clip",
      tag_name_prefix,
      tag_name
    );
    let mut area = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtClip"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Area" | b"Area" => {
              area.push(crate::schemas::page::Area::deserialize_from_reader_named(
                xml_reader,
                buf,
                Some((e, e_empty)),
                b"ofd:Area",
                b"Area",
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
    Ok(Self { area })
  }
}
impl std::str::FromStr for crate::schemas::page::TextObject {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:TextObject", b"TextObject")
  }
}
impl crate::schemas::page::TextObject {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:TextObject",
      b"TextObject",
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
      "TextObject",
      "TextObject",
      tag_name_prefix,
      tag_name
    );
    let mut boundary = None;
    let mut name = None;
    let mut visible = None;
    let mut ctm = None;
    let mut draw_param = None;
    let mut line_width = None;
    let mut cap = None;
    let mut join = None;
    let mut miter_limit = None;
    let mut dash_offset = None;
    let mut dash_pattern = None;
    let mut alpha = None;
    let mut font = None;
    let mut size = None;
    let mut stroke = None;
    let mut fill = None;
    let mut h_scale = None;
    let mut read_direction = None;
    let mut char_direction = None;
    let mut weight = None;
    let mut italic = None;
    let mut id = None;
    let mut actions = None;
    let mut clips = None;
    let mut fill_color = None;
    let mut stroke_color = None;
    let mut cg_transform = vec![];
    let mut text_code = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Boundary" => {
          boundary =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Visible" => {
          visible = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "visible",
          )?);
        }
        b"CTM" => {
          ctm = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"DrawParam" => {
          draw_param = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "draw_param",
          )?);
        }
        b"LineWidth" => {
          line_width = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "line_width",
          )?);
        }
        b"Cap" => {
          cap = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitCap>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitCap>()?
          });
        }
        b"Join" => {
          join = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitJoin>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitJoin>()?
          });
        }
        b"MiterLimit" => {
          miter_limit = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "miter_limit",
          )?);
        }
        b"DashOffset" => {
          dash_offset = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "dash_offset",
          )?);
        }
        b"DashPattern" => {
          dash_pattern =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Alpha" => {
          alpha = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "alpha",
          )?);
        }
        b"Font" => {
          font = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "font",
          )?);
        }
        b"Size" => {
          size = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "size",
          )?);
        }
        b"Stroke" => {
          stroke = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "stroke",
          )?);
        }
        b"Fill" => {
          fill = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "fill",
          )?);
        }
        b"HScale" => {
          h_scale = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "h_scale",
          )?);
        }
        b"ReadDirection" => {
          read_direction = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "read_direction",
          )?);
        }
        b"CharDirection" => {
          char_direction = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "char_direction",
          )?);
        }
        b"Weight" => {
          weight = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtTextWeight>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtTextWeight>()?
          });
        }
        b"Italic" => {
          italic = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "italic",
          )?);
        }
        b"ID" => {
          id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "id",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("TextObject"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Actions" | b"Actions" => {
              actions = Some(crate::schemas::page::Actions::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Actions",
                b"Actions",
              )?);
            }
            b"ofd:Clips" | b"Clips" => {
              clips = Some(crate::schemas::page::Clips::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Clips",
                b"Clips",
              )?);
            }
            b"ofd:FillColor" | b"FillColor" => {
              fill_color = Some(crate::schemas::page::CtColor::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:FillColor",
                b"FillColor",
              )?);
            }
            b"ofd:StrokeColor" | b"StrokeColor" => {
              stroke_color = Some(crate::schemas::page::CtColor::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:StrokeColor",
                b"StrokeColor",
              )?);
            }
            b"ofd:CGTransform" | b"CGTransform" => {
              cg_transform.push(
                crate::schemas::page::CtCgTransform::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:CGTransform",
                  b"CGTransform",
                )?,
              );
            }
            b"ofd:TextCode" | b"TextCode" => {
              text_code.push(crate::schemas::page::TextCode::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:TextCode",
                b"TextCode",
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
    let boundary =
      boundary.ok_or_else(|| crate::common::missing_field("TextObject", "boundary"))?;
    let font = font.ok_or_else(|| crate::common::missing_field("TextObject", "font"))?;
    let size = size.ok_or_else(|| crate::common::missing_field("TextObject", "size"))?;
    let id = id.ok_or_else(|| crate::common::missing_field("TextObject", "id"))?;
    Ok(Self {
      boundary,
      name,
      visible,
      ctm,
      draw_param,
      line_width,
      cap,
      join,
      miter_limit,
      dash_offset,
      dash_pattern,
      alpha,
      font,
      size,
      stroke,
      fill,
      h_scale,
      read_direction,
      char_direction,
      weight,
      italic,
      id,
      actions,
      clips,
      fill_color,
      stroke_color,
      cg_transform,
      text_code,
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
      "TextObject",
      "TextObject",
      tag_name_prefix,
      tag_name
    );
    let mut boundary = None;
    let mut name = None;
    let mut visible = None;
    let mut ctm = None;
    let mut draw_param = None;
    let mut line_width = None;
    let mut cap = None;
    let mut join = None;
    let mut miter_limit = None;
    let mut dash_offset = None;
    let mut dash_pattern = None;
    let mut alpha = None;
    let mut font = None;
    let mut size = None;
    let mut stroke = None;
    let mut fill = None;
    let mut h_scale = None;
    let mut read_direction = None;
    let mut char_direction = None;
    let mut weight = None;
    let mut italic = None;
    let mut id = None;
    let mut actions = None;
    let mut clips = None;
    let mut fill_color = None;
    let mut stroke_color = None;
    let mut cg_transform = vec![];
    let mut text_code = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Boundary" => {
          boundary =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Visible" => {
          visible = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "visible",
          )?);
        }
        b"CTM" => {
          ctm = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"DrawParam" => {
          draw_param = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "draw_param",
          )?);
        }
        b"LineWidth" => {
          line_width = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "line_width",
          )?);
        }
        b"Cap" => {
          cap = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitCap>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitCap>()?
          });
        }
        b"Join" => {
          join = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitJoin>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitJoin>()?
          });
        }
        b"MiterLimit" => {
          miter_limit = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "miter_limit",
          )?);
        }
        b"DashOffset" => {
          dash_offset = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "dash_offset",
          )?);
        }
        b"DashPattern" => {
          dash_pattern =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Alpha" => {
          alpha = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "alpha",
          )?);
        }
        b"Font" => {
          font = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "font",
          )?);
        }
        b"Size" => {
          size = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "size",
          )?);
        }
        b"Stroke" => {
          stroke = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "stroke",
          )?);
        }
        b"Fill" => {
          fill = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "fill",
          )?);
        }
        b"HScale" => {
          h_scale = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "h_scale",
          )?);
        }
        b"ReadDirection" => {
          read_direction = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "read_direction",
          )?);
        }
        b"CharDirection" => {
          char_direction = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "char_direction",
          )?);
        }
        b"Weight" => {
          weight = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtTextWeight>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtTextWeight>()?
          });
        }
        b"Italic" => {
          italic = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "italic",
          )?);
        }
        b"ID" => {
          id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "TextObject",
            "id",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("TextObject"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Actions" | b"Actions" => {
              actions = Some(
                crate::schemas::page::Actions::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Actions",
                  b"Actions",
                )?,
              );
            }
            b"ofd:Clips" | b"Clips" => {
              clips = Some(crate::schemas::page::Clips::deserialize_from_reader_named(
                xml_reader,
                buf,
                Some((e, e_empty)),
                b"ofd:Clips",
                b"Clips",
              )?);
            }
            b"ofd:FillColor" | b"FillColor" => {
              fill_color = Some(
                crate::schemas::page::CtColor::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:FillColor",
                  b"FillColor",
                )?,
              );
            }
            b"ofd:StrokeColor" | b"StrokeColor" => {
              stroke_color = Some(
                crate::schemas::page::CtColor::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:StrokeColor",
                  b"StrokeColor",
                )?,
              );
            }
            b"ofd:CGTransform" | b"CGTransform" => {
              cg_transform.push(
                crate::schemas::page::CtCgTransform::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:CGTransform",
                  b"CGTransform",
                )?,
              );
            }
            b"ofd:TextCode" | b"TextCode" => {
              text_code.push(
                crate::schemas::page::TextCode::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:TextCode",
                  b"TextCode",
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
    let boundary =
      boundary.ok_or_else(|| crate::common::missing_field("TextObject", "boundary"))?;
    let font = font.ok_or_else(|| crate::common::missing_field("TextObject", "font"))?;
    let size = size.ok_or_else(|| crate::common::missing_field("TextObject", "size"))?;
    let id = id.ok_or_else(|| crate::common::missing_field("TextObject", "id"))?;
    Ok(Self {
      boundary,
      name,
      visible,
      ctm,
      draw_param,
      line_width,
      cap,
      join,
      miter_limit,
      dash_offset,
      dash_pattern,
      alpha,
      font,
      size,
      stroke,
      fill,
      h_scale,
      read_direction,
      char_direction,
      weight,
      italic,
      id,
      actions,
      clips,
      fill_color,
      stroke_color,
      cg_transform,
      text_code,
    })
  }
}
impl std::str::FromStr for crate::schemas::page::PathObject {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:PathObject", b"PathObject")
  }
}
impl crate::schemas::page::PathObject {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:PathObject",
      b"PathObject",
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
      "PathObject",
      "PathObject",
      tag_name_prefix,
      tag_name
    );
    let mut boundary = None;
    let mut name = None;
    let mut visible = None;
    let mut ctm = None;
    let mut draw_param = None;
    let mut line_width = None;
    let mut cap = None;
    let mut join = None;
    let mut miter_limit = None;
    let mut dash_offset = None;
    let mut dash_pattern = None;
    let mut alpha = None;
    let mut stroke = None;
    let mut fill = None;
    let mut rule = None;
    let mut id = None;
    let mut actions = None;
    let mut clips = None;
    let mut stroke_color = None;
    let mut fill_color = None;
    let mut abbreviated_data = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Boundary" => {
          boundary =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Visible" => {
          visible = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "PathObject",
            "visible",
          )?);
        }
        b"CTM" => {
          ctm = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"DrawParam" => {
          draw_param = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "PathObject",
            "draw_param",
          )?);
        }
        b"LineWidth" => {
          line_width = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "PathObject",
            "line_width",
          )?);
        }
        b"Cap" => {
          cap = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitCap>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitCap>()?
          });
        }
        b"Join" => {
          join = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitJoin>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitJoin>()?
          });
        }
        b"MiterLimit" => {
          miter_limit = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "PathObject",
            "miter_limit",
          )?);
        }
        b"DashOffset" => {
          dash_offset = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "PathObject",
            "dash_offset",
          )?);
        }
        b"DashPattern" => {
          dash_pattern =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Alpha" => {
          alpha = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "PathObject",
            "alpha",
          )?);
        }
        b"Stroke" => {
          stroke = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "PathObject",
            "stroke",
          )?);
        }
        b"Fill" => {
          fill = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "PathObject",
            "fill",
          )?);
        }
        b"Rule" => {
          rule = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtPathRule>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtPathRule>()?
          });
        }
        b"ID" => {
          id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "PathObject",
            "id",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("PathObject"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Actions" | b"Actions" => {
              actions = Some(crate::schemas::page::Actions::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Actions",
                b"Actions",
              )?);
            }
            b"ofd:Clips" | b"Clips" => {
              clips = Some(crate::schemas::page::Clips::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Clips",
                b"Clips",
              )?);
            }
            b"ofd:StrokeColor" | b"StrokeColor" => {
              stroke_color = Some(crate::schemas::page::CtColor::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:StrokeColor",
                b"StrokeColor",
              )?);
            }
            b"ofd:FillColor" | b"FillColor" => {
              fill_color = Some(crate::schemas::page::CtColor::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:FillColor",
                b"FillColor",
              )?);
            }
            b"ofd:AbbreviatedData" | b"AbbreviatedData" => {
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
                          "PathObject",
                          "abbreviated_data",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:AbbreviatedData" || name == b"AbbreviatedData" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("PathObject"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              abbreviated_data = Some(parsed_value);
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
    let boundary =
      boundary.ok_or_else(|| crate::common::missing_field("PathObject", "boundary"))?;
    let id = id.ok_or_else(|| crate::common::missing_field("PathObject", "id"))?;
    let abbreviated_data = abbreviated_data
      .ok_or_else(|| crate::common::missing_field("PathObject", "abbreviated_data"))?;
    Ok(Self {
      boundary,
      name,
      visible,
      ctm,
      draw_param,
      line_width,
      cap,
      join,
      miter_limit,
      dash_offset,
      dash_pattern,
      alpha,
      stroke,
      fill,
      rule,
      id,
      actions,
      clips,
      stroke_color,
      fill_color,
      abbreviated_data,
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
      "PathObject",
      "PathObject",
      tag_name_prefix,
      tag_name
    );
    let mut boundary = None;
    let mut name = None;
    let mut visible = None;
    let mut ctm = None;
    let mut draw_param = None;
    let mut line_width = None;
    let mut cap = None;
    let mut join = None;
    let mut miter_limit = None;
    let mut dash_offset = None;
    let mut dash_pattern = None;
    let mut alpha = None;
    let mut stroke = None;
    let mut fill = None;
    let mut rule = None;
    let mut id = None;
    let mut actions = None;
    let mut clips = None;
    let mut stroke_color = None;
    let mut fill_color = None;
    let mut abbreviated_data = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Boundary" => {
          boundary =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Visible" => {
          visible = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "PathObject",
            "visible",
          )?);
        }
        b"CTM" => {
          ctm = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"DrawParam" => {
          draw_param = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "PathObject",
            "draw_param",
          )?);
        }
        b"LineWidth" => {
          line_width = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "PathObject",
            "line_width",
          )?);
        }
        b"Cap" => {
          cap = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitCap>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitCap>()?
          });
        }
        b"Join" => {
          join = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitJoin>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitJoin>()?
          });
        }
        b"MiterLimit" => {
          miter_limit = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "PathObject",
            "miter_limit",
          )?);
        }
        b"DashOffset" => {
          dash_offset = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "PathObject",
            "dash_offset",
          )?);
        }
        b"DashPattern" => {
          dash_pattern =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Alpha" => {
          alpha = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "PathObject",
            "alpha",
          )?);
        }
        b"Stroke" => {
          stroke = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "PathObject",
            "stroke",
          )?);
        }
        b"Fill" => {
          fill = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "PathObject",
            "fill",
          )?);
        }
        b"Rule" => {
          rule = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtPathRule>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtPathRule>()?
          });
        }
        b"ID" => {
          id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "PathObject",
            "id",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("PathObject"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Actions" | b"Actions" => {
              actions = Some(
                crate::schemas::page::Actions::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Actions",
                  b"Actions",
                )?,
              );
            }
            b"ofd:Clips" | b"Clips" => {
              clips = Some(crate::schemas::page::Clips::deserialize_from_reader_named(
                xml_reader,
                buf,
                Some((e, e_empty)),
                b"ofd:Clips",
                b"Clips",
              )?);
            }
            b"ofd:StrokeColor" | b"StrokeColor" => {
              stroke_color = Some(
                crate::schemas::page::CtColor::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:StrokeColor",
                  b"StrokeColor",
                )?,
              );
            }
            b"ofd:FillColor" | b"FillColor" => {
              fill_color = Some(
                crate::schemas::page::CtColor::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:FillColor",
                  b"FillColor",
                )?,
              );
            }
            b"ofd:AbbreviatedData" | b"AbbreviatedData" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "PathObject",
                      field: "abbreviated_data",
                      tag_name_prefix: b"ofd:AbbreviatedData",
                      tag_name: b"AbbreviatedData",
                    },
                  )?
                }
              };
              abbreviated_data = Some(parsed_value);
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
    let boundary =
      boundary.ok_or_else(|| crate::common::missing_field("PathObject", "boundary"))?;
    let id = id.ok_or_else(|| crate::common::missing_field("PathObject", "id"))?;
    let abbreviated_data = abbreviated_data
      .ok_or_else(|| crate::common::missing_field("PathObject", "abbreviated_data"))?;
    Ok(Self {
      boundary,
      name,
      visible,
      ctm,
      draw_param,
      line_width,
      cap,
      join,
      miter_limit,
      dash_offset,
      dash_pattern,
      alpha,
      stroke,
      fill,
      rule,
      id,
      actions,
      clips,
      stroke_color,
      fill_color,
      abbreviated_data,
    })
  }
}
impl std::str::FromStr for crate::schemas::page::ImageObject {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:ImageObject", b"ImageObject")
  }
}
impl crate::schemas::page::ImageObject {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:ImageObject",
      b"ImageObject",
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
      "ImageObject",
      "ImageObject",
      tag_name_prefix,
      tag_name
    );
    let mut boundary = None;
    let mut name = None;
    let mut visible = None;
    let mut ctm = None;
    let mut draw_param = None;
    let mut line_width = None;
    let mut cap = None;
    let mut join = None;
    let mut miter_limit = None;
    let mut dash_offset = None;
    let mut dash_pattern = None;
    let mut alpha = None;
    let mut resource_id = None;
    let mut substitution = None;
    let mut image_mask = None;
    let mut id = None;
    let mut actions = None;
    let mut clips = None;
    let mut border = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Boundary" => {
          boundary =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Visible" => {
          visible = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "ImageObject",
            "visible",
          )?);
        }
        b"CTM" => {
          ctm = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"DrawParam" => {
          draw_param = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "ImageObject",
            "draw_param",
          )?);
        }
        b"LineWidth" => {
          line_width = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "ImageObject",
            "line_width",
          )?);
        }
        b"Cap" => {
          cap = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitCap>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitCap>()?
          });
        }
        b"Join" => {
          join = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitJoin>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitJoin>()?
          });
        }
        b"MiterLimit" => {
          miter_limit = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "ImageObject",
            "miter_limit",
          )?);
        }
        b"DashOffset" => {
          dash_offset = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "ImageObject",
            "dash_offset",
          )?);
        }
        b"DashPattern" => {
          dash_pattern =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Alpha" => {
          alpha = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "ImageObject",
            "alpha",
          )?);
        }
        b"ResourceID" => {
          resource_id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "ImageObject",
            "resource_id",
          )?);
        }
        b"Substitution" => {
          substitution = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "ImageObject",
            "substitution",
          )?);
        }
        b"ImageMask" => {
          image_mask = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "ImageObject",
            "image_mask",
          )?);
        }
        b"ID" => {
          id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "ImageObject",
            "id",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("ImageObject"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Actions" | b"Actions" => {
              actions = Some(crate::schemas::page::Actions::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Actions",
                b"Actions",
              )?);
            }
            b"ofd:Clips" | b"Clips" => {
              clips = Some(crate::schemas::page::Clips::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Clips",
                b"Clips",
              )?);
            }
            b"ofd:Border" | b"Border" => {
              border = Some(crate::schemas::page::Border::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Border",
                b"Border",
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
    let boundary =
      boundary.ok_or_else(|| crate::common::missing_field("ImageObject", "boundary"))?;
    let resource_id =
      resource_id.ok_or_else(|| crate::common::missing_field("ImageObject", "resource_id"))?;
    let id = id.ok_or_else(|| crate::common::missing_field("ImageObject", "id"))?;
    Ok(Self {
      boundary,
      name,
      visible,
      ctm,
      draw_param,
      line_width,
      cap,
      join,
      miter_limit,
      dash_offset,
      dash_pattern,
      alpha,
      resource_id,
      substitution,
      image_mask,
      id,
      actions,
      clips,
      border,
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
      "ImageObject",
      "ImageObject",
      tag_name_prefix,
      tag_name
    );
    let mut boundary = None;
    let mut name = None;
    let mut visible = None;
    let mut ctm = None;
    let mut draw_param = None;
    let mut line_width = None;
    let mut cap = None;
    let mut join = None;
    let mut miter_limit = None;
    let mut dash_offset = None;
    let mut dash_pattern = None;
    let mut alpha = None;
    let mut resource_id = None;
    let mut substitution = None;
    let mut image_mask = None;
    let mut id = None;
    let mut actions = None;
    let mut clips = None;
    let mut border = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Boundary" => {
          boundary =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Visible" => {
          visible = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "ImageObject",
            "visible",
          )?);
        }
        b"CTM" => {
          ctm = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"DrawParam" => {
          draw_param = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "ImageObject",
            "draw_param",
          )?);
        }
        b"LineWidth" => {
          line_width = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "ImageObject",
            "line_width",
          )?);
        }
        b"Cap" => {
          cap = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitCap>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitCap>()?
          });
        }
        b"Join" => {
          join = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitJoin>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitJoin>()?
          });
        }
        b"MiterLimit" => {
          miter_limit = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "ImageObject",
            "miter_limit",
          )?);
        }
        b"DashOffset" => {
          dash_offset = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "ImageObject",
            "dash_offset",
          )?);
        }
        b"DashPattern" => {
          dash_pattern =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Alpha" => {
          alpha = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "ImageObject",
            "alpha",
          )?);
        }
        b"ResourceID" => {
          resource_id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "ImageObject",
            "resource_id",
          )?);
        }
        b"Substitution" => {
          substitution = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "ImageObject",
            "substitution",
          )?);
        }
        b"ImageMask" => {
          image_mask = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "ImageObject",
            "image_mask",
          )?);
        }
        b"ID" => {
          id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "ImageObject",
            "id",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("ImageObject"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Actions" | b"Actions" => {
              actions = Some(
                crate::schemas::page::Actions::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Actions",
                  b"Actions",
                )?,
              );
            }
            b"ofd:Clips" | b"Clips" => {
              clips = Some(crate::schemas::page::Clips::deserialize_from_reader_named(
                xml_reader,
                buf,
                Some((e, e_empty)),
                b"ofd:Clips",
                b"Clips",
              )?);
            }
            b"ofd:Border" | b"Border" => {
              border = Some(crate::schemas::page::Border::deserialize_from_reader_named(
                xml_reader,
                buf,
                Some((e, e_empty)),
                b"ofd:Border",
                b"Border",
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
    let boundary =
      boundary.ok_or_else(|| crate::common::missing_field("ImageObject", "boundary"))?;
    let resource_id =
      resource_id.ok_or_else(|| crate::common::missing_field("ImageObject", "resource_id"))?;
    let id = id.ok_or_else(|| crate::common::missing_field("ImageObject", "id"))?;
    Ok(Self {
      boundary,
      name,
      visible,
      ctm,
      draw_param,
      line_width,
      cap,
      join,
      miter_limit,
      dash_offset,
      dash_pattern,
      alpha,
      resource_id,
      substitution,
      image_mask,
      id,
      actions,
      clips,
      border,
    })
  }
}
impl std::str::FromStr for crate::schemas::page::CompositeObject {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(
      &mut xml_reader,
      None,
      b"ofd:CompositeObject",
      b"CompositeObject",
    )
  }
}
impl crate::schemas::page::CompositeObject {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:CompositeObject",
      b"CompositeObject",
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
      "CompositeObject",
      "CompositeObject",
      tag_name_prefix,
      tag_name
    );
    let mut boundary = None;
    let mut name = None;
    let mut visible = None;
    let mut ctm = None;
    let mut draw_param = None;
    let mut line_width = None;
    let mut cap = None;
    let mut join = None;
    let mut miter_limit = None;
    let mut dash_offset = None;
    let mut dash_pattern = None;
    let mut alpha = None;
    let mut resource_id = None;
    let mut id = None;
    let mut actions = None;
    let mut clips = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Boundary" => {
          boundary =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Visible" => {
          visible = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CompositeObject",
            "visible",
          )?);
        }
        b"CTM" => {
          ctm = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"DrawParam" => {
          draw_param = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CompositeObject",
            "draw_param",
          )?);
        }
        b"LineWidth" => {
          line_width = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CompositeObject",
            "line_width",
          )?);
        }
        b"Cap" => {
          cap = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitCap>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitCap>()?
          });
        }
        b"Join" => {
          join = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitJoin>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitJoin>()?
          });
        }
        b"MiterLimit" => {
          miter_limit = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CompositeObject",
            "miter_limit",
          )?);
        }
        b"DashOffset" => {
          dash_offset = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CompositeObject",
            "dash_offset",
          )?);
        }
        b"DashPattern" => {
          dash_pattern =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Alpha" => {
          alpha = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CompositeObject",
            "alpha",
          )?);
        }
        b"ResourceID" => {
          resource_id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CompositeObject",
            "resource_id",
          )?);
        }
        b"ID" => {
          id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CompositeObject",
            "id",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CompositeObject"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Actions" | b"Actions" => {
              actions = Some(crate::schemas::page::Actions::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Actions",
                b"Actions",
              )?);
            }
            b"ofd:Clips" | b"Clips" => {
              clips = Some(crate::schemas::page::Clips::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Clips",
                b"Clips",
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
    let boundary =
      boundary.ok_or_else(|| crate::common::missing_field("CompositeObject", "boundary"))?;
    let resource_id =
      resource_id.ok_or_else(|| crate::common::missing_field("CompositeObject", "resource_id"))?;
    let id = id.ok_or_else(|| crate::common::missing_field("CompositeObject", "id"))?;
    Ok(Self {
      boundary,
      name,
      visible,
      ctm,
      draw_param,
      line_width,
      cap,
      join,
      miter_limit,
      dash_offset,
      dash_pattern,
      alpha,
      resource_id,
      id,
      actions,
      clips,
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
      "CompositeObject",
      "CompositeObject",
      tag_name_prefix,
      tag_name
    );
    let mut boundary = None;
    let mut name = None;
    let mut visible = None;
    let mut ctm = None;
    let mut draw_param = None;
    let mut line_width = None;
    let mut cap = None;
    let mut join = None;
    let mut miter_limit = None;
    let mut dash_offset = None;
    let mut dash_pattern = None;
    let mut alpha = None;
    let mut resource_id = None;
    let mut id = None;
    let mut actions = None;
    let mut clips = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Boundary" => {
          boundary =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Visible" => {
          visible = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CompositeObject",
            "visible",
          )?);
        }
        b"CTM" => {
          ctm = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"DrawParam" => {
          draw_param = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CompositeObject",
            "draw_param",
          )?);
        }
        b"LineWidth" => {
          line_width = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CompositeObject",
            "line_width",
          )?);
        }
        b"Cap" => {
          cap = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitCap>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitCap>()?
          });
        }
        b"Join" => {
          join = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitJoin>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitJoin>()?
          });
        }
        b"MiterLimit" => {
          miter_limit = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CompositeObject",
            "miter_limit",
          )?);
        }
        b"DashOffset" => {
          dash_offset = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CompositeObject",
            "dash_offset",
          )?);
        }
        b"DashPattern" => {
          dash_pattern =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Alpha" => {
          alpha = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CompositeObject",
            "alpha",
          )?);
        }
        b"ResourceID" => {
          resource_id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CompositeObject",
            "resource_id",
          )?);
        }
        b"ID" => {
          id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CompositeObject",
            "id",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CompositeObject"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Actions" | b"Actions" => {
              actions = Some(
                crate::schemas::page::Actions::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Actions",
                  b"Actions",
                )?,
              );
            }
            b"ofd:Clips" | b"Clips" => {
              clips = Some(crate::schemas::page::Clips::deserialize_from_reader_named(
                xml_reader,
                buf,
                Some((e, e_empty)),
                b"ofd:Clips",
                b"Clips",
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
    let boundary =
      boundary.ok_or_else(|| crate::common::missing_field("CompositeObject", "boundary"))?;
    let resource_id =
      resource_id.ok_or_else(|| crate::common::missing_field("CompositeObject", "resource_id"))?;
    let id = id.ok_or_else(|| crate::common::missing_field("CompositeObject", "id"))?;
    Ok(Self {
      boundary,
      name,
      visible,
      ctm,
      draw_param,
      line_width,
      cap,
      join,
      miter_limit,
      dash_offset,
      dash_pattern,
      alpha,
      resource_id,
      id,
      actions,
      clips,
    })
  }
}
impl std::str::FromStr for crate::schemas::page::CtPageBlock {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_PageBlock", b"CT_PageBlock")
  }
}
impl crate::schemas::page::CtPageBlock {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:CT_PageBlock",
      b"CT_PageBlock",
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
      "CtPageBlock",
      "CT_PageBlock",
      tag_name_prefix,
      tag_name
    );
    let mut xml_children = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtPageBlock"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:TextObject" | b"TextObject" => {
              xml_children.push(crate::schemas::page::CtPageBlockContentChoice::TextObject(
                Box::new(crate::schemas::page::TextObject::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:TextObject",
                  b"TextObject",
                )?),
              ));
            }
            b"ofd:PathObject" | b"PathObject" => {
              xml_children.push(crate::schemas::page::CtPageBlockContentChoice::PathObject(
                Box::new(crate::schemas::page::PathObject::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:PathObject",
                  b"PathObject",
                )?),
              ));
            }
            b"ofd:ImageObject" | b"ImageObject" => {
              xml_children.push(crate::schemas::page::CtPageBlockContentChoice::ImageObject(
                Box::new(crate::schemas::page::ImageObject::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:ImageObject",
                  b"ImageObject",
                )?),
              ));
            }
            b"ofd:CompositeObject" | b"CompositeObject" => {
              xml_children.push(
                crate::schemas::page::CtPageBlockContentChoice::CompositeObject(Box::new(
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
              xml_children.push(crate::schemas::page::CtPageBlockContentChoice::PageBlock(
                Box::new(crate::schemas::page::PageBlock::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:PageBlock",
                  b"PageBlock",
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
    Ok(Self { xml_children })
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
      "CtPageBlock",
      "CT_PageBlock",
      tag_name_prefix,
      tag_name
    );
    let mut xml_children = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtPageBlock"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:TextObject" | b"TextObject" => {
              xml_children.push(crate::schemas::page::CtPageBlockContentChoice::TextObject(
                Box::new(
                  crate::schemas::page::TextObject::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:TextObject",
                    b"TextObject",
                  )?,
                ),
              ));
            }
            b"ofd:PathObject" | b"PathObject" => {
              xml_children.push(crate::schemas::page::CtPageBlockContentChoice::PathObject(
                Box::new(
                  crate::schemas::page::PathObject::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:PathObject",
                    b"PathObject",
                  )?,
                ),
              ));
            }
            b"ofd:ImageObject" | b"ImageObject" => {
              xml_children.push(crate::schemas::page::CtPageBlockContentChoice::ImageObject(
                Box::new(
                  crate::schemas::page::ImageObject::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:ImageObject",
                    b"ImageObject",
                  )?,
                ),
              ));
            }
            b"ofd:CompositeObject" | b"CompositeObject" => {
              xml_children.push(
                crate::schemas::page::CtPageBlockContentChoice::CompositeObject(Box::new(
                  crate::schemas::page::CompositeObject::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:CompositeObject",
                    b"CompositeObject",
                  )?,
                )),
              );
            }
            b"ofd:PageBlock" | b"PageBlock" => {
              xml_children.push(crate::schemas::page::CtPageBlockContentChoice::PageBlock(
                Box::new(
                  crate::schemas::page::PageBlock::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:PageBlock",
                    b"PageBlock",
                  )?,
                ),
              ));
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
    Ok(Self { xml_children })
  }
}
impl std::str::FromStr for crate::schemas::page::PageBlock {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:PageBlock", b"PageBlock")
  }
}
impl crate::schemas::page::PageBlock {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:PageBlock",
      b"PageBlock",
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
      "PageBlock",
      "PageBlock",
      tag_name_prefix,
      tag_name
    );
    let mut id = None;
    let mut xml_children = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"ID" => {
          id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "PageBlock",
            "id",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("PageBlock"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:TextObject" | b"TextObject" => {
              xml_children.push(crate::schemas::page::PageBlockContentChoice::TextObject(
                Box::new(crate::schemas::page::TextObject::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:TextObject",
                  b"TextObject",
                )?),
              ));
            }
            b"ofd:PathObject" | b"PathObject" => {
              xml_children.push(crate::schemas::page::PageBlockContentChoice::PathObject(
                Box::new(crate::schemas::page::PathObject::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:PathObject",
                  b"PathObject",
                )?),
              ));
            }
            b"ofd:ImageObject" | b"ImageObject" => {
              xml_children.push(crate::schemas::page::PageBlockContentChoice::ImageObject(
                Box::new(crate::schemas::page::ImageObject::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:ImageObject",
                  b"ImageObject",
                )?),
              ));
            }
            b"ofd:CompositeObject" | b"CompositeObject" => {
              xml_children.push(
                crate::schemas::page::PageBlockContentChoice::CompositeObject(Box::new(
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
              xml_children.push(crate::schemas::page::PageBlockContentChoice::PageBlock(
                Box::new(crate::schemas::page::PageBlock::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:PageBlock",
                  b"PageBlock",
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
    let id = id.ok_or_else(|| crate::common::missing_field("PageBlock", "id"))?;
    Ok(Self { id, xml_children })
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
      "PageBlock",
      "PageBlock",
      tag_name_prefix,
      tag_name
    );
    let mut id = None;
    let mut xml_children = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"ID" => {
          id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "PageBlock",
            "id",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("PageBlock"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:TextObject" | b"TextObject" => {
              xml_children.push(crate::schemas::page::PageBlockContentChoice::TextObject(
                Box::new(
                  crate::schemas::page::TextObject::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:TextObject",
                    b"TextObject",
                  )?,
                ),
              ));
            }
            b"ofd:PathObject" | b"PathObject" => {
              xml_children.push(crate::schemas::page::PageBlockContentChoice::PathObject(
                Box::new(
                  crate::schemas::page::PathObject::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:PathObject",
                    b"PathObject",
                  )?,
                ),
              ));
            }
            b"ofd:ImageObject" | b"ImageObject" => {
              xml_children.push(crate::schemas::page::PageBlockContentChoice::ImageObject(
                Box::new(
                  crate::schemas::page::ImageObject::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:ImageObject",
                    b"ImageObject",
                  )?,
                ),
              ));
            }
            b"ofd:CompositeObject" | b"CompositeObject" => {
              xml_children.push(
                crate::schemas::page::PageBlockContentChoice::CompositeObject(Box::new(
                  crate::schemas::page::CompositeObject::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:CompositeObject",
                    b"CompositeObject",
                  )?,
                )),
              );
            }
            b"ofd:PageBlock" | b"PageBlock" => {
              xml_children.push(crate::schemas::page::PageBlockContentChoice::PageBlock(
                Box::new(
                  crate::schemas::page::PageBlock::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:PageBlock",
                    b"PageBlock",
                  )?,
                ),
              ));
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
    let id = id.ok_or_else(|| crate::common::missing_field("PageBlock", "id"))?;
    Ok(Self { id, xml_children })
  }
}
impl std::str::FromStr for crate::schemas::page::CtLayer {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_Layer", b"CT_Layer")
  }
}
impl crate::schemas::page::CtLayer {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:CT_Layer",
      b"CT_Layer",
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
      "CtLayer",
      "CT_Layer",
      tag_name_prefix,
      tag_name
    );
    let mut r#type = None;
    let mut draw_param = None;
    let mut xml_children = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Type" => {
          r#type = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtLayerType>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtLayerType>()?
          });
        }
        b"DrawParam" => {
          draw_param = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtLayer",
            "draw_param",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtLayer"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:TextObject" | b"TextObject" => {
              xml_children.push(crate::schemas::page::CtLayerContentChoice::TextObject(
                Box::new(crate::schemas::page::TextObject::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:TextObject",
                  b"TextObject",
                )?),
              ));
            }
            b"ofd:PathObject" | b"PathObject" => {
              xml_children.push(crate::schemas::page::CtLayerContentChoice::PathObject(
                Box::new(crate::schemas::page::PathObject::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:PathObject",
                  b"PathObject",
                )?),
              ));
            }
            b"ofd:ImageObject" | b"ImageObject" => {
              xml_children.push(crate::schemas::page::CtLayerContentChoice::ImageObject(
                Box::new(crate::schemas::page::ImageObject::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:ImageObject",
                  b"ImageObject",
                )?),
              ));
            }
            b"ofd:CompositeObject" | b"CompositeObject" => {
              xml_children.push(crate::schemas::page::CtLayerContentChoice::CompositeObject(
                Box::new(
                  crate::schemas::page::CompositeObject::deserialize_inner_named(
                    xml_reader,
                    Some((e, e_empty)),
                    b"ofd:CompositeObject",
                    b"CompositeObject",
                  )?,
                ),
              ));
            }
            b"ofd:PageBlock" | b"PageBlock" => {
              xml_children.push(crate::schemas::page::CtLayerContentChoice::PageBlock(
                Box::new(crate::schemas::page::PageBlock::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:PageBlock",
                  b"PageBlock",
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
    Ok(Self {
      r#type,
      draw_param,
      xml_children,
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
      "CtLayer",
      "CT_Layer",
      tag_name_prefix,
      tag_name
    );
    let mut r#type = None;
    let mut draw_param = None;
    let mut xml_children = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Type" => {
          r#type = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtLayerType>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtLayerType>()?
          });
        }
        b"DrawParam" => {
          draw_param = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtLayer",
            "draw_param",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtLayer"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:TextObject" | b"TextObject" => {
              xml_children.push(crate::schemas::page::CtLayerContentChoice::TextObject(
                Box::new(
                  crate::schemas::page::TextObject::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:TextObject",
                    b"TextObject",
                  )?,
                ),
              ));
            }
            b"ofd:PathObject" | b"PathObject" => {
              xml_children.push(crate::schemas::page::CtLayerContentChoice::PathObject(
                Box::new(
                  crate::schemas::page::PathObject::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:PathObject",
                    b"PathObject",
                  )?,
                ),
              ));
            }
            b"ofd:ImageObject" | b"ImageObject" => {
              xml_children.push(crate::schemas::page::CtLayerContentChoice::ImageObject(
                Box::new(
                  crate::schemas::page::ImageObject::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:ImageObject",
                    b"ImageObject",
                  )?,
                ),
              ));
            }
            b"ofd:CompositeObject" | b"CompositeObject" => {
              xml_children.push(crate::schemas::page::CtLayerContentChoice::CompositeObject(
                Box::new(
                  crate::schemas::page::CompositeObject::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:CompositeObject",
                    b"CompositeObject",
                  )?,
                ),
              ));
            }
            b"ofd:PageBlock" | b"PageBlock" => {
              xml_children.push(crate::schemas::page::CtLayerContentChoice::PageBlock(
                Box::new(
                  crate::schemas::page::PageBlock::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:PageBlock",
                    b"PageBlock",
                  )?,
                ),
              ));
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
    Ok(Self {
      r#type,
      draw_param,
      xml_children,
    })
  }
}
impl std::str::FromStr for crate::schemas::page::Clips {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Clips", b"Clips")
  }
}
impl crate::schemas::page::Clips {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:Clips", b"Clips")
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
      "Clips",
      "Clips",
      tag_name_prefix,
      tag_name
    );
    let mut clip = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Clips"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Clip" | b"Clip" => {
              clip.push(crate::schemas::page::CtClip::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Clip",
                b"Clip",
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
    Ok(Self { clip })
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
      "Clips",
      "Clips",
      tag_name_prefix,
      tag_name
    );
    let mut clip = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Clips"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Clip" | b"Clip" => {
              clip.push(crate::schemas::page::CtClip::deserialize_from_reader_named(
                xml_reader,
                buf,
                Some((e, e_empty)),
                b"ofd:Clip",
                b"Clip",
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
    Ok(Self { clip })
  }
}
impl std::str::FromStr for crate::schemas::page::CtGraphicUnit {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(
      &mut xml_reader,
      None,
      b"ofd:CT_GraphicUnit",
      b"CT_GraphicUnit",
    )
  }
}
impl crate::schemas::page::CtGraphicUnit {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:CT_GraphicUnit",
      b"CT_GraphicUnit",
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
      "CtGraphicUnit",
      "CT_GraphicUnit",
      tag_name_prefix,
      tag_name
    );
    let mut boundary = None;
    let mut name = None;
    let mut visible = None;
    let mut ctm = None;
    let mut draw_param = None;
    let mut line_width = None;
    let mut cap = None;
    let mut join = None;
    let mut miter_limit = None;
    let mut dash_offset = None;
    let mut dash_pattern = None;
    let mut alpha = None;
    let mut actions = None;
    let mut clips = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Boundary" => {
          boundary =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Visible" => {
          visible = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtGraphicUnit",
            "visible",
          )?);
        }
        b"CTM" => {
          ctm = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"DrawParam" => {
          draw_param = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtGraphicUnit",
            "draw_param",
          )?);
        }
        b"LineWidth" => {
          line_width = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtGraphicUnit",
            "line_width",
          )?);
        }
        b"Cap" => {
          cap = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitCap>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitCap>()?
          });
        }
        b"Join" => {
          join = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitJoin>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitJoin>()?
          });
        }
        b"MiterLimit" => {
          miter_limit = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtGraphicUnit",
            "miter_limit",
          )?);
        }
        b"DashOffset" => {
          dash_offset = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtGraphicUnit",
            "dash_offset",
          )?);
        }
        b"DashPattern" => {
          dash_pattern =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Alpha" => {
          alpha = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtGraphicUnit",
            "alpha",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtGraphicUnit"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Actions" | b"Actions" => {
              actions = Some(crate::schemas::page::Actions::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Actions",
                b"Actions",
              )?);
            }
            b"ofd:Clips" | b"Clips" => {
              clips = Some(crate::schemas::page::Clips::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Clips",
                b"Clips",
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
    let boundary =
      boundary.ok_or_else(|| crate::common::missing_field("CtGraphicUnit", "boundary"))?;
    Ok(Self {
      boundary,
      name,
      visible,
      ctm,
      draw_param,
      line_width,
      cap,
      join,
      miter_limit,
      dash_offset,
      dash_pattern,
      alpha,
      actions,
      clips,
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
      "CtGraphicUnit",
      "CT_GraphicUnit",
      tag_name_prefix,
      tag_name
    );
    let mut boundary = None;
    let mut name = None;
    let mut visible = None;
    let mut ctm = None;
    let mut draw_param = None;
    let mut line_width = None;
    let mut cap = None;
    let mut join = None;
    let mut miter_limit = None;
    let mut dash_offset = None;
    let mut dash_pattern = None;
    let mut alpha = None;
    let mut actions = None;
    let mut clips = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Boundary" => {
          boundary =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Visible" => {
          visible = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtGraphicUnit",
            "visible",
          )?);
        }
        b"CTM" => {
          ctm = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"DrawParam" => {
          draw_param = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtGraphicUnit",
            "draw_param",
          )?);
        }
        b"LineWidth" => {
          line_width = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtGraphicUnit",
            "line_width",
          )?);
        }
        b"Cap" => {
          cap = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitCap>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitCap>()?
          });
        }
        b"Join" => {
          join = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitJoin>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitJoin>()?
          });
        }
        b"MiterLimit" => {
          miter_limit = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtGraphicUnit",
            "miter_limit",
          )?);
        }
        b"DashOffset" => {
          dash_offset = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtGraphicUnit",
            "dash_offset",
          )?);
        }
        b"DashPattern" => {
          dash_pattern =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Alpha" => {
          alpha = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtGraphicUnit",
            "alpha",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtGraphicUnit"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Actions" | b"Actions" => {
              actions = Some(
                crate::schemas::page::Actions::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Actions",
                  b"Actions",
                )?,
              );
            }
            b"ofd:Clips" | b"Clips" => {
              clips = Some(crate::schemas::page::Clips::deserialize_from_reader_named(
                xml_reader,
                buf,
                Some((e, e_empty)),
                b"ofd:Clips",
                b"Clips",
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
    let boundary =
      boundary.ok_or_else(|| crate::common::missing_field("CtGraphicUnit", "boundary"))?;
    Ok(Self {
      boundary,
      name,
      visible,
      ctm,
      draw_param,
      line_width,
      cap,
      join,
      miter_limit,
      dash_offset,
      dash_pattern,
      alpha,
      actions,
      clips,
    })
  }
}
impl std::str::FromStr for crate::schemas::page::TextCode {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:TextCode", b"TextCode")
  }
}
impl crate::schemas::page::TextCode {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:TextCode",
      b"TextCode",
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
      "TextCode",
      "TextCode",
      tag_name_prefix,
      tag_name
    );
    let mut x = None;
    let mut y = None;
    let mut delta_x = None;
    let mut delta_y = None;
    let mut xml_value_raw = None;
    let mut xml_value_seen = false;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"X" => {
          x = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "TextCode",
            "x",
          )?);
        }
        b"Y" => {
          y = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "TextCode",
            "y",
          )?);
        }
        b"DeltaX" => {
          delta_x =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"DeltaY" => {
          delta_y =
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
                  "TextCode",
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("TextCode"))?,
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
      Err(crate::common::missing_field("TextCode", "xml_value"))?
    };
    Ok(Self {
      x,
      y,
      delta_x,
      delta_y,
      xml_value,
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
      "TextCode",
      "TextCode",
      tag_name_prefix,
      tag_name
    );
    let mut x = None;
    let mut y = None;
    let mut delta_x = None;
    let mut delta_y = None;
    let mut xml_value_raw = None;
    let mut xml_value_seen = false;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"X" => {
          x = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "TextCode",
            "x",
          )?);
        }
        b"Y" => {
          y = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "TextCode",
            "y",
          )?);
        }
        b"DeltaX" => {
          delta_x =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"DeltaY" => {
          delta_y =
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
                  "TextCode",
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("TextCode"))?,
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
      Err(crate::common::missing_field("TextCode", "xml_value"))?
    };
    Ok(Self {
      x,
      y,
      delta_x,
      delta_y,
      xml_value,
    })
  }
}
impl std::str::FromStr for crate::schemas::page::CtText {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_Text", b"CT_Text")
  }
}
impl crate::schemas::page::CtText {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:CT_Text", b"CT_Text")
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
      "CtText",
      "CT_Text",
      tag_name_prefix,
      tag_name
    );
    let mut boundary = None;
    let mut name = None;
    let mut visible = None;
    let mut ctm = None;
    let mut draw_param = None;
    let mut line_width = None;
    let mut cap = None;
    let mut join = None;
    let mut miter_limit = None;
    let mut dash_offset = None;
    let mut dash_pattern = None;
    let mut alpha = None;
    let mut font = None;
    let mut size = None;
    let mut stroke = None;
    let mut fill = None;
    let mut h_scale = None;
    let mut read_direction = None;
    let mut char_direction = None;
    let mut weight = None;
    let mut italic = None;
    let mut actions = None;
    let mut clips = None;
    let mut fill_color = None;
    let mut stroke_color = None;
    let mut cg_transform = vec![];
    let mut text_code = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Boundary" => {
          boundary =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Visible" => {
          visible = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "visible",
          )?);
        }
        b"CTM" => {
          ctm = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"DrawParam" => {
          draw_param = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "draw_param",
          )?);
        }
        b"LineWidth" => {
          line_width = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "line_width",
          )?);
        }
        b"Cap" => {
          cap = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitCap>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitCap>()?
          });
        }
        b"Join" => {
          join = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitJoin>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitJoin>()?
          });
        }
        b"MiterLimit" => {
          miter_limit = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "miter_limit",
          )?);
        }
        b"DashOffset" => {
          dash_offset = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "dash_offset",
          )?);
        }
        b"DashPattern" => {
          dash_pattern =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Alpha" => {
          alpha = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "alpha",
          )?);
        }
        b"Font" => {
          font = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "font",
          )?);
        }
        b"Size" => {
          size = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "size",
          )?);
        }
        b"Stroke" => {
          stroke = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "stroke",
          )?);
        }
        b"Fill" => {
          fill = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "fill",
          )?);
        }
        b"HScale" => {
          h_scale = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "h_scale",
          )?);
        }
        b"ReadDirection" => {
          read_direction = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "read_direction",
          )?);
        }
        b"CharDirection" => {
          char_direction = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "char_direction",
          )?);
        }
        b"Weight" => {
          weight = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtTextWeight>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtTextWeight>()?
          });
        }
        b"Italic" => {
          italic = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "italic",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtText"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Actions" | b"Actions" => {
              actions = Some(crate::schemas::page::Actions::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Actions",
                b"Actions",
              )?);
            }
            b"ofd:Clips" | b"Clips" => {
              clips = Some(crate::schemas::page::Clips::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Clips",
                b"Clips",
              )?);
            }
            b"ofd:FillColor" | b"FillColor" => {
              fill_color = Some(crate::schemas::page::CtColor::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:FillColor",
                b"FillColor",
              )?);
            }
            b"ofd:StrokeColor" | b"StrokeColor" => {
              stroke_color = Some(crate::schemas::page::CtColor::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:StrokeColor",
                b"StrokeColor",
              )?);
            }
            b"ofd:CGTransform" | b"CGTransform" => {
              cg_transform.push(
                crate::schemas::page::CtCgTransform::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:CGTransform",
                  b"CGTransform",
                )?,
              );
            }
            b"ofd:TextCode" | b"TextCode" => {
              text_code.push(crate::schemas::page::TextCode::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:TextCode",
                b"TextCode",
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
    let boundary = boundary.ok_or_else(|| crate::common::missing_field("CtText", "boundary"))?;
    let font = font.ok_or_else(|| crate::common::missing_field("CtText", "font"))?;
    let size = size.ok_or_else(|| crate::common::missing_field("CtText", "size"))?;
    Ok(Self {
      boundary,
      name,
      visible,
      ctm,
      draw_param,
      line_width,
      cap,
      join,
      miter_limit,
      dash_offset,
      dash_pattern,
      alpha,
      font,
      size,
      stroke,
      fill,
      h_scale,
      read_direction,
      char_direction,
      weight,
      italic,
      actions,
      clips,
      fill_color,
      stroke_color,
      cg_transform,
      text_code,
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
      "CtText",
      "CT_Text",
      tag_name_prefix,
      tag_name
    );
    let mut boundary = None;
    let mut name = None;
    let mut visible = None;
    let mut ctm = None;
    let mut draw_param = None;
    let mut line_width = None;
    let mut cap = None;
    let mut join = None;
    let mut miter_limit = None;
    let mut dash_offset = None;
    let mut dash_pattern = None;
    let mut alpha = None;
    let mut font = None;
    let mut size = None;
    let mut stroke = None;
    let mut fill = None;
    let mut h_scale = None;
    let mut read_direction = None;
    let mut char_direction = None;
    let mut weight = None;
    let mut italic = None;
    let mut actions = None;
    let mut clips = None;
    let mut fill_color = None;
    let mut stroke_color = None;
    let mut cg_transform = vec![];
    let mut text_code = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Boundary" => {
          boundary =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Visible" => {
          visible = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "visible",
          )?);
        }
        b"CTM" => {
          ctm = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"DrawParam" => {
          draw_param = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "draw_param",
          )?);
        }
        b"LineWidth" => {
          line_width = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "line_width",
          )?);
        }
        b"Cap" => {
          cap = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitCap>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitCap>()?
          });
        }
        b"Join" => {
          join = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitJoin>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitJoin>()?
          });
        }
        b"MiterLimit" => {
          miter_limit = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "miter_limit",
          )?);
        }
        b"DashOffset" => {
          dash_offset = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "dash_offset",
          )?);
        }
        b"DashPattern" => {
          dash_pattern =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Alpha" => {
          alpha = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "alpha",
          )?);
        }
        b"Font" => {
          font = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "font",
          )?);
        }
        b"Size" => {
          size = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "size",
          )?);
        }
        b"Stroke" => {
          stroke = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "stroke",
          )?);
        }
        b"Fill" => {
          fill = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "fill",
          )?);
        }
        b"HScale" => {
          h_scale = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "h_scale",
          )?);
        }
        b"ReadDirection" => {
          read_direction = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "read_direction",
          )?);
        }
        b"CharDirection" => {
          char_direction = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "char_direction",
          )?);
        }
        b"Weight" => {
          weight = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtTextWeight>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtTextWeight>()?
          });
        }
        b"Italic" => {
          italic = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtText",
            "italic",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtText"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Actions" | b"Actions" => {
              actions = Some(
                crate::schemas::page::Actions::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Actions",
                  b"Actions",
                )?,
              );
            }
            b"ofd:Clips" | b"Clips" => {
              clips = Some(crate::schemas::page::Clips::deserialize_from_reader_named(
                xml_reader,
                buf,
                Some((e, e_empty)),
                b"ofd:Clips",
                b"Clips",
              )?);
            }
            b"ofd:FillColor" | b"FillColor" => {
              fill_color = Some(
                crate::schemas::page::CtColor::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:FillColor",
                  b"FillColor",
                )?,
              );
            }
            b"ofd:StrokeColor" | b"StrokeColor" => {
              stroke_color = Some(
                crate::schemas::page::CtColor::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:StrokeColor",
                  b"StrokeColor",
                )?,
              );
            }
            b"ofd:CGTransform" | b"CGTransform" => {
              cg_transform.push(
                crate::schemas::page::CtCgTransform::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:CGTransform",
                  b"CGTransform",
                )?,
              );
            }
            b"ofd:TextCode" | b"TextCode" => {
              text_code.push(
                crate::schemas::page::TextCode::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:TextCode",
                  b"TextCode",
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
    let boundary = boundary.ok_or_else(|| crate::common::missing_field("CtText", "boundary"))?;
    let font = font.ok_or_else(|| crate::common::missing_field("CtText", "font"))?;
    let size = size.ok_or_else(|| crate::common::missing_field("CtText", "size"))?;
    Ok(Self {
      boundary,
      name,
      visible,
      ctm,
      draw_param,
      line_width,
      cap,
      join,
      miter_limit,
      dash_offset,
      dash_pattern,
      alpha,
      font,
      size,
      stroke,
      fill,
      h_scale,
      read_direction,
      char_direction,
      weight,
      italic,
      actions,
      clips,
      fill_color,
      stroke_color,
      cg_transform,
      text_code,
    })
  }
}
impl std::str::FromStr for crate::schemas::page::CtCgTransform {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(
      &mut xml_reader,
      None,
      b"ofd:CT_CGTransform",
      b"CT_CGTransform",
    )
  }
}
impl crate::schemas::page::CtCgTransform {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:CT_CGTransform",
      b"CT_CGTransform",
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
      "CtCgTransform",
      "CT_CGTransform",
      tag_name_prefix,
      tag_name
    );
    let mut code_position = None;
    let mut code_count = None;
    let mut glyph_count = None;
    let mut glyphs = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"CodePosition" => {
          code_position = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtCgTransform",
            "code_position",
          )?);
        }
        b"CodeCount" => {
          code_count = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtCgTransform",
            "code_count",
          )?);
        }
        b"GlyphCount" => {
          glyph_count = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtCgTransform",
            "glyph_count",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtCgTransform"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Glyphs" | b"Glyphs" => {
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
                          "CtCgTransform",
                          "glyphs",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:Glyphs" || name == b"Glyphs" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtCgTransform"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              glyphs = Some(parsed_value);
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
    let code_position = code_position
      .ok_or_else(|| crate::common::missing_field("CtCgTransform", "code_position"))?;
    Ok(Self {
      code_position,
      code_count,
      glyph_count,
      glyphs,
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
      "CtCgTransform",
      "CT_CGTransform",
      tag_name_prefix,
      tag_name
    );
    let mut code_position = None;
    let mut code_count = None;
    let mut glyph_count = None;
    let mut glyphs = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"CodePosition" => {
          code_position = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtCgTransform",
            "code_position",
          )?);
        }
        b"CodeCount" => {
          code_count = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtCgTransform",
            "code_count",
          )?);
        }
        b"GlyphCount" => {
          glyph_count = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtCgTransform",
            "glyph_count",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtCgTransform"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Glyphs" | b"Glyphs" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtCgTransform",
                      field: "glyphs",
                      tag_name_prefix: b"ofd:Glyphs",
                      tag_name: b"Glyphs",
                    },
                  )?
                }
              };
              glyphs = Some(parsed_value);
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
    let code_position = code_position
      .ok_or_else(|| crate::common::missing_field("CtCgTransform", "code_position"))?;
    Ok(Self {
      code_position,
      code_count,
      glyph_count,
      glyphs,
    })
  }
}
impl std::str::FromStr for crate::schemas::page::Border {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Border", b"Border")
  }
}
impl crate::schemas::page::Border {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:Border", b"Border")
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
      "Border",
      "Border",
      tag_name_prefix,
      tag_name
    );
    let mut line_width = None;
    let mut horizonal_corner_radius = None;
    let mut vertical_corner_radius = None;
    let mut dash_offset = None;
    let mut dash_pattern = None;
    let mut border_color = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"LineWidth" => {
          line_width = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "Border",
            "line_width",
          )?);
        }
        b"HorizonalCornerRadius" => {
          horizonal_corner_radius = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "Border",
            "horizonal_corner_radius",
          )?);
        }
        b"VerticalCornerRadius" => {
          vertical_corner_radius = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "Border",
            "vertical_corner_radius",
          )?);
        }
        b"DashOffset" => {
          dash_offset = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "Border",
            "dash_offset",
          )?);
        }
        b"DashPattern" => {
          dash_pattern =
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Border"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:BorderColor" | b"BorderColor" => {
              border_color = Some(crate::schemas::page::CtColor::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:BorderColor",
                b"BorderColor",
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
    Ok(Self {
      line_width,
      horizonal_corner_radius,
      vertical_corner_radius,
      dash_offset,
      dash_pattern,
      border_color,
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
      "Border",
      "Border",
      tag_name_prefix,
      tag_name
    );
    let mut line_width = None;
    let mut horizonal_corner_radius = None;
    let mut vertical_corner_radius = None;
    let mut dash_offset = None;
    let mut dash_pattern = None;
    let mut border_color = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"LineWidth" => {
          line_width = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "Border",
            "line_width",
          )?);
        }
        b"HorizonalCornerRadius" => {
          horizonal_corner_radius = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "Border",
            "horizonal_corner_radius",
          )?);
        }
        b"VerticalCornerRadius" => {
          vertical_corner_radius = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "Border",
            "vertical_corner_radius",
          )?);
        }
        b"DashOffset" => {
          dash_offset = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "Border",
            "dash_offset",
          )?);
        }
        b"DashPattern" => {
          dash_pattern =
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Border"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:BorderColor" | b"BorderColor" => {
              border_color = Some(
                crate::schemas::page::CtColor::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:BorderColor",
                  b"BorderColor",
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
    Ok(Self {
      line_width,
      horizonal_corner_radius,
      vertical_corner_radius,
      dash_offset,
      dash_pattern,
      border_color,
    })
  }
}
impl std::str::FromStr for crate::schemas::page::CtImage {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_Image", b"CT_Image")
  }
}
impl crate::schemas::page::CtImage {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:CT_Image",
      b"CT_Image",
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
      "CtImage",
      "CT_Image",
      tag_name_prefix,
      tag_name
    );
    let mut boundary = None;
    let mut name = None;
    let mut visible = None;
    let mut ctm = None;
    let mut draw_param = None;
    let mut line_width = None;
    let mut cap = None;
    let mut join = None;
    let mut miter_limit = None;
    let mut dash_offset = None;
    let mut dash_pattern = None;
    let mut alpha = None;
    let mut resource_id = None;
    let mut substitution = None;
    let mut image_mask = None;
    let mut actions = None;
    let mut clips = None;
    let mut border = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Boundary" => {
          boundary =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Visible" => {
          visible = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtImage",
            "visible",
          )?);
        }
        b"CTM" => {
          ctm = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"DrawParam" => {
          draw_param = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtImage",
            "draw_param",
          )?);
        }
        b"LineWidth" => {
          line_width = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtImage",
            "line_width",
          )?);
        }
        b"Cap" => {
          cap = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitCap>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitCap>()?
          });
        }
        b"Join" => {
          join = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitJoin>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitJoin>()?
          });
        }
        b"MiterLimit" => {
          miter_limit = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtImage",
            "miter_limit",
          )?);
        }
        b"DashOffset" => {
          dash_offset = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtImage",
            "dash_offset",
          )?);
        }
        b"DashPattern" => {
          dash_pattern =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Alpha" => {
          alpha = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtImage",
            "alpha",
          )?);
        }
        b"ResourceID" => {
          resource_id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtImage",
            "resource_id",
          )?);
        }
        b"Substitution" => {
          substitution = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtImage",
            "substitution",
          )?);
        }
        b"ImageMask" => {
          image_mask = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtImage",
            "image_mask",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtImage"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Actions" | b"Actions" => {
              actions = Some(crate::schemas::page::Actions::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Actions",
                b"Actions",
              )?);
            }
            b"ofd:Clips" | b"Clips" => {
              clips = Some(crate::schemas::page::Clips::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Clips",
                b"Clips",
              )?);
            }
            b"ofd:Border" | b"Border" => {
              border = Some(crate::schemas::page::Border::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Border",
                b"Border",
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
    let boundary = boundary.ok_or_else(|| crate::common::missing_field("CtImage", "boundary"))?;
    let resource_id =
      resource_id.ok_or_else(|| crate::common::missing_field("CtImage", "resource_id"))?;
    Ok(Self {
      boundary,
      name,
      visible,
      ctm,
      draw_param,
      line_width,
      cap,
      join,
      miter_limit,
      dash_offset,
      dash_pattern,
      alpha,
      resource_id,
      substitution,
      image_mask,
      actions,
      clips,
      border,
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
      "CtImage",
      "CT_Image",
      tag_name_prefix,
      tag_name
    );
    let mut boundary = None;
    let mut name = None;
    let mut visible = None;
    let mut ctm = None;
    let mut draw_param = None;
    let mut line_width = None;
    let mut cap = None;
    let mut join = None;
    let mut miter_limit = None;
    let mut dash_offset = None;
    let mut dash_pattern = None;
    let mut alpha = None;
    let mut resource_id = None;
    let mut substitution = None;
    let mut image_mask = None;
    let mut actions = None;
    let mut clips = None;
    let mut border = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Boundary" => {
          boundary =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Visible" => {
          visible = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtImage",
            "visible",
          )?);
        }
        b"CTM" => {
          ctm = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"DrawParam" => {
          draw_param = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtImage",
            "draw_param",
          )?);
        }
        b"LineWidth" => {
          line_width = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtImage",
            "line_width",
          )?);
        }
        b"Cap" => {
          cap = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitCap>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitCap>()?
          });
        }
        b"Join" => {
          join = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitJoin>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitJoin>()?
          });
        }
        b"MiterLimit" => {
          miter_limit = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtImage",
            "miter_limit",
          )?);
        }
        b"DashOffset" => {
          dash_offset = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtImage",
            "dash_offset",
          )?);
        }
        b"DashPattern" => {
          dash_pattern =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Alpha" => {
          alpha = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtImage",
            "alpha",
          )?);
        }
        b"ResourceID" => {
          resource_id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtImage",
            "resource_id",
          )?);
        }
        b"Substitution" => {
          substitution = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtImage",
            "substitution",
          )?);
        }
        b"ImageMask" => {
          image_mask = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtImage",
            "image_mask",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtImage"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Actions" | b"Actions" => {
              actions = Some(
                crate::schemas::page::Actions::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Actions",
                  b"Actions",
                )?,
              );
            }
            b"ofd:Clips" | b"Clips" => {
              clips = Some(crate::schemas::page::Clips::deserialize_from_reader_named(
                xml_reader,
                buf,
                Some((e, e_empty)),
                b"ofd:Clips",
                b"Clips",
              )?);
            }
            b"ofd:Border" | b"Border" => {
              border = Some(crate::schemas::page::Border::deserialize_from_reader_named(
                xml_reader,
                buf,
                Some((e, e_empty)),
                b"ofd:Border",
                b"Border",
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
    let boundary = boundary.ok_or_else(|| crate::common::missing_field("CtImage", "boundary"))?;
    let resource_id =
      resource_id.ok_or_else(|| crate::common::missing_field("CtImage", "resource_id"))?;
    Ok(Self {
      boundary,
      name,
      visible,
      ctm,
      draw_param,
      line_width,
      cap,
      join,
      miter_limit,
      dash_offset,
      dash_pattern,
      alpha,
      resource_id,
      substitution,
      image_mask,
      actions,
      clips,
      border,
    })
  }
}
impl std::str::FromStr for crate::schemas::page::CtComposite {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_Composite", b"CT_Composite")
  }
}
impl crate::schemas::page::CtComposite {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:CT_Composite",
      b"CT_Composite",
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
      "CtComposite",
      "CT_Composite",
      tag_name_prefix,
      tag_name
    );
    let mut boundary = None;
    let mut name = None;
    let mut visible = None;
    let mut ctm = None;
    let mut draw_param = None;
    let mut line_width = None;
    let mut cap = None;
    let mut join = None;
    let mut miter_limit = None;
    let mut dash_offset = None;
    let mut dash_pattern = None;
    let mut alpha = None;
    let mut resource_id = None;
    let mut actions = None;
    let mut clips = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Boundary" => {
          boundary =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Visible" => {
          visible = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtComposite",
            "visible",
          )?);
        }
        b"CTM" => {
          ctm = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"DrawParam" => {
          draw_param = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtComposite",
            "draw_param",
          )?);
        }
        b"LineWidth" => {
          line_width = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtComposite",
            "line_width",
          )?);
        }
        b"Cap" => {
          cap = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitCap>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitCap>()?
          });
        }
        b"Join" => {
          join = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitJoin>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitJoin>()?
          });
        }
        b"MiterLimit" => {
          miter_limit = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtComposite",
            "miter_limit",
          )?);
        }
        b"DashOffset" => {
          dash_offset = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtComposite",
            "dash_offset",
          )?);
        }
        b"DashPattern" => {
          dash_pattern =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Alpha" => {
          alpha = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtComposite",
            "alpha",
          )?);
        }
        b"ResourceID" => {
          resource_id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtComposite",
            "resource_id",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtComposite"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Actions" | b"Actions" => {
              actions = Some(crate::schemas::page::Actions::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Actions",
                b"Actions",
              )?);
            }
            b"ofd:Clips" | b"Clips" => {
              clips = Some(crate::schemas::page::Clips::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Clips",
                b"Clips",
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
    let boundary =
      boundary.ok_or_else(|| crate::common::missing_field("CtComposite", "boundary"))?;
    let resource_id =
      resource_id.ok_or_else(|| crate::common::missing_field("CtComposite", "resource_id"))?;
    Ok(Self {
      boundary,
      name,
      visible,
      ctm,
      draw_param,
      line_width,
      cap,
      join,
      miter_limit,
      dash_offset,
      dash_pattern,
      alpha,
      resource_id,
      actions,
      clips,
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
      "CtComposite",
      "CT_Composite",
      tag_name_prefix,
      tag_name
    );
    let mut boundary = None;
    let mut name = None;
    let mut visible = None;
    let mut ctm = None;
    let mut draw_param = None;
    let mut line_width = None;
    let mut cap = None;
    let mut join = None;
    let mut miter_limit = None;
    let mut dash_offset = None;
    let mut dash_pattern = None;
    let mut alpha = None;
    let mut resource_id = None;
    let mut actions = None;
    let mut clips = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Boundary" => {
          boundary =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Visible" => {
          visible = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtComposite",
            "visible",
          )?);
        }
        b"CTM" => {
          ctm = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"DrawParam" => {
          draw_param = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtComposite",
            "draw_param",
          )?);
        }
        b"LineWidth" => {
          line_width = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtComposite",
            "line_width",
          )?);
        }
        b"Cap" => {
          cap = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitCap>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitCap>()?
          });
        }
        b"Join" => {
          join = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitJoin>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitJoin>()?
          });
        }
        b"MiterLimit" => {
          miter_limit = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtComposite",
            "miter_limit",
          )?);
        }
        b"DashOffset" => {
          dash_offset = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtComposite",
            "dash_offset",
          )?);
        }
        b"DashPattern" => {
          dash_pattern =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Alpha" => {
          alpha = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtComposite",
            "alpha",
          )?);
        }
        b"ResourceID" => {
          resource_id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtComposite",
            "resource_id",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtComposite"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Actions" | b"Actions" => {
              actions = Some(
                crate::schemas::page::Actions::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Actions",
                  b"Actions",
                )?,
              );
            }
            b"ofd:Clips" | b"Clips" => {
              clips = Some(crate::schemas::page::Clips::deserialize_from_reader_named(
                xml_reader,
                buf,
                Some((e, e_empty)),
                b"ofd:Clips",
                b"Clips",
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
    let boundary =
      boundary.ok_or_else(|| crate::common::missing_field("CtComposite", "boundary"))?;
    let resource_id =
      resource_id.ok_or_else(|| crate::common::missing_field("CtComposite", "resource_id"))?;
    Ok(Self {
      boundary,
      name,
      visible,
      ctm,
      draw_param,
      line_width,
      cap,
      join,
      miter_limit,
      dash_offset,
      dash_pattern,
      alpha,
      resource_id,
      actions,
      clips,
    })
  }
}
impl std::str::FromStr for crate::schemas::page::CtPath {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_Path", b"CT_Path")
  }
}
impl crate::schemas::page::CtPath {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:CT_Path", b"CT_Path")
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
      "CtPath",
      "CT_Path",
      tag_name_prefix,
      tag_name
    );
    let mut boundary = None;
    let mut name = None;
    let mut visible = None;
    let mut ctm = None;
    let mut draw_param = None;
    let mut line_width = None;
    let mut cap = None;
    let mut join = None;
    let mut miter_limit = None;
    let mut dash_offset = None;
    let mut dash_pattern = None;
    let mut alpha = None;
    let mut stroke = None;
    let mut fill = None;
    let mut rule = None;
    let mut actions = None;
    let mut clips = None;
    let mut stroke_color = None;
    let mut fill_color = None;
    let mut abbreviated_data = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Boundary" => {
          boundary =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Visible" => {
          visible = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtPath",
            "visible",
          )?);
        }
        b"CTM" => {
          ctm = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"DrawParam" => {
          draw_param = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtPath",
            "draw_param",
          )?);
        }
        b"LineWidth" => {
          line_width = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtPath",
            "line_width",
          )?);
        }
        b"Cap" => {
          cap = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitCap>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitCap>()?
          });
        }
        b"Join" => {
          join = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitJoin>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitJoin>()?
          });
        }
        b"MiterLimit" => {
          miter_limit = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtPath",
            "miter_limit",
          )?);
        }
        b"DashOffset" => {
          dash_offset = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtPath",
            "dash_offset",
          )?);
        }
        b"DashPattern" => {
          dash_pattern =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Alpha" => {
          alpha = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtPath",
            "alpha",
          )?);
        }
        b"Stroke" => {
          stroke = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtPath",
            "stroke",
          )?);
        }
        b"Fill" => {
          fill = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtPath",
            "fill",
          )?);
        }
        b"Rule" => {
          rule = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtPathRule>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtPathRule>()?
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtPath"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Actions" | b"Actions" => {
              actions = Some(crate::schemas::page::Actions::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Actions",
                b"Actions",
              )?);
            }
            b"ofd:Clips" | b"Clips" => {
              clips = Some(crate::schemas::page::Clips::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Clips",
                b"Clips",
              )?);
            }
            b"ofd:StrokeColor" | b"StrokeColor" => {
              stroke_color = Some(crate::schemas::page::CtColor::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:StrokeColor",
                b"StrokeColor",
              )?);
            }
            b"ofd:FillColor" | b"FillColor" => {
              fill_color = Some(crate::schemas::page::CtColor::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:FillColor",
                b"FillColor",
              )?);
            }
            b"ofd:AbbreviatedData" | b"AbbreviatedData" => {
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
                          "CtPath",
                          "abbreviated_data",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:AbbreviatedData" || name == b"AbbreviatedData" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtPath"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              abbreviated_data = Some(parsed_value);
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
    let boundary = boundary.ok_or_else(|| crate::common::missing_field("CtPath", "boundary"))?;
    let abbreviated_data =
      abbreviated_data.ok_or_else(|| crate::common::missing_field("CtPath", "abbreviated_data"))?;
    Ok(Self {
      boundary,
      name,
      visible,
      ctm,
      draw_param,
      line_width,
      cap,
      join,
      miter_limit,
      dash_offset,
      dash_pattern,
      alpha,
      stroke,
      fill,
      rule,
      actions,
      clips,
      stroke_color,
      fill_color,
      abbreviated_data,
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
      "CtPath",
      "CT_Path",
      tag_name_prefix,
      tag_name
    );
    let mut boundary = None;
    let mut name = None;
    let mut visible = None;
    let mut ctm = None;
    let mut draw_param = None;
    let mut line_width = None;
    let mut cap = None;
    let mut join = None;
    let mut miter_limit = None;
    let mut dash_offset = None;
    let mut dash_pattern = None;
    let mut alpha = None;
    let mut stroke = None;
    let mut fill = None;
    let mut rule = None;
    let mut actions = None;
    let mut clips = None;
    let mut stroke_color = None;
    let mut fill_color = None;
    let mut abbreviated_data = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Boundary" => {
          boundary =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Visible" => {
          visible = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtPath",
            "visible",
          )?);
        }
        b"CTM" => {
          ctm = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"DrawParam" => {
          draw_param = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtPath",
            "draw_param",
          )?);
        }
        b"LineWidth" => {
          line_width = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtPath",
            "line_width",
          )?);
        }
        b"Cap" => {
          cap = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitCap>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitCap>()?
          });
        }
        b"Join" => {
          join = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtGraphicUnitJoin>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtGraphicUnitJoin>()?
          });
        }
        b"MiterLimit" => {
          miter_limit = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtPath",
            "miter_limit",
          )?);
        }
        b"DashOffset" => {
          dash_offset = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtPath",
            "dash_offset",
          )?);
        }
        b"DashPattern" => {
          dash_pattern =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Alpha" => {
          alpha = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtPath",
            "alpha",
          )?);
        }
        b"Stroke" => {
          stroke = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtPath",
            "stroke",
          )?);
        }
        b"Fill" => {
          fill = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtPath",
            "fill",
          )?);
        }
        b"Rule" => {
          rule = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtPathRule>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtPathRule>()?
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtPath"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Actions" | b"Actions" => {
              actions = Some(
                crate::schemas::page::Actions::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Actions",
                  b"Actions",
                )?,
              );
            }
            b"ofd:Clips" | b"Clips" => {
              clips = Some(crate::schemas::page::Clips::deserialize_from_reader_named(
                xml_reader,
                buf,
                Some((e, e_empty)),
                b"ofd:Clips",
                b"Clips",
              )?);
            }
            b"ofd:StrokeColor" | b"StrokeColor" => {
              stroke_color = Some(
                crate::schemas::page::CtColor::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:StrokeColor",
                  b"StrokeColor",
                )?,
              );
            }
            b"ofd:FillColor" | b"FillColor" => {
              fill_color = Some(
                crate::schemas::page::CtColor::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:FillColor",
                  b"FillColor",
                )?,
              );
            }
            b"ofd:AbbreviatedData" | b"AbbreviatedData" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtPath",
                      field: "abbreviated_data",
                      tag_name_prefix: b"ofd:AbbreviatedData",
                      tag_name: b"AbbreviatedData",
                    },
                  )?
                }
              };
              abbreviated_data = Some(parsed_value);
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
    let boundary = boundary.ok_or_else(|| crate::common::missing_field("CtPath", "boundary"))?;
    let abbreviated_data =
      abbreviated_data.ok_or_else(|| crate::common::missing_field("CtPath", "abbreviated_data"))?;
    Ok(Self {
      boundary,
      name,
      visible,
      ctm,
      draw_param,
      line_width,
      cap,
      join,
      miter_limit,
      dash_offset,
      dash_pattern,
      alpha,
      stroke,
      fill,
      rule,
      actions,
      clips,
      stroke_color,
      fill_color,
      abbreviated_data,
    })
  }
}
impl std::str::FromStr for crate::schemas::page::CellContent {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CellContent", b"CellContent")
  }
}
impl crate::schemas::page::CellContent {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:CellContent",
      b"CellContent",
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
      "CellContent",
      "CellContent",
      tag_name_prefix,
      tag_name
    );
    let mut thumbnail = None;
    let mut xml_children = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Thumbnail" => {
          thumbnail = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CellContent",
            "thumbnail",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CellContent"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:TextObject" | b"TextObject" => {
              xml_children.push(crate::schemas::page::CellContentContentChoice::TextObject(
                Box::new(crate::schemas::page::TextObject::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:TextObject",
                  b"TextObject",
                )?),
              ));
            }
            b"ofd:PathObject" | b"PathObject" => {
              xml_children.push(crate::schemas::page::CellContentContentChoice::PathObject(
                Box::new(crate::schemas::page::PathObject::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:PathObject",
                  b"PathObject",
                )?),
              ));
            }
            b"ofd:ImageObject" | b"ImageObject" => {
              xml_children.push(crate::schemas::page::CellContentContentChoice::ImageObject(
                Box::new(crate::schemas::page::ImageObject::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:ImageObject",
                  b"ImageObject",
                )?),
              ));
            }
            b"ofd:CompositeObject" | b"CompositeObject" => {
              xml_children.push(
                crate::schemas::page::CellContentContentChoice::CompositeObject(Box::new(
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
              xml_children.push(crate::schemas::page::CellContentContentChoice::PageBlock(
                Box::new(crate::schemas::page::PageBlock::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:PageBlock",
                  b"PageBlock",
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
    Ok(Self {
      thumbnail,
      xml_children,
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
      "CellContent",
      "CellContent",
      tag_name_prefix,
      tag_name
    );
    let mut thumbnail = None;
    let mut xml_children = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Thumbnail" => {
          thumbnail = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CellContent",
            "thumbnail",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CellContent"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:TextObject" | b"TextObject" => {
              xml_children.push(crate::schemas::page::CellContentContentChoice::TextObject(
                Box::new(
                  crate::schemas::page::TextObject::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:TextObject",
                    b"TextObject",
                  )?,
                ),
              ));
            }
            b"ofd:PathObject" | b"PathObject" => {
              xml_children.push(crate::schemas::page::CellContentContentChoice::PathObject(
                Box::new(
                  crate::schemas::page::PathObject::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:PathObject",
                    b"PathObject",
                  )?,
                ),
              ));
            }
            b"ofd:ImageObject" | b"ImageObject" => {
              xml_children.push(crate::schemas::page::CellContentContentChoice::ImageObject(
                Box::new(
                  crate::schemas::page::ImageObject::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:ImageObject",
                    b"ImageObject",
                  )?,
                ),
              ));
            }
            b"ofd:CompositeObject" | b"CompositeObject" => {
              xml_children.push(
                crate::schemas::page::CellContentContentChoice::CompositeObject(Box::new(
                  crate::schemas::page::CompositeObject::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:CompositeObject",
                    b"CompositeObject",
                  )?,
                )),
              );
            }
            b"ofd:PageBlock" | b"PageBlock" => {
              xml_children.push(crate::schemas::page::CellContentContentChoice::PageBlock(
                Box::new(
                  crate::schemas::page::PageBlock::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:PageBlock",
                    b"PageBlock",
                  )?,
                ),
              ));
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
    Ok(Self {
      thumbnail,
      xml_children,
    })
  }
}
impl std::str::FromStr for crate::schemas::page::CtPattern {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_Pattern", b"CT_Pattern")
  }
}
impl crate::schemas::page::CtPattern {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:CT_Pattern",
      b"CT_Pattern",
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
      "CtPattern",
      "CT_Pattern",
      tag_name_prefix,
      tag_name
    );
    let mut width = None;
    let mut height = None;
    let mut x_step = None;
    let mut y_step = None;
    let mut reflect_method = None;
    let mut relative_to = None;
    let mut ctm = None;
    let mut cell_content = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Width" => {
          width = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtPattern",
            "width",
          )?);
        }
        b"Height" => {
          height = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtPattern",
            "height",
          )?);
        }
        b"XStep" => {
          x_step = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtPattern",
            "x_step",
          )?);
        }
        b"YStep" => {
          y_step = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtPattern",
            "y_step",
          )?);
        }
        b"ReflectMethod" => {
          reflect_method = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtPatternReflectMethod>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtPatternReflectMethod>()?
          });
        }
        b"RelativeTo" => {
          relative_to = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtPatternRelativeTo>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtPatternRelativeTo>()?
          });
        }
        b"CTM" => {
          ctm = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtPattern"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:CellContent" | b"CellContent" => {
              cell_content = Some(crate::schemas::page::CellContent::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:CellContent",
                b"CellContent",
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
    let width = width.ok_or_else(|| crate::common::missing_field("CtPattern", "width"))?;
    let height = height.ok_or_else(|| crate::common::missing_field("CtPattern", "height"))?;
    let cell_content =
      cell_content.ok_or_else(|| crate::common::missing_field("CtPattern", "cell_content"))?;
    Ok(Self {
      width,
      height,
      x_step,
      y_step,
      reflect_method,
      relative_to,
      ctm,
      cell_content,
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
      "CtPattern",
      "CT_Pattern",
      tag_name_prefix,
      tag_name
    );
    let mut width = None;
    let mut height = None;
    let mut x_step = None;
    let mut y_step = None;
    let mut reflect_method = None;
    let mut relative_to = None;
    let mut ctm = None;
    let mut cell_content = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Width" => {
          width = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtPattern",
            "width",
          )?);
        }
        b"Height" => {
          height = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtPattern",
            "height",
          )?);
        }
        b"XStep" => {
          x_step = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtPattern",
            "x_step",
          )?);
        }
        b"YStep" => {
          y_step = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtPattern",
            "y_step",
          )?);
        }
        b"ReflectMethod" => {
          reflect_method = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtPatternReflectMethod>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtPatternReflectMethod>()?
          });
        }
        b"RelativeTo" => {
          relative_to = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtPatternRelativeTo>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtPatternRelativeTo>()?
          });
        }
        b"CTM" => {
          ctm = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtPattern"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:CellContent" | b"CellContent" => {
              cell_content = Some(
                crate::schemas::page::CellContent::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:CellContent",
                  b"CellContent",
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
    let width = width.ok_or_else(|| crate::common::missing_field("CtPattern", "width"))?;
    let height = height.ok_or_else(|| crate::common::missing_field("CtPattern", "height"))?;
    let cell_content =
      cell_content.ok_or_else(|| crate::common::missing_field("CtPattern", "cell_content"))?;
    Ok(Self {
      width,
      height,
      x_step,
      y_step,
      reflect_method,
      relative_to,
      ctm,
      cell_content,
    })
  }
}
impl std::str::FromStr for crate::schemas::page::Segment {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Segment", b"Segment")
  }
}
impl crate::schemas::page::Segment {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:Segment", b"Segment")
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
      "Segment",
      "Segment",
      tag_name_prefix,
      tag_name
    );
    let mut position = None;
    let mut color = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Position" => {
          position = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "Segment",
            "position",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Segment"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Color" | b"Color" => {
              color = Some(crate::schemas::page::CtColor::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Color",
                b"Color",
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
    let color = color.ok_or_else(|| crate::common::missing_field("Segment", "color"))?;
    Ok(Self { position, color })
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
      "Segment",
      "Segment",
      tag_name_prefix,
      tag_name
    );
    let mut position = None;
    let mut color = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Position" => {
          position = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "Segment",
            "position",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Segment"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Color" | b"Color" => {
              color = Some(
                crate::schemas::page::CtColor::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Color",
                  b"Color",
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
    let color = color.ok_or_else(|| crate::common::missing_field("Segment", "color"))?;
    Ok(Self { position, color })
  }
}
impl std::str::FromStr for crate::schemas::page::CtAxialShd {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_AxialShd", b"CT_AxialShd")
  }
}
impl crate::schemas::page::CtAxialShd {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:CT_AxialShd",
      b"CT_AxialShd",
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
      "CtAxialShd",
      "CT_AxialShd",
      tag_name_prefix,
      tag_name
    );
    let mut map_type = None;
    let mut map_unit = None;
    let mut extend = None;
    let mut start_point = None;
    let mut end_point = None;
    let mut segment = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"MapType" => {
          map_type = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtAxialShdMapType>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtAxialShdMapType>()?
          });
        }
        b"MapUnit" => {
          map_unit = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtAxialShd",
            "map_unit",
          )?);
        }
        b"Extend" => {
          extend = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtAxialShdExtend>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtAxialShdExtend>()?
          });
        }
        b"StartPoint" => {
          start_point =
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtAxialShd"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Segment" | b"Segment" => {
              segment.push(crate::schemas::page::Segment::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Segment",
                b"Segment",
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
    let start_point =
      start_point.ok_or_else(|| crate::common::missing_field("CtAxialShd", "start_point"))?;
    let end_point =
      end_point.ok_or_else(|| crate::common::missing_field("CtAxialShd", "end_point"))?;
    Ok(Self {
      map_type,
      map_unit,
      extend,
      start_point,
      end_point,
      segment,
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
      "CtAxialShd",
      "CT_AxialShd",
      tag_name_prefix,
      tag_name
    );
    let mut map_type = None;
    let mut map_unit = None;
    let mut extend = None;
    let mut start_point = None;
    let mut end_point = None;
    let mut segment = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"MapType" => {
          map_type = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtAxialShdMapType>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtAxialShdMapType>()?
          });
        }
        b"MapUnit" => {
          map_unit = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtAxialShd",
            "map_unit",
          )?);
        }
        b"Extend" => {
          extend = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtAxialShdExtend>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtAxialShdExtend>()?
          });
        }
        b"StartPoint" => {
          start_point =
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtAxialShd"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Segment" | b"Segment" => {
              segment.push(
                crate::schemas::page::Segment::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Segment",
                  b"Segment",
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
    let start_point =
      start_point.ok_or_else(|| crate::common::missing_field("CtAxialShd", "start_point"))?;
    let end_point =
      end_point.ok_or_else(|| crate::common::missing_field("CtAxialShd", "end_point"))?;
    Ok(Self {
      map_type,
      map_unit,
      extend,
      start_point,
      end_point,
      segment,
    })
  }
}
impl std::str::FromStr for crate::schemas::page::CtRadialShd {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_RadialShd", b"CT_RadialShd")
  }
}
impl crate::schemas::page::CtRadialShd {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:CT_RadialShd",
      b"CT_RadialShd",
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
      "CtRadialShd",
      "CT_RadialShd",
      tag_name_prefix,
      tag_name
    );
    let mut map_type = None;
    let mut map_unit = None;
    let mut eccentricity = None;
    let mut angle = None;
    let mut start_point = None;
    let mut start_radius = None;
    let mut end_point = None;
    let mut end_radius = None;
    let mut extend = None;
    let mut segment = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"MapType" => {
          map_type = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtRadialShdMapType>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtRadialShdMapType>()?
          });
        }
        b"MapUnit" => {
          map_unit = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtRadialShd",
            "map_unit",
          )?);
        }
        b"Eccentricity" => {
          eccentricity = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtRadialShd",
            "eccentricity",
          )?);
        }
        b"Angle" => {
          angle = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtRadialShd",
            "angle",
          )?);
        }
        b"StartPoint" => {
          start_point =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"StartRadius" => {
          start_radius = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtRadialShd",
            "start_radius",
          )?);
        }
        b"EndPoint" => {
          end_point =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"EndRadius" => {
          end_radius = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtRadialShd",
            "end_radius",
          )?);
        }
        b"Extend" => {
          extend = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtRadialShd",
            "extend",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtRadialShd"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Segment" | b"Segment" => {
              segment.push(crate::schemas::page::Segment::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Segment",
                b"Segment",
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
    let start_point =
      start_point.ok_or_else(|| crate::common::missing_field("CtRadialShd", "start_point"))?;
    let end_point =
      end_point.ok_or_else(|| crate::common::missing_field("CtRadialShd", "end_point"))?;
    let end_radius =
      end_radius.ok_or_else(|| crate::common::missing_field("CtRadialShd", "end_radius"))?;
    Ok(Self {
      map_type,
      map_unit,
      eccentricity,
      angle,
      start_point,
      start_radius,
      end_point,
      end_radius,
      extend,
      segment,
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
      "CtRadialShd",
      "CT_RadialShd",
      tag_name_prefix,
      tag_name
    );
    let mut map_type = None;
    let mut map_unit = None;
    let mut eccentricity = None;
    let mut angle = None;
    let mut start_point = None;
    let mut start_radius = None;
    let mut end_point = None;
    let mut end_radius = None;
    let mut extend = None;
    let mut segment = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"MapType" => {
          map_type = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::CtRadialShdMapType>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::CtRadialShdMapType>()?
          });
        }
        b"MapUnit" => {
          map_unit = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtRadialShd",
            "map_unit",
          )?);
        }
        b"Eccentricity" => {
          eccentricity = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtRadialShd",
            "eccentricity",
          )?);
        }
        b"Angle" => {
          angle = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtRadialShd",
            "angle",
          )?);
        }
        b"StartPoint" => {
          start_point =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"StartRadius" => {
          start_radius = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtRadialShd",
            "start_radius",
          )?);
        }
        b"EndPoint" => {
          end_point =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"EndRadius" => {
          end_radius = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtRadialShd",
            "end_radius",
          )?);
        }
        b"Extend" => {
          extend = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtRadialShd",
            "extend",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtRadialShd"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Segment" | b"Segment" => {
              segment.push(
                crate::schemas::page::Segment::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Segment",
                  b"Segment",
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
    let start_point =
      start_point.ok_or_else(|| crate::common::missing_field("CtRadialShd", "start_point"))?;
    let end_point =
      end_point.ok_or_else(|| crate::common::missing_field("CtRadialShd", "end_point"))?;
    let end_radius =
      end_radius.ok_or_else(|| crate::common::missing_field("CtRadialShd", "end_radius"))?;
    Ok(Self {
      map_type,
      map_unit,
      eccentricity,
      angle,
      start_point,
      start_radius,
      end_point,
      end_radius,
      extend,
      segment,
    })
  }
}
impl std::str::FromStr for crate::schemas::page::Point {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Point", b"Point")
  }
}
impl crate::schemas::page::Point {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:Point", b"Point")
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
      "Point",
      "Point",
      tag_name_prefix,
      tag_name
    );
    let mut x = None;
    let mut y = None;
    let mut edge_flag = None;
    let mut color = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"X" => {
          x = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "Point",
            "x",
          )?);
        }
        b"Y" => {
          y = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "Point",
            "y",
          )?);
        }
        b"EdgeFlag" => {
          edge_flag = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::PointEdgeFlag>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::PointEdgeFlag>()?
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Point"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Color" | b"Color" => {
              color = Some(crate::schemas::page::CtColor::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Color",
                b"Color",
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
    let x = x.ok_or_else(|| crate::common::missing_field("Point", "x"))?;
    let y = y.ok_or_else(|| crate::common::missing_field("Point", "y"))?;
    let color = color.ok_or_else(|| crate::common::missing_field("Point", "color"))?;
    Ok(Self {
      x,
      y,
      edge_flag,
      color,
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
      "Point",
      "Point",
      tag_name_prefix,
      tag_name
    );
    let mut x = None;
    let mut y = None;
    let mut edge_flag = None;
    let mut color = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"X" => {
          x = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "Point",
            "x",
          )?);
        }
        b"Y" => {
          y = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "Point",
            "y",
          )?);
        }
        b"EdgeFlag" => {
          edge_flag = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::page::PointEdgeFlag>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::page::PointEdgeFlag>()?
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Point"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Color" | b"Color" => {
              color = Some(
                crate::schemas::page::CtColor::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Color",
                  b"Color",
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
    let x = x.ok_or_else(|| crate::common::missing_field("Point", "x"))?;
    let y = y.ok_or_else(|| crate::common::missing_field("Point", "y"))?;
    let color = color.ok_or_else(|| crate::common::missing_field("Point", "color"))?;
    Ok(Self {
      x,
      y,
      edge_flag,
      color,
    })
  }
}
impl std::str::FromStr for crate::schemas::page::CtGouraudShd {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(
      &mut xml_reader,
      None,
      b"ofd:CT_GouraudShd",
      b"CT_GouraudShd",
    )
  }
}
impl crate::schemas::page::CtGouraudShd {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:CT_GouraudShd",
      b"CT_GouraudShd",
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
      "CtGouraudShd",
      "CT_GouraudShd",
      tag_name_prefix,
      tag_name
    );
    let mut extend = None;
    let mut point = vec![];
    let mut back_color = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Extend" => {
          extend = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtGouraudShd",
            "extend",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtGouraudShd"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Point" | b"Point" => {
              point.push(crate::schemas::page::Point::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Point",
                b"Point",
              )?);
            }
            b"ofd:BackColor" | b"BackColor" => {
              back_color = Some(crate::schemas::page::CtColor::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:BackColor",
                b"BackColor",
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
    Ok(Self {
      extend,
      point,
      back_color,
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
      "CtGouraudShd",
      "CT_GouraudShd",
      tag_name_prefix,
      tag_name
    );
    let mut extend = None;
    let mut point = vec![];
    let mut back_color = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Extend" => {
          extend = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtGouraudShd",
            "extend",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtGouraudShd"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Point" | b"Point" => {
              point.push(crate::schemas::page::Point::deserialize_from_reader_named(
                xml_reader,
                buf,
                Some((e, e_empty)),
                b"ofd:Point",
                b"Point",
              )?);
            }
            b"ofd:BackColor" | b"BackColor" => {
              back_color = Some(
                crate::schemas::page::CtColor::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:BackColor",
                  b"BackColor",
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
    Ok(Self {
      extend,
      point,
      back_color,
    })
  }
}
impl std::str::FromStr for crate::schemas::page::CtLaGouraudShdPoint {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Point", b"Point")
  }
}
impl crate::schemas::page::CtLaGouraudShdPoint {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:Point", b"Point")
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
      "CtLaGouraudShdPoint",
      "Point",
      tag_name_prefix,
      tag_name
    );
    let mut x = None;
    let mut y = None;
    let mut color = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"X" => {
          x = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtLaGouraudShdPoint",
            "x",
          )?);
        }
        b"Y" => {
          y = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtLaGouraudShdPoint",
            "y",
          )?);
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
          quick_xml::events::Event::Eof => {
            Err(crate::common::unexpected_eof("CtLaGouraudShdPoint"))?
          }
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Color" | b"Color" => {
              color = Some(crate::schemas::page::CtColor::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Color",
                b"Color",
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
    let color =
      color.ok_or_else(|| crate::common::missing_field("CtLaGouraudShdPoint", "color"))?;
    Ok(Self { x, y, color })
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
      "CtLaGouraudShdPoint",
      "Point",
      tag_name_prefix,
      tag_name
    );
    let mut x = None;
    let mut y = None;
    let mut color = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"X" => {
          x = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtLaGouraudShdPoint",
            "x",
          )?);
        }
        b"Y" => {
          y = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtLaGouraudShdPoint",
            "y",
          )?);
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
          quick_xml::events::Event::Eof => {
            Err(crate::common::unexpected_eof("CtLaGouraudShdPoint"))?
          }
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Color" | b"Color" => {
              color = Some(
                crate::schemas::page::CtColor::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Color",
                  b"Color",
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
    let color =
      color.ok_or_else(|| crate::common::missing_field("CtLaGouraudShdPoint", "color"))?;
    Ok(Self { x, y, color })
  }
}
impl std::str::FromStr for crate::schemas::page::CtLaGouraudShd {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(
      &mut xml_reader,
      None,
      b"ofd:CT_LaGouraudShd",
      b"CT_LaGouraudShd",
    )
  }
}
impl crate::schemas::page::CtLaGouraudShd {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:CT_LaGouraudShd",
      b"CT_LaGouraudShd",
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
      "CtLaGouraudShd",
      "CT_LaGouraudShd",
      tag_name_prefix,
      tag_name
    );
    let mut vertices_per_row = None;
    let mut extend = None;
    let mut point = vec![];
    let mut back_color = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"VerticesPerRow" => {
          vertices_per_row = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtLaGouraudShd",
            "vertices_per_row",
          )?);
        }
        b"Extend" => {
          extend = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtLaGouraudShd",
            "extend",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtLaGouraudShd"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Point" | b"Point" => {
              point.push(
                crate::schemas::page::CtLaGouraudShdPoint::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Point",
                  b"Point",
                )?,
              );
            }
            b"ofd:BackColor" | b"BackColor" => {
              back_color = Some(crate::schemas::page::CtColor::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:BackColor",
                b"BackColor",
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
    let vertices_per_row = vertices_per_row
      .ok_or_else(|| crate::common::missing_field("CtLaGouraudShd", "vertices_per_row"))?;
    Ok(Self {
      vertices_per_row,
      extend,
      point,
      back_color,
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
      "CtLaGouraudShd",
      "CT_LaGouraudShd",
      tag_name_prefix,
      tag_name
    );
    let mut vertices_per_row = None;
    let mut extend = None;
    let mut point = vec![];
    let mut back_color = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"VerticesPerRow" => {
          vertices_per_row = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtLaGouraudShd",
            "vertices_per_row",
          )?);
        }
        b"Extend" => {
          extend = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtLaGouraudShd",
            "extend",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtLaGouraudShd"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Point" | b"Point" => {
              point.push(
                crate::schemas::page::CtLaGouraudShdPoint::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Point",
                  b"Point",
                )?,
              );
            }
            b"ofd:BackColor" | b"BackColor" => {
              back_color = Some(
                crate::schemas::page::CtColor::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:BackColor",
                  b"BackColor",
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
    let vertices_per_row = vertices_per_row
      .ok_or_else(|| crate::common::missing_field("CtLaGouraudShd", "vertices_per_row"))?;
    Ok(Self {
      vertices_per_row,
      extend,
      point,
      back_color,
    })
  }
}
impl std::str::FromStr for crate::schemas::page::CtColor {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_Color", b"CT_Color")
  }
}
impl crate::schemas::page::CtColor {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:CT_Color",
      b"CT_Color",
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
      "CtColor",
      "CT_Color",
      tag_name_prefix,
      tag_name
    );
    let mut value = None;
    let mut index = None;
    let mut color_space = None;
    let mut alpha = None;
    let mut xml_children = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Value" => {
          value = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Index" => {
          index = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtColor",
            "index",
          )?);
        }
        b"ColorSpace" => {
          color_space = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtColor",
            "color_space",
          )?);
        }
        b"Alpha" => {
          alpha = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtColor",
            "alpha",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtColor"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Pattern" | b"Pattern" => {
              xml_children.push(crate::schemas::page::CtColorContentChoice::Pattern(
                Box::new(crate::schemas::page::CtPattern::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Pattern",
                  b"Pattern",
                )?),
              ));
            }
            b"ofd:AxialShd" | b"AxialShd" => {
              xml_children.push(crate::schemas::page::CtColorContentChoice::AxialShd(
                Box::new(crate::schemas::page::CtAxialShd::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:AxialShd",
                  b"AxialShd",
                )?),
              ));
            }
            b"ofd:RadialShd" | b"RadialShd" => {
              xml_children.push(crate::schemas::page::CtColorContentChoice::RadialShd(
                Box::new(crate::schemas::page::CtRadialShd::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:RadialShd",
                  b"RadialShd",
                )?),
              ));
            }
            b"ofd:GouraudShd" | b"GouraudShd" => {
              xml_children.push(crate::schemas::page::CtColorContentChoice::GouraudShd(
                Box::new(crate::schemas::page::CtGouraudShd::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:GouraudShd",
                  b"GouraudShd",
                )?),
              ));
            }
            b"ofd:LaGourandShd" | b"LaGourandShd" => {
              xml_children.push(crate::schemas::page::CtColorContentChoice::LaGourandShd(
                Box::new(
                  crate::schemas::page::CtLaGouraudShd::deserialize_inner_named(
                    xml_reader,
                    Some((e, e_empty)),
                    b"ofd:LaGourandShd",
                    b"LaGourandShd",
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
    Ok(Self {
      value,
      index,
      color_space,
      alpha,
      xml_children,
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
      "CtColor",
      "CT_Color",
      tag_name_prefix,
      tag_name
    );
    let mut value = None;
    let mut index = None;
    let mut color_space = None;
    let mut alpha = None;
    let mut xml_children = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Value" => {
          value = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Index" => {
          index = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtColor",
            "index",
          )?);
        }
        b"ColorSpace" => {
          color_space = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtColor",
            "color_space",
          )?);
        }
        b"Alpha" => {
          alpha = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtColor",
            "alpha",
          )?);
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtColor"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Pattern" | b"Pattern" => {
              xml_children.push(crate::schemas::page::CtColorContentChoice::Pattern(
                Box::new(
                  crate::schemas::page::CtPattern::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:Pattern",
                    b"Pattern",
                  )?,
                ),
              ));
            }
            b"ofd:AxialShd" | b"AxialShd" => {
              xml_children.push(crate::schemas::page::CtColorContentChoice::AxialShd(
                Box::new(
                  crate::schemas::page::CtAxialShd::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:AxialShd",
                    b"AxialShd",
                  )?,
                ),
              ));
            }
            b"ofd:RadialShd" | b"RadialShd" => {
              xml_children.push(crate::schemas::page::CtColorContentChoice::RadialShd(
                Box::new(
                  crate::schemas::page::CtRadialShd::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:RadialShd",
                    b"RadialShd",
                  )?,
                ),
              ));
            }
            b"ofd:GouraudShd" | b"GouraudShd" => {
              xml_children.push(crate::schemas::page::CtColorContentChoice::GouraudShd(
                Box::new(
                  crate::schemas::page::CtGouraudShd::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:GouraudShd",
                    b"GouraudShd",
                  )?,
                ),
              ));
            }
            b"ofd:LaGourandShd" | b"LaGourandShd" => {
              xml_children.push(crate::schemas::page::CtColorContentChoice::LaGourandShd(
                Box::new(
                  crate::schemas::page::CtLaGouraudShd::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:LaGourandShd",
                    b"LaGourandShd",
                  )?,
                ),
              ));
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
    Ok(Self {
      value,
      index,
      color_space,
      alpha,
      xml_children,
    })
  }
}
impl std::str::FromStr for crate::schemas::page::Action {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Action", b"Action")
  }
}
impl crate::schemas::page::Action {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:Action", b"Action")
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::definitions::CtAction>::deserialize_inner_named(
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
      <crate::schemas::definitions::CtAction>::deserialize_from_reader_named(
        xml_reader,
        buf,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
impl std::str::FromStr for crate::schemas::page::Path {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Path", b"Path")
  }
}
impl crate::schemas::page::Path {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:Path", b"Path")
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::page::CtPath>::deserialize_inner_named(
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
      <crate::schemas::page::CtPath>::deserialize_from_reader_named(
        xml_reader,
        buf,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
impl std::str::FromStr for crate::schemas::page::Text {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Text", b"Text")
  }
}
impl crate::schemas::page::Text {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:Text", b"Text")
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::page::CtText>::deserialize_inner_named(
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
      <crate::schemas::page::CtText>::deserialize_from_reader_named(
        xml_reader,
        buf,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
impl std::str::FromStr for crate::schemas::page::Clip {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Clip", b"Clip")
  }
}
impl crate::schemas::page::Clip {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:Clip", b"Clip")
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::page::CtClip>::deserialize_inner_named(
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
      <crate::schemas::page::CtClip>::deserialize_from_reader_named(
        xml_reader,
        buf,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
impl std::str::FromStr for crate::schemas::page::FillColor {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:FillColor", b"FillColor")
  }
}
impl crate::schemas::page::FillColor {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:FillColor",
      b"FillColor",
    )
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::page::CtColor>::deserialize_inner_named(
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
      <crate::schemas::page::CtColor>::deserialize_from_reader_named(
        xml_reader,
        buf,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
impl std::str::FromStr for crate::schemas::page::StrokeColor {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:StrokeColor", b"StrokeColor")
  }
}
impl crate::schemas::page::StrokeColor {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:StrokeColor",
      b"StrokeColor",
    )
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::page::CtColor>::deserialize_inner_named(
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
      <crate::schemas::page::CtColor>::deserialize_from_reader_named(
        xml_reader,
        buf,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
impl std::str::FromStr for crate::schemas::page::CgTransform {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CGTransform", b"CGTransform")
  }
}
impl crate::schemas::page::CgTransform {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:CGTransform",
      b"CGTransform",
    )
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::page::CtCgTransform>::deserialize_inner_named(
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
      <crate::schemas::page::CtCgTransform>::deserialize_from_reader_named(
        xml_reader,
        buf,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
impl std::str::FromStr for crate::schemas::page::BorderColor {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:BorderColor", b"BorderColor")
  }
}
impl crate::schemas::page::BorderColor {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:BorderColor",
      b"BorderColor",
    )
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::page::CtColor>::deserialize_inner_named(
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
      <crate::schemas::page::CtColor>::deserialize_from_reader_named(
        xml_reader,
        buf,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
impl std::str::FromStr for crate::schemas::page::Color {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Color", b"Color")
  }
}
impl crate::schemas::page::Color {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:Color", b"Color")
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::page::CtColor>::deserialize_inner_named(
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
      <crate::schemas::page::CtColor>::deserialize_from_reader_named(
        xml_reader,
        buf,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
impl std::str::FromStr for crate::schemas::page::BackColor {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:BackColor", b"BackColor")
  }
}
impl crate::schemas::page::BackColor {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:BackColor",
      b"BackColor",
    )
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::page::CtColor>::deserialize_inner_named(
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
      <crate::schemas::page::CtColor>::deserialize_from_reader_named(
        xml_reader,
        buf,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
impl std::str::FromStr for crate::schemas::page::Pattern {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Pattern", b"Pattern")
  }
}
impl crate::schemas::page::Pattern {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:Pattern", b"Pattern")
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::page::CtPattern>::deserialize_inner_named(
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
      <crate::schemas::page::CtPattern>::deserialize_from_reader_named(
        xml_reader,
        buf,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
impl std::str::FromStr for crate::schemas::page::AxialShd {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:AxialShd", b"AxialShd")
  }
}
impl crate::schemas::page::AxialShd {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:AxialShd",
      b"AxialShd",
    )
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::page::CtAxialShd>::deserialize_inner_named(
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
      <crate::schemas::page::CtAxialShd>::deserialize_from_reader_named(
        xml_reader,
        buf,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
impl std::str::FromStr for crate::schemas::page::RadialShd {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:RadialShd", b"RadialShd")
  }
}
impl crate::schemas::page::RadialShd {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:RadialShd",
      b"RadialShd",
    )
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::page::CtRadialShd>::deserialize_inner_named(
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
      <crate::schemas::page::CtRadialShd>::deserialize_from_reader_named(
        xml_reader,
        buf,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
impl std::str::FromStr for crate::schemas::page::GouraudShd {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:GouraudShd", b"GouraudShd")
  }
}
impl crate::schemas::page::GouraudShd {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:GouraudShd",
      b"GouraudShd",
    )
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::page::CtGouraudShd>::deserialize_inner_named(
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
      <crate::schemas::page::CtGouraudShd>::deserialize_from_reader_named(
        xml_reader,
        buf,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
impl std::str::FromStr for crate::schemas::page::LaGourandShd {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:LaGourandShd", b"LaGourandShd")
  }
}
impl crate::schemas::page::LaGourandShd {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:LaGourandShd",
      b"LaGourandShd",
    )
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::page::CtLaGouraudShd>::deserialize_inner_named(
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
      <crate::schemas::page::CtLaGouraudShd>::deserialize_from_reader_named(
        xml_reader,
        buf,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
