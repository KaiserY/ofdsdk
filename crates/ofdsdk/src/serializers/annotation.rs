//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

impl crate::schemas::annotation::AnnotType {
  pub fn as_xml_str(&self) -> &'static str {
    match self {
      Self::Link => "Link",
      Self::Path => "Path",
      Self::Highlight => "Highlight",
      Self::Stamp => "Stamp",
      Self::Watermark => "Watermark",
    }
  }
  pub fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}
impl std::fmt::Display for crate::schemas::annotation::AnnotType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_xml_str())
  }
}
impl crate::schemas::annotation::Parameter {
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
    self.write_xml_named(writer, with_xmlns, "Parameter")
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
impl std::fmt::Display for crate::schemas::annotation::Parameter {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::annotation::Parameters {
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
    self.write_xml_named(writer, with_xmlns, "Parameters")
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
    for child in &self.parameter {
      child.write_xml_named(writer, false, "Parameter")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::annotation::Parameters {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::annotation::Appearance {
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
    self.write_xml_named(writer, with_xmlns, "Appearance")
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
    if let Some(boundary) = &self.boundary {
      writer.write_str(" Boundary=\"")?;
      writer.write_str(&quick_xml::escape::escape(boundary.as_str()))?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    for child in &self.xml_children {
      match child {
        crate::schemas::annotation::AppearanceContentChoice::TextObject(child) => {
          child.write_xml_named(writer, false, "TextObject")?
        }
        crate::schemas::annotation::AppearanceContentChoice::PathObject(child) => {
          child.write_xml_named(writer, false, "PathObject")?
        }
        crate::schemas::annotation::AppearanceContentChoice::ImageObject(child) => {
          child.write_xml_named(writer, false, "ImageObject")?
        }
        crate::schemas::annotation::AppearanceContentChoice::CompositeObject(child) => {
          child.write_xml_named(writer, false, "CompositeObject")?
        }
        crate::schemas::annotation::AppearanceContentChoice::PageBlock(child) => {
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
impl std::fmt::Display for crate::schemas::annotation::Appearance {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::annotation::Annot {
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
    self.write_xml_named(writer, with_xmlns, "Annot")
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
      writer.write_str(" Type=\"")?;
      writer.write_str(self.r#type.as_xml_str())?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" Creator=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.creator.as_str()))?;
      writer.write_char('"')?;
    }
    {
      writer.write_str(" LastModDate=\"")?;
      writer.write_str(&quick_xml::escape::escape(self.last_mod_date.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(visible) = &self.visible {
      writer.write_str(" Visible=\"")?;
      writer.write_str(if *visible { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(subtype) = &self.subtype {
      writer.write_str(" Subtype=\"")?;
      writer.write_str(&quick_xml::escape::escape(subtype.as_str()))?;
      writer.write_char('"')?;
    }
    if let Some(print) = &self.print {
      writer.write_str(" Print=\"")?;
      writer.write_str(if *print { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(no_zoom) = &self.no_zoom {
      writer.write_str(" NoZoom=\"")?;
      writer.write_str(if *no_zoom { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(no_rotate) = &self.no_rotate {
      writer.write_str(" NoRotate=\"")?;
      writer.write_str(if *no_rotate { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    if let Some(read_only) = &self.read_only {
      writer.write_str(" ReadOnly=\"")?;
      writer.write_str(if *read_only { "true" } else { "false" })?;
      writer.write_char('"')?;
    }
    writer.write_char('>')?;
    if let Some(remark) = &self.remark {
      writer.write_char('<')?;
      writer.write_str("ofd:Remark")?;
      writer.write_char('>')?;
      writer.write_str(&quick_xml::escape::escape(remark.as_str()))?;
      writer.write_str("</ofd:Remark>")?;
    }
    if let Some(parameters) = &self.parameters {
      parameters.write_xml_named(writer, false, "Parameters")?;
    }
    self
      .appearance
      .write_xml_named(writer, false, "Appearance")?;
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::annotation::Annot {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
impl crate::schemas::annotation::PageAnnot {
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
    self.write_xml_named(writer, with_xmlns, "PageAnnot")
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
    for child in &self.annot {
      child.write_xml_named(writer, false, "Annot")?;
    }
    writer.write_str("</ofd:")?;
    writer.write_str(tag_name)?;
    writer.write_char('>')?;
    Ok(())
  }
}
impl std::fmt::Display for crate::schemas::annotation::PageAnnot {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.write_xml(f, true)
  }
}
