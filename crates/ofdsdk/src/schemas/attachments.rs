//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

///附件列表根节点。
#[derive(Clone, Debug, Default)]
pub struct Attachments {
  ///附件。
  pub attachment: Vec<CtAttachment>,
}
///附件属性。
#[derive(Clone, Debug, Default)]
pub struct CtAttachment {
  ///附件标识。
  pub id: String,
  ///附件名称。
  pub name: String,
  ///附件格式。
  pub format: Option<String>,
  ///创建时间。
  pub creation_date: Option<String>,
  ///修改时间。
  pub mod_date: Option<String>,
  ///附件大小，以 KB 为单位。
  pub size: Option<f64>,
  ///附件是否可见。默认为 true。
  pub visible: Option<bool>,
  ///附件用途。默认为 none。
  pub usage: Option<String>,
  ///附件内容在包内的路径。
  pub file_loc: crate::schemas::definitions::StLoc,
}
#[derive(Clone, Debug, Default)]
pub struct Attachment(pub CtAttachment);
impl From<CtAttachment> for Attachment {
  fn from(value: CtAttachment) -> Self {
    Self(value)
  }
}
impl From<Attachment> for CtAttachment {
  fn from(value: Attachment) -> Self {
    value.0
  }
}
impl std::ops::Deref for Attachment {
  type Target = CtAttachment;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for Attachment {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
