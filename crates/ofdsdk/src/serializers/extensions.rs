//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

impl crate::schemas::extensions::Extensions {
  pub fn to_xml(&self) -> Result<String, std::fmt::Error> {
    let mut writer = String::with_capacity(64);
    self.write_xml(&mut writer, true)?;
    Ok(writer)
  }
  pub(crate) fn write_xml<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
  ) -> Result<(), std::fmt::Error> {
    self.write_xml_named(writer, with_xmlns, "Extensions")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    writer.write_char('<')?;
    writer.write_str("ofd:")?;
    writer.write_str(tag_name)?;
    if with_xmlns {
      writer.write_str(r#" xmlns:ofd="http://www.ofdspec.org/2016""#)?;
    }
    writer.write_char('>')?;
    for child in &self.extension {
      child.write_xml_named(writer, false, "Extension")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::extensions::Extensions {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::extensions::Property {
  pub fn to_xml(&self) -> Result<String, std::fmt::Error> {
    let mut writer = String::with_capacity(64);
    self.write_xml(&mut writer, true)?;
    Ok(writer)
  }
  pub(crate) fn write_xml<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
  ) -> Result<(), std::fmt::Error> {
    self.write_xml_named(writer, with_xmlns, "Property")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    writer.write_char('<')?;
    writer.write_str("ofd:")?;
    writer.write_str(tag_name)?;
    if with_xmlns {
      writer.write_str(r#" xmlns:ofd="http://www.ofdspec.org/2016""#)?;
    }
    {
      writer.write_str(" Name=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.name.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(r#type) = &self.r#type {
      writer.write_str(" Type=\"")?;
      writer.write_str(&quick_xml::escape::escape(r#type.as_str()))?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    writer.write_str(&quick_xml::escape::escape(self.xml_value.as_str()))?;
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::extensions::Property {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::extensions::CtExtension {
  pub fn to_xml(&self) -> Result<String, std::fmt::Error> {
    let mut writer = String::with_capacity(64);
    self.write_xml(&mut writer, true)?;
    Ok(writer)
  }
  pub(crate) fn write_xml<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
  ) -> Result<(), std::fmt::Error> {
    self.write_xml_named(writer, with_xmlns, "CT_Extension")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    writer.write_char('<')?;
    writer.write_str("ofd:")?;
    writer.write_str(tag_name)?;
    if with_xmlns {
      writer.write_str(r#" xmlns:ofd="http://www.ofdspec.org/2016""#)?;
    }
    {
      writer.write_str(" AppName=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.app_name.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(company) = &self.company {
      writer.write_str(" Company=\"")?;
      writer.write_str(&quick_xml::escape::escape(company.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(app_version) = &self.app_version {
      writer.write_str(" AppVersion=\"")?;
      writer.write_str(&quick_xml::escape::escape(app_version.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(date) = &self.date {
      writer.write_str(" Date=\"")?;
      writer.write_str(&quick_xml::escape::escape(date.as_str()))?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" RefId=\"")?;
      write!(writer, "{}", self.ref_id)?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    for child in &self.xml_children {
      match child {
        crate::schemas::extensions::CtExtensionContentChoice::Property(child) => {
          child.write_xml_named(writer, false, "Property")?
        }
        crate::schemas::extensions::CtExtensionContentChoice::Data(child) => {
          writer.write_char('<')?;
          writer.write_str("ofd:Data")?;
          writer.write_char('>')?;
          writer.write_str(&quick_xml::escape::escape(child.as_str()))?;
          writer.write_str("</ofd:Data>")?;
        }
        crate::schemas::extensions::CtExtensionContentChoice::ExtendData(child) => {
          writer.write_char('<')?;
          writer.write_str("ofd:ExtendData")?;
          writer.write_char('>')?;
          writer.write_str(&quick_xml::escape::escape(child.as_str()))?;
          writer.write_str("</ofd:ExtendData>")?;
        }
      }
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::extensions::CtExtension {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::extensions::Extension {
  pub fn to_xml(&self) -> Result<String, std::fmt::Error> {
    let mut writer = String::with_capacity(64);
    self.write_xml(&mut writer, true)?;
    Ok(writer)
  }
  pub(crate) fn write_xml<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
  ) -> Result<(), std::fmt::Error> {
    self.write_xml_named(writer, with_xmlns, "Extension")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    <crate::schemas::extensions::CtExtension>::write_xml_named(
      &self.0, writer, with_xmlns, tag_name,
    )
  }
}
impl std::fmt::Display for crate::schemas::extensions::Extension {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
