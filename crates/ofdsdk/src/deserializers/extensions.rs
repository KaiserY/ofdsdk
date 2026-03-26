//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

impl std::str::FromStr for crate::schemas::extensions::Extensions {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Extensions", b"Extensions")
  }
}
impl crate::schemas::extensions::Extensions {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:Extensions",
      b"Extensions",
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
      "Extensions",
      "Extensions",
      tag_name_prefix,
      tag_name
    );
    let mut extension = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Extensions"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Extension" | b"Extension" => {
              extension.push(
                crate::schemas::extensions::CtExtension::deserialize_inner_named(
                  xml_reader,
                  Some((e, e_empty)),
                  b"ofd:Extension",
                  b"Extension",
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
    Ok(Self { extension })
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
      "Extensions",
      "Extensions",
      tag_name_prefix,
      tag_name
    );
    let mut extension = vec![];
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Extensions"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Extension" | b"Extension" => {
              extension.push(
                crate::schemas::extensions::CtExtension::deserialize_from_reader_named(
                  xml_reader,
                  buf,
                  Some((e, e_empty)),
                  b"ofd:Extension",
                  b"Extension",
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
    Ok(Self { extension })
  }
}
impl std::str::FromStr for crate::schemas::extensions::Property {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Property", b"Property")
  }
}
impl crate::schemas::extensions::Property {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:Property",
      b"Property",
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
      "Property",
      "Property",
      tag_name_prefix,
      tag_name
    );
    let mut name = None;
    let mut r#type = None;
    let mut xml_value_raw = None;
    let mut xml_value_seen = false;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Type" => {
          r#type =
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
                  "Property",
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Property"))?,
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
      Err(crate::common::missing_field("Property", "xml_value"))?
    };
    let name = name.ok_or_else(|| crate::common::missing_field("Property", "name"))?;
    Ok(Self {
      name,
      r#type,
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
      "Property",
      "Property",
      tag_name_prefix,
      tag_name
    );
    let mut name = None;
    let mut r#type = None;
    let mut xml_value_raw = None;
    let mut xml_value_seen = false;
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"Name" => {
          name = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Type" => {
          r#type =
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
                  "Property",
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("Property"))?,
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
      Err(crate::common::missing_field("Property", "xml_value"))?
    };
    let name = name.ok_or_else(|| crate::common::missing_field("Property", "name"))?;
    Ok(Self {
      name,
      r#type,
      xml_value,
    })
  }
}
impl std::str::FromStr for crate::schemas::extensions::CtExtension {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:CT_Extension", b"CT_Extension")
  }
}
impl crate::schemas::extensions::CtExtension {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:CT_Extension",
      b"CT_Extension",
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
      "CtExtension",
      "CT_Extension",
      tag_name_prefix,
      tag_name
    );
    let mut app_name = None;
    let mut company = None;
    let mut app_version = None;
    let mut date = None;
    let mut ref_id = None;
    let mut xml_children = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"AppName" => {
          app_name =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Company" => {
          company =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"AppVersion" => {
          app_version =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Date" => {
          date = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"RefId" => {
          ref_id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtExtension",
            "ref_id",
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtExtension"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Property" | b"Property" => {
              xml_children.push(
                crate::schemas::extensions::CtExtensionContentChoice::Property(Box::new(
                  crate::schemas::extensions::Property::deserialize_inner_named(
                    xml_reader,
                    Some((e, e_empty)),
                    b"ofd:Property",
                    b"Property",
                  )?,
                )),
              );
            }
            b"ofd:Data" | b"Data" => {
              let parsed_value: String = {
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
                          "CtExtension",
                          "Data",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:Data" || name == b"Data" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtExtension"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              xml_children.push(crate::schemas::extensions::CtExtensionContentChoice::Data(
                Box::new(parsed_value),
              ));
            }
            b"ofd:ExtendData" | b"ExtendData" => {
              let parsed_value: String = {
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
                          "CtExtension",
                          "ExtendData",
                        )?;
                      }
                      quick_xml::events::Event::End(end) => match end.name().as_ref() {
                        name if name == b"ofd:ExtendData" || name == b"ExtendData" => {
                          break value.unwrap_or_default();
                        }
                        _ => {}
                      },
                      quick_xml::events::Event::Eof => {
                        Err(crate::common::unexpected_eof("CtExtension"))?
                      }
                      _ => {}
                    }
                  }
                }
              };
              xml_children.push(
                crate::schemas::extensions::CtExtensionContentChoice::ExtendData(Box::new(
                  parsed_value,
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
    let app_name =
      app_name.ok_or_else(|| crate::common::missing_field("CtExtension", "app_name"))?;
    let ref_id = ref_id.ok_or_else(|| crate::common::missing_field("CtExtension", "ref_id"))?;
    Ok(Self {
      app_name,
      company,
      app_version,
      date,
      ref_id,
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
      "CtExtension",
      "CT_Extension",
      tag_name_prefix,
      tag_name
    );
    let mut app_name = None;
    let mut company = None;
    let mut app_version = None;
    let mut date = None;
    let mut ref_id = None;
    let mut xml_children = vec![];
    for attr in e.attributes().with_checks(false) {
      let attr = attr?;
      #[allow(clippy::single_match)]
      match attr.key.as_ref() {
        b"AppName" => {
          app_name =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Company" => {
          company =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"AppVersion" => {
          app_version =
            Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"Date" => {
          date = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?.into_owned());
        }
        b"RefId" => {
          ref_id = Some(crate::common::parse_u32_attr(
            &attr,
            xml_reader.decoder(),
            "CtExtension",
            "ref_id",
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
          quick_xml::events::Event::Eof => Err(crate::common::unexpected_eof("CtExtension"))?,
          _ => {}
        }
        if let Some(e) = e_opt {
          match e.name().as_ref() {
            b"ofd:Property" | b"Property" => {
              xml_children.push(
                crate::schemas::extensions::CtExtensionContentChoice::Property(Box::new(
                  crate::schemas::extensions::Property::deserialize_from_reader_named(
                    xml_reader,
                    buf,
                    Some((e, e_empty)),
                    b"ofd:Property",
                    b"Property",
                  )?,
                )),
              );
            }
            b"ofd:Data" | b"Data" => {
              let parsed_value: String = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtExtension",
                      field: "Data",
                      tag_name_prefix: b"ofd:Data",
                      tag_name: b"Data",
                    },
                  )?
                }
              };
              xml_children.push(crate::schemas::extensions::CtExtensionContentChoice::Data(
                Box::new(parsed_value),
              ));
            }
            b"ofd:ExtendData" | b"ExtendData" => {
              let parsed_value: String = {
                if e_empty {
                  String::new()
                } else {
                  crate::common::read_text_content_io(
                    xml_reader,
                    buf,
                    crate::common::TextReadSpec {
                      ty: "CtExtension",
                      field: "ExtendData",
                      tag_name_prefix: b"ofd:ExtendData",
                      tag_name: b"ExtendData",
                    },
                  )?
                }
              };
              xml_children.push(
                crate::schemas::extensions::CtExtensionContentChoice::ExtendData(Box::new(
                  parsed_value,
                )),
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
    let app_name =
      app_name.ok_or_else(|| crate::common::missing_field("CtExtension", "app_name"))?;
    let ref_id = ref_id.ok_or_else(|| crate::common::missing_field("CtExtension", "ref_id"))?;
    Ok(Self {
      app_name,
      company,
      app_version,
      date,
      ref_id,
      xml_children,
    })
  }
}
impl std::str::FromStr for crate::schemas::extensions::Extension {
  type Err = crate::common::SdkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = crate::common::from_str_inner(s)?;
    Self::deserialize_inner_named(&mut xml_reader, None, b"ofd:Extension", b"Extension")
  }
}
impl crate::schemas::extensions::Extension {
  pub fn from_reader<R: std::io::BufRead>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut xml_reader = crate::common::from_reader_inner(reader)?;
    let mut buf = Vec::new();
    Self::deserialize_from_reader_named(
      &mut xml_reader,
      &mut buf,
      None,
      b"ofd:Extension",
      b"Extension",
    )
  }
  pub(crate) fn deserialize_inner_named<'de>(
    xml_reader: &mut quick_xml::Reader<&'de [u8]>,
    xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
    tag_name_prefix: &[u8],
    tag_name: &[u8],
  ) -> Result<Self, crate::common::SdkError> {
    Ok(Self(
      <crate::schemas::extensions::CtExtension>::deserialize_inner_named(
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
      <crate::schemas::extensions::CtExtension>::deserialize_from_reader_named(
        xml_reader,
        buf,
        xml_event,
        tag_name_prefix,
        tag_name,
      )?,
    ))
  }
}
