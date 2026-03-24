//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

///文件列表文件描述。
#[derive(Clone, Debug, Default)]
pub struct File {
  ///文件列表文件标识。
  pub id: String,
  pub xml_value: crate::schemas::definitions::StLoc,
}
///版本包含的文件列表。
#[derive(Clone, Debug, Default)]
pub struct FileList {
  ///版本包含的文件。
  pub file: Vec<File>,
}
///版本属性。
#[derive(Clone, Debug, Default)]
pub struct DocVersion {
  ///版本标识。
  pub id: String,
  ///该文件适用的格式版本。
  pub version: Option<String>,
  ///版本名称。
  pub name: Option<String>,
  ///创建时间。
  pub creation_date: Option<String>,
  ///版本包含的文件列表。
  pub file_list: FileList,
  ///该版本的入口文件。
  pub doc_root: crate::schemas::definitions::StLoc,
}
