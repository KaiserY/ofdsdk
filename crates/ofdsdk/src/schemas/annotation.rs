//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

///注释参数(键值对)。
#[derive(Clone, Debug, Default)]
pub struct Parameter {
  ///注释参数名称。
  pub name: String,
  pub xml_value: String,
}
///一组注释参数。
#[derive(Clone, Debug, Default)]
pub struct Parameters {
  ///注释参数(键值对)。
  pub parameter: Vec<Parameter>,
}
#[derive(Clone, Debug)]
pub enum AppearanceContentChoice {
  TextObject(Box<crate::schemas::page::TextObject>),
  PathObject(Box<crate::schemas::page::PathObject>),
  ImageObject(Box<crate::schemas::page::ImageObject>),
  CompositeObject(Box<crate::schemas::page::CompositeObject>),
  PageBlock(Box<crate::schemas::page::PageBlock>),
}
///注释的静态呈现效果，使用页面块定义来描述。
#[derive(Clone, Debug, Default)]
pub struct Appearance {
  pub boundary: Option<crate::schemas::definitions::StBox>,
  pub xml_children: Vec<AppearanceContentChoice>,
}
///注释信息属性。
#[derive(Clone, Debug, Default)]
pub struct Annot {
  ///注释的标识。
  pub id: crate::schemas::definitions::StId,
  ///注释类型，具体取值请见表 62。
  pub r#type: AnnotType,
  ///注释创建者。
  pub creator: String,
  ///最近一次修改的时间。
  pub last_mod_date: String,
  ///表示该注释对象是否显示。默认值为 true。
  pub visible: Option<bool>,
  ///注释子类型。
  pub subtype: Option<String>,
  ///对象的 Remark 信息是否随页面一起打印。默认值为 true。
  pub print: Option<bool>,
  ///对象的 Remark 信息是否不随页面缩放而同步缩放。默认值为 false。
  pub no_zoom: Option<bool>,
  ///对象的 Remark 信息是否不随页面旋转而同步旋转。默认值为 false。
  pub no_rotate: Option<bool>,
  ///对象的 Remark 信息是否不能被用户更改。默认值为 true。
  pub read_only: Option<bool>,
  ///注释说明内容。
  pub remark: Option<String>,
  ///一组注释参数。
  pub parameters: Option<Parameters>,
  ///注释的静态呈现效果，使用页面块定义来描述。
  pub appearance: Appearance,
}
///注释信息集合。
#[derive(Clone, Debug, Default)]
pub struct PageAnnot {
  ///注释信息。
  pub annot: Vec<Annot>,
}
///注释类型取值。
#[derive(Clone, Debug, Default)]
pub enum AnnotType {
  #[default]
  Link,
  Path,
  Highlight,
  Stamp,
  Watermark,
}
