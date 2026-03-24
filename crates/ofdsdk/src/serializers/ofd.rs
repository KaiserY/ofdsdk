//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

impl crate::schemas::ofd::OfdDocType {
  pub fn as_xml_str(&self) -> &'static str {
    match self {
      Self::Ofd => "OFD",
    }
  }
  pub fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}
impl std::fmt::Display for crate::schemas::ofd::OfdDocType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_xml_str())
  }
}
impl crate::schemas::ofd::Version {
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
    self.write_xml_named(writer, with_xmlns, "Version")
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
      writer.write_str(" Index=\"")?;
      write!(writer, "{}", self.index)?;
      writer.write_char('"')?;
    }
    if let Some(current) = &self.current {
      writer.write_str(" Current=\"")?;
      writer.write_str(if *current { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" BaseLoc=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.base_loc.as_str()))?;
      writer.write_char('"')?;
    }
    writer.write_str("/>")?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::ofd::Version {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::ofd::Versions {
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
    self.write_xml_named(writer, with_xmlns, "Versions")
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
    for child in &self.version {
      child.write_xml_named(writer, false, "Version")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::ofd::Versions {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::ofd::DocBody {
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
    self.write_xml_named(writer, with_xmlns, "DocBody")
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
    self.doc_info.write_xml_named(writer, false, "DocInfo")?;
    {
      writer.write_char('<')?;
      writer.write_str("ofd:DocRoot")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(self.doc_root.as_str()))?;
      writer.write_str("</ofd:DocRoot>")?;
    }
    if let Some(versions) = &self.versions {
      versions.write_xml_named(writer, false, "Versions")?;
    }
    if let Some(signatures) = &self.signatures {
      writer.write_char('<')?;
      writer.write_str("ofd:Signatures")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(signatures.as_str()))?;
      writer.write_str("</ofd:Signatures>")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::ofd::DocBody {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::ofd::Ofd {
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
    self.write_xml_named(writer, with_xmlns, "OFD")
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
      writer.write_str(" Version=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.version.as_str()))?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" DocType=\"")?;
      writer.write_str(self.doc_type.as_xml_str())?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    for child in &self.doc_body {
      child.write_xml_named(writer, false, "DocBody")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::ofd::Ofd {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::ofd::Keywords {
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
    self.write_xml_named(writer, with_xmlns, "Keywords")
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
    for child in &self.keyword {
      writer.write_char('<')?;
      writer.write_str("ofd:Keyword")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(child.as_str()))?;
      writer.write_str("</ofd:Keyword>")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::ofd::Keywords {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::ofd::CustomData {
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
    self.write_xml_named(writer, with_xmlns, "CustomData")
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
    writer.write_char('>')?;
    writer.write_str(&quick_xml::escape::escape(self.xml_value.as_str()))?;
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::ofd::CustomData {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::ofd::CustomDatas {
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
    self.write_xml_named(writer, with_xmlns, "CustomDatas")
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
    for child in &self.custom_data {
      child.write_xml_named(writer, false, "CustomData")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::ofd::CustomDatas {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::ofd::CtDocInfo {
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
    self.write_xml_named(writer, with_xmlns, "CT_DocInfo")
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
      writer.write_str("ofd:DocID")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(self.doc_id.as_str()))?;
      writer.write_str("</ofd:DocID>")?;
    }
    if let Some(title) = &self.title {
      writer.write_char('<')?;
      writer.write_str("ofd:Title")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(title.as_str()))?;
      writer.write_str("</ofd:Title>")?;
    }
    if let Some(author) = &self.author {
      writer.write_char('<')?;
      writer.write_str("ofd:Author")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(author.as_str()))?;
      writer.write_str("</ofd:Author>")?;
    }
    if let Some(subject) = &self.subject {
      writer.write_char('<')?;
      writer.write_str("ofd:Subject")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(subject.as_str()))?;
      writer.write_str("</ofd:Subject>")?;
    }
    if let Some(r#abstract) = &self.r#abstract {
      writer.write_char('<')?;
      writer.write_str("ofd:Abstract")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(r#abstract.as_str()))?;
      writer.write_str("</ofd:Abstract>")?;
    }
    if let Some(creation_date) = &self.creation_date {
      writer.write_char('<')?;
      writer.write_str("ofd:CreationDate")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(creation_date.as_str()))?;
      writer.write_str("</ofd:CreationDate>")?;
    }
    if let Some(mod_date) = &self.mod_date {
      writer.write_char('<')?;
      writer.write_str("ofd:ModDate")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(mod_date.as_str()))?;
      writer.write_str("</ofd:ModDate>")?;
    }
    if let Some(doc_usage) = &self.doc_usage {
      writer.write_char('<')?;
      writer.write_str("ofd:DocUsage")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(doc_usage.as_str()))?;
      writer.write_str("</ofd:DocUsage>")?;
    }
    if let Some(cover) = &self.cover {
      writer.write_char('<')?;
      writer.write_str("ofd:Cover")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(cover.as_str()))?;
      writer.write_str("</ofd:Cover>")?;
    }
    if let Some(keywords) = &self.keywords {
      keywords.write_xml_named(writer, false, "Keywords")?;
    }
    if let Some(creator) = &self.creator {
      writer.write_char('<')?;
      writer.write_str("ofd:Creator")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(creator.as_str()))?;
      writer.write_str("</ofd:Creator>")?;
    }
    if let Some(creator_version) = &self.creator_version {
      writer.write_char('<')?;
      writer.write_str("ofd:CreatorVersion")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(creator_version.as_str()))?;
      writer.write_str("</ofd:CreatorVersion>")?;
    }
    if let Some(custom_datas) = &self.custom_datas {
      custom_datas.write_xml_named(writer, false, "CustomDatas")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::ofd::CtDocInfo {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::ofd::DocInfo {
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
    self.write_xml_named(writer, with_xmlns, "DocInfo")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    <crate::schemas::ofd::CtDocInfo>::write_xml_named(&self.0, writer, with_xmlns, tag_name)
  }
}
impl std::fmt::Display for crate::schemas::ofd::DocInfo {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
