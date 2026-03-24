//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

///版本属性。
#[derive(Clone, Debug, Default)]
pub struct Version {
  ///版本标识。
  pub id: String,
  ///版本号。
  pub index: i32,
  ///是否是默认版本。默认为 false。
  pub current: Option<bool>,
  ///指向包内的版本描述文件。
  pub base_loc: crate::schemas::definitions::StLoc,
}
///版本序列。
#[derive(Clone, Debug, Default)]
pub struct Versions {
  ///版本描述入口。
  pub version: Vec<Version>,
}
///文件对象入口。
#[derive(Clone, Debug, Default)]
pub struct DocBody {
  ///文档元数据信息描述，文档元数据信息具体结构见图 4。
  pub doc_info: CtDocInfo,
  ///指向文档根节点文档，有关文档根节点描述见 7.5 文档根节点。
  pub doc_root: crate::schemas::definitions::StLoc,
  ///包含多个版本描述节点，用于定义文件因注释和其他改动产生的版本信息，见第 19 章。
  pub versions: Option<Versions>,
  ///指向该文档中签名和签章结构，见第 18 章。
  pub signatures: Option<crate::schemas::definitions::StLoc>,
}
///OFD 主入口。
#[derive(Clone, Debug, Default)]
pub struct Ofd {
  ///文件格式的版本号，取值为“1.0”。
  pub version: String,
  ///文件格式子集类型，取值为“OFD”，表明此文件符合本标准。取值为“OFD-A”，表明此文件符合 OFD 存档规范。
  pub doc_type: OfdDocType,
  ///文件对象入口，可以存在多个，以便在一个文档中包含多个版式文档。
  pub doc_body: Vec<DocBody>,
}
///关键词集合。
#[derive(Clone, Debug, Default)]
pub struct Keywords {
  ///关键词。
  pub keyword: Vec<String>,
}
///用户自定义元数据。
#[derive(Clone, Debug, Default)]
pub struct CustomData {
  ///用户自定义元数据名称。
  pub name: String,
  pub xml_value: String,
}
///用户自定义元数据集合。
#[derive(Clone, Debug, Default)]
pub struct CustomDatas {
  ///用户自定义元数据，可以指定一个名称及其对应的值。
  pub custom_data: Vec<CustomData>,
}
///文档元数据信息描述。
#[derive(Clone, Debug, Default)]
pub struct CtDocInfo {
  ///采用 UUID 算法生成的由 32 个字符组成的文件标识。每个 DocID 在文档创建或生成的时候进行分配。
  pub doc_id: String,
  ///文档标题。标题可以与文件名不同。
  pub title: Option<String>,
  ///文档作者。
  pub author: Option<String>,
  ///文档主题。
  pub subject: Option<String>,
  ///文档摘要与注释。
  pub r#abstract: Option<String>,
  ///文档创建日期。
  pub creation_date: Option<String>,
  ///文档最近修改日期。
  pub mod_date: Option<String>,
  ///文档分类，可取值如下：
  ///Normal——普通文档
  ///EBook——电子书
  ///ENewsPaper——电子报纸
  ///EMagzine——电子期刊杂志
  ///默认值为 Normal。
  pub doc_usage: Option<String>,
  ///文档封面，此路径指向一个图片文件。
  pub cover: Option<crate::schemas::definitions::StLoc>,
  ///关键词集合，每一个关键词用一个“Keyword”子节点来表达。
  pub keywords: Option<Keywords>,
  ///创建文档的应用程序。
  pub creator: Option<String>,
  ///创建文档的应用程序的版本信息。
  pub creator_version: Option<String>,
  ///用户自定义元数据集合。其子节点为 CustomData。
  pub custom_datas: Option<CustomDatas>,
}
#[derive(Clone, Debug, Default)]
pub struct DocInfo(pub CtDocInfo);
impl From<CtDocInfo> for DocInfo {
  fn from(value: CtDocInfo) -> Self {
    Self(value)
  }
}
impl From<DocInfo> for CtDocInfo {
  fn from(value: DocInfo) -> Self {
    value.0
  }
}
impl std::ops::Deref for DocInfo {
  type Target = CtDocInfo;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for DocInfo {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
#[derive(Clone, Debug, Default)]
pub enum OfdDocType {
  #[default]
  Ofd,
}
