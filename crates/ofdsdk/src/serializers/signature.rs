//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

impl crate::schemas::signature::ReferencesCheckMethod {
  pub fn as_xml_str(&self) -> &'static str {
    match self {
      Self::Md5 => "MD5",
      Self::Sha1 => "SHA1",
    }
  }
  pub fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}
impl std::fmt::Display for crate::schemas::signature::ReferencesCheckMethod {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_xml_str())
  }
}
impl crate::schemas::signature::Provider {
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
    self.write_xml_named(writer, with_xmlns, "Provider")
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
      writer.write_str(" ProviderName=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.provider_name.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(version) = &self.version {
      writer.write_str(" Version=\"")?;
      writer.write_str(&quick_xml::escape::escape(version.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(company) = &self.company {
      writer.write_str(" Company=\"")?;
      writer.write_str(&quick_xml::escape::escape(company.as_str()))?;
      writer.write_char('"')?;
    }
    writer.write_str("/>")?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::signature::Provider {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::signature::Reference {
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
    self.write_xml_named(writer, with_xmlns, "Reference")
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
      writer.write_str(" FileRef=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.file_ref.as_str()))?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    {
      writer.write_char('<')?;
      writer.write_str("ofd:CheckValue")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(self.check_value.as_str()))?;
      writer.write_str("</ofd:CheckValue>")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::signature::Reference {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::signature::References {
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
    self.write_xml_named(writer, with_xmlns, "References")
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
      writer.write_str(" CheckMethod=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.check_method.as_str()))?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    for child in &self.reference {
      child.write_xml_named(writer, false, "Reference")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::signature::References {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::signature::StampAnnot {
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
    self.write_xml_named(writer, with_xmlns, "StampAnnot")
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
      writer.write_str(" PageRef=\"")?;
      write!(writer, "{}", self.page_ref)?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" Boundary=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.boundary.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(clip) = &self.clip {
      writer.write_str(" Clip=\"")?;
      writer.write_str(&quick_xml::escape::escape(clip.as_str()))?;
      writer.write_char('"')?;
    }
    writer.write_str("/>")?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::signature::StampAnnot {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::signature::Seal {
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
    self.write_xml_named(writer, with_xmlns, "Seal")
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
    {
      writer.write_char('<')?;
      writer.write_str("ofd:BaseLoc")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(self.base_loc.as_str()))?;
      writer.write_str("</ofd:BaseLoc>")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::signature::Seal {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::signature::SignedInfo {
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
    self.write_xml_named(writer, with_xmlns, "SignedInfo")
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
    self.provider.write_xml_named(writer, false, "Provider")?;
    if let Some(signature_method) = &self.signature_method {
      writer.write_char('<')?;
      writer.write_str("ofd:SignatureMethod")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(signature_method.as_str()))?;
      writer.write_str("</ofd:SignatureMethod>")?;
    }
    if let Some(signature_date_time) = &self.signature_date_time {
      writer.write_char('<')?;
      writer.write_str("ofd:SignatureDateTime")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(signature_date_time.as_str()))?;
      writer.write_str("</ofd:SignatureDateTime>")?;
    }
    self
      .references
      .write_xml_named(writer, false, "References")?;
    for child in &self.stamp_annot {
      child.write_xml_named(writer, false, "StampAnnot")?;
    }
    if let Some(seal) = &self.seal {
      seal.write_xml_named(writer, false, "Seal")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::signature::SignedInfo {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::signature::Signature {
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
    self.write_xml_named(writer, with_xmlns, "Signature")
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
    self
      .signed_info
      .write_xml_named(writer, false, "SignedInfo")?;
    {
      writer.write_char('<')?;
      writer.write_str("ofd:SignedValue")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(self.signed_value.as_str()))?;
      writer.write_str("</ofd:SignedValue>")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::signature::Signature {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
