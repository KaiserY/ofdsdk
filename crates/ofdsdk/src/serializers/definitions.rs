//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

impl crate::schemas::definitions::CtDestType {
  pub fn as_xml_str(&self) -> &'static str {
    match self {
      Self::Xyz => "XYZ",
      Self::Fit => "Fit",
      Self::FitH => "FitH",
      Self::FitV => "FitV",
      Self::FitR => "FitR",
    }
  }
  pub fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}
impl std::fmt::Display for crate::schemas::definitions::CtDestType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_xml_str())
  }
}
impl crate::schemas::definitions::MovieOperator {
  pub fn as_xml_str(&self) -> &'static str {
    match self {
      Self::Play => "Play",
      Self::Stop => "Stop",
      Self::Pause => "Pause",
      Self::Resume => "Resume",
    }
  }
  pub fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}
impl std::fmt::Display for crate::schemas::definitions::MovieOperator {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_xml_str())
  }
}
impl crate::schemas::definitions::CtActionEvent {
  pub fn as_xml_str(&self) -> &'static str {
    match self {
      Self::Do => "DO",
      Self::Po => "PO",
      Self::Click => "CLICK",
    }
  }
  pub fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}
impl std::fmt::Display for crate::schemas::definitions::CtActionEvent {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_xml_str())
  }
}
impl crate::schemas::definitions::CtDest {
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
    self.write_xml_named(writer, with_xmlns, "CT_Dest")
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
      writer.write_str(" Type=\"")?;
      writer.write_str(self.r#type.as_xml_str())?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" PageID=\"")?;
      write!(writer, "{}", self.page_id)?;
      writer.write_char('"')?;
    }
    if let Some(left) = &self.left {
      writer.write_str(" Left=\"")?;
      write!(writer, "{}", left)?;
      writer.write_char('"')?;
    }
    if let Some(top) = &self.top {
      writer.write_str(" Top=\"")?;
      write!(writer, "{}", top)?;
      writer.write_char('"')?;
    }
    if let Some(right) = &self.right {
      writer.write_str(" Right=\"")?;
      write!(writer, "{}", right)?;
      writer.write_char('"')?;
    }
    if let Some(bottom) = &self.bottom {
      writer.write_str(" Bottom=\"")?;
      write!(writer, "{}", bottom)?;
      writer.write_char('"')?;
    }
    if let Some(zoom) = &self.zoom {
      writer.write_str(" Zoom=\"")?;
      write!(writer, "{}", zoom)?;
      writer.write_char('"')?;
    }
    writer.write_str("/>")?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::definitions::CtDest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::definitions::CtPageArea {
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
    self.write_xml_named(writer, with_xmlns, "CT_PageArea")
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
      writer.write_str("ofd:PhysicalBox")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(self.physical_box.as_str()))?;
      writer.write_str("</ofd:PhysicalBox>")?;
    }
    if let Some(application_box) = &self.application_box {
      writer.write_char('<')?;
      writer.write_str("ofd:ApplicationBox")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(application_box.as_str()))?;
      writer.write_str("</ofd:ApplicationBox>")?;
    }
    if let Some(content_box) = &self.content_box {
      writer.write_char('<')?;
      writer.write_str("ofd:ContentBox")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(content_box.as_str()))?;
      writer.write_str("</ofd:ContentBox>")?;
    }
    if let Some(bleed_box) = &self.bleed_box {
      writer.write_char('<')?;
      writer.write_str("ofd:BleedBox")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(bleed_box.as_str()))?;
      writer.write_str("</ofd:BleedBox>")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::definitions::CtPageArea {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::definitions::Bookmark {
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
    writer.write_str("/>")?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::definitions::Bookmark {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::definitions::Goto {
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
    self.write_xml_named(writer, with_xmlns, "Goto")
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
    for child in &self.xml_children {
      match child {
        crate::schemas::definitions::GotoContentChoice::Dest(child) => {
          child.write_xml_named(writer, false, "Dest")?
        }
        crate::schemas::definitions::GotoContentChoice::Bookmark(child) => {
          child.write_xml_named(writer, false, "Bookmark")?
        }
      }
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::definitions::Goto {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::definitions::Uri {
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
    self.write_xml_named(writer, with_xmlns, "URI")
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
      writer.write_str(" URI=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.uri.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(base) = &self.base {
      writer.write_str(" Base=\"")?;
      writer.write_str(&quick_xml::escape::escape(base.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(target) = &self.target {
      writer.write_str(" Target=\"")?;
      writer.write_str(&quick_xml::escape::escape(target.as_str()))?;
      writer.write_char('"')?;
    }
    writer.write_str("/>")?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::definitions::Uri {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::definitions::GotoA {
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
    self.write_xml_named(writer, with_xmlns, "GotoA")
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
      writer.write_str(" AttachID=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.attach_id.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(new_window) = &self.new_window {
      writer.write_str(" NewWindow=\"")?;
      writer.write_str(if *new_window { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    writer.write_str("/>")?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::definitions::GotoA {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::definitions::Sound {
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
    self.write_xml_named(writer, with_xmlns, "Sound")
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
      writer.write_str(" ResourceID=\"")?;
      write!(writer, "{}", self.resource_id)?;
      writer.write_char('"')?;
    }
    if let Some(volume) = &self.volume {
      writer.write_str(" Volume=\"")?;
      write!(writer, "{}", volume)?;
      writer.write_char('"')?;
    }
    if let Some(repeat) = &self.repeat {
      writer.write_str(" Repeat=\"")?;
      writer.write_str(if *repeat { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(synchronous) = &self.synchronous {
      writer.write_str(" Synchronous=\"")?;
      writer.write_str(if *synchronous { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    writer.write_str("/>")?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::definitions::Sound {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::definitions::Movie {
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
    self.write_xml_named(writer, with_xmlns, "Movie")
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
      writer.write_str(" ResourceID=\"")?;
      write!(writer, "{}", self.resource_id)?;
      writer.write_char('"')?;
    }
    if let Some(operator) = &self.operator {
      writer.write_str(" Operator=\"")?;
      writer.write_str(operator.as_xml_str())?;
      writer.write_char('"')?;
    }
    writer.write_str("/>")?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::definitions::Movie {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::definitions::CtAction {
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
    self.write_xml_named(writer, with_xmlns, "CT_Action")
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
      writer.write_str(" Event=\"")?;
      writer.write_str(self.event.as_xml_str())?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    if let Some(region) = &self.region {
      region.write_xml_named(writer, false, "Region")?;
    }
    for child in &self.xml_children {
      match child {
        crate::schemas::definitions::CtActionContentChoice::Goto(child) => {
          child.write_xml_named(writer, false, "Goto")?
        }
        crate::schemas::definitions::CtActionContentChoice::Uri(child) => {
          child.write_xml_named(writer, false, "URI")?
        }
        crate::schemas::definitions::CtActionContentChoice::GotoA(child) => {
          child.write_xml_named(writer, false, "GotoA")?
        }
        crate::schemas::definitions::CtActionContentChoice::Sound(child) => {
          child.write_xml_named(writer, false, "Sound")?
        }
        crate::schemas::definitions::CtActionContentChoice::Movie(child) => {
          child.write_xml_named(writer, false, "Movie")?
        }
      }
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::definitions::CtAction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::definitions::Move {
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
    self.write_xml_named(writer, with_xmlns, "Move")
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
      writer.write_str(" Point1=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.point1.as_str()))?;
      writer.write_char('"')?;
    }
    writer.write_str("/>")?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::definitions::Move {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::definitions::Line {
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
    self.write_xml_named(writer, with_xmlns, "Line")
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
      writer.write_str(" Point1=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.point1.as_str()))?;
      writer.write_char('"')?;
    }
    writer.write_str("/>")?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::definitions::Line {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::definitions::QuadraticBezier {
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
    self.write_xml_named(writer, with_xmlns, "QuadraticBezier")
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
      writer.write_str(" Point1=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.point1.as_str()))?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" Point2=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.point2.as_str()))?;
      writer.write_char('"')?;
    }
    writer.write_str("/>")?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::definitions::QuadraticBezier {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::definitions::CubicBezier {
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
    self.write_xml_named(writer, with_xmlns, "CubicBezier")
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
    if let Some(point1) = &self.point1 {
      writer.write_str(" Point1=\"")?;
      writer.write_str(&quick_xml::escape::escape(point1.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(point2) = &self.point2 {
      writer.write_str(" Point2=\"")?;
      writer.write_str(&quick_xml::escape::escape(point2.as_str()))?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" Point3=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.point3.as_str()))?;
      writer.write_char('"')?;
    }
    writer.write_str("/>")?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::definitions::CubicBezier {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::definitions::Arc {
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
    self.write_xml_named(writer, with_xmlns, "Arc")
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
      writer.write_str(" SweepDirection=\"")?;
      writer.write_str(if self.sweep_direction {
        "true"
      } else {
        "false"
      })?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" LargeArc=\"")?;
      writer.write_str(if self.large_arc { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" RotationAngle=\"")?;
      write!(writer, "{}", self.rotation_angle)?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" EllipseSize=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.ellipse_size.as_str()))?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" EndPoint=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.end_point.as_str()))?;
      writer.write_char('"')?;
    }
    writer.write_str("/>")?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::definitions::Arc {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::definitions::Close {
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
    self.write_xml_named(writer, with_xmlns, "Close")
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
    writer.write_str("/>")?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::definitions::Close {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::definitions::Area {
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
    self.write_xml_named(writer, with_xmlns, "Area")
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
      writer.write_str(" Start=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.start.as_str()))?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    for child in &self.xml_children {
      match child {
        crate::schemas::definitions::AreaContentChoice::Move(child) => {
          child.write_xml_named(writer, false, "Move")?
        }
        crate::schemas::definitions::AreaContentChoice::Line(child) => {
          child.write_xml_named(writer, false, "Line")?
        }
        crate::schemas::definitions::AreaContentChoice::QuadraticBezier(child) => {
          child.write_xml_named(writer, false, "QuadraticBezier")?
        }
        crate::schemas::definitions::AreaContentChoice::CubicBezier(child) => {
          child.write_xml_named(writer, false, "CubicBezier")?
        }
        crate::schemas::definitions::AreaContentChoice::Arc(child) => {
          child.write_xml_named(writer, false, "Arc")?
        }
        crate::schemas::definitions::AreaContentChoice::Close(child) => {
          child.write_xml_named(writer, false, "Close")?
        }
      }
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::definitions::Area {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::definitions::CtRegion {
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
    self.write_xml_named(writer, with_xmlns, "CT_Region")
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
    for child in &self.area {
      child.write_xml_named(writer, false, "Area")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::definitions::CtRegion {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::definitions::Region {
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
    self.write_xml_named(writer, with_xmlns, "Region")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    <crate::schemas::definitions::CtRegion>::write_xml_named(&self.0, writer, with_xmlns, tag_name)
  }
}
impl std::fmt::Display for crate::schemas::definitions::Region {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::definitions::Dest {
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
impl std::fmt::Display for crate::schemas::definitions::Dest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
