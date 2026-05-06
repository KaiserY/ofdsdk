//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

///注释所在页。
#[derive(Clone, Debug, Default)]
pub struct Page {
  ///引用注释所在页面的标识。
  pub page_id: crate::schemas::definitions::StRefId,
  ///指向包内的分页注释文件。
  pub file_loc: crate::schemas::definitions::StLoc,
  pub xml_other_attrs: Vec<(std::boxed::Box<str>, std::boxed::Box<str>)>,
  pub xml_other_children: Vec<(usize, std::boxed::Box<str>)>,
}
///注释列表属性。
#[derive(Clone, Debug, Default)]
pub struct Annotations {
  ///注释所在页。
  pub page: Vec<Page>,
  pub xml_other_attrs: Vec<(std::boxed::Box<str>, std::boxed::Box<str>)>,
  pub xml_other_children: Vec<(usize, std::boxed::Box<str>)>,
}
