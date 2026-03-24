//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

impl crate::schemas::document::TemplatePageZOrder {
  pub fn as_xml_str(&self) -> &'static str {
    match self {
      Self::Background => "Background",
      Self::Foreground => "Foreground",
    }
  }
  pub fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}
impl std::fmt::Display for crate::schemas::document::TemplatePageZOrder {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_xml_str())
  }
}
impl crate::schemas::document::PageMode {
  pub fn as_xml_str(&self) -> &'static str {
    match self {
      Self::None => "None",
      Self::FullScreen => "FullScreen",
      Self::UseOutlines => "UseOutlines",
      Self::UseThumbs => "UseThumbs",
      Self::UseCustomTags => "UseCustomTags",
      Self::UseLayers => "UseLayers",
      Self::UseAttatchs => "UseAttatchs",
      Self::UseBookmarks => "UseBookmarks",
    }
  }
  pub fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}
impl std::fmt::Display for crate::schemas::document::PageMode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_xml_str())
  }
}
impl crate::schemas::document::PageLayout {
  pub fn as_xml_str(&self) -> &'static str {
    match self {
      Self::OnePage => "OnePage",
      Self::OneColumn => "OneColumn",
      Self::TwoPageL => "TwoPageL",
      Self::TwoColumnL => "TwoColumnL",
      Self::TwoPageR => "TwoPageR",
      Self::TwoColumnR => "TwoColumnR",
    }
  }
  pub fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}
impl std::fmt::Display for crate::schemas::document::PageLayout {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_xml_str())
  }
}
impl crate::schemas::document::TabDisplay {
  pub fn as_xml_str(&self) -> &'static str {
    match self {
      Self::DocTitle => "DocTitle",
      Self::FileName => "FileName",
    }
  }
  pub fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}
impl std::fmt::Display for crate::schemas::document::TabDisplay {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_xml_str())
  }
}
impl crate::schemas::document::ZoomMode {
  pub fn as_xml_str(&self) -> &'static str {
    match self {
      Self::Default => "Default",
      Self::FitHeight => "FitHeight",
      Self::FitWidth => "FitWidth",
      Self::FitRect => "FitRect",
    }
  }
  pub fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}
impl std::fmt::Display for crate::schemas::document::ZoomMode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_xml_str())
  }
}
impl crate::schemas::document::TemplatePage {
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
    self.write_xml_named(writer, with_xmlns, "TemplatePage")
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
      write!(writer, "{}", self.id)?;
      writer.write_char('"')?;
    }
    if let Some(name) = &self.name {
      writer.write_str(" Name=\"")?;
      writer.write_str(&quick_xml::escape::escape(name.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(z_order) = &self.z_order {
      writer.write_str(" ZOrder=\"")?;
      writer.write_str(z_order.as_xml_str())?;
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
impl std::fmt::Display for crate::schemas::document::TemplatePage {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::document::CommonData {
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
    self.write_xml_named(writer, with_xmlns, "CommonData")
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
      writer.write_str("ofd:MaxUnitID")?;
      writer.write_char('>')?;
      write!(writer, "{}", self.max_unit_id)?;
      writer.write_str("</ofd:MaxUnitID>")?;
    }
    self.page_area.write_xml_named(writer, false, "PageArea")?;
    for child in &self.public_res {
      writer.write_char('<')?;
      writer.write_str("ofd:PublicRes")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(child.as_str()))?;
      writer.write_str("</ofd:PublicRes>")?;
    }
    for child in &self.document_res {
      writer.write_char('<')?;
      writer.write_str("ofd:DocumentRes")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(child.as_str()))?;
      writer.write_str("</ofd:DocumentRes>")?;
    }
    for child in &self.template_page {
      child.write_xml_named(writer, false, "TemplatePage")?;
    }
    if let Some(default_cs) = &self.default_cs {
      writer.write_char('<')?;
      writer.write_str("ofd:DefaultCS")?;
      writer.write_char('>')?;
      write!(writer, "{}", default_cs)?;
      writer.write_str("</ofd:DefaultCS>")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::document::CommonData {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::document::Page {
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
    self.write_xml_named(writer, with_xmlns, "Page")
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
      write!(writer, "{}", self.id)?;
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
impl std::fmt::Display for crate::schemas::document::Page {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::document::Pages {
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
    self.write_xml_named(writer, with_xmlns, "Pages")
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
    for child in &self.page {
      child.write_xml_named(writer, false, "Page")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::document::Pages {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::document::Outlines {
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
    self.write_xml_named(writer, with_xmlns, "Outlines")
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
    for child in &self.outline_elem {
      child.write_xml_named(writer, false, "OutlineElem")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::document::Outlines {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::document::Actions {
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
    self.write_xml_named(writer, with_xmlns, "Actions")
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
    for child in &self.action {
      child.write_xml_named(writer, false, "Action")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::document::Actions {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::document::Bookmarks {
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
    self.write_xml_named(writer, with_xmlns, "Bookmarks")
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
    for child in &self.bookmark {
      child.write_xml_named(writer, false, "Bookmark")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::document::Bookmarks {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::document::Document {
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
    self.write_xml_named(writer, with_xmlns, "Document")
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
      .common_data
      .write_xml_named(writer, false, "CommonData")?;
    self.pages.write_xml_named(writer, false, "Pages")?;
    if let Some(outlines) = &self.outlines {
      outlines.write_xml_named(writer, false, "Outlines")?;
    }
    if let Some(permissions) = &self.permissions {
      permissions.write_xml_named(writer, false, "Permissions")?;
    }
    if let Some(actions) = &self.actions {
      actions.write_xml_named(writer, false, "Actions")?;
    }
    if let Some(v_preferences) = &self.v_preferences {
      v_preferences.write_xml_named(writer, false, "VPreferences")?;
    }
    if let Some(bookmarks) = &self.bookmarks {
      bookmarks.write_xml_named(writer, false, "Bookmarks")?;
    }
    if let Some(annotations) = &self.annotations {
      writer.write_char('<')?;
      writer.write_str("ofd:Annotations")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(annotations.as_str()))?;
      writer.write_str("</ofd:Annotations>")?;
    }
    if let Some(custom_tags) = &self.custom_tags {
      writer.write_char('<')?;
      writer.write_str("ofd:CustomTags")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(custom_tags.as_str()))?;
      writer.write_str("</ofd:CustomTags>")?;
    }
    if let Some(attachments) = &self.attachments {
      writer.write_char('<')?;
      writer.write_str("ofd:Attachments")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(attachments.as_str()))?;
      writer.write_str("</ofd:Attachments>")?;
    }
    if let Some(extensions) = &self.extensions {
      writer.write_char('<')?;
      writer.write_str("ofd:Extensions")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(extensions.as_str()))?;
      writer.write_str("</ofd:Extensions>")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::document::Document {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::document::Print {
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
    self.write_xml_named(writer, with_xmlns, "Print")
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
      writer.write_str(" Printable=\"")?;
      writer.write_str(if self.printable { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(copies) = &self.copies {
      writer.write_str(" Copies=\"")?;
      write!(writer, "{}", copies)?;
      writer.write_char('"')?;
    }
    writer.write_str("/>")?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::document::Print {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::document::ValidPeriod {
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
    self.write_xml_named(writer, with_xmlns, "ValidPeriod")
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
    if let Some(start_date) = &self.start_date {
      writer.write_str(" StartDate=\"")?;
      writer.write_str(&quick_xml::escape::escape(start_date.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(end_date) = &self.end_date {
      writer.write_str(" EndDate=\"")?;
      writer.write_str(&quick_xml::escape::escape(end_date.as_str()))?;
      writer.write_char('"')?;
    }
    writer.write_str("/>")?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::document::ValidPeriod {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::document::CtPermission {
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
    self.write_xml_named(writer, with_xmlns, "CT_Permission")
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
    if let Some(edit) = &self.edit {
      writer.write_char('<')?;
      writer.write_str("ofd:Edit")?;
      writer.write_char('>')?;
      writer.write_str(if *edit { "true" } else { "false" })?;
      writer.write_str("</ofd:Edit>")?;
    }
    if let Some(annot) = &self.annot {
      writer.write_char('<')?;
      writer.write_str("ofd:Annot")?;
      writer.write_char('>')?;
      writer.write_str(if *annot { "true" } else { "false" })?;
      writer.write_str("</ofd:Annot>")?;
    }
    if let Some(export) = &self.export {
      writer.write_char('<')?;
      writer.write_str("ofd:Export")?;
      writer.write_char('>')?;
      writer.write_str(if *export { "true" } else { "false" })?;
      writer.write_str("</ofd:Export>")?;
    }
    if let Some(signature) = &self.signature {
      writer.write_char('<')?;
      writer.write_str("ofd:Signature")?;
      writer.write_char('>')?;
      writer.write_str(if *signature { "true" } else { "false" })?;
      writer.write_str("</ofd:Signature>")?;
    }
    if let Some(watermark) = &self.watermark {
      writer.write_char('<')?;
      writer.write_str("ofd:Watermark")?;
      writer.write_char('>')?;
      writer.write_str(if *watermark { "true" } else { "false" })?;
      writer.write_str("</ofd:Watermark>")?;
    }
    if let Some(print_screen) = &self.print_screen {
      writer.write_char('<')?;
      writer.write_str("ofd:PrintScreen")?;
      writer.write_char('>')?;
      writer.write_str(if *print_screen { "true" } else { "false" })?;
      writer.write_str("</ofd:PrintScreen>")?;
    }
    if let Some(print) = &self.print {
      print.write_xml_named(writer, false, "Print")?;
    }
    if let Some(valid_period) = &self.valid_period {
      valid_period.write_xml_named(writer, false, "ValidPeriod")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::document::CtPermission {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::document::CtVPreferences {
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
    self.write_xml_named(writer, with_xmlns, "CT_VPreferences")
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
    if let Some(page_mode) = &self.page_mode {
      writer.write_char('<')?;
      writer.write_str("ofd:PageMode")?;
      writer.write_char('>')?;
      writer.write_str(page_mode.as_xml_str())?;
      writer.write_str("</ofd:PageMode>")?;
    }
    if let Some(page_layout) = &self.page_layout {
      writer.write_char('<')?;
      writer.write_str("ofd:PageLayout")?;
      writer.write_char('>')?;
      writer.write_str(page_layout.as_xml_str())?;
      writer.write_str("</ofd:PageLayout>")?;
    }
    if let Some(tab_display) = &self.tab_display {
      writer.write_char('<')?;
      writer.write_str("ofd:TabDisplay")?;
      writer.write_char('>')?;
      writer.write_str(tab_display.as_xml_str())?;
      writer.write_str("</ofd:TabDisplay>")?;
    }
    if let Some(hide_toolbar) = &self.hide_toolbar {
      writer.write_char('<')?;
      writer.write_str("ofd:HideToolbar")?;
      writer.write_char('>')?;
      writer.write_str(if *hide_toolbar { "true" } else { "false" })?;
      writer.write_str("</ofd:HideToolbar>")?;
    }
    if let Some(hide_menubar) = &self.hide_menubar {
      writer.write_char('<')?;
      writer.write_str("ofd:HideMenubar")?;
      writer.write_char('>')?;
      writer.write_str(if *hide_menubar { "true" } else { "false" })?;
      writer.write_str("</ofd:HideMenubar>")?;
    }
    if let Some(hide_window_ui) = &self.hide_window_ui {
      writer.write_char('<')?;
      writer.write_str("ofd:HideWindowUI")?;
      writer.write_char('>')?;
      writer.write_str(if *hide_window_ui { "true" } else { "false" })?;
      writer.write_str("</ofd:HideWindowUI>")?;
    }
    for child in &self.xml_children {
      match child {
        crate::schemas::document::CtVPreferencesContentChoice::ZoomMode(child) => {
          writer.write_char('<')?;
          writer.write_str("ofd:ZoomMode")?;
          writer.write_char('>')?;
          writer.write_str(child.as_xml_str())?;
          writer.write_str("</ofd:ZoomMode>")?;
        }
        crate::schemas::document::CtVPreferencesContentChoice::Zoom(child) => {
          writer.write_char('<')?;
          writer.write_str("ofd:Zoom")?;
          writer.write_char('>')?;
          write!(writer, "{}", child)?;
          writer.write_str("</ofd:Zoom>")?;
        }
      }
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::document::CtVPreferences {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::document::CtOutlineElem {
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
    self.write_xml_named(writer, with_xmlns, "CT_OutlineElem")
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
      writer.write_str(" Title=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.title.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(count) = &self.count {
      writer.write_str(" Count=\"")?;
      write!(writer, "{}", count)?;
      writer.write_char('"')?;
    }
    if let Some(expanded) = &self.expanded {
      writer.write_str(" Expanded=\"")?;
      writer.write_str(if *expanded { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    if let Some(actions) = &self.actions {
      actions.write_xml_named(writer, false, "Actions")?;
    }
    for child in &self.outline_elem {
      child.write_xml_named(writer, false, "OutlineElem")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::document::CtOutlineElem {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::document::CtBookmark {
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
    self.write_xml_named(writer, with_xmlns, "CT_Bookmark")
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
    self.dest.write_xml_named(writer, false, "Dest")?;
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::document::CtBookmark {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::document::PageArea {
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
    self.write_xml_named(writer, with_xmlns, "PageArea")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    <crate::schemas::definitions::CtPageArea>::write_xml_named(
      &self.0, writer, with_xmlns, tag_name,
    )
  }
}
impl std::fmt::Display for crate::schemas::document::PageArea {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::document::OutlineElem {
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
    self.write_xml_named(writer, with_xmlns, "OutlineElem")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    <crate::schemas::document::CtOutlineElem>::write_xml_named(
      &self.0, writer, with_xmlns, tag_name,
    )
  }
}
impl std::fmt::Display for crate::schemas::document::OutlineElem {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::document::Permissions {
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
    self.write_xml_named(writer, with_xmlns, "Permissions")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    <crate::schemas::document::CtPermission>::write_xml_named(&self.0, writer, with_xmlns, tag_name)
  }
}
impl std::fmt::Display for crate::schemas::document::Permissions {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::document::Action {
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
    self.write_xml_named(writer, with_xmlns, "Action")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    <crate::schemas::definitions::CtAction>::write_xml_named(&self.0, writer, with_xmlns, tag_name)
  }
}
impl std::fmt::Display for crate::schemas::document::Action {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::document::VPreferences {
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
    self.write_xml_named(writer, with_xmlns, "VPreferences")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    <crate::schemas::document::CtVPreferences>::write_xml_named(
      &self.0, writer, with_xmlns, tag_name,
    )
  }
}
impl std::fmt::Display for crate::schemas::document::VPreferences {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::document::Bookmark {
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
    self.write_xml_named(writer, with_xmlns, "Bookmark")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    <crate::schemas::document::CtBookmark>::write_xml_named(&self.0, writer, with_xmlns, tag_name)
  }
}
impl std::fmt::Display for crate::schemas::document::Bookmark {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::document::Dest {
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
    self.write_xml_named(writer, with_xmlns, "Dest")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    <crate::schemas::definitions::CtDest>::write_xml_named(&self.0, writer, with_xmlns, tag_name)
  }
}
impl std::fmt::Display for crate::schemas::document::Dest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
