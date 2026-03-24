//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

impl crate::schemas::res::CtColorSpaceType {
  pub fn as_xml_str(&self) -> &'static str {
    match self {
      Self::Gray => "GRAY",
      Self::Rgb => "RGB",
      Self::Cmyk => "CMYK",
    }
  }
  pub fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}
impl std::fmt::Display for crate::schemas::res::CtColorSpaceType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_xml_str())
  }
}
impl crate::schemas::res::CtFontCharset {
  pub fn as_xml_str(&self) -> &'static str {
    match self {
      Self::Symbol => "symbol",
      Self::Prc => "prc",
      Self::Big5 => "big5",
      Self::_ShiftJis => "shift-jis",
      Self::Wansung => "wansung",
      Self::Johab => "johab",
      Self::Unicode => "unicode",
    }
  }
  pub fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}
impl std::fmt::Display for crate::schemas::res::CtFontCharset {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_xml_str())
  }
}
impl crate::schemas::res::CtMultiMediaType {
  pub fn as_xml_str(&self) -> &'static str {
    match self {
      Self::Image => "Image",
      Self::Audio => "Audio",
      Self::Video => "Video",
    }
  }
  pub fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}
impl std::fmt::Display for crate::schemas::res::CtMultiMediaType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_xml_str())
  }
}
impl crate::schemas::res::ColorSpace {
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
    self.write_xml_named(writer, with_xmlns, "ColorSpace")
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
    if let Some(bits_per_component) = &self.bits_per_component {
      writer.write_str(" BitsPerComponent=\"")?;
      write!(writer, "{}", bits_per_component)?;
      writer.write_char('"')?;
    }
    if let Some(profile) = &self.profile {
      writer.write_str(" Profile=\"")?;
      writer.write_str(&quick_xml::escape::escape(profile.as_str()))?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" ID=\"")?;
      write!(writer, "{}", self.id)?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    if let Some(palette) = &self.palette {
      palette.write_xml_named(writer, false, "Palette")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::res::ColorSpace {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::res::ColorSpaces {
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
    self.write_xml_named(writer, with_xmlns, "ColorSpaces")
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
    for child in &self.color_space {
      child.write_xml_named(writer, false, "ColorSpace")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::res::ColorSpaces {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::res::DrawParam {
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
    self.write_xml_named(writer, with_xmlns, "DrawParam")
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
    if let Some(relative) = &self.relative {
      writer.write_str(" Relative=\"")?;
      write!(writer, "{}", relative)?;
      writer.write_char('"')?;
    }
    if let Some(line_width) = &self.line_width {
      writer.write_str(" LineWidth=\"")?;
      write!(writer, "{}", line_width)?;
      writer.write_char('"')?;
    }
    if let Some(join) = &self.join {
      writer.write_str(" Join=\"")?;
      writer.write_str(&quick_xml::escape::escape(join.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(cap) = &self.cap {
      writer.write_str(" Cap=\"")?;
      writer.write_str(&quick_xml::escape::escape(cap.as_str()))?;
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
    if let Some(miter_limit) = &self.miter_limit {
      writer.write_str(" MiterLimit=\"")?;
      write!(writer, "{}", miter_limit)?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" ID=\"")?;
      write!(writer, "{}", self.id)?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    if let Some(fill_color) = &self.fill_color {
      fill_color.write_xml_named(writer, false, "FillColor")?;
    }
    if let Some(stroke_color) = &self.stroke_color {
      stroke_color.write_xml_named(writer, false, "StrokeColor")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::res::DrawParam {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::res::DrawParams {
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
    self.write_xml_named(writer, with_xmlns, "DrawParams")
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
    for child in &self.draw_param {
      child.write_xml_named(writer, false, "DrawParam")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::res::DrawParams {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::res::Font {
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
    self.write_xml_named(writer, with_xmlns, "Font")
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
      writer.write_str(" FontName=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.font_name.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(family_name) = &self.family_name {
      writer.write_str(" FamilyName=\"")?;
      writer.write_str(&quick_xml::escape::escape(family_name.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(charset) = &self.charset {
      writer.write_str(" Charset=\"")?;
      writer.write_str(charset.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(italic) = &self.italic {
      writer.write_str(" Italic=\"")?;
      writer.write_str(if *italic { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(bold) = &self.bold {
      writer.write_str(" Bold=\"")?;
      writer.write_str(if *bold { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(serif) = &self.serif {
      writer.write_str(" Serif=\"")?;
      writer.write_str(if *serif { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(fixed_width) = &self.fixed_width {
      writer.write_str(" FixedWidth=\"")?;
      writer.write_str(if *fixed_width { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" ID=\"")?;
      write!(writer, "{}", self.id)?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    if let Some(font_file) = &self.font_file {
      writer.write_char('<')?;
      writer.write_str("ofd:FontFile")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(font_file.as_str()))?;
      writer.write_str("</ofd:FontFile>")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::res::Font {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::res::Fonts {
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
    self.write_xml_named(writer, with_xmlns, "Fonts")
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
    for child in &self.font {
      child.write_xml_named(writer, false, "Font")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::res::Fonts {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::res::MultiMedia {
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
    self.write_xml_named(writer, with_xmlns, "MultiMedia")
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
    if let Some(format) = &self.format {
      writer.write_str(" Format=\"")?;
      writer.write_str(&quick_xml::escape::escape(format.as_str()))?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" ID=\"")?;
      write!(writer, "{}", self.id)?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    {
      writer.write_char('<')?;
      writer.write_str("ofd:MediaFile")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(self.media_file.as_str()))?;
      writer.write_str("</ofd:MediaFile>")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::res::MultiMedia {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::res::MultiMedias {
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
    self.write_xml_named(writer, with_xmlns, "MultiMedias")
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
    for child in &self.multi_media {
      child.write_xml_named(writer, false, "MultiMedia")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::res::MultiMedias {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::res::CompositeGraphicUnit {
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
    self.write_xml_named(writer, with_xmlns, "CompositeGraphicUnit")
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
    {
      writer.write_str(" ID=\"")?;
      write!(writer, "{}", self.id)?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    if let Some(thumbnail) = &self.thumbnail {
      writer.write_char('<')?;
      writer.write_str("ofd:Thumbnail")?;
      writer.write_char('>')?;
      write!(writer, "{}", thumbnail)?;
      writer.write_str("</ofd:Thumbnail>")?;
    }
    if let Some(substitution) = &self.substitution {
      writer.write_char('<')?;
      writer.write_str("ofd:Substitution")?;
      writer.write_char('>')?;
      write!(writer, "{}", substitution)?;
      writer.write_str("</ofd:Substitution>")?;
    }
    self.content.write_xml_named(writer, false, "Content")?;
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::res::CompositeGraphicUnit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::res::CompositeGraphicUnits {
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
    self.write_xml_named(writer, with_xmlns, "CompositeGraphicUnits")
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
    for child in &self.composite_graphic_unit {
      child.write_xml_named(writer, false, "CompositeGraphicUnit")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::res::CompositeGraphicUnits {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::res::Res {
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
    self.write_xml_named(writer, with_xmlns, "Res")
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
      writer.write_str(" BaseLoc=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.base_loc.as_str()))?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    for child in &self.xml_children {
      match child {
        crate::schemas::res::ResContentChoice::ColorSpaces(child) => {
          child.write_xml_named(writer, false, "ColorSpaces")?
        }
        crate::schemas::res::ResContentChoice::DrawParams(child) => {
          child.write_xml_named(writer, false, "DrawParams")?
        }
        crate::schemas::res::ResContentChoice::Fonts(child) => {
          child.write_xml_named(writer, false, "Fonts")?
        }
        crate::schemas::res::ResContentChoice::MultiMedias(child) => {
          child.write_xml_named(writer, false, "MultiMedias")?
        }
        crate::schemas::res::ResContentChoice::CompositeGraphicUnits(child) => {
          child.write_xml_named(writer, false, "CompositeGraphicUnits")?
        }
      }
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::res::Res {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::res::Palette {
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
    self.write_xml_named(writer, with_xmlns, "Palette")
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
    for child in &self.cv {
      writer.write_char('<')?;
      writer.write_str("ofd:CV")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(child.as_str()))?;
      writer.write_str("</ofd:CV>")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::res::Palette {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::res::CtColorSpace {
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
    self.write_xml_named(writer, with_xmlns, "CT_ColorSpace")
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
    if let Some(bits_per_component) = &self.bits_per_component {
      writer.write_str(" BitsPerComponent=\"")?;
      write!(writer, "{}", bits_per_component)?;
      writer.write_char('"')?;
    }
    if let Some(profile) = &self.profile {
      writer.write_str(" Profile=\"")?;
      writer.write_str(&quick_xml::escape::escape(profile.as_str()))?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    if let Some(palette) = &self.palette {
      palette.write_xml_named(writer, false, "Palette")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::res::CtColorSpace {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::res::CtDrawParam {
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
    self.write_xml_named(writer, with_xmlns, "CT_DrawParam")
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
    if let Some(relative) = &self.relative {
      writer.write_str(" Relative=\"")?;
      write!(writer, "{}", relative)?;
      writer.write_char('"')?;
    }
    if let Some(line_width) = &self.line_width {
      writer.write_str(" LineWidth=\"")?;
      write!(writer, "{}", line_width)?;
      writer.write_char('"')?;
    }
    if let Some(join) = &self.join {
      writer.write_str(" Join=\"")?;
      writer.write_str(&quick_xml::escape::escape(join.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(cap) = &self.cap {
      writer.write_str(" Cap=\"")?;
      writer.write_str(&quick_xml::escape::escape(cap.as_str()))?;
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
    if let Some(miter_limit) = &self.miter_limit {
      writer.write_str(" MiterLimit=\"")?;
      write!(writer, "{}", miter_limit)?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    if let Some(fill_color) = &self.fill_color {
      fill_color.write_xml_named(writer, false, "FillColor")?;
    }
    if let Some(stroke_color) = &self.stroke_color {
      stroke_color.write_xml_named(writer, false, "StrokeColor")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::res::CtDrawParam {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::res::CtFont {
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
    self.write_xml_named(writer, with_xmlns, "CT_Font")
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
      writer.write_str(" FontName=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.font_name.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(family_name) = &self.family_name {
      writer.write_str(" FamilyName=\"")?;
      writer.write_str(&quick_xml::escape::escape(family_name.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(charset) = &self.charset {
      writer.write_str(" Charset=\"")?;
      writer.write_str(charset.as_xml_str())?;
      writer.write_char('"')?;
    }
    if let Some(italic) = &self.italic {
      writer.write_str(" Italic=\"")?;
      writer.write_str(if *italic { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(bold) = &self.bold {
      writer.write_str(" Bold=\"")?;
      writer.write_str(if *bold { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(serif) = &self.serif {
      writer.write_str(" Serif=\"")?;
      writer.write_str(if *serif { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(fixed_width) = &self.fixed_width {
      writer.write_str(" FixedWidth=\"")?;
      writer.write_str(if *fixed_width { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    if let Some(font_file) = &self.font_file {
      writer.write_char('<')?;
      writer.write_str("ofd:FontFile")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(font_file.as_str()))?;
      writer.write_str("</ofd:FontFile>")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::res::CtFont {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::res::CtMultiMedia {
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
    self.write_xml_named(writer, with_xmlns, "CT_MultiMedia")
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
    if let Some(format) = &self.format {
      writer.write_str(" Format=\"")?;
      writer.write_str(&quick_xml::escape::escape(format.as_str()))?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    {
      writer.write_char('<')?;
      writer.write_str("ofd:MediaFile")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(self.media_file.as_str()))?;
      writer.write_str("</ofd:MediaFile>")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::res::CtMultiMedia {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::res::CtVectorG {
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
    self.write_xml_named(writer, with_xmlns, "CT_VectorG")
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
    writer.write_char('>')?;
    if let Some(thumbnail) = &self.thumbnail {
      writer.write_char('<')?;
      writer.write_str("ofd:Thumbnail")?;
      writer.write_char('>')?;
      write!(writer, "{}", thumbnail)?;
      writer.write_str("</ofd:Thumbnail>")?;
    }
    if let Some(substitution) = &self.substitution {
      writer.write_char('<')?;
      writer.write_str("ofd:Substitution")?;
      writer.write_char('>')?;
      write!(writer, "{}", substitution)?;
      writer.write_str("</ofd:Substitution>")?;
    }
    self.content.write_xml_named(writer, false, "Content")?;
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::res::CtVectorG {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::res::FillColor {
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
impl std::fmt::Display for crate::schemas::res::FillColor {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::res::StrokeColor {
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
impl std::fmt::Display for crate::schemas::res::StrokeColor {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::res::Content {
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
    <crate::schemas::page::CtPageBlock>::write_xml_named(&self.0, writer, with_xmlns, tag_name)
  }
}
impl std::fmt::Display for crate::schemas::res::Content {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
