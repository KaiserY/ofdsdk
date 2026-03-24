//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

impl std::str::FromStr for crate::schemas::res::CtColorSpaceType {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "GRAY" => Ok(Self::Gray),
      "RGB" => Ok(Self::Rgb),
      "CMYK" => Ok(Self::Cmyk),
      _ => Err(crate::common::invalid_enum_value("CtColorSpaceType", s)),
    }
  }
}
impl crate::schemas::res::CtColorSpaceType {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"GRAY" => Ok(Self::Gray),
      b"RGB" => Ok(Self::Rgb),
      b"CMYK" => Ok(Self::Cmyk),
      other => Err(crate::common::invalid_enum_value(
        "CtColorSpaceType",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::res::CtFontCharset {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "symbol" => Ok(Self::Symbol),
      "prc" => Ok(Self::Prc),
      "big5" => Ok(Self::Big5),
      "shift-jis" => Ok(Self::_ShiftJis),
      "wansung" => Ok(Self::Wansung),
      "johab" => Ok(Self::Johab),
      "unicode" => Ok(Self::Unicode),
      _ => Err(crate::common::invalid_enum_value("CtFontCharset", s)),
    }
  }
}
impl crate::schemas::res::CtFontCharset {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"symbol" => Ok(Self::Symbol),
      b"prc" => Ok(Self::Prc),
      b"big5" => Ok(Self::Big5),
      b"shift-jis" => Ok(Self::_ShiftJis),
      b"wansung" => Ok(Self::Wansung),
      b"johab" => Ok(Self::Johab),
      b"unicode" => Ok(Self::Unicode),
      other => Err(crate::common::invalid_enum_value(
        "CtFontCharset",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::res::CtMultiMediaType {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Image" => Ok(Self::Image),
      "Audio" => Ok(Self::Audio),
      "Video" => Ok(Self::Video),
      _ => Err(crate::common::invalid_enum_value("CtMultiMediaType", s)),
    }
  }
}
impl crate::schemas::res::CtMultiMediaType {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"Image" => Ok(Self::Image),
      b"Audio" => Ok(Self::Audio),
      b"Video" => Ok(Self::Video),
      other => Err(crate::common::invalid_enum_value(
        "CtMultiMediaType",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::res::ColorSpace {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:ColorSpace", b"ColorSpace")
  }
}
impl crate::schemas::res::ColorSpace {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:ColorSpace", b"ColorSpace")
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
      "ColorSpace",
      "ColorSpace",
      tag_name_prefix,
      tag_name
    );
    let mut r#type = None;
    let mut bits_per_component = None;
    let mut profile = None;
    let mut id = None;
    let mut palette = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Type" => {
          r#type = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::res::CtColorSpaceType>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::res::CtColorSpaceType>()?
          });
        }
        b"BitsPerComponent" => {
          bits_per_component = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "ColorSpace",
            "bits_per_component",
          )?);
        }
        b"Profile" => {
          profile =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"ID" => {
          id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "ColorSpace",
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("ColorSpace"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Palette" | b"Palette" => {
              palette = Some(crate::schemas::res::Palette::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Palette",
                b"Palette",
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
    let r#type = r#type.ok_or_else(|| crate::common::missing_field("ColorSpace", "type"))?;
    let id = id.ok_or_else(|| crate::common::missing_field("ColorSpace", "id"))?;
    Ok(Self {
      r#type,
      bits_per_component,
      profile,
      id,
      palette,
    })
  }
}
impl std::str::FromStr for crate::schemas::res::ColorSpaces {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:ColorSpaces", b"ColorSpaces")
  }
}
impl crate::schemas::res::ColorSpaces {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:ColorSpaces", b"ColorSpaces")
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
      "ColorSpaces",
      "ColorSpaces",
      tag_name_prefix,
      tag_name
    );
    let mut color_space = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("ColorSpaces"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:ColorSpace" | b"ColorSpace" => {
              color_space.push(crate::schemas::res::ColorSpace::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:ColorSpace",
                b"ColorSpace",
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
    Ok(Self { color_space })
  }
}
impl std::str::FromStr for crate::schemas::res::DrawParam {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:DrawParam", b"DrawParam")
  }
}
impl crate::schemas::res::DrawParam {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:DrawParam", b"DrawParam")
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
      "DrawParam",
      "DrawParam",
      tag_name_prefix,
      tag_name
    );
    let mut relative = None;
    let mut line_width = None;
    let mut join = None;
    let mut cap = None;
    let mut dash_offset = None;
    let mut dash_pattern = None;
    let mut miter_limit = None;
    let mut id = None;
    let mut fill_color = None;
    let mut stroke_color = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Relative" => {
          relative = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "DrawParam",
            "relative",
          )?);
        }
        b"LineWidth" => {
          line_width = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "DrawParam",
            "line_width",
          )?);
        }
        b"Join" => {
          join = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Cap" => {
          cap = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"DashOffset" => {
          dash_offset = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "DrawParam",
            "dash_offset",
          )?);
        }
        b"DashPattern" => {
          dash_pattern =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"MiterLimit" => {
          miter_limit = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "DrawParam",
            "miter_limit",
          )?);
        }
        b"ID" => {
          id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "DrawParam",
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("DrawParam"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
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
            _ => {
              if !e_empty {
                xml_reader.read_to_end(e.to_end().name())?;
              }
            }
          }
        }
      }
    }
    let id = id.ok_or_else(|| crate::common::missing_field("DrawParam", "id"))?;
    Ok(Self {
      relative,
      line_width,
      join,
      cap,
      dash_offset,
      dash_pattern,
      miter_limit,
      id,
      fill_color,
      stroke_color,
    })
  }
}
impl std::str::FromStr for crate::schemas::res::DrawParams {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:DrawParams", b"DrawParams")
  }
}
impl crate::schemas::res::DrawParams {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:DrawParams", b"DrawParams")
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
      "DrawParams",
      "DrawParams",
      tag_name_prefix,
      tag_name
    );
    let mut draw_param = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("DrawParams"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:DrawParam" | b"DrawParam" => {
              draw_param.push(crate::schemas::res::DrawParam::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:DrawParam",
                b"DrawParam",
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
    Ok(Self { draw_param })
  }
}
impl std::str::FromStr for crate::schemas::res::Font {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Font", b"Font")
  }
}
impl crate::schemas::res::Font {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Font", b"Font")
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
      "Font",
      "Font",
      tag_name_prefix,
      tag_name
    );
    let mut font_name = None;
    let mut family_name = None;
    let mut charset = None;
    let mut italic = None;
    let mut bold = None;
    let mut serif = None;
    let mut fixed_width = None;
    let mut id = None;
    let mut font_file = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"FontName" => {
          font_name =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"FamilyName" => {
          family_name =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Charset" => {
          charset = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::res::CtFontCharset>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::res::CtFontCharset>()?
          });
        }
        b"Italic" => {
          italic = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "Font",
            "italic",
          )?);
        }
        b"Bold" => {
          bold = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "Font",
            "bold",
          )?);
        }
        b"Serif" => {
          serif = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "Font",
            "serif",
          )?);
        }
        b"FixedWidth" => {
          fixed_width = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "Font",
            "fixed_width",
          )?);
        }
        b"ID" => {
          id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "Font",
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Font"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:FontFile" | b"FontFile" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content(
                    xml_reader,
                    "Font",
                    "font_file",
                    b"ofd:FontFile",
                    b"FontFile",
                  )?
                }
              };
              font_file = Some(parsed_value);
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
    let font_name = font_name.ok_or_else(|| crate::common::missing_field("Font", "font_name"))?;
    let id = id.ok_or_else(|| crate::common::missing_field("Font", "id"))?;
    Ok(Self {
      font_name,
      family_name,
      charset,
      italic,
      bold,
      serif,
      fixed_width,
      id,
      font_file,
    })
  }
}
impl std::str::FromStr for crate::schemas::res::Fonts {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Fonts", b"Fonts")
  }
}
impl crate::schemas::res::Fonts {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Fonts", b"Fonts")
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
      "Fonts",
      "Fonts",
      tag_name_prefix,
      tag_name
    );
    let mut font = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Fonts"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Font" | b"Font" => {
              font.push(crate::schemas::res::Font::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Font",
                b"Font",
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
    Ok(Self { font })
  }
}
impl std::str::FromStr for crate::schemas::res::MultiMedia {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:MultiMedia", b"MultiMedia")
  }
}
impl crate::schemas::res::MultiMedia {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:MultiMedia", b"MultiMedia")
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
      "MultiMedia",
      "MultiMedia",
      tag_name_prefix,
      tag_name
    );
    let mut r#type = None;
    let mut format = None;
    let mut id = None;
    let mut media_file = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Type" => {
          r#type = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::res::CtMultiMediaType>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::res::CtMultiMediaType>()?
          });
        }
        b"Format" => {
          format =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"ID" => {
          id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "MultiMedia",
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("MultiMedia"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:MediaFile" | b"MediaFile" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content(
                    xml_reader,
                    "MultiMedia",
                    "media_file",
                    b"ofd:MediaFile",
                    b"MediaFile",
                  )?
                }
              };
              media_file = Some(parsed_value);
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
    let r#type = r#type.ok_or_else(|| crate::common::missing_field("MultiMedia", "type"))?;
    let id = id.ok_or_else(|| crate::common::missing_field("MultiMedia", "id"))?;
    let media_file =
      media_file.ok_or_else(|| crate::common::missing_field("MultiMedia", "media_file"))?;
    Ok(Self {
      r#type,
      format,
      id,
      media_file,
    })
  }
}
impl std::str::FromStr for crate::schemas::res::MultiMedias {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:MultiMedias", b"MultiMedias")
  }
}
impl crate::schemas::res::MultiMedias {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:MultiMedias", b"MultiMedias")
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
      "MultiMedias",
      "MultiMedias",
      tag_name_prefix,
      tag_name
    );
    let mut multi_media = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("MultiMedias"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:MultiMedia" | b"MultiMedia" => {
              multi_media.push(crate::schemas::res::MultiMedia::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:MultiMedia",
                b"MultiMedia",
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
    Ok(Self { multi_media })
  }
}
impl std::str::FromStr for crate::schemas::res::CompositeGraphicUnit {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(
      &mut xml_reader,
      None,
      b"ofd:CompositeGraphicUnit",
      b"CompositeGraphicUnit",
    )
  }
}
impl crate::schemas::res::CompositeGraphicUnit {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(
      &mut xml_reader,
      None,
      b"ofd:CompositeGraphicUnit",
      b"CompositeGraphicUnit",
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
      "CompositeGraphicUnit",
      "CompositeGraphicUnit",
      tag_name_prefix,
      tag_name
    );
    let mut width = None;
    let mut height = None;
    let mut id = None;
    let mut thumbnail = None;
    let mut substitution = None;
    let mut content = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Width" => {
          width = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CompositeGraphicUnit",
            "width",
          )?);
        }
        b"Height" => {
          height = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CompositeGraphicUnit",
            "height",
          )?);
        }
        b"ID" => {
          id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CompositeGraphicUnit",
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
          quick_xml::events::Event::Eof => {
            Err(crate::common::unexpected_eof("CompositeGraphicUnit"))?
          }
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Thumbnail" | b"Thumbnail" => {
              let parsed_value = {
                let value = if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content(
                    xml_reader,
                    "CompositeGraphicUnit",
                    "thumbnail",
                    b"ofd:Thumbnail",
                    b"Thumbnail",
                  )?
                };
                {
                  value.parse::<u32>().map_err(|_| {
                    crate::common::invalid_field_value(
                      "CompositeGraphicUnit",
                      "thumbnail",
                      value.clone(),
                    )
                  })?
                }
              };
              thumbnail = Some(parsed_value);
            }
            b"ofd:Substitution" | b"Substitution" => {
              let parsed_value = {
                let value = if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content(
                    xml_reader,
                    "CompositeGraphicUnit",
                    "substitution",
                    b"ofd:Substitution",
                    b"Substitution",
                  )?
                };
                {
                  value.parse::<u32>().map_err(|_| {
                    crate::common::invalid_field_value(
                      "CompositeGraphicUnit",
                      "substitution",
                      value.clone(),
                    )
                  })?
                }
              };
              substitution = Some(parsed_value);
            }
            b"ofd:Content" | b"Content" => {
              content = Some(crate::schemas::page::CtPageBlock::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Content",
                b"Content",
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
    let width =
      width.ok_or_else(|| crate::common::missing_field("CompositeGraphicUnit", "width"))?;
    let height =
      height.ok_or_else(|| crate::common::missing_field("CompositeGraphicUnit", "height"))?;
    let id = id.ok_or_else(|| crate::common::missing_field("CompositeGraphicUnit", "id"))?;
    let content =
      content.ok_or_else(|| crate::common::missing_field("CompositeGraphicUnit", "content"))?;
    Ok(Self {
      width,
      height,
      id,
      thumbnail,
      substitution,
      content,
    })
  }
}
impl std::str::FromStr for crate::schemas::res::CompositeGraphicUnits {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(
      &mut xml_reader,
      None,
      b"ofd:CompositeGraphicUnits",
      b"CompositeGraphicUnits",
    )
  }
}
impl crate::schemas::res::CompositeGraphicUnits {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(
      &mut xml_reader,
      None,
      b"ofd:CompositeGraphicUnits",
      b"CompositeGraphicUnits",
    )
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
      "CompositeGraphicUnits",
      "CompositeGraphicUnits",
      tag_name_prefix,
      tag_name
    );
    let mut composite_graphic_unit = vec![];
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
          quick_xml::events::Event::Eof => {
            Err(crate::common::unexpected_eof("CompositeGraphicUnits"))?
          }
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:CompositeGraphicUnit" | b"CompositeGraphicUnit" => {
              composite_graphic_unit.push(
                crate::schemas::res::CompositeGraphicUnit::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:CompositeGraphicUnit",
                  b"CompositeGraphicUnit",
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
      composite_graphic_unit,
    })
  }
}
impl std::str::FromStr for crate::schemas::res::Res {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Res", b"Res")
  }
}
impl crate::schemas::res::Res {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Res", b"Res")
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
      "Res",
      "Res",
      tag_name_prefix,
      tag_name
    );
    let mut base_loc = None;
    let mut xml_children = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Res"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:ColorSpaces" | b"ColorSpaces" => {
              xml_children.push(crate::schemas::res::ResContentChoice::ColorSpaces(
                Box::new(crate::schemas::res::ColorSpaces::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:ColorSpaces",
                  b"ColorSpaces",
                )?),
              ));
            }
            b"ofd:DrawParams" | b"DrawParams" => {
              xml_children.push(crate::schemas::res::ResContentChoice::DrawParams(Box::new(
                crate::schemas::res::DrawParams::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:DrawParams",
                  b"DrawParams",
                )?,
              )));
            }
            b"ofd:Fonts" | b"Fonts" => {
              xml_children.push(crate::schemas::res::ResContentChoice::Fonts(Box::new(
                crate::schemas::res::Fonts::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Fonts",
                  b"Fonts",
                )?,
              )));
            }
            b"ofd:MultiMedias" | b"MultiMedias" => {
              xml_children.push(crate::schemas::res::ResContentChoice::MultiMedias(
                Box::new(crate::schemas::res::MultiMedias::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:MultiMedias",
                  b"MultiMedias",
                )?),
              ));
            }
            b"ofd:CompositeGraphicUnits" | b"CompositeGraphicUnits" => {
              xml_children.push(
                crate::schemas::res::ResContentChoice::CompositeGraphicUnits(Box::new(
                  crate::schemas::res::CompositeGraphicUnits::deserialize_inner_named(
                    xml_reader,
                    Some((e, e_empty)),
                    b"ofd:CompositeGraphicUnits",
                    b"CompositeGraphicUnits",
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
    let base_loc = base_loc.ok_or_else(|| crate::common::missing_field("Res", "base_loc"))?;
    Ok(Self {
      base_loc,
      xml_children,
    })
  }
}
impl std::str::FromStr for crate::schemas::res::Palette {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Palette", b"Palette")
  }
}
impl crate::schemas::res::Palette {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Palette", b"Palette")
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
      "Palette",
      "Palette",
      tag_name_prefix,
      tag_name
    );
    let mut cv = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Palette"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:CV" | b"CV" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content(xml_reader, "Palette", "cv", b"ofd:CV", b"CV")?
                }
              };
              cv.push(parsed_value);
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
    Ok(Self { cv })
  }
}
impl std::str::FromStr for crate::schemas::res::CtColorSpace {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(
      &mut xml_reader,
      None,
      b"ofd:CT_ColorSpace",
      b"CT_ColorSpace",
    )
  }
}
impl crate::schemas::res::CtColorSpace {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(
      &mut xml_reader,
      None,
      b"ofd:CT_ColorSpace",
      b"CT_ColorSpace",
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
      "CtColorSpace",
      "CT_ColorSpace",
      tag_name_prefix,
      tag_name
    );
    let mut r#type = None;
    let mut bits_per_component = None;
    let mut profile = None;
    let mut palette = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Type" => {
          r#type = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::res::CtColorSpaceType>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::res::CtColorSpaceType>()?
          });
        }
        b"BitsPerComponent" => {
          bits_per_component = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtColorSpace",
            "bits_per_component",
          )?);
        }
        b"Profile" => {
          profile =
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtColorSpace"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Palette" | b"Palette" => {
              palette = Some(crate::schemas::res::Palette::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Palette",
                b"Palette",
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
    let r#type = r#type.ok_or_else(|| crate::common::missing_field("CtColorSpace", "type"))?;
    Ok(Self {
      r#type,
      bits_per_component,
      profile,
      palette,
    })
  }
}
impl std::str::FromStr for crate::schemas::res::CtDrawParam {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_DrawParam", b"CT_DrawParam")
  }
}
impl crate::schemas::res::CtDrawParam {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_DrawParam", b"CT_DrawParam")
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
      "CtDrawParam",
      "CT_DrawParam",
      tag_name_prefix,
      tag_name
    );
    let mut relative = None;
    let mut line_width = None;
    let mut join = None;
    let mut cap = None;
    let mut dash_offset = None;
    let mut dash_pattern = None;
    let mut miter_limit = None;
    let mut fill_color = None;
    let mut stroke_color = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Relative" => {
          relative = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtDrawParam",
            "relative",
          )?);
        }
        b"LineWidth" => {
          line_width = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtDrawParam",
            "line_width",
          )?);
        }
        b"Join" => {
          join = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Cap" => {
          cap = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"DashOffset" => {
          dash_offset = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtDrawParam",
            "dash_offset",
          )?);
        }
        b"DashPattern" => {
          dash_pattern =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"MiterLimit" => {
          miter_limit = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtDrawParam",
            "miter_limit",
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtDrawParam"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
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
      relative,
      line_width,
      join,
      cap,
      dash_offset,
      dash_pattern,
      miter_limit,
      fill_color,
      stroke_color,
    })
  }
}
impl std::str::FromStr for crate::schemas::res::CtFont {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_Font", b"CT_Font")
  }
}
impl crate::schemas::res::CtFont {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_Font", b"CT_Font")
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
      "CtFont",
      "CT_Font",
      tag_name_prefix,
      tag_name
    );
    let mut font_name = None;
    let mut family_name = None;
    let mut charset = None;
    let mut italic = None;
    let mut bold = None;
    let mut serif = None;
    let mut fixed_width = None;
    let mut font_file = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"FontName" => {
          font_name =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"FamilyName" => {
          family_name =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Charset" => {
          charset = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::res::CtFontCharset>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::res::CtFontCharset>()?
          });
        }
        b"Italic" => {
          italic = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtFont",
            "italic",
          )?);
        }
        b"Bold" => {
          bold = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtFont",
            "bold",
          )?);
        }
        b"Serif" => {
          serif = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtFont",
            "serif",
          )?);
        }
        b"FixedWidth" => {
          fixed_width = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtFont",
            "fixed_width",
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtFont"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:FontFile" | b"FontFile" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content(
                    xml_reader,
                    "CtFont",
                    "font_file",
                    b"ofd:FontFile",
                    b"FontFile",
                  )?
                }
              };
              font_file = Some(parsed_value);
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
    let font_name = font_name.ok_or_else(|| crate::common::missing_field("CtFont", "font_name"))?;
    Ok(Self {
      font_name,
      family_name,
      charset,
      italic,
      bold,
      serif,
      fixed_width,
      font_file,
    })
  }
}
impl std::str::FromStr for crate::schemas::res::CtMultiMedia {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(
      &mut xml_reader,
      None,
      b"ofd:CT_MultiMedia",
      b"CT_MultiMedia",
    )
  }
}
impl crate::schemas::res::CtMultiMedia {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(
      &mut xml_reader,
      None,
      b"ofd:CT_MultiMedia",
      b"CT_MultiMedia",
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
      "CtMultiMedia",
      "CT_MultiMedia",
      tag_name_prefix,
      tag_name
    );
    let mut r#type = None;
    let mut format = None;
    let mut media_file = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Type" => {
          r#type = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::res::CtMultiMediaType>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::res::CtMultiMediaType>()?
          });
        }
        b"Format" => {
          format =
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtMultiMedia"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:MediaFile" | b"MediaFile" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content(
                    xml_reader,
                    "CtMultiMedia",
                    "media_file",
                    b"ofd:MediaFile",
                    b"MediaFile",
                  )?
                }
              };
              media_file = Some(parsed_value);
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
    let r#type = r#type.ok_or_else(|| crate::common::missing_field("CtMultiMedia", "type"))?;
    let media_file =
      media_file.ok_or_else(|| crate::common::missing_field("CtMultiMedia", "media_file"))?;
    Ok(Self {
      r#type,
      format,
      media_file,
    })
  }
}
impl std::str::FromStr for crate::schemas::res::CtVectorG {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_VectorG", b"CT_VectorG")
  }
}
impl crate::schemas::res::CtVectorG {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_VectorG", b"CT_VectorG")
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
      "CtVectorG",
      "CT_VectorG",
      tag_name_prefix,
      tag_name
    );
    let mut width = None;
    let mut height = None;
    let mut thumbnail = None;
    let mut substitution = None;
    let mut content = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Width" => {
          width = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtVectorG",
            "width",
          )?);
        }
        b"Height" => {
          height = Some(crate::common::parse_f64_attr(
            &attr,
            xml_reader.decoder(),
            "CtVectorG",
            "height",
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtVectorG"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Thumbnail" | b"Thumbnail" => {
              let parsed_value = {
                let value = if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content(
                    xml_reader,
                    "CtVectorG",
                    "thumbnail",
                    b"ofd:Thumbnail",
                    b"Thumbnail",
                  )?
                };
                {
                  value.parse::<u32>().map_err(|_| {
                    crate::common::invalid_field_value("CtVectorG", "thumbnail", value.clone())
                  })?
                }
              };
              thumbnail = Some(parsed_value);
            }
            b"ofd:Substitution" | b"Substitution" => {
              let parsed_value = {
                let value = if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content(
                    xml_reader,
                    "CtVectorG",
                    "substitution",
                    b"ofd:Substitution",
                    b"Substitution",
                  )?
                };
                {
                  value.parse::<u32>().map_err(|_| {
                    crate::common::invalid_field_value("CtVectorG", "substitution", value.clone())
                  })?
                }
              };
              substitution = Some(parsed_value);
            }
            b"ofd:Content" | b"Content" => {
              content = Some(crate::schemas::page::CtPageBlock::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Content",
                b"Content",
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
    let width = width.ok_or_else(|| crate::common::missing_field("CtVectorG", "width"))?;
    let height = height.ok_or_else(|| crate::common::missing_field("CtVectorG", "height"))?;
    let content = content.ok_or_else(|| crate::common::missing_field("CtVectorG", "content"))?;
    Ok(Self {
      width,
      height,
      thumbnail,
      substitution,
      content,
    })
  }
}
impl std::str::FromStr for crate::schemas::res::FillColor {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:FillColor", b"FillColor")
  }
}
impl crate::schemas::res::FillColor {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:FillColor", b"FillColor")
  }
  pub(crate) fn deserialize_inner_named<'de, R: crate::common::XmlReader<'de>>(
    xml_reader: &mut R,
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
}
impl std::str::FromStr for crate::schemas::res::StrokeColor {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:StrokeColor", b"StrokeColor")
  }
}
impl crate::schemas::res::StrokeColor {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:StrokeColor", b"StrokeColor")
  }
  pub(crate) fn deserialize_inner_named<'de, R: crate::common::XmlReader<'de>>(
    xml_reader: &mut R,
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
}
impl std::str::FromStr for crate::schemas::res::Content {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Content", b"Content")
  }
}
impl crate::schemas::res::Content {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Content", b"Content")
  }
  pub(crate) fn deserialize_inner_named<'de, R: crate::common::XmlReader<'de>>(
    xml_reader: &mut R,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::page::CtPageBlock>::deserialize_inner_named(
        xml_reader,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
