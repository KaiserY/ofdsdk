//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

impl std::str::FromStr for crate::schemas::document::TemplatePageZOrder {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Background" => Ok(Self::Background),
      "Foreground" => Ok(Self::Foreground),
      _ => Err(crate::common::invalid_enum_value("TemplatePageZOrder", s)),
    }
  }
}
impl crate::schemas::document::TemplatePageZOrder {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"Background" => Ok(Self::Background),
      b"Foreground" => Ok(Self::Foreground),
      other => Err(crate::common::invalid_enum_value(
        "TemplatePageZOrder",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::document::PageMode {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "None" => Ok(Self::None),
      "FullScreen" => Ok(Self::FullScreen),
      "UseOutlines" => Ok(Self::UseOutlines),
      "UseThumbs" => Ok(Self::UseThumbs),
      "UseCustomTags" => Ok(Self::UseCustomTags),
      "UseLayers" => Ok(Self::UseLayers),
      "UseAttatchs" => Ok(Self::UseAttatchs),
      "UseBookmarks" => Ok(Self::UseBookmarks),
      _ => Err(crate::common::invalid_enum_value("PageMode", s)),
    }
  }
}
impl crate::schemas::document::PageMode {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"None" => Ok(Self::None),
      b"FullScreen" => Ok(Self::FullScreen),
      b"UseOutlines" => Ok(Self::UseOutlines),
      b"UseThumbs" => Ok(Self::UseThumbs),
      b"UseCustomTags" => Ok(Self::UseCustomTags),
      b"UseLayers" => Ok(Self::UseLayers),
      b"UseAttatchs" => Ok(Self::UseAttatchs),
      b"UseBookmarks" => Ok(Self::UseBookmarks),
      other => Err(crate::common::invalid_enum_value(
        "PageMode",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::document::PageLayout {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "OnePage" => Ok(Self::OnePage),
      "OneColumn" => Ok(Self::OneColumn),
      "TwoPageL" => Ok(Self::TwoPageL),
      "TwoColumnL" => Ok(Self::TwoColumnL),
      "TwoPageR" => Ok(Self::TwoPageR),
      "TwoColumnR" => Ok(Self::TwoColumnR),
      _ => Err(crate::common::invalid_enum_value("PageLayout", s)),
    }
  }
}
impl crate::schemas::document::PageLayout {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"OnePage" => Ok(Self::OnePage),
      b"OneColumn" => Ok(Self::OneColumn),
      b"TwoPageL" => Ok(Self::TwoPageL),
      b"TwoColumnL" => Ok(Self::TwoColumnL),
      b"TwoPageR" => Ok(Self::TwoPageR),
      b"TwoColumnR" => Ok(Self::TwoColumnR),
      other => Err(crate::common::invalid_enum_value(
        "PageLayout",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::document::TabDisplay {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "DocTitle" => Ok(Self::DocTitle),
      "FileName" => Ok(Self::FileName),
      _ => Err(crate::common::invalid_enum_value("TabDisplay", s)),
    }
  }
}
impl crate::schemas::document::TabDisplay {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"DocTitle" => Ok(Self::DocTitle),
      b"FileName" => Ok(Self::FileName),
      other => Err(crate::common::invalid_enum_value(
        "TabDisplay",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::document::ZoomMode {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Default" => Ok(Self::Default),
      "FitHeight" => Ok(Self::FitHeight),
      "FitWidth" => Ok(Self::FitWidth),
      "FitRect" => Ok(Self::FitRect),
      _ => Err(crate::common::invalid_enum_value("ZoomMode", s)),
    }
  }
}
impl crate::schemas::document::ZoomMode {
  pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
    match b {
      b"Default" => Ok(Self::Default),
      b"FitHeight" => Ok(Self::FitHeight),
      b"FitWidth" => Ok(Self::FitWidth),
      b"FitRect" => Ok(Self::FitRect),
      other => Err(crate::common::invalid_enum_value(
        "ZoomMode",
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  }
}
impl std::str::FromStr for crate::schemas::document::TemplatePage {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:TemplatePage", b"TemplatePage")
  }
}
impl crate::schemas::document::TemplatePage {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:TemplatePage",
      b"TemplatePage",
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
      "TemplatePage",
      "TemplatePage",
      tag_name_prefix,
      tag_name
    );
    let mut id = None;
    let mut name = None;
    let mut z_order = None;
    let mut base_loc = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"ID" => {
          id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "TemplatePage",
            "id",
          )?);
        }
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"ZOrder" => {
          z_order = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::document::TemplatePageZOrder>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::document::TemplatePageZOrder>()?
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("TemplatePage"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end(e.to_end().name())?;
        }
      }
    }
    let id = id.ok_or_else(|| crate::common::missing_field("TemplatePage", "id"))?;
    let base_loc =
      base_loc.ok_or_else(|| crate::common::missing_field("TemplatePage", "base_loc"))?;
    Ok(Self {
      id,
      name,
      z_order,
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
      "TemplatePage",
      "TemplatePage",
      tag_name_prefix,
      tag_name
    );
    let mut id = None;
    let mut name = None;
    let mut z_order = None;
    let mut base_loc = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"ID" => {
          id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "TemplatePage",
            "id",
          )?);
        }
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"ZOrder" => {
          z_order = Some(if let Some(value) = crate::common::attr_raw_value(&attr) {
            <crate::schemas::document::TemplatePageZOrder>::from_bytes(value)?
          } else {
            crate::common::decode_attr_value(&attr, xml_reader.decoder())?
              .parse::<crate::schemas::document::TemplatePageZOrder>()?
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("TemplatePage"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end_into(e.to_end().name(), buf)?;
        }
      }
    }
    let id = id.ok_or_else(|| crate::common::missing_field("TemplatePage", "id"))?;
    let base_loc =
      base_loc.ok_or_else(|| crate::common::missing_field("TemplatePage", "base_loc"))?;
    Ok(Self {
      id,
      name,
      z_order,
      base_loc,
    })
  }
}
impl std::str::FromStr for crate::schemas::document::CommonData {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CommonData", b"CommonData")
  }
}
impl crate::schemas::document::CommonData {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:CommonData",
      b"CommonData",
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
      "CommonData",
      "CommonData",
      tag_name_prefix,
      tag_name
    );
    let mut max_unit_id = None;
    let mut page_area = None;
    let mut public_res = vec![];
    let mut document_res = vec![];
    let mut template_page = vec![];
    let mut default_cs = None;
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CommonData"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:MaxUnitID" | b"MaxUnitID" => {
              let parsed_value = {
                if e_empty {
                  let value = String::new();
                  {
                    value.parse::<u32>().map_err(|_| {
                      crate::common::invalid_field_value("CommonData", "max_unit_id", value.clone())
                    })?
                  }
                } else {
                  let parse_bytes = |value: &[u8]| {
                    std::str::from_utf8(value)
                      .ok()
                      .and_then(|value| value.parse::<u32>().ok())
                      .ok_or_else(|| {
                        crate::common::invalid_field_value(
                          "CommonData",
                          "max_unit_id",
                          String::from_utf8_lossy(value).into_owned(),
                        )
                      })
                  };
                  let mut first_text = None;
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                          crate::common::push_xml_text(&mut value, text)?;
                        } else if value.is_some() {
                          crate::common::push_xml_text(&mut value, text)?;
                        } else {
                          first_text = Some(text);
                        }
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                        }
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CommonData",
                          "max_unit_id",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:MaxUnitID" || name == b"MaxUnitID" => {
                          break if let Some(first) = first_text {
                            match parse_bytes(first.as_ref()) {
                              Ok(value) => value,
                              Err(_) => {
                                let value = first.xml10_content()?.into_owned();
                                {
                                  value.parse::<u32>().map_err(|_| {
                                    crate::common::invalid_field_value(
                                      "CommonData",
                                      "max_unit_id",
                                      value.clone(),
                                    )
                                  })?
                                }
                              }
                            }
                          } else {
                            let value = value.unwrap_or_default();
                            match parse_bytes(value.as_bytes()) {
                              Ok(value) => value,
                              Err(_) => value.parse::<u32>().map_err(|_| {
                                crate::common::invalid_field_value(
                                  "CommonData",
                                  "max_unit_id",
                                  value.clone(),
                                )
                              })?,
                            }
                          };
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CommonData"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              max_unit_id = Some(parsed_value);
            }
            b"ofd:PageArea" | b"PageArea" => {
              page_area = Some(
                crate::schemas::definitions::CtPageArea::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:PageArea",
                  b"PageArea",
                )?,
              );
            }
            b"ofd:PublicRes" | b"PublicRes" => {
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
                          "CommonData",
                          "public_res",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:PublicRes" || name == b"PublicRes" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CommonData"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              public_res.push(parsed_value);
            }
            b"ofd:DocumentRes" | b"DocumentRes" => {
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
                          "CommonData",
                          "document_res",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:DocumentRes" || name == b"DocumentRes" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CommonData"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              document_res.push(parsed_value);
            }
            b"ofd:TemplatePage" | b"TemplatePage" => {
              template_page.push(
                crate::schemas::document::TemplatePage::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:TemplatePage",
                  b"TemplatePage",
                )?,
              );
            }
            b"ofd:DefaultCS" | b"DefaultCS" => {
              let parsed_value = {
                if e_empty {
                  let value = String::new();
                  {
                    value.parse::<u32>().map_err(|_| {
                      crate::common::invalid_field_value("CommonData", "default_cs", value.clone())
                    })?
                  }
                } else {
                  let parse_bytes = |value: &[u8]| {
                    std::str::from_utf8(value)
                      .ok()
                      .and_then(|value| value.parse::<u32>().ok())
                      .ok_or_else(|| {
                        crate::common::invalid_field_value(
                          "CommonData",
                          "default_cs",
                          String::from_utf8_lossy(value).into_owned(),
                        )
                      })
                  };
                  let mut first_text = None;
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                          crate::common::push_xml_text(&mut value, text)?;
                        } else if value.is_some() {
                          crate::common::push_xml_text(&mut value, text)?;
                        } else {
                          first_text = Some(text);
                        }
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                        }
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CommonData",
                          "default_cs",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:DefaultCS" || name == b"DefaultCS" => {
                          break if let Some(first) = first_text {
                            match parse_bytes(first.as_ref()) {
                              Ok(value) => value,
                              Err(_) => {
                                let value = first.xml10_content()?.into_owned();
                                {
                                  value.parse::<u32>().map_err(|_| {
                                    crate::common::invalid_field_value(
                                      "CommonData",
                                      "default_cs",
                                      value.clone(),
                                    )
                                  })?
                                }
                              }
                            }
                          } else {
                            let value = value.unwrap_or_default();
                            match parse_bytes(value.as_bytes()) {
                              Ok(value) => value,
                              Err(_) => value.parse::<u32>().map_err(|_| {
                                crate::common::invalid_field_value(
                                  "CommonData",
                                  "default_cs",
                                  value.clone(),
                                )
                              })?,
                            }
                          };
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CommonData"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              default_cs = Some(parsed_value);
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
    let max_unit_id =
      max_unit_id.ok_or_else(|| crate::common::missing_field("CommonData", "max_unit_id"))?;
    let page_area =
      page_area.ok_or_else(|| crate::common::missing_field("CommonData", "page_area"))?;
    Ok(Self {
      max_unit_id,
      page_area,
      public_res,
      document_res,
      template_page,
      default_cs,
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
      "CommonData",
      "CommonData",
      tag_name_prefix,
      tag_name
    );
    let mut max_unit_id = None;
    let mut page_area = None;
    let mut public_res = vec![];
    let mut document_res = vec![];
    let mut template_page = vec![];
    let mut default_cs = None;
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CommonData"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:MaxUnitID" | b"MaxUnitID" => {
              let parsed_value = {
                if e_empty {
                  let value = String::new();
                  {
                    value.parse::<u32>().map_err(|_| {
                      crate::common::invalid_field_value("CommonData", "max_unit_id", value.clone())
                    })?
                  }
                } else {
                  crate::common::read_text_parsed_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CommonData",
                      field: "max_unit_id",
                      tag_name_prefix: b"ofd:MaxUnitID",
                      tag_name: b"MaxUnitID",
                    },
                    |value| {
                      std::str::from_utf8(value)
                        .ok()
                        .and_then(|value| value.parse::<u32>().ok())
                        .ok_or_else(|| {
                          crate::common::invalid_field_value(
                            "CommonData",
                            "max_unit_id",
                            String::from_utf8_lossy(value).into_owned(),
                          )
                        })
                    },
                    |value| {
                      let value = value.to_string();
                      Ok({
                        value.parse::<u32>().map_err(|_| {
                          crate::common::invalid_field_value(
                            "CommonData",
                            "max_unit_id",
                            value.clone(),
                          )
                        })?
                      })
                    },
                  )?
                }
              };
              max_unit_id = Some(parsed_value);
            }
            b"ofd:PageArea" | b"PageArea" => {
              page_area = Some(
                crate::schemas::definitions::CtPageArea::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:PageArea",
                  b"PageArea",
                )?,
              );
            }
            b"ofd:PublicRes" | b"PublicRes" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CommonData",
                      field: "public_res",
                      tag_name_prefix: b"ofd:PublicRes",
                      tag_name: b"PublicRes",
                    },
                  )?
                }
              };
              public_res.push(parsed_value);
            }
            b"ofd:DocumentRes" | b"DocumentRes" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CommonData",
                      field: "document_res",
                      tag_name_prefix: b"ofd:DocumentRes",
                      tag_name: b"DocumentRes",
                    },
                  )?
                }
              };
              document_res.push(parsed_value);
            }
            b"ofd:TemplatePage" | b"TemplatePage" => {
              template_page.push(
                crate::schemas::document::TemplatePage::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:TemplatePage",
                  b"TemplatePage",
                )?,
              );
            }
            b"ofd:DefaultCS" | b"DefaultCS" => {
              let parsed_value = {
                if e_empty {
                  let value = String::new();
                  {
                    value.parse::<u32>().map_err(|_| {
                      crate::common::invalid_field_value("CommonData", "default_cs", value.clone())
                    })?
                  }
                } else {
                  crate::common::read_text_parsed_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CommonData",
                      field: "default_cs",
                      tag_name_prefix: b"ofd:DefaultCS",
                      tag_name: b"DefaultCS",
                    },
                    |value| {
                      std::str::from_utf8(value)
                        .ok()
                        .and_then(|value| value.parse::<u32>().ok())
                        .ok_or_else(|| {
                          crate::common::invalid_field_value(
                            "CommonData",
                            "default_cs",
                            String::from_utf8_lossy(value).into_owned(),
                          )
                        })
                    },
                    |value| {
                      let value = value.to_string();
                      Ok({
                        value.parse::<u32>().map_err(|_| {
                          crate::common::invalid_field_value(
                            "CommonData",
                            "default_cs",
                            value.clone(),
                          )
                        })?
                      })
                    },
                  )?
                }
              };
              default_cs = Some(parsed_value);
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
    let max_unit_id =
      max_unit_id.ok_or_else(|| crate::common::missing_field("CommonData", "max_unit_id"))?;
    let page_area =
      page_area.ok_or_else(|| crate::common::missing_field("CommonData", "page_area"))?;
    Ok(Self {
      max_unit_id,
      page_area,
      public_res,
      document_res,
      template_page,
      default_cs,
    })
  }
}
impl std::str::FromStr for crate::schemas::document::Page {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Page", b"Page")
  }
}
impl crate::schemas::document::Page {
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
    let (e, empty_tag) = crate::common::expect_event_start_slice!(
      xml_reader,
      xml_event,
      "Page",
      "Page",
      tag_name_prefix,
      tag_name
    );
    let mut id = None;
    let mut base_loc = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"ID" => {
          id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "Page",
            "id",
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Page"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end(e.to_end().name())?;
        }
      }
    }
    let id = id.ok_or_else(|| crate::common::missing_field("Page", "id"))?;
    let base_loc = base_loc.ok_or_else(|| crate::common::missing_field("Page", "base_loc"))?;
    Ok(Self { id, base_loc })
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
      "Page",
      "Page",
      tag_name_prefix,
      tag_name
    );
    let mut id = None;
    let mut base_loc = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"ID" => {
          id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "Page",
            "id",
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Page"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end_into(e.to_end().name(), buf)?;
        }
      }
    }
    let id = id.ok_or_else(|| crate::common::missing_field("Page", "id"))?;
    let base_loc = base_loc.ok_or_else(|| crate::common::missing_field("Page", "base_loc"))?;
    Ok(Self { id, base_loc })
  }
}
impl std::str::FromStr for crate::schemas::document::Pages {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Pages", b"Pages")
  }
}
impl crate::schemas::document::Pages {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:Pages", b"Pages")
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
      "Pages",
      "Pages",
      tag_name_prefix,
      tag_name
    );
    let mut page = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Pages"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Page" | b"Page" => {
              page.push(crate::schemas::document::Page::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Page",
                b"Page",
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
    Ok(Self { page })
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
      "Pages",
      "Pages",
      tag_name_prefix,
      tag_name
    );
    let mut page = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Pages"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Page" | b"Page" => {
              page.push(
                crate::schemas::document::Page::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Page",
                  b"Page",
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
    Ok(Self { page })
  }
}
impl std::str::FromStr for crate::schemas::document::Outlines {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Outlines", b"Outlines")
  }
}
impl crate::schemas::document::Outlines {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:Outlines",
      b"Outlines",
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
      "Outlines",
      "Outlines",
      tag_name_prefix,
      tag_name
    );
    let mut outline_elem = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Outlines"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:OutlineElem" | b"OutlineElem" => {
              outline_elem.push(
                crate::schemas::document::CtOutlineElem::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:OutlineElem",
                  b"OutlineElem",
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
    Ok(Self { outline_elem })
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
      "Outlines",
      "Outlines",
      tag_name_prefix,
      tag_name
    );
    let mut outline_elem = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Outlines"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:OutlineElem" | b"OutlineElem" => {
              outline_elem.push(
                crate::schemas::document::CtOutlineElem::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:OutlineElem",
                  b"OutlineElem",
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
    Ok(Self { outline_elem })
  }
}
impl std::str::FromStr for crate::schemas::document::Actions {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Actions", b"Actions")
  }
}
impl crate::schemas::document::Actions {
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
impl std::str::FromStr for crate::schemas::document::Bookmarks {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Bookmarks", b"Bookmarks")
  }
}
impl crate::schemas::document::Bookmarks {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:Bookmarks",
      b"Bookmarks",
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
      "Bookmarks",
      "Bookmarks",
      tag_name_prefix,
      tag_name
    );
    let mut bookmark = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Bookmarks"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Bookmark" | b"Bookmark" => {
              bookmark.push(
                crate::schemas::document::CtBookmark::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Bookmark",
                  b"Bookmark",
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
    Ok(Self { bookmark })
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
      "Bookmarks",
      "Bookmarks",
      tag_name_prefix,
      tag_name
    );
    let mut bookmark = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Bookmarks"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Bookmark" | b"Bookmark" => {
              bookmark.push(
                crate::schemas::document::CtBookmark::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Bookmark",
                  b"Bookmark",
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
    Ok(Self { bookmark })
  }
}
impl std::str::FromStr for crate::schemas::document::Document {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Document", b"Document")
  }
}
impl crate::schemas::document::Document {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:Document",
      b"Document",
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
      "Document",
      "Document",
      tag_name_prefix,
      tag_name
    );
    let mut common_data = None;
    let mut pages = None;
    let mut outlines = None;
    let mut permissions = None;
    let mut actions = None;
    let mut v_preferences = None;
    let mut bookmarks = None;
    let mut annotations = None;
    let mut custom_tags = None;
    let mut attachments = None;
    let mut extensions = None;
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Document"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:CommonData" | b"CommonData" => {
              common_data = Some(
                crate::schemas::document::CommonData::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:CommonData",
                  b"CommonData",
                )?,
              );
            }
            b"ofd:Pages" | b"Pages" => {
              pages = Some(crate::schemas::document::Pages::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Pages",
                b"Pages",
              )?);
            }
            b"ofd:Outlines" | b"Outlines" => {
              outlines = Some(crate::schemas::document::Outlines::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Outlines",
                b"Outlines",
              )?);
            }
            b"ofd:Permissions" | b"Permissions" => {
              permissions = Some(
                crate::schemas::document::CtPermission::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Permissions",
                  b"Permissions",
                )?,
              );
            }
            b"ofd:Actions" | b"Actions" => {
              actions = Some(crate::schemas::document::Actions::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Actions",
                b"Actions",
              )?);
            }
            b"ofd:VPreferences" | b"VPreferences" => {
              v_preferences = Some(
                crate::schemas::document::CtVPreferences::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:VPreferences",
                  b"VPreferences",
                )?,
              );
            }
            b"ofd:Bookmarks" | b"Bookmarks" => {
              bookmarks = Some(
                crate::schemas::document::Bookmarks::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Bookmarks",
                  b"Bookmarks",
                )?,
              );
            }
            b"ofd:Annotations" | b"Annotations" => {
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
                          "Document",
                          "annotations",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:Annotations" || name == b"Annotations" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("Document"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              annotations = Some(parsed_value);
            }
            b"ofd:CustomTags" | b"CustomTags" => {
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
                          "Document",
                          "custom_tags",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:CustomTags" || name == b"CustomTags" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("Document"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              custom_tags = Some(parsed_value);
            }
            b"ofd:Attachments" | b"Attachments" => {
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
                          "Document",
                          "attachments",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:Attachments" || name == b"Attachments" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("Document"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              attachments = Some(parsed_value);
            }
            b"ofd:Extensions" | b"Extensions" => {
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
                          "Document",
                          "extensions",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:Extensions" || name == b"Extensions" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("Document"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              extensions = Some(parsed_value);
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
    let common_data =
      common_data.ok_or_else(|| crate::common::missing_field("Document", "common_data"))?;
    let pages = pages.ok_or_else(|| crate::common::missing_field("Document", "pages"))?;
    Ok(Self {
      common_data,
      pages,
      outlines,
      permissions,
      actions,
      v_preferences,
      bookmarks,
      annotations,
      custom_tags,
      attachments,
      extensions,
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
      "Document",
      "Document",
      tag_name_prefix,
      tag_name
    );
    let mut common_data = None;
    let mut pages = None;
    let mut outlines = None;
    let mut permissions = None;
    let mut actions = None;
    let mut v_preferences = None;
    let mut bookmarks = None;
    let mut annotations = None;
    let mut custom_tags = None;
    let mut attachments = None;
    let mut extensions = None;
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Document"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:CommonData" | b"CommonData" => {
              common_data = Some(
                crate::schemas::document::CommonData::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:CommonData",
                  b"CommonData",
                )?,
              );
            }
            b"ofd:Pages" | b"Pages" => {
              pages = Some(
                crate::schemas::document::Pages::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Pages",
                  b"Pages",
                )?,
              );
            }
            b"ofd:Outlines" | b"Outlines" => {
              outlines = Some(
                crate::schemas::document::Outlines::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Outlines",
                  b"Outlines",
                )?,
              );
            }
            b"ofd:Permissions" | b"Permissions" => {
              permissions = Some(
                crate::schemas::document::CtPermission::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Permissions",
                  b"Permissions",
                )?,
              );
            }
            b"ofd:Actions" | b"Actions" => {
              actions = Some(
                crate::schemas::document::Actions::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Actions",
                  b"Actions",
                )?,
              );
            }
            b"ofd:VPreferences" | b"VPreferences" => {
              v_preferences = Some(
                crate::schemas::document::CtVPreferences::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:VPreferences",
                  b"VPreferences",
                )?,
              );
            }
            b"ofd:Bookmarks" | b"Bookmarks" => {
              bookmarks = Some(
                crate::schemas::document::Bookmarks::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Bookmarks",
                  b"Bookmarks",
                )?,
              );
            }
            b"ofd:Annotations" | b"Annotations" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "Document",
                      field: "annotations",
                      tag_name_prefix: b"ofd:Annotations",
                      tag_name: b"Annotations",
                    },
                  )?
                }
              };
              annotations = Some(parsed_value);
            }
            b"ofd:CustomTags" | b"CustomTags" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "Document",
                      field: "custom_tags",
                      tag_name_prefix: b"ofd:CustomTags",
                      tag_name: b"CustomTags",
                    },
                  )?
                }
              };
              custom_tags = Some(parsed_value);
            }
            b"ofd:Attachments" | b"Attachments" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "Document",
                      field: "attachments",
                      tag_name_prefix: b"ofd:Attachments",
                      tag_name: b"Attachments",
                    },
                  )?
                }
              };
              attachments = Some(parsed_value);
            }
            b"ofd:Extensions" | b"Extensions" => {
              let parsed_value = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "Document",
                      field: "extensions",
                      tag_name_prefix: b"ofd:Extensions",
                      tag_name: b"Extensions",
                    },
                  )?
                }
              };
              extensions = Some(parsed_value);
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
    let common_data =
      common_data.ok_or_else(|| crate::common::missing_field("Document", "common_data"))?;
    let pages = pages.ok_or_else(|| crate::common::missing_field("Document", "pages"))?;
    Ok(Self {
      common_data,
      pages,
      outlines,
      permissions,
      actions,
      v_preferences,
      bookmarks,
      annotations,
      custom_tags,
      attachments,
      extensions,
    })
  }
}
impl std::str::FromStr for crate::schemas::document::Print {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Print", b"Print")
  }
}
impl crate::schemas::document::Print {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:Print", b"Print")
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
      "Print",
      "Print",
      tag_name_prefix,
      tag_name
    );
    let mut printable = None;
    let mut copies = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Printable" => {
          printable = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "Print",
            "printable",
          )?);
        }
        b"Copies" => {
          copies = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "Print",
            "copies",
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Print"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end(e.to_end().name())?;
        }
      }
    }
    let printable = printable.ok_or_else(|| crate::common::missing_field("Print", "printable"))?;
    Ok(Self { printable, copies })
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
      "Print",
      "Print",
      tag_name_prefix,
      tag_name
    );
    let mut printable = None;
    let mut copies = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Printable" => {
          printable = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "Print",
            "printable",
          )?);
        }
        b"Copies" => {
          copies = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "Print",
            "copies",
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Print"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end_into(e.to_end().name(), buf)?;
        }
      }
    }
    let printable = printable.ok_or_else(|| crate::common::missing_field("Print", "printable"))?;
    Ok(Self { printable, copies })
  }
}
impl std::str::FromStr for crate::schemas::document::ValidPeriod {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:ValidPeriod", b"ValidPeriod")
  }
}
impl crate::schemas::document::ValidPeriod {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:ValidPeriod",
      b"ValidPeriod",
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
      "ValidPeriod",
      "ValidPeriod",
      tag_name_prefix,
      tag_name
    );
    let mut start_date = None;
    let mut end_date = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"StartDate" => {
          start_date =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"EndDate" => {
          end_date =
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("ValidPeriod"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end(e.to_end().name())?;
        }
      }
    }
    Ok(Self {
      start_date,
      end_date,
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
      "ValidPeriod",
      "ValidPeriod",
      tag_name_prefix,
      tag_name
    );
    let mut start_date = None;
    let mut end_date = None;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"StartDate" => {
          start_date =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"EndDate" => {
          end_date =
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("ValidPeriod"))?,
          _ => {}
        }
        if let Some(e) = e_opt
          && !e_empty
        {
          xml_reader.read_to_end_into(e.to_end().name(), buf)?;
        }
      }
    }
    Ok(Self {
      start_date,
      end_date,
    })
  }
}
impl std::str::FromStr for crate::schemas::document::CtPermission {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(
      &mut xml_reader,
      None,
      b"ofd:CT_Permission",
      b"CT_Permission",
    )
  }
}
impl crate::schemas::document::CtPermission {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:CT_Permission",
      b"CT_Permission",
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
      "CtPermission",
      "CT_Permission",
      tag_name_prefix,
      tag_name
    );
    let mut edit = None;
    let mut annot = None;
    let mut export = None;
    let mut signature = None;
    let mut watermark = None;
    let mut print_screen = None;
    let mut print = None;
    let mut valid_period = None;
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtPermission"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Edit" | b"Edit" => {
              let parsed_value = {
                if e_empty {
                  let value = String::new();
                  {
                    crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                      crate::common::invalid_field_value("CtPermission", "edit", value.clone())
                    })?
                  }
                } else {
                  let parse_bytes = |value: &[u8]| {
                    crate::common::parse_bool_bytes(value).map_err(|_| {
                      crate::common::invalid_field_value(
                        "CtPermission",
                        "edit",
                        String::from_utf8_lossy(value).into_owned(),
                      )
                    })
                  };
                  let mut first_text = None;
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                          crate::common::push_xml_text(&mut value, text)?;
                        } else if value.is_some() {
                          crate::common::push_xml_text(&mut value, text)?;
                        } else {
                          first_text = Some(text);
                        }
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                        }
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CtPermission",
                          "edit",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:Edit" || name == b"Edit" => {
                          break if let Some(first) = first_text {
                            match parse_bytes(first.as_ref()) {
                              Ok(value) => value,
                              Err(_) => {
                                let value = first.xml10_content()?.into_owned();
                                {
                                  crate::common::parse_bool_bytes(value.as_bytes()).map_err(
                                    |_| {
                                      crate::common::invalid_field_value(
                                        "CtPermission",
                                        "edit",
                                        value.clone(),
                                      )
                                    },
                                  )?
                                }
                              }
                            }
                          } else {
                            let value = value.unwrap_or_default();
                            match parse_bytes(value.as_bytes()) {
                              Ok(value) => value,
                              Err(_) => {
                                crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                                  crate::common::invalid_field_value(
                                    "CtPermission",
                                    "edit",
                                    value.clone(),
                                  )
                                })?
                              }
                            }
                          };
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtPermission"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              edit = Some(parsed_value);
            }
            b"ofd:Annot" | b"Annot" => {
              let parsed_value = {
                if e_empty {
                  let value = String::new();
                  {
                    crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                      crate::common::invalid_field_value("CtPermission", "annot", value.clone())
                    })?
                  }
                } else {
                  let parse_bytes = |value: &[u8]| {
                    crate::common::parse_bool_bytes(value).map_err(|_| {
                      crate::common::invalid_field_value(
                        "CtPermission",
                        "annot",
                        String::from_utf8_lossy(value).into_owned(),
                      )
                    })
                  };
                  let mut first_text = None;
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                          crate::common::push_xml_text(&mut value, text)?;
                        } else if value.is_some() {
                          crate::common::push_xml_text(&mut value, text)?;
                        } else {
                          first_text = Some(text);
                        }
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                        }
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CtPermission",
                          "annot",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:Annot" || name == b"Annot" => {
                          break if let Some(first) = first_text {
                            match parse_bytes(first.as_ref()) {
                              Ok(value) => value,
                              Err(_) => {
                                let value = first.xml10_content()?.into_owned();
                                {
                                  crate::common::parse_bool_bytes(value.as_bytes()).map_err(
                                    |_| {
                                      crate::common::invalid_field_value(
                                        "CtPermission",
                                        "annot",
                                        value.clone(),
                                      )
                                    },
                                  )?
                                }
                              }
                            }
                          } else {
                            let value = value.unwrap_or_default();
                            match parse_bytes(value.as_bytes()) {
                              Ok(value) => value,
                              Err(_) => {
                                crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                                  crate::common::invalid_field_value(
                                    "CtPermission",
                                    "annot",
                                    value.clone(),
                                  )
                                })?
                              }
                            }
                          };
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtPermission"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              annot = Some(parsed_value);
            }
            b"ofd:Export" | b"Export" => {
              let parsed_value = {
                if e_empty {
                  let value = String::new();
                  {
                    crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                      crate::common::invalid_field_value("CtPermission", "export", value.clone())
                    })?
                  }
                } else {
                  let parse_bytes = |value: &[u8]| {
                    crate::common::parse_bool_bytes(value).map_err(|_| {
                      crate::common::invalid_field_value(
                        "CtPermission",
                        "export",
                        String::from_utf8_lossy(value).into_owned(),
                      )
                    })
                  };
                  let mut first_text = None;
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                          crate::common::push_xml_text(&mut value, text)?;
                        } else if value.is_some() {
                          crate::common::push_xml_text(&mut value, text)?;
                        } else {
                          first_text = Some(text);
                        }
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                        }
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CtPermission",
                          "export",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:Export" || name == b"Export" => {
                          break if let Some(first) = first_text {
                            match parse_bytes(first.as_ref()) {
                              Ok(value) => value,
                              Err(_) => {
                                let value = first.xml10_content()?.into_owned();
                                {
                                  crate::common::parse_bool_bytes(value.as_bytes()).map_err(
                                    |_| {
                                      crate::common::invalid_field_value(
                                        "CtPermission",
                                        "export",
                                        value.clone(),
                                      )
                                    },
                                  )?
                                }
                              }
                            }
                          } else {
                            let value = value.unwrap_or_default();
                            match parse_bytes(value.as_bytes()) {
                              Ok(value) => value,
                              Err(_) => {
                                crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                                  crate::common::invalid_field_value(
                                    "CtPermission",
                                    "export",
                                    value.clone(),
                                  )
                                })?
                              }
                            }
                          };
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtPermission"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              export = Some(parsed_value);
            }
            b"ofd:Signature" | b"Signature" => {
              let parsed_value = {
                if e_empty {
                  let value = String::new();
                  {
                    crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                      crate::common::invalid_field_value("CtPermission", "signature", value.clone())
                    })?
                  }
                } else {
                  let parse_bytes = |value: &[u8]| {
                    crate::common::parse_bool_bytes(value).map_err(|_| {
                      crate::common::invalid_field_value(
                        "CtPermission",
                        "signature",
                        String::from_utf8_lossy(value).into_owned(),
                      )
                    })
                  };
                  let mut first_text = None;
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                          crate::common::push_xml_text(&mut value, text)?;
                        } else if value.is_some() {
                          crate::common::push_xml_text(&mut value, text)?;
                        } else {
                          first_text = Some(text);
                        }
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                        }
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CtPermission",
                          "signature",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:Signature" || name == b"Signature" => {
                          break if let Some(first) = first_text {
                            match parse_bytes(first.as_ref()) {
                              Ok(value) => value,
                              Err(_) => {
                                let value = first.xml10_content()?.into_owned();
                                {
                                  crate::common::parse_bool_bytes(value.as_bytes()).map_err(
                                    |_| {
                                      crate::common::invalid_field_value(
                                        "CtPermission",
                                        "signature",
                                        value.clone(),
                                      )
                                    },
                                  )?
                                }
                              }
                            }
                          } else {
                            let value = value.unwrap_or_default();
                            match parse_bytes(value.as_bytes()) {
                              Ok(value) => value,
                              Err(_) => {
                                crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                                  crate::common::invalid_field_value(
                                    "CtPermission",
                                    "signature",
                                    value.clone(),
                                  )
                                })?
                              }
                            }
                          };
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtPermission"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              signature = Some(parsed_value);
            }
            b"ofd:Watermark" | b"Watermark" => {
              let parsed_value = {
                if e_empty {
                  let value = String::new();
                  {
                    crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                      crate::common::invalid_field_value("CtPermission", "watermark", value.clone())
                    })?
                  }
                } else {
                  let parse_bytes = |value: &[u8]| {
                    crate::common::parse_bool_bytes(value).map_err(|_| {
                      crate::common::invalid_field_value(
                        "CtPermission",
                        "watermark",
                        String::from_utf8_lossy(value).into_owned(),
                      )
                    })
                  };
                  let mut first_text = None;
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                          crate::common::push_xml_text(&mut value, text)?;
                        } else if value.is_some() {
                          crate::common::push_xml_text(&mut value, text)?;
                        } else {
                          first_text = Some(text);
                        }
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                        }
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CtPermission",
                          "watermark",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:Watermark" || name == b"Watermark" => {
                          break if let Some(first) = first_text {
                            match parse_bytes(first.as_ref()) {
                              Ok(value) => value,
                              Err(_) => {
                                let value = first.xml10_content()?.into_owned();
                                {
                                  crate::common::parse_bool_bytes(value.as_bytes()).map_err(
                                    |_| {
                                      crate::common::invalid_field_value(
                                        "CtPermission",
                                        "watermark",
                                        value.clone(),
                                      )
                                    },
                                  )?
                                }
                              }
                            }
                          } else {
                            let value = value.unwrap_or_default();
                            match parse_bytes(value.as_bytes()) {
                              Ok(value) => value,
                              Err(_) => {
                                crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                                  crate::common::invalid_field_value(
                                    "CtPermission",
                                    "watermark",
                                    value.clone(),
                                  )
                                })?
                              }
                            }
                          };
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtPermission"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              watermark = Some(parsed_value);
            }
            b"ofd:PrintScreen" | b"PrintScreen" => {
              let parsed_value = {
                if e_empty {
                  let value = String::new();
                  {
                    crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                      crate::common::invalid_field_value(
                        "CtPermission",
                        "print_screen",
                        value.clone(),
                      )
                    })?
                  }
                } else {
                  let parse_bytes = |value: &[u8]| {
                    crate::common::parse_bool_bytes(value).map_err(|_| {
                      crate::common::invalid_field_value(
                        "CtPermission",
                        "print_screen",
                        String::from_utf8_lossy(value).into_owned(),
                      )
                    })
                  };
                  let mut first_text = None;
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                          crate::common::push_xml_text(&mut value, text)?;
                        } else if value.is_some() {
                          crate::common::push_xml_text(&mut value, text)?;
                        } else {
                          first_text = Some(text);
                        }
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                        }
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CtPermission",
                          "print_screen",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:PrintScreen" || name == b"PrintScreen" => {
                          break if let Some(first) = first_text {
                            match parse_bytes(first.as_ref()) {
                              Ok(value) => value,
                              Err(_) => {
                                let value = first.xml10_content()?.into_owned();
                                {
                                  crate::common::parse_bool_bytes(value.as_bytes()).map_err(
                                    |_| {
                                      crate::common::invalid_field_value(
                                        "CtPermission",
                                        "print_screen",
                                        value.clone(),
                                      )
                                    },
                                  )?
                                }
                              }
                            }
                          } else {
                            let value = value.unwrap_or_default();
                            match parse_bytes(value.as_bytes()) {
                              Ok(value) => value,
                              Err(_) => {
                                crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                                  crate::common::invalid_field_value(
                                    "CtPermission",
                                    "print_screen",
                                    value.clone(),
                                  )
                                })?
                              }
                            }
                          };
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtPermission"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              print_screen = Some(parsed_value);
            }
            b"ofd:Print" | b"Print" => {
              print = Some(crate::schemas::document::Print::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Print",
                b"Print",
              )?);
            }
            b"ofd:ValidPeriod" | b"ValidPeriod" => {
              valid_period = Some(
                crate::schemas::document::ValidPeriod::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:ValidPeriod",
                  b"ValidPeriod",
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
      edit,
      annot,
      export,
      signature,
      watermark,
      print_screen,
      print,
      valid_period,
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
      "CtPermission",
      "CT_Permission",
      tag_name_prefix,
      tag_name
    );
    let mut edit = None;
    let mut annot = None;
    let mut export = None;
    let mut signature = None;
    let mut watermark = None;
    let mut print_screen = None;
    let mut print = None;
    let mut valid_period = None;
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtPermission"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Edit" | b"Edit" => {
              let parsed_value = {
                if e_empty {
                  let value = String::new();
                  {
                    crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                      crate::common::invalid_field_value("CtPermission", "edit", value.clone())
                    })?
                  }
                } else {
                  crate::common::read_text_parsed_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtPermission",
                      field: "edit",
                      tag_name_prefix: b"ofd:Edit",
                      tag_name: b"Edit",
                    },
                    |value| {
                      crate::common::parse_bool_bytes(value).map_err(|_| {
                        crate::common::invalid_field_value(
                          "CtPermission",
                          "edit",
                          String::from_utf8_lossy(value).into_owned(),
                        )
                      })
                    },
                    |value| {
                      let value = value.to_string();
                      Ok({
                        crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                          crate::common::invalid_field_value("CtPermission", "edit", value.clone())
                        })?
                      })
                    },
                  )?
                }
              };
              edit = Some(parsed_value);
            }
            b"ofd:Annot" | b"Annot" => {
              let parsed_value = {
                if e_empty {
                  let value = String::new();
                  {
                    crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                      crate::common::invalid_field_value("CtPermission", "annot", value.clone())
                    })?
                  }
                } else {
                  crate::common::read_text_parsed_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtPermission",
                      field: "annot",
                      tag_name_prefix: b"ofd:Annot",
                      tag_name: b"Annot",
                    },
                    |value| {
                      crate::common::parse_bool_bytes(value).map_err(|_| {
                        crate::common::invalid_field_value(
                          "CtPermission",
                          "annot",
                          String::from_utf8_lossy(value).into_owned(),
                        )
                      })
                    },
                    |value| {
                      let value = value.to_string();
                      Ok({
                        crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                          crate::common::invalid_field_value("CtPermission", "annot", value.clone())
                        })?
                      })
                    },
                  )?
                }
              };
              annot = Some(parsed_value);
            }
            b"ofd:Export" | b"Export" => {
              let parsed_value = {
                if e_empty {
                  let value = String::new();
                  {
                    crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                      crate::common::invalid_field_value("CtPermission", "export", value.clone())
                    })?
                  }
                } else {
                  crate::common::read_text_parsed_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtPermission",
                      field: "export",
                      tag_name_prefix: b"ofd:Export",
                      tag_name: b"Export",
                    },
                    |value| {
                      crate::common::parse_bool_bytes(value).map_err(|_| {
                        crate::common::invalid_field_value(
                          "CtPermission",
                          "export",
                          String::from_utf8_lossy(value).into_owned(),
                        )
                      })
                    },
                    |value| {
                      let value = value.to_string();
                      Ok({
                        crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                          crate::common::invalid_field_value(
                            "CtPermission",
                            "export",
                            value.clone(),
                          )
                        })?
                      })
                    },
                  )?
                }
              };
              export = Some(parsed_value);
            }
            b"ofd:Signature" | b"Signature" => {
              let parsed_value = {
                if e_empty {
                  let value = String::new();
                  {
                    crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                      crate::common::invalid_field_value("CtPermission", "signature", value.clone())
                    })?
                  }
                } else {
                  crate::common::read_text_parsed_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtPermission",
                      field: "signature",
                      tag_name_prefix: b"ofd:Signature",
                      tag_name: b"Signature",
                    },
                    |value| {
                      crate::common::parse_bool_bytes(value).map_err(|_| {
                        crate::common::invalid_field_value(
                          "CtPermission",
                          "signature",
                          String::from_utf8_lossy(value).into_owned(),
                        )
                      })
                    },
                    |value| {
                      let value = value.to_string();
                      Ok({
                        crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                          crate::common::invalid_field_value(
                            "CtPermission",
                            "signature",
                            value.clone(),
                          )
                        })?
                      })
                    },
                  )?
                }
              };
              signature = Some(parsed_value);
            }
            b"ofd:Watermark" | b"Watermark" => {
              let parsed_value = {
                if e_empty {
                  let value = String::new();
                  {
                    crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                      crate::common::invalid_field_value("CtPermission", "watermark", value.clone())
                    })?
                  }
                } else {
                  crate::common::read_text_parsed_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtPermission",
                      field: "watermark",
                      tag_name_prefix: b"ofd:Watermark",
                      tag_name: b"Watermark",
                    },
                    |value| {
                      crate::common::parse_bool_bytes(value).map_err(|_| {
                        crate::common::invalid_field_value(
                          "CtPermission",
                          "watermark",
                          String::from_utf8_lossy(value).into_owned(),
                        )
                      })
                    },
                    |value| {
                      let value = value.to_string();
                      Ok({
                        crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                          crate::common::invalid_field_value(
                            "CtPermission",
                            "watermark",
                            value.clone(),
                          )
                        })?
                      })
                    },
                  )?
                }
              };
              watermark = Some(parsed_value);
            }
            b"ofd:PrintScreen" | b"PrintScreen" => {
              let parsed_value = {
                if e_empty {
                  let value = String::new();
                  {
                    crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                      crate::common::invalid_field_value(
                        "CtPermission",
                        "print_screen",
                        value.clone(),
                      )
                    })?
                  }
                } else {
                  crate::common::read_text_parsed_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtPermission",
                      field: "print_screen",
                      tag_name_prefix: b"ofd:PrintScreen",
                      tag_name: b"PrintScreen",
                    },
                    |value| {
                      crate::common::parse_bool_bytes(value).map_err(|_| {
                        crate::common::invalid_field_value(
                          "CtPermission",
                          "print_screen",
                          String::from_utf8_lossy(value).into_owned(),
                        )
                      })
                    },
                    |value| {
                      let value = value.to_string();
                      Ok({
                        crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                          crate::common::invalid_field_value(
                            "CtPermission",
                            "print_screen",
                            value.clone(),
                          )
                        })?
                      })
                    },
                  )?
                }
              };
              print_screen = Some(parsed_value);
            }
            b"ofd:Print" | b"Print" => {
              print = Some(
                crate::schemas::document::Print::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Print",
                  b"Print",
                )?,
              );
            }
            b"ofd:ValidPeriod" | b"ValidPeriod" => {
              valid_period = Some(
                crate::schemas::document::ValidPeriod::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:ValidPeriod",
                  b"ValidPeriod",
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
      edit,
      annot,
      export,
      signature,
      watermark,
      print_screen,
      print,
      valid_period,
    })
  }
}
impl std::str::FromStr for crate::schemas::document::CtVPreferences {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(
      &mut xml_reader,
      None,
      b"ofd:CT_VPreferences",
      b"CT_VPreferences",
    )
  }
}
impl crate::schemas::document::CtVPreferences {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:CT_VPreferences",
      b"CT_VPreferences",
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
      "CtVPreferences",
      "CT_VPreferences",
      tag_name_prefix,
      tag_name
    );
    let mut page_mode = None;
    let mut page_layout = None;
    let mut tab_display = None;
    let mut hide_toolbar = None;
    let mut hide_menubar = None;
    let mut hide_window_ui = None;
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtVPreferences"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:ZoomMode" | b"ZoomMode" => {
              let parsed_value: crate::schemas::document::ZoomMode = {
                if e_empty {
                  <crate::schemas::document::ZoomMode>::from_bytes(b"")?
                } else {
                  let mut first_text = None;
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                          crate::common::push_xml_text(&mut value, text)?;
                        } else if value.is_some() {
                          crate::common::push_xml_text(&mut value, text)?;
                        } else {
                          first_text = Some(text);
                        }
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                        }
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CtVPreferences",
                          "ZoomMode",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:ZoomMode" || name == b"ZoomMode" => {
                          break if let Some(first) = first_text {
                            match <crate::schemas::document::ZoomMode>::from_bytes(first.as_ref()) {
                              Ok(value) => value,
                              Err(_) => first
                                .xml10_content()?
                                .as_ref()
                                .parse::<crate::schemas::document::ZoomMode>()?,
                            }
                          } else {
                            value
                              .unwrap_or_default()
                              .parse::<crate::schemas::document::ZoomMode>()?
                          };
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtVPreferences"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              xml_children.push(
                crate::schemas::document::CtVPreferencesContentChoice::ZoomMode(Box::new(
                  parsed_value,
                )),
              );
            }
            b"ofd:Zoom" | b"Zoom" => {
              let parsed_value: f64 = {
                if e_empty {
                  let value = String::new();
                  {
                    value.parse::<f64>().map_err(|_| {
                      crate::common::invalid_field_value("CtVPreferences", "Zoom", value.clone())
                    })?
                  }
                } else {
                  let parse_bytes = |value: &[u8]| {
                    std::str::from_utf8(value)
                      .ok()
                      .and_then(|value| value.parse::<f64>().ok())
                      .ok_or_else(|| {
                        crate::common::invalid_field_value(
                          "CtVPreferences",
                          "Zoom",
                          String::from_utf8_lossy(value).into_owned(),
                        )
                      })
                  };
                  let mut first_text = None;
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                          crate::common::push_xml_text(&mut value, text)?;
                        } else if value.is_some() {
                          crate::common::push_xml_text(&mut value, text)?;
                        } else {
                          first_text = Some(text);
                        }
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                        }
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CtVPreferences",
                          "Zoom",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:Zoom" || name == b"Zoom" => {
                          break if let Some(first) = first_text {
                            match parse_bytes(first.as_ref()) {
                              Ok(value) => value,
                              Err(_) => {
                                let value = first.xml10_content()?.into_owned();
                                {
                                  value.parse::<f64>().map_err(|_| {
                                    crate::common::invalid_field_value(
                                      "CtVPreferences",
                                      "Zoom",
                                      value.clone(),
                                    )
                                  })?
                                }
                              }
                            }
                          } else {
                            let value = value.unwrap_or_default();
                            match parse_bytes(value.as_bytes()) {
                              Ok(value) => value,
                              Err(_) => value.parse::<f64>().map_err(|_| {
                                crate::common::invalid_field_value(
                                  "CtVPreferences",
                                  "Zoom",
                                  value.clone(),
                                )
                              })?,
                            }
                          };
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtVPreferences"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              xml_children.push(crate::schemas::document::CtVPreferencesContentChoice::Zoom(
                Box::new(parsed_value),
              ));
            }
            b"ofd:PageMode" | b"PageMode" => {
              let parsed_value = {
                if e_empty {
                  <crate::schemas::document::PageMode>::from_bytes(b"")?
                } else {
                  let mut first_text = None;
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                          crate::common::push_xml_text(&mut value, text)?;
                        } else if value.is_some() {
                          crate::common::push_xml_text(&mut value, text)?;
                        } else {
                          first_text = Some(text);
                        }
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                        }
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CtVPreferences",
                          "page_mode",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:PageMode" || name == b"PageMode" => {
                          break if let Some(first) = first_text {
                            match <crate::schemas::document::PageMode>::from_bytes(first.as_ref()) {
                              Ok(value) => value,
                              Err(_) => first
                                .xml10_content()?
                                .as_ref()
                                .parse::<crate::schemas::document::PageMode>()?,
                            }
                          } else {
                            value
                              .unwrap_or_default()
                              .parse::<crate::schemas::document::PageMode>()?
                          };
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtVPreferences"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              page_mode = Some(parsed_value);
            }
            b"ofd:PageLayout" | b"PageLayout" => {
              let parsed_value = {
                if e_empty {
                  <crate::schemas::document::PageLayout>::from_bytes(b"")?
                } else {
                  let mut first_text = None;
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                          crate::common::push_xml_text(&mut value, text)?;
                        } else if value.is_some() {
                          crate::common::push_xml_text(&mut value, text)?;
                        } else {
                          first_text = Some(text);
                        }
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                        }
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CtVPreferences",
                          "page_layout",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:PageLayout" || name == b"PageLayout" => {
                          break if let Some(first) = first_text {
                            match <crate::schemas::document::PageLayout>::from_bytes(first.as_ref())
                            {
                              Ok(value) => value,
                              Err(_) => first
                                .xml10_content()?
                                .as_ref()
                                .parse::<crate::schemas::document::PageLayout>()?,
                            }
                          } else {
                            value
                              .unwrap_or_default()
                              .parse::<crate::schemas::document::PageLayout>()?
                          };
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtVPreferences"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              page_layout = Some(parsed_value);
            }
            b"ofd:TabDisplay" | b"TabDisplay" => {
              let parsed_value = {
                if e_empty {
                  <crate::schemas::document::TabDisplay>::from_bytes(b"")?
                } else {
                  let mut first_text = None;
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                          crate::common::push_xml_text(&mut value, text)?;
                        } else if value.is_some() {
                          crate::common::push_xml_text(&mut value, text)?;
                        } else {
                          first_text = Some(text);
                        }
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                        }
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CtVPreferences",
                          "tab_display",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:TabDisplay" || name == b"TabDisplay" => {
                          break if let Some(first) = first_text {
                            match <crate::schemas::document::TabDisplay>::from_bytes(first.as_ref())
                            {
                              Ok(value) => value,
                              Err(_) => first
                                .xml10_content()?
                                .as_ref()
                                .parse::<crate::schemas::document::TabDisplay>()?,
                            }
                          } else {
                            value
                              .unwrap_or_default()
                              .parse::<crate::schemas::document::TabDisplay>()?
                          };
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtVPreferences"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              tab_display = Some(parsed_value);
            }
            b"ofd:HideToolbar" | b"HideToolbar" => {
              let parsed_value = {
                if e_empty {
                  let value = String::new();
                  {
                    crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                      crate::common::invalid_field_value(
                        "CtVPreferences",
                        "hide_toolbar",
                        value.clone(),
                      )
                    })?
                  }
                } else {
                  let parse_bytes = |value: &[u8]| {
                    crate::common::parse_bool_bytes(value).map_err(|_| {
                      crate::common::invalid_field_value(
                        "CtVPreferences",
                        "hide_toolbar",
                        String::from_utf8_lossy(value).into_owned(),
                      )
                    })
                  };
                  let mut first_text = None;
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                          crate::common::push_xml_text(&mut value, text)?;
                        } else if value.is_some() {
                          crate::common::push_xml_text(&mut value, text)?;
                        } else {
                          first_text = Some(text);
                        }
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                        }
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CtVPreferences",
                          "hide_toolbar",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:HideToolbar" || name == b"HideToolbar" => {
                          break if let Some(first) = first_text {
                            match parse_bytes(first.as_ref()) {
                              Ok(value) => value,
                              Err(_) => {
                                let value = first.xml10_content()?.into_owned();
                                {
                                  crate::common::parse_bool_bytes(value.as_bytes()).map_err(
                                    |_| {
                                      crate::common::invalid_field_value(
                                        "CtVPreferences",
                                        "hide_toolbar",
                                        value.clone(),
                                      )
                                    },
                                  )?
                                }
                              }
                            }
                          } else {
                            let value = value.unwrap_or_default();
                            match parse_bytes(value.as_bytes()) {
                              Ok(value) => value,
                              Err(_) => {
                                crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                                  crate::common::invalid_field_value(
                                    "CtVPreferences",
                                    "hide_toolbar",
                                    value.clone(),
                                  )
                                })?
                              }
                            }
                          };
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtVPreferences"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              hide_toolbar = Some(parsed_value);
            }
            b"ofd:HideMenubar" | b"HideMenubar" => {
              let parsed_value = {
                if e_empty {
                  let value = String::new();
                  {
                    crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                      crate::common::invalid_field_value(
                        "CtVPreferences",
                        "hide_menubar",
                        value.clone(),
                      )
                    })?
                  }
                } else {
                  let parse_bytes = |value: &[u8]| {
                    crate::common::parse_bool_bytes(value).map_err(|_| {
                      crate::common::invalid_field_value(
                        "CtVPreferences",
                        "hide_menubar",
                        String::from_utf8_lossy(value).into_owned(),
                      )
                    })
                  };
                  let mut first_text = None;
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                          crate::common::push_xml_text(&mut value, text)?;
                        } else if value.is_some() {
                          crate::common::push_xml_text(&mut value, text)?;
                        } else {
                          first_text = Some(text);
                        }
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                        }
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CtVPreferences",
                          "hide_menubar",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:HideMenubar" || name == b"HideMenubar" => {
                          break if let Some(first) = first_text {
                            match parse_bytes(first.as_ref()) {
                              Ok(value) => value,
                              Err(_) => {
                                let value = first.xml10_content()?.into_owned();
                                {
                                  crate::common::parse_bool_bytes(value.as_bytes()).map_err(
                                    |_| {
                                      crate::common::invalid_field_value(
                                        "CtVPreferences",
                                        "hide_menubar",
                                        value.clone(),
                                      )
                                    },
                                  )?
                                }
                              }
                            }
                          } else {
                            let value = value.unwrap_or_default();
                            match parse_bytes(value.as_bytes()) {
                              Ok(value) => value,
                              Err(_) => {
                                crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                                  crate::common::invalid_field_value(
                                    "CtVPreferences",
                                    "hide_menubar",
                                    value.clone(),
                                  )
                                })?
                              }
                            }
                          };
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtVPreferences"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              hide_menubar = Some(parsed_value);
            }
            b"ofd:HideWindowUI" | b"HideWindowUI" => {
              let parsed_value = {
                if e_empty {
                  let value = String::new();
                  {
                    crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                      crate::common::invalid_field_value(
                        "CtVPreferences",
                        "hide_window_ui",
                        value.clone(),
                      )
                    })?
                  }
                } else {
                  let parse_bytes = |value: &[u8]| {
                    crate::common::parse_bool_bytes(value).map_err(|_| {
                      crate::common::invalid_field_value(
                        "CtVPreferences",
                        "hide_window_ui",
                        String::from_utf8_lossy(value).into_owned(),
                      )
                    })
                  };
                  let mut first_text = None;
                  let mut value = None;
                  loop {
                    match xml_reader.read_event()? {
                      quick_xml::events::Event::Text(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                          crate::common::push_xml_text(&mut value, text)?;
                        } else if value.is_some() {
                          crate::common::push_xml_text(&mut value, text)?;
                        } else {
                          first_text = Some(text);
                        }
                      }
                      quick_xml::events::Event::GeneralRef(text) => {
                        if let Some(first) = first_text.take() {
                          crate::common::push_xml_text(&mut value, first)?;
                        }
                        crate::common::push_xml_general_ref(
                          &mut value,
                          text,
                          "CtVPreferences",
                          "hide_window_ui",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:HideWindowUI" || name == b"HideWindowUI" => {
                          break if let Some(first) = first_text {
                            match parse_bytes(first.as_ref()) {
                              Ok(value) => value,
                              Err(_) => {
                                let value = first.xml10_content()?.into_owned();
                                {
                                  crate::common::parse_bool_bytes(value.as_bytes()).map_err(
                                    |_| {
                                      crate::common::invalid_field_value(
                                        "CtVPreferences",
                                        "hide_window_ui",
                                        value.clone(),
                                      )
                                    },
                                  )?
                                }
                              }
                            }
                          } else {
                            let value = value.unwrap_or_default();
                            match parse_bytes(value.as_bytes()) {
                              Ok(value) => value,
                              Err(_) => {
                                crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                                  crate::common::invalid_field_value(
                                    "CtVPreferences",
                                    "hide_window_ui",
                                    value.clone(),
                                  )
                                })?
                              }
                            }
                          };
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtVPreferences"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              hide_window_ui = Some(parsed_value);
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
      page_mode,
      page_layout,
      tab_display,
      hide_toolbar,
      hide_menubar,
      hide_window_ui,
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
    let (_e, empty_tag) = crate::common::expect_event_start_io!(
      xml_reader,
      buf,
      xml_event,
      "CtVPreferences",
      "CT_VPreferences",
      tag_name_prefix,
      tag_name
    );
    let mut page_mode = None;
    let mut page_layout = None;
    let mut tab_display = None;
    let mut hide_toolbar = None;
    let mut hide_menubar = None;
    let mut hide_window_ui = None;
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtVPreferences"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:ZoomMode" | b"ZoomMode" => {
              let parsed_value: crate::schemas::document::ZoomMode = {
                if e_empty {
                  <crate::schemas::document::ZoomMode>::from_bytes(b"")?
                } else {
                  crate::common::read_text_enum_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtVPreferences",
                      field: "ZoomMode",
                      tag_name_prefix: b"ofd:ZoomMode",
                      tag_name: b"ZoomMode",
                    },
                    <crate::schemas::document::ZoomMode>::from_bytes,
                  )?
                }
              };
              xml_children.push(
                crate::schemas::document::CtVPreferencesContentChoice::ZoomMode(Box::new(
                  parsed_value,
                )),
              );
            }
            b"ofd:Zoom" | b"Zoom" => {
              let parsed_value: f64 = {
                if e_empty {
                  let value = String::new();
                  {
                    value.parse::<f64>().map_err(|_| {
                      crate::common::invalid_field_value("CtVPreferences", "Zoom", value.clone())
                    })?
                  }
                } else {
                  crate::common::read_text_parsed_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtVPreferences",
                      field: "Zoom",
                      tag_name_prefix: b"ofd:Zoom",
                      tag_name: b"Zoom",
                    },
                    |value| {
                      std::str::from_utf8(value)
                        .ok()
                        .and_then(|value| value.parse::<f64>().ok())
                        .ok_or_else(|| {
                          crate::common::invalid_field_value(
                            "CtVPreferences",
                            "Zoom",
                            String::from_utf8_lossy(value).into_owned(),
                          )
                        })
                    },
                    |value| {
                      let value = value.to_string();
                      Ok({
                        value.parse::<f64>().map_err(|_| {
                          crate::common::invalid_field_value(
                            "CtVPreferences",
                            "Zoom",
                            value.clone(),
                          )
                        })?
                      })
                    },
                  )?
                }
              };
              xml_children.push(crate::schemas::document::CtVPreferencesContentChoice::Zoom(
                Box::new(parsed_value),
              ));
            }
            b"ofd:PageMode" | b"PageMode" => {
              let parsed_value = {
                if e_empty {
                  <crate::schemas::document::PageMode>::from_bytes(b"")?
                } else {
                  crate::common::read_text_enum_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtVPreferences",
                      field: "page_mode",
                      tag_name_prefix: b"ofd:PageMode",
                      tag_name: b"PageMode",
                    },
                    <crate::schemas::document::PageMode>::from_bytes,
                  )?
                }
              };
              page_mode = Some(parsed_value);
            }
            b"ofd:PageLayout" | b"PageLayout" => {
              let parsed_value = {
                if e_empty {
                  <crate::schemas::document::PageLayout>::from_bytes(b"")?
                } else {
                  crate::common::read_text_enum_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtVPreferences",
                      field: "page_layout",
                      tag_name_prefix: b"ofd:PageLayout",
                      tag_name: b"PageLayout",
                    },
                    <crate::schemas::document::PageLayout>::from_bytes,
                  )?
                }
              };
              page_layout = Some(parsed_value);
            }
            b"ofd:TabDisplay" | b"TabDisplay" => {
              let parsed_value = {
                if e_empty {
                  <crate::schemas::document::TabDisplay>::from_bytes(b"")?
                } else {
                  crate::common::read_text_enum_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtVPreferences",
                      field: "tab_display",
                      tag_name_prefix: b"ofd:TabDisplay",
                      tag_name: b"TabDisplay",
                    },
                    <crate::schemas::document::TabDisplay>::from_bytes,
                  )?
                }
              };
              tab_display = Some(parsed_value);
            }
            b"ofd:HideToolbar" | b"HideToolbar" => {
              let parsed_value = {
                if e_empty {
                  let value = String::new();
                  {
                    crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                      crate::common::invalid_field_value(
                        "CtVPreferences",
                        "hide_toolbar",
                        value.clone(),
                      )
                    })?
                  }
                } else {
                  crate::common::read_text_parsed_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtVPreferences",
                      field: "hide_toolbar",
                      tag_name_prefix: b"ofd:HideToolbar",
                      tag_name: b"HideToolbar",
                    },
                    |value| {
                      crate::common::parse_bool_bytes(value).map_err(|_| {
                        crate::common::invalid_field_value(
                          "CtVPreferences",
                          "hide_toolbar",
                          String::from_utf8_lossy(value).into_owned(),
                        )
                      })
                    },
                    |value| {
                      let value = value.to_string();
                      Ok({
                        crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                          crate::common::invalid_field_value(
                            "CtVPreferences",
                            "hide_toolbar",
                            value.clone(),
                          )
                        })?
                      })
                    },
                  )?
                }
              };
              hide_toolbar = Some(parsed_value);
            }
            b"ofd:HideMenubar" | b"HideMenubar" => {
              let parsed_value = {
                if e_empty {
                  let value = String::new();
                  {
                    crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                      crate::common::invalid_field_value(
                        "CtVPreferences",
                        "hide_menubar",
                        value.clone(),
                      )
                    })?
                  }
                } else {
                  crate::common::read_text_parsed_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtVPreferences",
                      field: "hide_menubar",
                      tag_name_prefix: b"ofd:HideMenubar",
                      tag_name: b"HideMenubar",
                    },
                    |value| {
                      crate::common::parse_bool_bytes(value).map_err(|_| {
                        crate::common::invalid_field_value(
                          "CtVPreferences",
                          "hide_menubar",
                          String::from_utf8_lossy(value).into_owned(),
                        )
                      })
                    },
                    |value| {
                      let value = value.to_string();
                      Ok({
                        crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                          crate::common::invalid_field_value(
                            "CtVPreferences",
                            "hide_menubar",
                            value.clone(),
                          )
                        })?
                      })
                    },
                  )?
                }
              };
              hide_menubar = Some(parsed_value);
            }
            b"ofd:HideWindowUI" | b"HideWindowUI" => {
              let parsed_value = {
                if e_empty {
                  let value = String::new();
                  {
                    crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                      crate::common::invalid_field_value(
                        "CtVPreferences",
                        "hide_window_ui",
                        value.clone(),
                      )
                    })?
                  }
                } else {
                  crate::common::read_text_parsed_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtVPreferences",
                      field: "hide_window_ui",
                      tag_name_prefix: b"ofd:HideWindowUI",
                      tag_name: b"HideWindowUI",
                    },
                    |value| {
                      crate::common::parse_bool_bytes(value).map_err(|_| {
                        crate::common::invalid_field_value(
                          "CtVPreferences",
                          "hide_window_ui",
                          String::from_utf8_lossy(value).into_owned(),
                        )
                      })
                    },
                    |value| {
                      let value = value.to_string();
                      Ok({
                        crate::common::parse_bool_bytes(value.as_bytes()).map_err(|_| {
                          crate::common::invalid_field_value(
                            "CtVPreferences",
                            "hide_window_ui",
                            value.clone(),
                          )
                        })?
                      })
                    },
                  )?
                }
              };
              hide_window_ui = Some(parsed_value);
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
      page_mode,
      page_layout,
      tab_display,
      hide_toolbar,
      hide_menubar,
      hide_window_ui,
      xml_children,
    })
  }
}
impl std::str::FromStr for crate::schemas::document::CtOutlineElem {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(
      &mut xml_reader,
      None,
      b"ofd:CT_OutlineElem",
      b"CT_OutlineElem",
    )
  }
}
impl crate::schemas::document::CtOutlineElem {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:CT_OutlineElem",
      b"CT_OutlineElem",
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
      "CtOutlineElem",
      "CT_OutlineElem",
      tag_name_prefix,
      tag_name
    );
    let mut title = None;
    let mut count = None;
    let mut expanded = None;
    let mut actions = None;
    let mut outline_elem = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Title" => {
          title = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Count" => {
          count = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtOutlineElem",
            "count",
          )?);
        }
        b"Expanded" => {
          expanded = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtOutlineElem",
            "expanded",
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtOutlineElem"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Actions" | b"Actions" => {
              actions = Some(crate::schemas::document::Actions::deserialize_inner_named(
                xml_reader,
                Some((e, e_empty)),
                b"ofd:Actions",
                b"Actions",
              )?);
            }
            b"ofd:OutlineElem" | b"OutlineElem" => {
              outline_elem.push(
                crate::schemas::document::CtOutlineElem::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:OutlineElem",
                  b"OutlineElem",
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
    let title = title.ok_or_else(|| crate::common::missing_field("CtOutlineElem", "title"))?;
    Ok(Self {
      title,
      count,
      expanded,
      actions,
      outline_elem,
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
      "CtOutlineElem",
      "CT_OutlineElem",
      tag_name_prefix,
      tag_name
    );
    let mut title = None;
    let mut count = None;
    let mut expanded = None;
    let mut actions = None;
    let mut outline_elem = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Title" => {
          title = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Count" => {
          count = Some(crate::common::parse_i32_attr(
            &attr,
            xml_reader.decoder(),
            "CtOutlineElem",
            "count",
          )?);
        }
        b"Expanded" => {
          expanded = Some(crate::common::parse_bool_attr(
            &attr,
            xml_reader.decoder(),
            "CtOutlineElem",
            "expanded",
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtOutlineElem"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Actions" | b"Actions" => {
              actions = Some(
                crate::schemas::document::Actions::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Actions",
                  b"Actions",
                )?,
              );
            }
            b"ofd:OutlineElem" | b"OutlineElem" => {
              outline_elem.push(
                crate::schemas::document::CtOutlineElem::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:OutlineElem",
                  b"OutlineElem",
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
    let title = title.ok_or_else(|| crate::common::missing_field("CtOutlineElem", "title"))?;
    Ok(Self {
      title,
      count,
      expanded,
      actions,
      outline_elem,
    })
  }
}
impl std::str::FromStr for crate::schemas::document::CtBookmark {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_Bookmark", b"CT_Bookmark")
  }
}
impl crate::schemas::document::CtBookmark {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:CT_Bookmark",
      b"CT_Bookmark",
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
      "CtBookmark",
      "CT_Bookmark",
      tag_name_prefix,
      tag_name
    );
    let mut name = None;
    let mut dest = None;
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
          quick_xml::events::Event::End(e) => match e.name().as_ref() {
            name if name == tag_name_prefix || name == tag_name => {
              break;
            }
            _ => {}
          },
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtBookmark"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Dest" | b"Dest" => {
              dest = Some(
                crate::schemas::definitions::CtDest::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Dest",
                  b"Dest",
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
    let name = name.ok_or_else(|| crate::common::missing_field("CtBookmark", "name"))?;
    let dest = dest.ok_or_else(|| crate::common::missing_field("CtBookmark", "dest"))?;
    Ok(Self { name, dest })
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
      "CtBookmark",
      "CT_Bookmark",
      tag_name_prefix,
      tag_name
    );
    let mut name = None;
    let mut dest = None;
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
          quick_xml::events::Event::End(e) => match e.name().as_ref() {
            name if name == tag_name_prefix || name == tag_name => {
              break;
            }
            _ => {}
          },
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtBookmark"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Dest" | b"Dest" => {
              dest = Some(
                crate::schemas::definitions::CtDest::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Dest",
                  b"Dest",
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
    let name = name.ok_or_else(|| crate::common::missing_field("CtBookmark", "name"))?;
    let dest = dest.ok_or_else(|| crate::common::missing_field("CtBookmark", "dest"))?;
    Ok(Self { name, dest })
  }
}
impl std::str::FromStr for crate::schemas::document::PageArea {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:PageArea", b"PageArea")
  }
}
impl crate::schemas::document::PageArea {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:PageArea",
      b"PageArea",
    )
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::definitions::CtPageArea>::deserialize_inner_named(
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
      <crate::schemas::definitions::CtPageArea>::deserialize_from_reader_named(
        xml_reader,
        buf,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
impl std::str::FromStr for crate::schemas::document::OutlineElem {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:OutlineElem", b"OutlineElem")
  }
}
impl crate::schemas::document::OutlineElem {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:OutlineElem",
      b"OutlineElem",
    )
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::document::CtOutlineElem>::deserialize_inner_named(
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
      <crate::schemas::document::CtOutlineElem>::deserialize_from_reader_named(
        xml_reader,
        buf,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
impl std::str::FromStr for crate::schemas::document::Permissions {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Permissions", b"Permissions")
  }
}
impl crate::schemas::document::Permissions {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:Permissions",
      b"Permissions",
    )
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::document::CtPermission>::deserialize_inner_named(
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
      <crate::schemas::document::CtPermission>::deserialize_from_reader_named(
        xml_reader,
        buf,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
impl std::str::FromStr for crate::schemas::document::Action {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Action", b"Action")
  }
}
impl crate::schemas::document::Action {
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
impl std::str::FromStr for crate::schemas::document::VPreferences {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:VPreferences", b"VPreferences")
  }
}
impl crate::schemas::document::VPreferences {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:VPreferences",
      b"VPreferences",
    )
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::document::CtVPreferences>::deserialize_inner_named(
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
      <crate::schemas::document::CtVPreferences>::deserialize_from_reader_named(
        xml_reader,
        buf,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
impl std::str::FromStr for crate::schemas::document::Bookmark {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Bookmark", b"Bookmark")
  }
}
impl crate::schemas::document::Bookmark {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:Bookmark",
      b"Bookmark",
    )
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::document::CtBookmark>::deserialize_inner_named(
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
      <crate::schemas::document::CtBookmark>::deserialize_from_reader_named(
        xml_reader,
        buf,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
impl std::str::FromStr for crate::schemas::document::Dest {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Dest", b"Dest")
  }
}
impl crate::schemas::document::Dest {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(&mut xml_reader, &mut buf, None, b"ofd:Dest", b"Dest")
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
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
  pub(crate) fn deserialize_from_reader_named<R: std::io::BufRead>(
    xml_reader: &mut quick_xml::Reader<R>,
    buf: &mut Vec<u8>,
    xml_event: Option<(quick_xml::events::BytesStart<'static>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::definitions::CtDest>::deserialize_from_reader_named(
        xml_reader,
        buf,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
