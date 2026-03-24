//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

///颜色空间资源描述，在基础类型上扩展定义 ID 属性，类型为 ST_ID。
#[derive(Clone, Debug, Default)]
pub struct ColorSpace {
  pub r#type: CtColorSpaceType,
  pub bits_per_component: Option<i32>,
  pub profile: Option<crate::schemas::definitions::StLoc>,
  pub id: crate::schemas::definitions::StId,
  pub palette: Option<Palette>,
}
///包含了一组颜色空间的描述。
#[derive(Clone, Debug, Default)]
pub struct ColorSpaces {
  pub color_space: Vec<ColorSpace>,
}
///绘制参数资源描述，在基础类型上扩展定义 ID 属性，类型为 ST_ID。
#[derive(Clone, Debug, Default)]
pub struct DrawParam {
  pub relative: Option<crate::schemas::definitions::StRefId>,
  pub line_width: Option<f64>,
  pub join: Option<String>,
  pub cap: Option<String>,
  pub dash_offset: Option<f64>,
  pub dash_pattern: Option<crate::schemas::definitions::StArray>,
  pub miter_limit: Option<f64>,
  pub id: crate::schemas::definitions::StId,
  pub fill_color: Option<crate::schemas::page::CtColor>,
  pub stroke_color: Option<crate::schemas::page::CtColor>,
}
///包含了一组绘制参数的描述。
#[derive(Clone, Debug, Default)]
pub struct DrawParams {
  pub draw_param: Vec<DrawParam>,
}
///字型资源描述，在基础类型上扩展定义 ID 属性，类型为 ST_ID。
#[derive(Clone, Debug, Default)]
pub struct Font {
  ///字型名。
  pub font_name: String,
  ///字型族名，用于匹配替代字型。
  pub family_name: Option<String>,
  ///字型适用的字符分类，用于匹配替代字型。可取值为 symbol、prc、big5、unicode 等。默认值为 unicode。
  pub charset: Option<CtFontCharset>,
  ///是否是斜体字型，用于匹配替代字型。默认值是 false。
  pub italic: Option<bool>,
  ///是否是粗体字型，用于匹配替代字型。默认值是 false。
  pub bold: Option<bool>,
  ///是否是带衬线字型，用于匹配替代字型。默认值是 false。
  pub serif: Option<bool>,
  ///是否是等宽字型，用于匹配替代字型。默认值是 false。
  pub fixed_width: Option<bool>,
  pub id: crate::schemas::definitions::StId,
  ///指向内嵌字型文件，嵌入字型文件应使用 OpenType 格式。
  pub font_file: Option<crate::schemas::definitions::StLoc>,
}
///包含了一组文档所用字型的描述。
#[derive(Clone, Debug, Default)]
pub struct Fonts {
  pub font: Vec<Font>,
}
///多媒体资源描述，在基础类型上扩展定义 ID 属性，类型为 ST_ID。
#[derive(Clone, Debug, Default)]
pub struct MultiMedia {
  pub r#type: CtMultiMediaType,
  pub format: Option<String>,
  pub id: crate::schemas::definitions::StId,
  pub media_file: crate::schemas::definitions::StLoc,
}
///包含了一组文档所用多媒体对象的描述。
#[derive(Clone, Debug, Default)]
pub struct MultiMedias {
  pub multi_media: Vec<MultiMedia>,
}
///矢量图像资源描述，在基础类型上扩展定义 ID 属性，类型为 ST_ID。
#[derive(Clone, Debug, Default)]
pub struct CompositeGraphicUnit {
  ///矢量图像的宽度。超出部分做裁剪处理。
  pub width: f64,
  ///矢量图像的高度。超出部分做裁剪处理。
  pub height: f64,
  pub id: crate::schemas::definitions::StId,
  ///缩略图，指向包内的图像文件。
  pub thumbnail: Option<crate::schemas::definitions::StRefId>,
  ///替换图像，用于高分辨率输出时将缩略图替换为此高分辨率的图像，指向包内的图像文件。
  pub substitution: Option<crate::schemas::definitions::StRefId>,
  ///内容的矢量描述。
  pub content: crate::schemas::page::CtPageBlock,
}
///包含了一组矢量图像（被复合图元对象所引用）的描述。
#[derive(Clone, Debug, Default)]
pub struct CompositeGraphicUnits {
  pub composite_graphic_unit: Vec<CompositeGraphicUnit>,
}
#[derive(Clone, Debug)]
pub enum ResContentChoice {
  ColorSpaces(Box<ColorSpaces>),
  DrawParams(Box<DrawParams>),
  Fonts(Box<Fonts>),
  MultiMedias(Box<MultiMedias>),
  CompositeGraphicUnits(Box<CompositeGraphicUnits>),
}
///资源文件。
#[derive(Clone, Debug, Default)]
pub struct Res {
  ///定义此资源文件的通用数据存储路径。BaseLoc 属性的意义在于明确资源文件存储的位置，比如 R1.xml 中可以指定 BaseLoc 为“./Res”，表明该资源文件中所有数据文件的默认存储位置在当前路径的 Res 目录下。
  pub base_loc: crate::schemas::definitions::StLoc,
  pub xml_children: Vec<ResContentChoice>,
}
///调色板。
#[derive(Clone, Debug, Default)]
pub struct Palette {
  ///调色板中预定义颜色。调色板中颜色的索引编号从 0 开始。
  pub cv: Vec<crate::schemas::definitions::StArray>,
}
///颜色空间属性。
#[derive(Clone, Debug, Default)]
pub struct CtColorSpace {
  ///颜色空间的类型，可取值如下：GRAY、RGB、CMYK。
  pub r#type: CtColorSpaceType,
  ///每个颜色通道所使用的位数，有效取值为 1、2、4、8、16，默认值为 8。
  pub bits_per_component: Option<i32>,
  ///指向包内颜色配置文件。
  pub profile: Option<crate::schemas::definitions::StLoc>,
  ///调色板。
  pub palette: Option<Palette>,
}
///绘制参数属性。
#[derive(Clone, Debug, Default)]
pub struct CtDrawParam {
  ///基础绘制参数，引用资源文件中的绘制参数的标识。
  pub relative: Option<crate::schemas::definitions::StRefId>,
  ///线宽，非负浮点数，指定了路径绘制时线的宽度。默认值为 0.353 mm。
  pub line_width: Option<f64>,
  ///线条连接样式，指定了两个线的端点结合时采用的样式。可取值为 Miter、Round、Bevel，默认值为 Miter。线条连接样式的取值和显示效果之间的关系见表 22。
  pub join: Option<String>,
  ///线端点样式，枚举值，指定了一条线的端点样式。可取值为 Butt、Round、Square，默认值为 Butt。
  pub cap: Option<String>,
  ///线条虚线样式开始的位置，默认值为 0。当 DashPattern 不出现时，该参数无效。
  pub dash_offset: Option<f64>,
  ///线条虚线的重复样式，数组中共含两个值，第一个值代表虚线线段的长度，第二个值代表虚线间隔的长度。默认值为空。
  pub dash_pattern: Option<crate::schemas::definitions::StArray>,
  ///Join 为 Miter 时小角度结合点长度的截断值，默认值为 3.528。当 Join 不等于 Miter 时该参数无效。
  pub miter_limit: Option<f64>,
  ///填充颜色，用以填充路径形成的区域以及文字轮廓内的区域，默认值为透明色。关于颜色的描述见 8.3。
  pub fill_color: Option<crate::schemas::page::CtColor>,
  ///勾边颜色，指定路径绘制的颜色以及文字轮廓的颜色，默认值为黑色。颜色的描述见 8.3。
  pub stroke_color: Option<crate::schemas::page::CtColor>,
}
///字型资源描述。
#[derive(Clone, Debug, Default)]
pub struct CtFont {
  pub font_name: String,
  pub family_name: Option<String>,
  pub charset: Option<CtFontCharset>,
  pub italic: Option<bool>,
  pub bold: Option<bool>,
  pub serif: Option<bool>,
  pub fixed_width: Option<bool>,
  pub font_file: Option<crate::schemas::definitions::StLoc>,
}
///多媒体资源属性。
#[derive(Clone, Debug, Default)]
pub struct CtMultiMedia {
  ///多媒体类型。支持位图图像、视频、音频三种多媒体类型。
  pub r#type: CtMultiMediaType,
  ///资源的格式。支持 BMP、JPEG、PNG、TIFF 及 AVS 等格式，其中 TIFF 格式不支持多页。
  pub format: Option<String>,
  ///指向 OFD 包内的多媒体文件的位置。
  pub media_file: crate::schemas::definitions::StLoc,
}
///矢量图像资源描述。
#[derive(Clone, Debug, Default)]
pub struct CtVectorG {
  pub width: f64,
  pub height: f64,
  pub thumbnail: Option<crate::schemas::definitions::StRefId>,
  pub substitution: Option<crate::schemas::definitions::StRefId>,
  pub content: crate::schemas::page::CtPageBlock,
}
#[derive(Clone, Debug, Default)]
pub struct FillColor(pub crate::schemas::page::CtColor);
impl From<crate::schemas::page::CtColor> for FillColor {
  fn from(value: crate::schemas::page::CtColor) -> Self {
    Self(value)
  }
}
impl From<FillColor> for crate::schemas::page::CtColor {
  fn from(value: FillColor) -> Self {
    value.0
  }
}
impl std::ops::Deref for FillColor {
  type Target = crate::schemas::page::CtColor;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for FillColor {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
#[derive(Clone, Debug, Default)]
pub struct StrokeColor(pub crate::schemas::page::CtColor);
impl From<crate::schemas::page::CtColor> for StrokeColor {
  fn from(value: crate::schemas::page::CtColor) -> Self {
    Self(value)
  }
}
impl From<StrokeColor> for crate::schemas::page::CtColor {
  fn from(value: StrokeColor) -> Self {
    value.0
  }
}
impl std::ops::Deref for StrokeColor {
  type Target = crate::schemas::page::CtColor;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for StrokeColor {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
#[derive(Clone, Debug, Default)]
pub struct Content(pub crate::schemas::page::CtPageBlock);
impl From<crate::schemas::page::CtPageBlock> for Content {
  fn from(value: crate::schemas::page::CtPageBlock) -> Self {
    Self(value)
  }
}
impl From<Content> for crate::schemas::page::CtPageBlock {
  fn from(value: Content) -> Self {
    value.0
  }
}
impl std::ops::Deref for Content {
  type Target = crate::schemas::page::CtPageBlock;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for Content {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
#[derive(Clone, Debug, Default)]
pub enum CtColorSpaceType {
  #[default]
  Gray,
  Rgb,
  Cmyk,
}
#[derive(Clone, Debug, Default)]
pub enum CtFontCharset {
  #[default]
  Symbol,
  Prc,
  Big5,
  _ShiftJis,
  Wansung,
  Johab,
  Unicode,
}
#[derive(Clone, Debug, Default)]
pub enum CtMultiMediaType {
  #[default]
  Image,
  Audio,
  Video,
}
