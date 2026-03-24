//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

impl crate::schemas::attachments::Attachments {
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
    self.write_xml_named(writer, with_xmlns, "Attachments")
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
    for child in &self.attachment {
      child.write_xml_named(writer, false, "Attachment")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::attachments::Attachments {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::attachments::CtAttachment {
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
    self.write_xml_named(writer, with_xmlns, "CT_Attachment")
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
    {
      writer.write_str(" Name=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.name.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(format) = &self.format {
      writer.write_str(" Format=\"")?;
      writer.write_str(&quick_xml::escape::escape(format.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(creation_date) = &self.creation_date {
      writer.write_str(" CreationDate=\"")?;
      writer.write_str(&quick_xml::escape::escape(creation_date.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(mod_date) = &self.mod_date {
      writer.write_str(" ModDate=\"")?;
      writer.write_str(&quick_xml::escape::escape(mod_date.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(size) = &self.size {
      writer.write_str(" Size=\"")?;
      write!(writer, "{}", size)?;
      writer.write_char('"')?;
    }
    if let Some(visible) = &self.visible {
      writer.write_str(" Visible=\"")?;
      writer.write_str(if *visible { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(usage) = &self.usage {
      writer.write_str(" Usage=\"")?;
      writer.write_str(&quick_xml::escape::escape(usage.as_str()))?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    {
      writer.write_char('<')?;
      writer.write_str("ofd:FileLoc")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(self.file_loc.as_str()))?;
      writer.write_str("</ofd:FileLoc>")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::attachments::CtAttachment {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::attachments::Attachment {
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
    self.write_xml_named(writer, with_xmlns, "Attachment")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    <crate::schemas::attachments::CtAttachment>::write_xml_named(
      &self.0, writer, with_xmlns, tag_name,
    )
  }
}
impl std::fmt::Display for crate::schemas::attachments::Attachment {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
