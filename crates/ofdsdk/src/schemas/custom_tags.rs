//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

///自定义标引入口。
#[derive(Clone, Debug, Default)]
pub struct CustomTag {
  pub name_space: String,
  ///指向自定义标引内容节点适用的 Schema 文件。
  pub schema_loc: Option<crate::schemas::definitions::StLoc>,
  ///指向自定义标引文件。该类文件中通过“非接触方式”引用版式内容流中的图元和相关信息。
  pub file_loc: crate::schemas::definitions::StLoc,
}
///自定义标引列表属性。
#[derive(Clone, Debug, Default)]
pub struct CustomTags {
  ///自定义标引入口。
  pub custom_tag: Vec<CustomTag>,
}
