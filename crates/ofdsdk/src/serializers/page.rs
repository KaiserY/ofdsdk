//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

impl crate::schemas::page::TemplateZOrder {
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
impl std::fmt::Display for crate::schemas::page::TemplateZOrder {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_xml_str())
  }
}
impl crate::schemas::page::CtLayerType {
  pub fn as_xml_str(&self) -> &'static str {
    match self {
      Self::Body => "Body",
      Self::Background => "Background",
      Self::Foreground => "Foreground",
      Self::Custom => "Custom",
    }
  }
  pub fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}
impl std::fmt::Display for crate::schemas::page::CtLayerType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_xml_str())
  }
}
impl crate::schemas::page::CtGraphicUnitCap {
  pub fn as_xml_str(&self) -> &'static str {
    match self {
      Self::Butt => "Butt",
      Self::Round => "Round",
      Self::Square => "Square",
    }
  }
  pub fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}
impl std::fmt::Display for crate::schemas::page::CtGraphicUnitCap {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_xml_str())
  }
}
impl crate::schemas::page::CtGraphicUnitJoin {
  pub fn as_xml_str(&self) -> &'static str {
    match self {
      Self::Miter => "Miter",
      Self::Round => "Round",
      Self::Bevel => "Bevel",
    }
  }
  pub fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}
impl std::fmt::Display for crate::schemas::page::CtGraphicUnitJoin {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_xml_str())
  }
}
impl crate::schemas::page::CtTextWeight {
  pub fn as_xml_str(&self) -> &'static str {
    match self {
      Self::_0 => "0",
      Self::_100 => "100",
      Self::_200 => "200",
      Self::_300 => "300",
      Self::_400 => "400",
      Self::_500 => "500",
      Self::_600 => "600",
      Self::_700 => "700",
      Self::_800 => "800",
      Self::_900 => "900",
      Self::_1000 => "1000",
    }
  }
  pub fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}
impl std::fmt::Display for crate::schemas::page::CtTextWeight {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_xml_str())
  }
}
impl crate::schemas::page::CtPathRule {
  pub fn as_xml_str(&self) -> &'static str {
    match self {
      Self::NonZero => "NonZero",
      Self::_EvenOdd => "Even-Odd",
    }
  }
  pub fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}
impl std::fmt::Display for crate::schemas::page::CtPathRule {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_xml_str())
  }
}
impl crate::schemas::page::CtPatternReflectMethod {
  pub fn as_xml_str(&self) -> &'static str {
    match self {
      Self::Normal => "Normal",
      Self::Row => "Row",
      Self::Column => "Column",
      Self::RowAndColumn => "RowAndColumn",
    }
  }
  pub fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}
impl std::fmt::Display for crate::schemas::page::CtPatternReflectMethod {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_xml_str())
  }
}
impl crate::schemas::page::CtPatternRelativeTo {
  pub fn as_xml_str(&self) -> &'static str {
    match self {
      Self::Page => "Page",
      Self::Object => "Object",
    }
  }
  pub fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}
impl std::fmt::Display for crate::schemas::page::CtPatternRelativeTo {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_xml_str())
  }
}
impl crate::schemas::page::CtAxialShdMapType {
  pub fn as_xml_str(&self) -> &'static str {
    match self {
      Self::Direct => "Direct",
      Self::Repeat => "Repeat",
      Self::Reflect => "Reflect",
    }
  }
  pub fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}
impl std::fmt::Display for crate::schemas::page::CtAxialShdMapType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_xml_str())
  }
}
impl crate::schemas::page::CtAxialShdExtend {
  pub fn as_xml_str(&self) -> &'static str {
    match self {
      Self::_0 => "0",
      Self::_1 => "1",
      Self::_2 => "2",
      Self::_3 => "3",
    }
  }
  pub fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}
impl std::fmt::Display for crate::schemas::page::CtAxialShdExtend {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_xml_str())
  }
}
impl crate::schemas::page::CtRadialShdMapType {
  pub fn as_xml_str(&self) -> &'static str {
    match self {
      Self::Direct => "Direct",
      Self::Repeat => "Repeat",
      Self::Reflect => "Reflect",
    }
  }
  pub fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}
impl std::fmt::Display for crate::schemas::page::CtRadialShdMapType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_xml_str())
  }
}
impl crate::schemas::page::PointEdgeFlag {
  pub fn as_xml_str(&self) -> &'static str {
    match self {
      Self::_0 => "0",
      Self::_1 => "1",
      Self::_2 => "2",
    }
  }
  pub fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}
impl std::fmt::Display for crate::schemas::page::PointEdgeFlag {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_xml_str())
  }
}
impl crate::schemas::page::Template {
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
    self.write_xml_named(writer, with_xmlns, "Template")
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
      writer.write_str(" TemplateID=\"")?;
      write!(writer, "{}", self.template_id)?;
      writer.write_char('"')?;
    }
    if let Some(z_order) = &self.z_order {
      writer.write_str(" ZOrder=\"")?;
      writer.write_str(z_order.as_xml_str())?;
      writer.write_char('"')?;
    }
    writer.write_str("/>")?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::Template {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::Layer {
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
    self.write_xml_named(writer, with_xmlns, "Layer")
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
    if let Some(r#type) = &self.r#type {
      writer.write_str(" Type=\"")?;
      writer.write_str(r#type.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(draw_param) = &self.draw_param {
      writer.write_str(" DrawParam=\"")?;
      write!(writer, "{}", draw_param)?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" ID=\"")?;
      write!(writer, "{}", self.id)?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    for child in &self.xml_children {
      match child {
        crate::schemas::page::LayerContentChoice::TextObject(child) => {
          child.write_xml_named(writer, false, "TextObject")?
        }
        crate::schemas::page::LayerContentChoice::PathObject(child) => {
          child.write_xml_named(writer, false, "PathObject")?
        }
        crate::schemas::page::LayerContentChoice::ImageObject(child) => {
          child.write_xml_named(writer, false, "ImageObject")?
        }
        crate::schemas::page::LayerContentChoice::CompositeObject(child) => {
          child.write_xml_named(writer, false, "CompositeObject")?
        }
        crate::schemas::page::LayerContentChoice::PageBlock(child) => {
          child.write_xml_named(writer, false, "PageBlock")?
        }
      }
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::Layer {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::Content {
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
    self.write_xml_named(writer, with_xmlns, "Content")
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
    for child in &self.layer {
      child.write_xml_named(writer, false, "Layer")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::Content {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::Actions {
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
impl std::fmt::Display for crate::schemas::page::Actions {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::Page {
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
    writer.write_char('>')?;
    for child in &self.template {
      child.write_xml_named(writer, false, "Template")?;
    }
    for child in &self.page_res {
      writer.write_char('<')?;
      writer.write_str("ofd:PageRes")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(child.as_str()))?;
      writer.write_str("</ofd:PageRes>")?;
    }
    if let Some(area) = &self.area {
      area.write_xml_named(writer, false, "Area")?;
    }
    if let Some(content) = &self.content {
      content.write_xml_named(writer, false, "Content")?;
    }
    if let Some(actions) = &self.actions {
      actions.write_xml_named(writer, false, "Actions")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::Page {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::Area {
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
    if let Some(draw_param) = &self.draw_param {
      writer.write_str(" DrawParam=\"")?;
      write!(writer, "{}", draw_param)?;
      writer.write_char('"')?;
    }
    if let Some(ctm) = &self.ctm {
      writer.write_str(" CTM=\"")?;
      writer.write_str(&quick_xml::escape::escape(ctm.as_str()))?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    for child in &self.xml_children {
      match child {
        crate::schemas::page::AreaContentChoice::Path(child) => {
          child.write_xml_named(writer, false, "Path")?
        }
        crate::schemas::page::AreaContentChoice::Text(child) => {
          child.write_xml_named(writer, false, "Text")?
        }
      }
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::Area {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::CtClip {
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
    self.write_xml_named(writer, with_xmlns, "CT_Clip")
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
impl std::fmt::Display for crate::schemas::page::CtClip {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::TextObject {
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
    self.write_xml_named(writer, with_xmlns, "TextObject")
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
      writer.write_str(" Boundary=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.boundary.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(name) = &self.name {
      writer.write_str(" Name=\"")?;
      writer.write_str(&quick_xml::escape::escape(name.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(visible) = &self.visible {
      writer.write_str(" Visible=\"")?;
      writer.write_str(if *visible { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(ctm) = &self.ctm {
      writer.write_str(" CTM=\"")?;
      writer.write_str(&quick_xml::escape::escape(ctm.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(draw_param) = &self.draw_param {
      writer.write_str(" DrawParam=\"")?;
      write!(writer, "{}", draw_param)?;
      writer.write_char('"')?;
    }
    if let Some(line_width) = &self.line_width {
      writer.write_str(" LineWidth=\"")?;
      write!(writer, "{}", line_width)?;
      writer.write_char('"')?;
    }
    if let Some(cap) = &self.cap {
      writer.write_str(" Cap=\"")?;
      writer.write_str(cap.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(join) = &self.join {
      writer.write_str(" Join=\"")?;
      writer.write_str(join.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(miter_limit) = &self.miter_limit {
      writer.write_str(" MiterLimit=\"")?;
      write!(writer, "{}", miter_limit)?;
      writer.write_char('"')?;
    }
    if let Some(dash_offset) = &self.dash_offset {
      writer.write_str(" DashOffset=\"")?;
      write!(writer, "{}", dash_offset)?;
      writer.write_char('"')?;
    }
    if let Some(dash_pattern) = &self.dash_pattern {
      writer.write_str(" DashPattern=\"")?;
      writer.write_str(&quick_xml::escape::escape(dash_pattern.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(alpha) = &self.alpha {
      writer.write_str(" Alpha=\"")?;
      write!(writer, "{}", alpha)?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" Font=\"")?;
      write!(writer, "{}", self.font)?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" Size=\"")?;
      write!(writer, "{}", self.size)?;
      writer.write_char('"')?;
    }
    if let Some(stroke) = &self.stroke {
      writer.write_str(" Stroke=\"")?;
      writer.write_str(if *stroke { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(fill) = &self.fill {
      writer.write_str(" Fill=\"")?;
      writer.write_str(if *fill { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(h_scale) = &self.h_scale {
      writer.write_str(" HScale=\"")?;
      write!(writer, "{}", h_scale)?;
      writer.write_char('"')?;
    }
    if let Some(read_direction) = &self.read_direction {
      writer.write_str(" ReadDirection=\"")?;
      write!(writer, "{}", read_direction)?;
      writer.write_char('"')?;
    }
    if let Some(char_direction) = &self.char_direction {
      writer.write_str(" CharDirection=\"")?;
      write!(writer, "{}", char_direction)?;
      writer.write_char('"')?;
    }
    if let Some(weight) = &self.weight {
      writer.write_str(" Weight=\"")?;
      writer.write_str(weight.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(italic) = &self.italic {
      writer.write_str(" Italic=\"")?;
      writer.write_str(if *italic { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" ID=\"")?;
      write!(writer, "{}", self.id)?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    if let Some(actions) = &self.actions {
      actions.write_xml_named(writer, false, "Actions")?;
    }
    if let Some(clips) = &self.clips {
      clips.write_xml_named(writer, false, "Clips")?;
    }
    if let Some(fill_color) = &self.fill_color {
      fill_color.write_xml_named(writer, false, "FillColor")?;
    }
    if let Some(stroke_color) = &self.stroke_color {
      stroke_color.write_xml_named(writer, false, "StrokeColor")?;
    }
    for child in &self.cg_transform {
      child.write_xml_named(writer, false, "CGTransform")?;
    }
    for child in &self.text_code {
      child.write_xml_named(writer, false, "TextCode")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::TextObject {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::PathObject {
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
    self.write_xml_named(writer, with_xmlns, "PathObject")
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
      writer.write_str(" Boundary=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.boundary.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(name) = &self.name {
      writer.write_str(" Name=\"")?;
      writer.write_str(&quick_xml::escape::escape(name.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(visible) = &self.visible {
      writer.write_str(" Visible=\"")?;
      writer.write_str(if *visible { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(ctm) = &self.ctm {
      writer.write_str(" CTM=\"")?;
      writer.write_str(&quick_xml::escape::escape(ctm.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(draw_param) = &self.draw_param {
      writer.write_str(" DrawParam=\"")?;
      write!(writer, "{}", draw_param)?;
      writer.write_char('"')?;
    }
    if let Some(line_width) = &self.line_width {
      writer.write_str(" LineWidth=\"")?;
      write!(writer, "{}", line_width)?;
      writer.write_char('"')?;
    }
    if let Some(cap) = &self.cap {
      writer.write_str(" Cap=\"")?;
      writer.write_str(cap.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(join) = &self.join {
      writer.write_str(" Join=\"")?;
      writer.write_str(join.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(miter_limit) = &self.miter_limit {
      writer.write_str(" MiterLimit=\"")?;
      write!(writer, "{}", miter_limit)?;
      writer.write_char('"')?;
    }
    if let Some(dash_offset) = &self.dash_offset {
      writer.write_str(" DashOffset=\"")?;
      write!(writer, "{}", dash_offset)?;
      writer.write_char('"')?;
    }
    if let Some(dash_pattern) = &self.dash_pattern {
      writer.write_str(" DashPattern=\"")?;
      writer.write_str(&quick_xml::escape::escape(dash_pattern.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(alpha) = &self.alpha {
      writer.write_str(" Alpha=\"")?;
      write!(writer, "{}", alpha)?;
      writer.write_char('"')?;
    }
    if let Some(stroke) = &self.stroke {
      writer.write_str(" Stroke=\"")?;
      writer.write_str(if *stroke { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(fill) = &self.fill {
      writer.write_str(" Fill=\"")?;
      writer.write_str(if *fill { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(rule) = &self.rule {
      writer.write_str(" Rule=\"")?;
      writer.write_str(rule.as_xml_str())?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" ID=\"")?;
      write!(writer, "{}", self.id)?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    if let Some(actions) = &self.actions {
      actions.write_xml_named(writer, false, "Actions")?;
    }
    if let Some(clips) = &self.clips {
      clips.write_xml_named(writer, false, "Clips")?;
    }
    if let Some(stroke_color) = &self.stroke_color {
      stroke_color.write_xml_named(writer, false, "StrokeColor")?;
    }
    if let Some(fill_color) = &self.fill_color {
      fill_color.write_xml_named(writer, false, "FillColor")?;
    }
    {
      writer.write_char('<')?;
      writer.write_str("ofd:AbbreviatedData")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(self.abbreviated_data.as_str()))?;
      writer.write_str("</ofd:AbbreviatedData>")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::PathObject {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::ImageObject {
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
    self.write_xml_named(writer, with_xmlns, "ImageObject")
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
      writer.write_str(" Boundary=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.boundary.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(name) = &self.name {
      writer.write_str(" Name=\"")?;
      writer.write_str(&quick_xml::escape::escape(name.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(visible) = &self.visible {
      writer.write_str(" Visible=\"")?;
      writer.write_str(if *visible { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(ctm) = &self.ctm {
      writer.write_str(" CTM=\"")?;
      writer.write_str(&quick_xml::escape::escape(ctm.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(draw_param) = &self.draw_param {
      writer.write_str(" DrawParam=\"")?;
      write!(writer, "{}", draw_param)?;
      writer.write_char('"')?;
    }
    if let Some(line_width) = &self.line_width {
      writer.write_str(" LineWidth=\"")?;
      write!(writer, "{}", line_width)?;
      writer.write_char('"')?;
    }
    if let Some(cap) = &self.cap {
      writer.write_str(" Cap=\"")?;
      writer.write_str(cap.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(join) = &self.join {
      writer.write_str(" Join=\"")?;
      writer.write_str(join.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(miter_limit) = &self.miter_limit {
      writer.write_str(" MiterLimit=\"")?;
      write!(writer, "{}", miter_limit)?;
      writer.write_char('"')?;
    }
    if let Some(dash_offset) = &self.dash_offset {
      writer.write_str(" DashOffset=\"")?;
      write!(writer, "{}", dash_offset)?;
      writer.write_char('"')?;
    }
    if let Some(dash_pattern) = &self.dash_pattern {
      writer.write_str(" DashPattern=\"")?;
      writer.write_str(&quick_xml::escape::escape(dash_pattern.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(alpha) = &self.alpha {
      writer.write_str(" Alpha=\"")?;
      write!(writer, "{}", alpha)?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" ResourceID=\"")?;
      write!(writer, "{}", self.resource_id)?;
      writer.write_char('"')?;
    }
    if let Some(substitution) = &self.substitution {
      writer.write_str(" Substitution=\"")?;
      write!(writer, "{}", substitution)?;
      writer.write_char('"')?;
    }
    if let Some(image_mask) = &self.image_mask {
      writer.write_str(" ImageMask=\"")?;
      write!(writer, "{}", image_mask)?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" ID=\"")?;
      write!(writer, "{}", self.id)?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    if let Some(actions) = &self.actions {
      actions.write_xml_named(writer, false, "Actions")?;
    }
    if let Some(clips) = &self.clips {
      clips.write_xml_named(writer, false, "Clips")?;
    }
    if let Some(border) = &self.border {
      border.write_xml_named(writer, false, "Border")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::ImageObject {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::CompositeObject {
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
    self.write_xml_named(writer, with_xmlns, "CompositeObject")
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
      writer.write_str(" Boundary=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.boundary.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(name) = &self.name {
      writer.write_str(" Name=\"")?;
      writer.write_str(&quick_xml::escape::escape(name.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(visible) = &self.visible {
      writer.write_str(" Visible=\"")?;
      writer.write_str(if *visible { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(ctm) = &self.ctm {
      writer.write_str(" CTM=\"")?;
      writer.write_str(&quick_xml::escape::escape(ctm.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(draw_param) = &self.draw_param {
      writer.write_str(" DrawParam=\"")?;
      write!(writer, "{}", draw_param)?;
      writer.write_char('"')?;
    }
    if let Some(line_width) = &self.line_width {
      writer.write_str(" LineWidth=\"")?;
      write!(writer, "{}", line_width)?;
      writer.write_char('"')?;
    }
    if let Some(cap) = &self.cap {
      writer.write_str(" Cap=\"")?;
      writer.write_str(cap.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(join) = &self.join {
      writer.write_str(" Join=\"")?;
      writer.write_str(join.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(miter_limit) = &self.miter_limit {
      writer.write_str(" MiterLimit=\"")?;
      write!(writer, "{}", miter_limit)?;
      writer.write_char('"')?;
    }
    if let Some(dash_offset) = &self.dash_offset {
      writer.write_str(" DashOffset=\"")?;
      write!(writer, "{}", dash_offset)?;
      writer.write_char('"')?;
    }
    if let Some(dash_pattern) = &self.dash_pattern {
      writer.write_str(" DashPattern=\"")?;
      writer.write_str(&quick_xml::escape::escape(dash_pattern.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(alpha) = &self.alpha {
      writer.write_str(" Alpha=\"")?;
      write!(writer, "{}", alpha)?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" ResourceID=\"")?;
      write!(writer, "{}", self.resource_id)?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" ID=\"")?;
      write!(writer, "{}", self.id)?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    if let Some(actions) = &self.actions {
      actions.write_xml_named(writer, false, "Actions")?;
    }
    if let Some(clips) = &self.clips {
      clips.write_xml_named(writer, false, "Clips")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::CompositeObject {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::CtPageBlock {
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
    self.write_xml_named(writer, with_xmlns, "CT_PageBlock")
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
        crate::schemas::page::CtPageBlockContentChoice::TextObject(child) => {
          child.write_xml_named(writer, false, "TextObject")?
        }
        crate::schemas::page::CtPageBlockContentChoice::PathObject(child) => {
          child.write_xml_named(writer, false, "PathObject")?
        }
        crate::schemas::page::CtPageBlockContentChoice::ImageObject(child) => {
          child.write_xml_named(writer, false, "ImageObject")?
        }
        crate::schemas::page::CtPageBlockContentChoice::CompositeObject(child) => {
          child.write_xml_named(writer, false, "CompositeObject")?
        }
        crate::schemas::page::CtPageBlockContentChoice::PageBlock(child) => {
          child.write_xml_named(writer, false, "PageBlock")?
        }
      }
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::CtPageBlock {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::PageBlock {
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
    self.write_xml_named(writer, with_xmlns, "PageBlock")
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
    writer.write_char('>')?;
    for child in &self.xml_children {
      match child {
        crate::schemas::page::PageBlockContentChoice::TextObject(child) => {
          child.write_xml_named(writer, false, "TextObject")?
        }
        crate::schemas::page::PageBlockContentChoice::PathObject(child) => {
          child.write_xml_named(writer, false, "PathObject")?
        }
        crate::schemas::page::PageBlockContentChoice::ImageObject(child) => {
          child.write_xml_named(writer, false, "ImageObject")?
        }
        crate::schemas::page::PageBlockContentChoice::CompositeObject(child) => {
          child.write_xml_named(writer, false, "CompositeObject")?
        }
        crate::schemas::page::PageBlockContentChoice::PageBlock(child) => {
          child.write_xml_named(writer, false, "PageBlock")?
        }
      }
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::PageBlock {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::CtLayer {
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
    self.write_xml_named(writer, with_xmlns, "CT_Layer")
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
    if let Some(r#type) = &self.r#type {
      writer.write_str(" Type=\"")?;
      writer.write_str(r#type.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(draw_param) = &self.draw_param {
      writer.write_str(" DrawParam=\"")?;
      write!(writer, "{}", draw_param)?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    for child in &self.xml_children {
      match child {
        crate::schemas::page::CtLayerContentChoice::TextObject(child) => {
          child.write_xml_named(writer, false, "TextObject")?
        }
        crate::schemas::page::CtLayerContentChoice::PathObject(child) => {
          child.write_xml_named(writer, false, "PathObject")?
        }
        crate::schemas::page::CtLayerContentChoice::ImageObject(child) => {
          child.write_xml_named(writer, false, "ImageObject")?
        }
        crate::schemas::page::CtLayerContentChoice::CompositeObject(child) => {
          child.write_xml_named(writer, false, "CompositeObject")?
        }
        crate::schemas::page::CtLayerContentChoice::PageBlock(child) => {
          child.write_xml_named(writer, false, "PageBlock")?
        }
      }
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::CtLayer {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::Clips {
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
    self.write_xml_named(writer, with_xmlns, "Clips")
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
    for child in &self.clip {
      child.write_xml_named(writer, false, "Clip")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::Clips {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::CtGraphicUnit {
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
    self.write_xml_named(writer, with_xmlns, "CT_GraphicUnit")
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
      writer.write_str(" Boundary=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.boundary.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(name) = &self.name {
      writer.write_str(" Name=\"")?;
      writer.write_str(&quick_xml::escape::escape(name.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(visible) = &self.visible {
      writer.write_str(" Visible=\"")?;
      writer.write_str(if *visible { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(ctm) = &self.ctm {
      writer.write_str(" CTM=\"")?;
      writer.write_str(&quick_xml::escape::escape(ctm.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(draw_param) = &self.draw_param {
      writer.write_str(" DrawParam=\"")?;
      write!(writer, "{}", draw_param)?;
      writer.write_char('"')?;
    }
    if let Some(line_width) = &self.line_width {
      writer.write_str(" LineWidth=\"")?;
      write!(writer, "{}", line_width)?;
      writer.write_char('"')?;
    }
    if let Some(cap) = &self.cap {
      writer.write_str(" Cap=\"")?;
      writer.write_str(cap.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(join) = &self.join {
      writer.write_str(" Join=\"")?;
      writer.write_str(join.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(miter_limit) = &self.miter_limit {
      writer.write_str(" MiterLimit=\"")?;
      write!(writer, "{}", miter_limit)?;
      writer.write_char('"')?;
    }
    if let Some(dash_offset) = &self.dash_offset {
      writer.write_str(" DashOffset=\"")?;
      write!(writer, "{}", dash_offset)?;
      writer.write_char('"')?;
    }
    if let Some(dash_pattern) = &self.dash_pattern {
      writer.write_str(" DashPattern=\"")?;
      writer.write_str(&quick_xml::escape::escape(dash_pattern.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(alpha) = &self.alpha {
      writer.write_str(" Alpha=\"")?;
      write!(writer, "{}", alpha)?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    if let Some(actions) = &self.actions {
      actions.write_xml_named(writer, false, "Actions")?;
    }
    if let Some(clips) = &self.clips {
      clips.write_xml_named(writer, false, "Clips")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::CtGraphicUnit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::TextCode {
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
    self.write_xml_named(writer, with_xmlns, "TextCode")
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
    if let Some(x) = &self.x {
      writer.write_str(" X=\"")?;
      write!(writer, "{}", x)?;
      writer.write_char('"')?;
    }
    if let Some(y) = &self.y {
      writer.write_str(" Y=\"")?;
      write!(writer, "{}", y)?;
      writer.write_char('"')?;
    }
    if let Some(delta_x) = &self.delta_x {
      writer.write_str(" DeltaX=\"")?;
      writer.write_str(&quick_xml::escape::escape(delta_x.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(delta_y) = &self.delta_y {
      writer.write_str(" DeltaY=\"")?;
      writer.write_str(&quick_xml::escape::escape(delta_y.as_str()))?;
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
impl std::fmt::Display for crate::schemas::page::TextCode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::CtText {
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
    self.write_xml_named(writer, with_xmlns, "CT_Text")
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
      writer.write_str(" Boundary=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.boundary.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(name) = &self.name {
      writer.write_str(" Name=\"")?;
      writer.write_str(&quick_xml::escape::escape(name.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(visible) = &self.visible {
      writer.write_str(" Visible=\"")?;
      writer.write_str(if *visible { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(ctm) = &self.ctm {
      writer.write_str(" CTM=\"")?;
      writer.write_str(&quick_xml::escape::escape(ctm.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(draw_param) = &self.draw_param {
      writer.write_str(" DrawParam=\"")?;
      write!(writer, "{}", draw_param)?;
      writer.write_char('"')?;
    }
    if let Some(line_width) = &self.line_width {
      writer.write_str(" LineWidth=\"")?;
      write!(writer, "{}", line_width)?;
      writer.write_char('"')?;
    }
    if let Some(cap) = &self.cap {
      writer.write_str(" Cap=\"")?;
      writer.write_str(cap.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(join) = &self.join {
      writer.write_str(" Join=\"")?;
      writer.write_str(join.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(miter_limit) = &self.miter_limit {
      writer.write_str(" MiterLimit=\"")?;
      write!(writer, "{}", miter_limit)?;
      writer.write_char('"')?;
    }
    if let Some(dash_offset) = &self.dash_offset {
      writer.write_str(" DashOffset=\"")?;
      write!(writer, "{}", dash_offset)?;
      writer.write_char('"')?;
    }
    if let Some(dash_pattern) = &self.dash_pattern {
      writer.write_str(" DashPattern=\"")?;
      writer.write_str(&quick_xml::escape::escape(dash_pattern.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(alpha) = &self.alpha {
      writer.write_str(" Alpha=\"")?;
      write!(writer, "{}", alpha)?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" Font=\"")?;
      write!(writer, "{}", self.font)?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" Size=\"")?;
      write!(writer, "{}", self.size)?;
      writer.write_char('"')?;
    }
    if let Some(stroke) = &self.stroke {
      writer.write_str(" Stroke=\"")?;
      writer.write_str(if *stroke { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(fill) = &self.fill {
      writer.write_str(" Fill=\"")?;
      writer.write_str(if *fill { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(h_scale) = &self.h_scale {
      writer.write_str(" HScale=\"")?;
      write!(writer, "{}", h_scale)?;
      writer.write_char('"')?;
    }
    if let Some(read_direction) = &self.read_direction {
      writer.write_str(" ReadDirection=\"")?;
      write!(writer, "{}", read_direction)?;
      writer.write_char('"')?;
    }
    if let Some(char_direction) = &self.char_direction {
      writer.write_str(" CharDirection=\"")?;
      write!(writer, "{}", char_direction)?;
      writer.write_char('"')?;
    }
    if let Some(weight) = &self.weight {
      writer.write_str(" Weight=\"")?;
      writer.write_str(weight.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(italic) = &self.italic {
      writer.write_str(" Italic=\"")?;
      writer.write_str(if *italic { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    if let Some(actions) = &self.actions {
      actions.write_xml_named(writer, false, "Actions")?;
    }
    if let Some(clips) = &self.clips {
      clips.write_xml_named(writer, false, "Clips")?;
    }
    if let Some(fill_color) = &self.fill_color {
      fill_color.write_xml_named(writer, false, "FillColor")?;
    }
    if let Some(stroke_color) = &self.stroke_color {
      stroke_color.write_xml_named(writer, false, "StrokeColor")?;
    }
    for child in &self.cg_transform {
      child.write_xml_named(writer, false, "CGTransform")?;
    }
    for child in &self.text_code {
      child.write_xml_named(writer, false, "TextCode")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::CtText {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::CtCgTransform {
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
    self.write_xml_named(writer, with_xmlns, "CT_CGTransform")
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
      writer.write_str(" CodePosition=\"")?;
      write!(writer, "{}", self.code_position)?;
      writer.write_char('"')?;
    }
    if let Some(code_count) = &self.code_count {
      writer.write_str(" CodeCount=\"")?;
      write!(writer, "{}", code_count)?;
      writer.write_char('"')?;
    }
    if let Some(glyph_count) = &self.glyph_count {
      writer.write_str(" GlyphCount=\"")?;
      write!(writer, "{}", glyph_count)?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    if let Some(glyphs) = &self.glyphs {
      writer.write_char('<')?;
      writer.write_str("ofd:Glyphs")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(glyphs.as_str()))?;
      writer.write_str("</ofd:Glyphs>")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::CtCgTransform {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::Border {
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
    self.write_xml_named(writer, with_xmlns, "Border")
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
    if let Some(line_width) = &self.line_width {
      writer.write_str(" LineWidth=\"")?;
      write!(writer, "{}", line_width)?;
      writer.write_char('"')?;
    }
    if let Some(horizonal_corner_radius) = &self.horizonal_corner_radius {
      writer.write_str(" HorizonalCornerRadius=\"")?;
      write!(writer, "{}", horizonal_corner_radius)?;
      writer.write_char('"')?;
    }
    if let Some(vertical_corner_radius) = &self.vertical_corner_radius {
      writer.write_str(" VerticalCornerRadius=\"")?;
      write!(writer, "{}", vertical_corner_radius)?;
      writer.write_char('"')?;
    }
    if let Some(dash_offset) = &self.dash_offset {
      writer.write_str(" DashOffset=\"")?;
      write!(writer, "{}", dash_offset)?;
      writer.write_char('"')?;
    }
    if let Some(dash_pattern) = &self.dash_pattern {
      writer.write_str(" DashPattern=\"")?;
      writer.write_str(&quick_xml::escape::escape(dash_pattern.as_str()))?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    if let Some(border_color) = &self.border_color {
      border_color.write_xml_named(writer, false, "BorderColor")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::Border {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::CtImage {
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
    self.write_xml_named(writer, with_xmlns, "CT_Image")
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
      writer.write_str(" Boundary=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.boundary.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(name) = &self.name {
      writer.write_str(" Name=\"")?;
      writer.write_str(&quick_xml::escape::escape(name.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(visible) = &self.visible {
      writer.write_str(" Visible=\"")?;
      writer.write_str(if *visible { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(ctm) = &self.ctm {
      writer.write_str(" CTM=\"")?;
      writer.write_str(&quick_xml::escape::escape(ctm.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(draw_param) = &self.draw_param {
      writer.write_str(" DrawParam=\"")?;
      write!(writer, "{}", draw_param)?;
      writer.write_char('"')?;
    }
    if let Some(line_width) = &self.line_width {
      writer.write_str(" LineWidth=\"")?;
      write!(writer, "{}", line_width)?;
      writer.write_char('"')?;
    }
    if let Some(cap) = &self.cap {
      writer.write_str(" Cap=\"")?;
      writer.write_str(cap.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(join) = &self.join {
      writer.write_str(" Join=\"")?;
      writer.write_str(join.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(miter_limit) = &self.miter_limit {
      writer.write_str(" MiterLimit=\"")?;
      write!(writer, "{}", miter_limit)?;
      writer.write_char('"')?;
    }
    if let Some(dash_offset) = &self.dash_offset {
      writer.write_str(" DashOffset=\"")?;
      write!(writer, "{}", dash_offset)?;
      writer.write_char('"')?;
    }
    if let Some(dash_pattern) = &self.dash_pattern {
      writer.write_str(" DashPattern=\"")?;
      writer.write_str(&quick_xml::escape::escape(dash_pattern.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(alpha) = &self.alpha {
      writer.write_str(" Alpha=\"")?;
      write!(writer, "{}", alpha)?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" ResourceID=\"")?;
      write!(writer, "{}", self.resource_id)?;
      writer.write_char('"')?;
    }
    if let Some(substitution) = &self.substitution {
      writer.write_str(" Substitution=\"")?;
      write!(writer, "{}", substitution)?;
      writer.write_char('"')?;
    }
    if let Some(image_mask) = &self.image_mask {
      writer.write_str(" ImageMask=\"")?;
      write!(writer, "{}", image_mask)?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    if let Some(actions) = &self.actions {
      actions.write_xml_named(writer, false, "Actions")?;
    }
    if let Some(clips) = &self.clips {
      clips.write_xml_named(writer, false, "Clips")?;
    }
    if let Some(border) = &self.border {
      border.write_xml_named(writer, false, "Border")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::CtImage {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::CtComposite {
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
    self.write_xml_named(writer, with_xmlns, "CT_Composite")
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
      writer.write_str(" Boundary=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.boundary.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(name) = &self.name {
      writer.write_str(" Name=\"")?;
      writer.write_str(&quick_xml::escape::escape(name.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(visible) = &self.visible {
      writer.write_str(" Visible=\"")?;
      writer.write_str(if *visible { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(ctm) = &self.ctm {
      writer.write_str(" CTM=\"")?;
      writer.write_str(&quick_xml::escape::escape(ctm.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(draw_param) = &self.draw_param {
      writer.write_str(" DrawParam=\"")?;
      write!(writer, "{}", draw_param)?;
      writer.write_char('"')?;
    }
    if let Some(line_width) = &self.line_width {
      writer.write_str(" LineWidth=\"")?;
      write!(writer, "{}", line_width)?;
      writer.write_char('"')?;
    }
    if let Some(cap) = &self.cap {
      writer.write_str(" Cap=\"")?;
      writer.write_str(cap.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(join) = &self.join {
      writer.write_str(" Join=\"")?;
      writer.write_str(join.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(miter_limit) = &self.miter_limit {
      writer.write_str(" MiterLimit=\"")?;
      write!(writer, "{}", miter_limit)?;
      writer.write_char('"')?;
    }
    if let Some(dash_offset) = &self.dash_offset {
      writer.write_str(" DashOffset=\"")?;
      write!(writer, "{}", dash_offset)?;
      writer.write_char('"')?;
    }
    if let Some(dash_pattern) = &self.dash_pattern {
      writer.write_str(" DashPattern=\"")?;
      writer.write_str(&quick_xml::escape::escape(dash_pattern.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(alpha) = &self.alpha {
      writer.write_str(" Alpha=\"")?;
      write!(writer, "{}", alpha)?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" ResourceID=\"")?;
      write!(writer, "{}", self.resource_id)?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    if let Some(actions) = &self.actions {
      actions.write_xml_named(writer, false, "Actions")?;
    }
    if let Some(clips) = &self.clips {
      clips.write_xml_named(writer, false, "Clips")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::CtComposite {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::CtPath {
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
    self.write_xml_named(writer, with_xmlns, "CT_Path")
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
      writer.write_str(" Boundary=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.boundary.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(name) = &self.name {
      writer.write_str(" Name=\"")?;
      writer.write_str(&quick_xml::escape::escape(name.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(visible) = &self.visible {
      writer.write_str(" Visible=\"")?;
      writer.write_str(if *visible { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(ctm) = &self.ctm {
      writer.write_str(" CTM=\"")?;
      writer.write_str(&quick_xml::escape::escape(ctm.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(draw_param) = &self.draw_param {
      writer.write_str(" DrawParam=\"")?;
      write!(writer, "{}", draw_param)?;
      writer.write_char('"')?;
    }
    if let Some(line_width) = &self.line_width {
      writer.write_str(" LineWidth=\"")?;
      write!(writer, "{}", line_width)?;
      writer.write_char('"')?;
    }
    if let Some(cap) = &self.cap {
      writer.write_str(" Cap=\"")?;
      writer.write_str(cap.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(join) = &self.join {
      writer.write_str(" Join=\"")?;
      writer.write_str(join.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(miter_limit) = &self.miter_limit {
      writer.write_str(" MiterLimit=\"")?;
      write!(writer, "{}", miter_limit)?;
      writer.write_char('"')?;
    }
    if let Some(dash_offset) = &self.dash_offset {
      writer.write_str(" DashOffset=\"")?;
      write!(writer, "{}", dash_offset)?;
      writer.write_char('"')?;
    }
    if let Some(dash_pattern) = &self.dash_pattern {
      writer.write_str(" DashPattern=\"")?;
      writer.write_str(&quick_xml::escape::escape(dash_pattern.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(alpha) = &self.alpha {
      writer.write_str(" Alpha=\"")?;
      write!(writer, "{}", alpha)?;
      writer.write_char('"')?;
    }
    if let Some(stroke) = &self.stroke {
      writer.write_str(" Stroke=\"")?;
      writer.write_str(if *stroke { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(fill) = &self.fill {
      writer.write_str(" Fill=\"")?;
      writer.write_str(if *fill { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(rule) = &self.rule {
      writer.write_str(" Rule=\"")?;
      writer.write_str(rule.as_xml_str())?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    if let Some(actions) = &self.actions {
      actions.write_xml_named(writer, false, "Actions")?;
    }
    if let Some(clips) = &self.clips {
      clips.write_xml_named(writer, false, "Clips")?;
    }
    if let Some(stroke_color) = &self.stroke_color {
      stroke_color.write_xml_named(writer, false, "StrokeColor")?;
    }
    if let Some(fill_color) = &self.fill_color {
      fill_color.write_xml_named(writer, false, "FillColor")?;
    }
    {
      writer.write_char('<')?;
      writer.write_str("ofd:AbbreviatedData")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(self.abbreviated_data.as_str()))?;
      writer.write_str("</ofd:AbbreviatedData>")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::CtPath {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::CellContent {
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
    self.write_xml_named(writer, with_xmlns, "CellContent")
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
    if let Some(thumbnail) = &self.thumbnail {
      writer.write_str(" Thumbnail=\"")?;
      write!(writer, "{}", thumbnail)?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    for child in &self.xml_children {
      match child {
        crate::schemas::page::CellContentContentChoice::TextObject(child) => {
          child.write_xml_named(writer, false, "TextObject")?
        }
        crate::schemas::page::CellContentContentChoice::PathObject(child) => {
          child.write_xml_named(writer, false, "PathObject")?
        }
        crate::schemas::page::CellContentContentChoice::ImageObject(child) => {
          child.write_xml_named(writer, false, "ImageObject")?
        }
        crate::schemas::page::CellContentContentChoice::CompositeObject(child) => {
          child.write_xml_named(writer, false, "CompositeObject")?
        }
        crate::schemas::page::CellContentContentChoice::PageBlock(child) => {
          child.write_xml_named(writer, false, "PageBlock")?
        }
      }
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::CellContent {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::CtPattern {
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
    self.write_xml_named(writer, with_xmlns, "CT_Pattern")
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
      writer.write_str(" Width=\"")?;
      write!(writer, "{}", self.width)?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" Height=\"")?;
      write!(writer, "{}", self.height)?;
      writer.write_char('"')?;
    }
    if let Some(x_step) = &self.x_step {
      writer.write_str(" XStep=\"")?;
      write!(writer, "{}", x_step)?;
      writer.write_char('"')?;
    }
    if let Some(y_step) = &self.y_step {
      writer.write_str(" YStep=\"")?;
      write!(writer, "{}", y_step)?;
      writer.write_char('"')?;
    }
    if let Some(reflect_method) = &self.reflect_method {
      writer.write_str(" ReflectMethod=\"")?;
      writer.write_str(reflect_method.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(relative_to) = &self.relative_to {
      writer.write_str(" RelativeTo=\"")?;
      writer.write_str(relative_to.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(ctm) = &self.ctm {
      writer.write_str(" CTM=\"")?;
      writer.write_str(&quick_xml::escape::escape(ctm.as_str()))?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    self
      .cell_content
      .write_xml_named(writer, false, "CellContent")?;
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::CtPattern {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::Segment {
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
    self.write_xml_named(writer, with_xmlns, "Segment")
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
    if let Some(position) = &self.position {
      writer.write_str(" Position=\"")?;
      write!(writer, "{}", position)?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    self.color.write_xml_named(writer, false, "Color")?;
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::Segment {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::CtAxialShd {
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
    self.write_xml_named(writer, with_xmlns, "CT_AxialShd")
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
    if let Some(map_type) = &self.map_type {
      writer.write_str(" MapType=\"")?;
      writer.write_str(map_type.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(map_unit) = &self.map_unit {
      writer.write_str(" MapUnit=\"")?;
      write!(writer, "{}", map_unit)?;
      writer.write_char('"')?;
    }
    if let Some(extend) = &self.extend {
      writer.write_str(" Extend=\"")?;
      writer.write_str(extend.as_xml_str())?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" StartPoint=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.start_point.as_str()))?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" EndPoint=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.end_point.as_str()))?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    for child in &self.segment {
      child.write_xml_named(writer, false, "Segment")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::CtAxialShd {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::CtRadialShd {
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
    self.write_xml_named(writer, with_xmlns, "CT_RadialShd")
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
    if let Some(map_type) = &self.map_type {
      writer.write_str(" MapType=\"")?;
      writer.write_str(map_type.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(map_unit) = &self.map_unit {
      writer.write_str(" MapUnit=\"")?;
      write!(writer, "{}", map_unit)?;
      writer.write_char('"')?;
    }
    if let Some(eccentricity) = &self.eccentricity {
      writer.write_str(" Eccentricity=\"")?;
      write!(writer, "{}", eccentricity)?;
      writer.write_char('"')?;
    }
    if let Some(angle) = &self.angle {
      writer.write_str(" Angle=\"")?;
      write!(writer, "{}", angle)?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" StartPoint=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.start_point.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(start_radius) = &self.start_radius {
      writer.write_str(" StartRadius=\"")?;
      write!(writer, "{}", start_radius)?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" EndPoint=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.end_point.as_str()))?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" EndRadius=\"")?;
      write!(writer, "{}", self.end_radius)?;
      writer.write_char('"')?;
    }
    if let Some(extend) = &self.extend {
      writer.write_str(" Extend=\"")?;
      write!(writer, "{}", extend)?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    for child in &self.segment {
      child.write_xml_named(writer, false, "Segment")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::CtRadialShd {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::Point {
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
    self.write_xml_named(writer, with_xmlns, "Point")
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
      writer.write_str(" X=\"")?;
      write!(writer, "{}", self.x)?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" Y=\"")?;
      write!(writer, "{}", self.y)?;
      writer.write_char('"')?;
    }
    if let Some(edge_flag) = &self.edge_flag {
      writer.write_str(" EdgeFlag=\"")?;
      writer.write_str(edge_flag.as_xml_str())?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    self.color.write_xml_named(writer, false, "Color")?;
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::Point {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::CtGouraudShd {
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
    self.write_xml_named(writer, with_xmlns, "CT_GouraudShd")
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
    if let Some(extend) = &self.extend {
      writer.write_str(" Extend=\"")?;
      write!(writer, "{}", extend)?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    for child in &self.point {
      child.write_xml_named(writer, false, "Point")?;
    }
    if let Some(back_color) = &self.back_color {
      back_color.write_xml_named(writer, false, "BackColor")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::CtGouraudShd {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::CtLaGouraudShdPoint {
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
    self.write_xml_named(writer, with_xmlns, "Point")
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
    if let Some(x) = &self.x {
      writer.write_str(" X=\"")?;
      write!(writer, "{}", x)?;
      writer.write_char('"')?;
    }
    if let Some(y) = &self.y {
      writer.write_str(" Y=\"")?;
      write!(writer, "{}", y)?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    self.color.write_xml_named(writer, false, "Color")?;
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::CtLaGouraudShdPoint {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::CtLaGouraudShd {
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
    self.write_xml_named(writer, with_xmlns, "CT_LaGouraudShd")
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
      writer.write_str(" VerticesPerRow=\"")?;
      write!(writer, "{}", self.vertices_per_row)?;
      writer.write_char('"')?;
    }
    if let Some(extend) = &self.extend {
      writer.write_str(" Extend=\"")?;
      write!(writer, "{}", extend)?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    for child in &self.point {
      child.write_xml_named(writer, false, "Point")?;
    }
    if let Some(back_color) = &self.back_color {
      back_color.write_xml_named(writer, false, "BackColor")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::CtLaGouraudShd {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::CtColor {
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
    self.write_xml_named(writer, with_xmlns, "CT_Color")
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
    if let Some(value) = &self.value {
      writer.write_str(" Value=\"")?;
      writer.write_str(&quick_xml::escape::escape(value.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(index) = &self.index {
      writer.write_str(" Index=\"")?;
      write!(writer, "{}", index)?;
      writer.write_char('"')?;
    }
    if let Some(color_space) = &self.color_space {
      writer.write_str(" ColorSpace=\"")?;
      write!(writer, "{}", color_space)?;
      writer.write_char('"')?;
    }
    if let Some(alpha) = &self.alpha {
      writer.write_str(" Alpha=\"")?;
      write!(writer, "{}", alpha)?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    for child in &self.xml_children {
      match child {
        crate::schemas::page::CtColorContentChoice::Pattern(child) => {
          child.write_xml_named(writer, false, "Pattern")?
        }
        crate::schemas::page::CtColorContentChoice::AxialShd(child) => {
          child.write_xml_named(writer, false, "AxialShd")?
        }
        crate::schemas::page::CtColorContentChoice::RadialShd(child) => {
          child.write_xml_named(writer, false, "RadialShd")?
        }
        crate::schemas::page::CtColorContentChoice::GouraudShd(child) => {
          child.write_xml_named(writer, false, "GouraudShd")?
        }
        crate::schemas::page::CtColorContentChoice::LaGourandShd(child) => {
          child.write_xml_named(writer, false, "LaGourandShd")?
        }
      }
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::page::CtColor {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::Action {
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
impl std::fmt::Display for crate::schemas::page::Action {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::Path {
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
    self.write_xml_named(writer, with_xmlns, "Path")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    <crate::schemas::page::CtPath>::write_xml_named(&self.0, writer, with_xmlns, tag_name)
  }
}
impl std::fmt::Display for crate::schemas::page::Path {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::Text {
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
    self.write_xml_named(writer, with_xmlns, "Text")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    <crate::schemas::page::CtText>::write_xml_named(&self.0, writer, with_xmlns, tag_name)
  }
}
impl std::fmt::Display for crate::schemas::page::Text {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::Clip {
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
    self.write_xml_named(writer, with_xmlns, "Clip")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    <crate::schemas::page::CtClip>::write_xml_named(&self.0, writer, with_xmlns, tag_name)
  }
}
impl std::fmt::Display for crate::schemas::page::Clip {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::FillColor {
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
    self.write_xml_named(writer, with_xmlns, "FillColor")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    <crate::schemas::page::CtColor>::write_xml_named(&self.0, writer, with_xmlns, tag_name)
  }
}
impl std::fmt::Display for crate::schemas::page::FillColor {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::StrokeColor {
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
    self.write_xml_named(writer, with_xmlns, "StrokeColor")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    <crate::schemas::page::CtColor>::write_xml_named(&self.0, writer, with_xmlns, tag_name)
  }
}
impl std::fmt::Display for crate::schemas::page::StrokeColor {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::CgTransform {
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
    self.write_xml_named(writer, with_xmlns, "CGTransform")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    <crate::schemas::page::CtCgTransform>::write_xml_named(&self.0, writer, with_xmlns, tag_name)
  }
}
impl std::fmt::Display for crate::schemas::page::CgTransform {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::BorderColor {
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
    self.write_xml_named(writer, with_xmlns, "BorderColor")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    <crate::schemas::page::CtColor>::write_xml_named(&self.0, writer, with_xmlns, tag_name)
  }
}
impl std::fmt::Display for crate::schemas::page::BorderColor {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::Color {
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
    self.write_xml_named(writer, with_xmlns, "Color")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    <crate::schemas::page::CtColor>::write_xml_named(&self.0, writer, with_xmlns, tag_name)
  }
}
impl std::fmt::Display for crate::schemas::page::Color {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::BackColor {
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
    self.write_xml_named(writer, with_xmlns, "BackColor")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    <crate::schemas::page::CtColor>::write_xml_named(&self.0, writer, with_xmlns, tag_name)
  }
}
impl std::fmt::Display for crate::schemas::page::BackColor {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::Pattern {
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
    self.write_xml_named(writer, with_xmlns, "Pattern")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    <crate::schemas::page::CtPattern>::write_xml_named(&self.0, writer, with_xmlns, tag_name)
  }
}
impl std::fmt::Display for crate::schemas::page::Pattern {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::AxialShd {
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
    self.write_xml_named(writer, with_xmlns, "AxialShd")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    <crate::schemas::page::CtAxialShd>::write_xml_named(&self.0, writer, with_xmlns, tag_name)
  }
}
impl std::fmt::Display for crate::schemas::page::AxialShd {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::RadialShd {
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
    self.write_xml_named(writer, with_xmlns, "RadialShd")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    <crate::schemas::page::CtRadialShd>::write_xml_named(&self.0, writer, with_xmlns, tag_name)
  }
}
impl std::fmt::Display for crate::schemas::page::RadialShd {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::GouraudShd {
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
    self.write_xml_named(writer, with_xmlns, "GouraudShd")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    <crate::schemas::page::CtGouraudShd>::write_xml_named(&self.0, writer, with_xmlns, tag_name)
  }
}
impl std::fmt::Display for crate::schemas::page::GouraudShd {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::page::LaGourandShd {
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
    self.write_xml_named(writer, with_xmlns, "LaGourandShd")
  }
  pub(crate) fn write_xml_named<W: std::fmt::Write>(
    &self,
    writer: &mut W,
    with_xmlns: bool,
    tag_name: &str,
  ) -> Result<(), std::fmt::Error> {
    <crate::schemas::page::CtLaGouraudShd>::write_xml_named(&self.0, writer, with_xmlns, tag_name)
  }
}
impl std::fmt::Display for crate::schemas::page::LaGourandShd {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
