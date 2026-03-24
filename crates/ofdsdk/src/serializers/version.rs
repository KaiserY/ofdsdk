//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

impl crate::schemas::version::File {
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
    self.write_xml_named(writer, with_xmlns, "File")
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
      writer.write_str(" ID=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.id.as_str()))?;
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
impl std::fmt::Display for crate::schemas::version::File {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::version::FileList {
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
    self.write_xml_named(writer, with_xmlns, "FileList")
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
    for child in &self.file {
      child.write_xml_named(writer, false, "File")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::version::FileList {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::version::DocVersion {
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
    self.write_xml_named(writer, with_xmlns, "DocVersion")
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
      writer.write_str(" ID=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.id.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(version) = &self.version {
      writer.write_str(" Version=\"")?;
      writer.write_str(&quick_xml::escape::escape(version.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(name) = &self.name {
      writer.write_str(" Name=\"")?;
      writer.write_str(&quick_xml::escape::escape(name.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(creation_date) = &self.creation_date {
      writer.write_str(" CreationDate=\"")?;
      writer.write_str(&quick_xml::escape::escape(creation_date.as_str()))?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    self.file_list.write_xml_named(writer, false, "FileList")?;
    {
      writer.write_char('<')?;
      writer.write_str("ofd:DocRoot")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(self.doc_root.as_str()))?;
      writer.write_str("</ofd:DocRoot>")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::version::DocVersion {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
