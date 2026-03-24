//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

///扩展信息列表属性。
#[derive(Clone, Debug, Default)]
pub struct Extensions {
  ///扩展信息节点。
  pub extension: Vec<CtExtension>,
}
///扩展属性。
#[derive(Clone, Debug, Default)]
pub struct Property {
  ///扩展属性名称。
  pub name: String,
  ///扩展属性值类型。
  pub r#type: Option<String>,
  ///扩展属性值。
  pub xml_value: String,
}
#[derive(Clone, Debug)]
pub enum CtExtensionContentChoice {
  Property(Box<Property>),
  Data(Box<String>),
  ExtendData(Box<crate::schemas::definitions::StLoc>),
}
///扩展信息属性。
#[derive(Clone, Debug, Default)]
pub struct CtExtension {
  ///用于生成或解释该自定义对象数据的扩展应用程序名称。
  pub app_name: String,
  ///形成此扩展信息的软件厂商标识。
  pub company: Option<String>,
  ///形成此扩展信息的软件版本。
  pub app_version: Option<String>,
  ///形成此扩展信息的日期时间。
  pub date: Option<String>,
  ///引用扩展项针对的文档项目的标识。
  pub ref_id: crate::schemas::definitions::StRefId,
  pub xml_children: Vec<CtExtensionContentChoice>,
}
#[derive(Clone, Debug, Default)]
pub struct Extension(pub CtExtension);
impl From<CtExtension> for Extension {
  fn from(value: CtExtension) -> Self {
    Self(value)
  }
}
impl From<Extension> for CtExtension {
  fn from(value: Extension) -> Self {
    value.0
  }
}
impl std::ops::Deref for Extension {
  type Target = CtExtension;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for Extension {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
