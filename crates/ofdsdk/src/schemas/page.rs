//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

///该页所使用的模板页。模板页的内容结构和普通页相同，定义在 CommonData 指定的 XML 文件中。一个页可以使用多个模板页。该节点使用时通过 TemplateID 来引用具体的模板，并通过 ZOrder 属性来控制模板在页面中的呈现顺序。
#[derive(Clone, Debug, Default)]
pub struct Template {
  ///引用在文档公用数据 (CommonData) 中定义的模板页标识。
  pub template_id: crate::schemas::definitions::StRefId,
  ///控制模板在页面中的呈现顺序，其类型描述和呈现顺序与 Layer 中 Type 的描述和处理一致。如果多个图层的此属性相同，则应根据其出现的顺序来显示，先出现者先绘制。默认值为 Background。
  pub z_order: Option<TemplateZOrder>,
}
#[derive(Clone, Debug)]
pub enum LayerContentChoice {
  TextObject(Box<TextObject>),
  PathObject(Box<PathObject>),
  ImageObject(Box<ImageObject>),
  CompositeObject(Box<CompositeObject>),
  PageBlock(Box<PageBlock>),
}
///层节点，一页可包含一个或多个层。
#[derive(Clone, Debug, Default)]
pub struct Layer {
  ///层类型描述，预定义的值见表 15。默认为 Body。
  pub r#type: Option<CtLayerType>,
  ///图层的绘制参数，引用资源文件中定义的绘制参数标识。
  pub draw_param: Option<crate::schemas::definitions::StRefId>,
  pub id: crate::schemas::definitions::StId,
  pub xml_children: Vec<LayerContentChoice>,
}
///页面内容描述。
#[derive(Clone, Debug, Default)]
pub struct Content {
  ///层节点，一页可包含一个或多个层。
  pub layer: Vec<Layer>,
}
#[derive(Clone, Debug, Default)]
pub struct Actions {
  pub action: Vec<crate::schemas::definitions::CtAction>,
}
///页对象。
#[derive(Clone, Debug, Default)]
pub struct Page {
  ///该页所使用的模板页。
  pub template: Vec<Template>,
  ///页资源，指向该页使用的资源文件。
  pub page_res: Vec<crate::schemas::definitions::StLoc>,
  ///定义该页页面区域的大小和位置，仅对该页有效。该节点不出现时则使用模板页中的定义，如果模板页不存在或模板页中没有定义页面区域，则使用文件 CommonData 中的定义。
  pub area: Option<crate::schemas::definitions::CtPageArea>,
  ///页面内容描述，该节点不存在时，表示空白页。
  pub content: Option<Content>,
  ///与页面关联的动作序列。当存在多个 Action 对象时，所有动作依次执行。
  pub actions: Option<Actions>,
}
#[derive(Clone, Debug)]
pub enum AreaContentChoice {
  Path(Box<CtPath>),
  Text(Box<CtText>),
}
#[derive(Clone, Debug, Default)]
pub struct Area {
  pub draw_param: Option<crate::schemas::definitions::StRefId>,
  pub ctm: Option<crate::schemas::definitions::StArray>,
  pub xml_children: Vec<AreaContentChoice>,
}
#[derive(Clone, Debug, Default)]
pub struct CtClip {
  pub area: Vec<Area>,
}
///文字对象属性，见 11.2。
#[derive(Clone, Debug, Default)]
pub struct TextObject {
  pub boundary: crate::schemas::definitions::StBox,
  pub name: Option<String>,
  pub visible: Option<bool>,
  pub ctm: Option<crate::schemas::definitions::StArray>,
  pub draw_param: Option<crate::schemas::definitions::StRefId>,
  pub line_width: Option<f64>,
  pub cap: Option<CtGraphicUnitCap>,
  pub join: Option<CtGraphicUnitJoin>,
  pub miter_limit: Option<f64>,
  pub dash_offset: Option<f64>,
  pub dash_pattern: Option<crate::schemas::definitions::StArray>,
  pub alpha: Option<i32>,
  ///引用资源文件中定义的字型的标识。
  pub font: crate::schemas::definitions::StRefId,
  ///字号，单位为毫米。
  pub size: f64,
  ///是否勾边。默认值为 false。
  pub stroke: Option<bool>,
  ///是否填充。默认值为 true。
  pub fill: Option<bool>,
  ///字型在水平方向的放缩比。默认值为 1.0。例如，当 HScale 值为 0.5 时表示实际显示的字宽为原来字宽的一半。
  pub h_scale: Option<f64>,
  ///阅读方向，指定了文字排列的方向，描述见 11.3 文字定位。默认值为 0。
  pub read_direction: Option<i32>,
  ///字符方向，指定了文字放置的方式，具体内容见 11.3 文字定位。默认值为 0。
  pub char_direction: Option<i32>,
  ///文字对象的粗细值；可选取值为 100、200、300、400、500、600、700、800、900。默认值为 400。
  pub weight: Option<CtTextWeight>,
  ///是否是斜体样式。默认值为 false。
  pub italic: Option<bool>,
  pub id: crate::schemas::definitions::StId,
  pub actions: Option<Actions>,
  pub clips: Option<Clips>,
  ///填充颜色。默认为黑色。
  pub fill_color: Option<CtColor>,
  ///勾边颜色。默认为透明色。
  pub stroke_color: Option<CtColor>,
  ///指定字符编码到字符索引之间的变换关系，描述见 11.4 字符变换。
  pub cg_transform: Vec<CtCgTransform>,
  ///文字内容，也就是一段字符编码串。如果字符编码不在 XML 编码方式的字符范围之内，应采用“\”加四位十六进制数的格式转义；文字内容中出现的空格也需要转义。若 TextCode 作为占位符使用时，一律采用“¤”(u00A4) 占位。
  pub text_code: Vec<TextCode>,
}
///图形对象属性，见 9.1。
#[derive(Clone, Debug, Default)]
pub struct PathObject {
  pub boundary: crate::schemas::definitions::StBox,
  pub name: Option<String>,
  pub visible: Option<bool>,
  pub ctm: Option<crate::schemas::definitions::StArray>,
  pub draw_param: Option<crate::schemas::definitions::StRefId>,
  pub line_width: Option<f64>,
  pub cap: Option<CtGraphicUnitCap>,
  pub join: Option<CtGraphicUnitJoin>,
  pub miter_limit: Option<f64>,
  pub dash_offset: Option<f64>,
  pub dash_pattern: Option<crate::schemas::definitions::StArray>,
  pub alpha: Option<i32>,
  ///图形是否被勾边。默认值为 true。
  pub stroke: Option<bool>,
  ///图形是否被填充。默认值为 false。
  pub fill: Option<bool>,
  ///图形的填充规则，当 Fill 属性存在时出现。可选值为 NonZero 和 Even-Odd。默认值为 NonZero。
  pub rule: Option<CtPathRule>,
  pub id: crate::schemas::definitions::StId,
  pub actions: Option<Actions>,
  pub clips: Option<Clips>,
  ///勾边颜色。默认为黑色。
  pub stroke_color: Option<CtColor>,
  ///填充颜色。默认为透明色。
  pub fill_color: Option<CtColor>,
  ///图形轮廓数据，由一系列紧缩的操作符和操作数构成。
  pub abbreviated_data: String,
}
///图像对象，见第 10 章。带有播放视频动作时，见第 12 章。
#[derive(Clone, Debug, Default)]
pub struct ImageObject {
  pub boundary: crate::schemas::definitions::StBox,
  pub name: Option<String>,
  pub visible: Option<bool>,
  pub ctm: Option<crate::schemas::definitions::StArray>,
  pub draw_param: Option<crate::schemas::definitions::StRefId>,
  pub line_width: Option<f64>,
  pub cap: Option<CtGraphicUnitCap>,
  pub join: Option<CtGraphicUnitJoin>,
  pub miter_limit: Option<f64>,
  pub dash_offset: Option<f64>,
  pub dash_pattern: Option<crate::schemas::definitions::StArray>,
  pub alpha: Option<i32>,
  ///引用资源文件中定义的多媒体的标识。
  pub resource_id: crate::schemas::definitions::StRefId,
  ///可替换图像，引用资源文件中定义的多媒体的标识，用于某些情况如高分辨率输出时进行图像替换。
  pub substitution: Option<crate::schemas::definitions::StRefId>,
  ///图像蒙版，引用资源文件中定义的多媒体的标识，用作蒙版的图像应是与 ResourceID 指向的图像相同大小的二值图。
  pub image_mask: Option<crate::schemas::definitions::StRefId>,
  pub id: crate::schemas::definitions::StId,
  pub actions: Option<Actions>,
  pub clips: Option<Clips>,
  ///图像边框设置。
  pub border: Option<Border>,
}
///复合对象，见第 13 章。
#[derive(Clone, Debug, Default)]
pub struct CompositeObject {
  pub boundary: crate::schemas::definitions::StBox,
  pub name: Option<String>,
  pub visible: Option<bool>,
  pub ctm: Option<crate::schemas::definitions::StArray>,
  pub draw_param: Option<crate::schemas::definitions::StRefId>,
  pub line_width: Option<f64>,
  pub cap: Option<CtGraphicUnitCap>,
  pub join: Option<CtGraphicUnitJoin>,
  pub miter_limit: Option<f64>,
  pub dash_offset: Option<f64>,
  pub dash_pattern: Option<crate::schemas::definitions::StArray>,
  pub alpha: Option<i32>,
  ///引用资源文件中定义的矢量图像的标识。
  pub resource_id: crate::schemas::definitions::StRefId,
  pub id: crate::schemas::definitions::StId,
  pub actions: Option<Actions>,
  pub clips: Option<Clips>,
}
#[derive(Clone, Debug)]
pub enum CtPageBlockContentChoice {
  TextObject(Box<TextObject>),
  PathObject(Box<PathObject>),
  ImageObject(Box<ImageObject>),
  CompositeObject(Box<CompositeObject>),
  PageBlock(Box<PageBlock>),
}
///页面块，可以嵌套。
#[derive(Clone, Debug, Default)]
pub struct CtPageBlock {
  pub xml_children: Vec<CtPageBlockContentChoice>,
}
#[derive(Clone, Debug)]
pub enum PageBlockContentChoice {
  TextObject(Box<TextObject>),
  PathObject(Box<PathObject>),
  ImageObject(Box<ImageObject>),
  CompositeObject(Box<CompositeObject>),
  PageBlock(Box<PageBlock>),
}
///页面块，可以嵌套。
#[derive(Clone, Debug, Default)]
pub struct PageBlock {
  pub id: crate::schemas::definitions::StId,
  pub xml_children: Vec<PageBlockContentChoice>,
}
#[derive(Clone, Debug)]
pub enum CtLayerContentChoice {
  TextObject(Box<TextObject>),
  PathObject(Box<PathObject>),
  ImageObject(Box<ImageObject>),
  CompositeObject(Box<CompositeObject>),
  PageBlock(Box<PageBlock>),
}
///层节点。
#[derive(Clone, Debug, Default)]
pub struct CtLayer {
  pub r#type: Option<CtLayerType>,
  pub draw_param: Option<crate::schemas::definitions::StRefId>,
  pub xml_children: Vec<CtLayerContentChoice>,
}
#[derive(Clone, Debug, Default)]
pub struct Clips {
  pub clip: Vec<CtClip>,
}
#[derive(Clone, Debug, Default)]
pub struct CtGraphicUnit {
  pub boundary: crate::schemas::definitions::StBox,
  pub name: Option<String>,
  pub visible: Option<bool>,
  pub ctm: Option<crate::schemas::definitions::StArray>,
  pub draw_param: Option<crate::schemas::definitions::StRefId>,
  pub line_width: Option<f64>,
  pub cap: Option<CtGraphicUnitCap>,
  pub join: Option<CtGraphicUnitJoin>,
  pub miter_limit: Option<f64>,
  pub dash_offset: Option<f64>,
  pub dash_pattern: Option<crate::schemas::definitions::StArray>,
  pub alpha: Option<i32>,
  pub actions: Option<Actions>,
  pub clips: Option<Clips>,
}
///文字内容，也就是一段字符编码串。
#[derive(Clone, Debug, Default)]
pub struct TextCode {
  ///第一个文字的字型原点在对象坐标系下的 X 坐标。当 X 不出现，则采用上一个 TextCode 的 X 值，文字对象中的第一个 TextCode 的 X 属性必选。
  pub x: Option<f64>,
  ///第一个文字的字型原点在对象坐标系下的 Y 坐标。当 Y 不出现，则采用上一个 TextCode 的 Y 值，文字对象中的第一个 TextCode 的 Y 属性必选。
  pub y: Option<f64>,
  ///double 型数值队列，队列中的每个值代表后一个文字与前一个文字之间在 X 方向的偏移值。DeltaX 不出现时，表示文字的绘制点在 X 方向不做偏移。
  pub delta_x: Option<crate::schemas::definitions::StArray>,
  ///double 型数值队列，队列中的每个值代表后一个文字与前一个文字之间在 Y 方向的偏移值。DeltaY 不出现时，表示文字的绘制点在 Y 方向不做偏移。
  pub delta_y: Option<crate::schemas::definitions::StArray>,
  pub xml_value: String,
}
#[derive(Clone, Debug, Default)]
pub struct CtText {
  pub boundary: crate::schemas::definitions::StBox,
  pub name: Option<String>,
  pub visible: Option<bool>,
  pub ctm: Option<crate::schemas::definitions::StArray>,
  pub draw_param: Option<crate::schemas::definitions::StRefId>,
  pub line_width: Option<f64>,
  pub cap: Option<CtGraphicUnitCap>,
  pub join: Option<CtGraphicUnitJoin>,
  pub miter_limit: Option<f64>,
  pub dash_offset: Option<f64>,
  pub dash_pattern: Option<crate::schemas::definitions::StArray>,
  pub alpha: Option<i32>,
  pub font: crate::schemas::definitions::StRefId,
  pub size: f64,
  pub stroke: Option<bool>,
  pub fill: Option<bool>,
  pub h_scale: Option<f64>,
  pub read_direction: Option<i32>,
  pub char_direction: Option<i32>,
  pub weight: Option<CtTextWeight>,
  pub italic: Option<bool>,
  pub actions: Option<Actions>,
  pub clips: Option<Clips>,
  pub fill_color: Option<CtColor>,
  pub stroke_color: Option<CtColor>,
  pub cg_transform: Vec<CtCgTransform>,
  pub text_code: Vec<TextCode>,
}
///指定字符编码到字符索引之间的变换关系，描述见 11.4 字符变换。
#[derive(Clone, Debug, Default)]
pub struct CtCgTransform {
  ///TextCode 中字符编码的起始位置，从 0 开始。
  pub code_position: i32,
  ///变换关系中字符的数量，该数值应大于或等于 1，否则属于错误描述，默认为 1。
  pub code_count: Option<i32>,
  ///变换关系中字形索引的个数，该数值应大于或等于 1，否则属于错误描述，默认为 1。
  pub glyph_count: Option<i32>,
  ///变换后的字形索引列表。
  pub glyphs: Option<crate::schemas::definitions::StArray>,
}
///图像边框设置。
#[derive(Clone, Debug, Default)]
pub struct Border {
  ///边框线宽，如果为 0 则表示边框不进行绘制。默认值为 0.353 mm。
  pub line_width: Option<f64>,
  pub horizonal_corner_radius: Option<f64>,
  ///边框垂直角半径。默认值为 0。
  pub vertical_corner_radius: Option<f64>,
  ///边框虚线重复样式开始的位置，边框的起始点位置为左上角，绕行方向为顺时针。默认值为 0。
  pub dash_offset: Option<f64>,
  ///边框虚线重复样式，边框的起始点位置为左上角，绕行方向为顺时针。
  pub dash_pattern: Option<crate::schemas::definitions::StArray>,
  ///边框颜色，有关边框颜色描述见 8.3.2 基本颜色。默认为黑色。
  pub border_color: Option<CtColor>,
}
#[derive(Clone, Debug, Default)]
pub struct CtImage {
  pub boundary: crate::schemas::definitions::StBox,
  pub name: Option<String>,
  pub visible: Option<bool>,
  pub ctm: Option<crate::schemas::definitions::StArray>,
  pub draw_param: Option<crate::schemas::definitions::StRefId>,
  pub line_width: Option<f64>,
  pub cap: Option<CtGraphicUnitCap>,
  pub join: Option<CtGraphicUnitJoin>,
  pub miter_limit: Option<f64>,
  pub dash_offset: Option<f64>,
  pub dash_pattern: Option<crate::schemas::definitions::StArray>,
  pub alpha: Option<i32>,
  pub resource_id: crate::schemas::definitions::StRefId,
  pub substitution: Option<crate::schemas::definitions::StRefId>,
  pub image_mask: Option<crate::schemas::definitions::StRefId>,
  pub actions: Option<Actions>,
  pub clips: Option<Clips>,
  pub border: Option<Border>,
}
#[derive(Clone, Debug, Default)]
pub struct CtComposite {
  pub boundary: crate::schemas::definitions::StBox,
  pub name: Option<String>,
  pub visible: Option<bool>,
  pub ctm: Option<crate::schemas::definitions::StArray>,
  pub draw_param: Option<crate::schemas::definitions::StRefId>,
  pub line_width: Option<f64>,
  pub cap: Option<CtGraphicUnitCap>,
  pub join: Option<CtGraphicUnitJoin>,
  pub miter_limit: Option<f64>,
  pub dash_offset: Option<f64>,
  pub dash_pattern: Option<crate::schemas::definitions::StArray>,
  pub alpha: Option<i32>,
  pub resource_id: crate::schemas::definitions::StRefId,
  pub actions: Option<Actions>,
  pub clips: Option<Clips>,
}
#[derive(Clone, Debug, Default)]
pub struct CtPath {
  pub boundary: crate::schemas::definitions::StBox,
  pub name: Option<String>,
  pub visible: Option<bool>,
  pub ctm: Option<crate::schemas::definitions::StArray>,
  pub draw_param: Option<crate::schemas::definitions::StRefId>,
  pub line_width: Option<f64>,
  pub cap: Option<CtGraphicUnitCap>,
  pub join: Option<CtGraphicUnitJoin>,
  pub miter_limit: Option<f64>,
  pub dash_offset: Option<f64>,
  pub dash_pattern: Option<crate::schemas::definitions::StArray>,
  pub alpha: Option<i32>,
  pub stroke: Option<bool>,
  pub fill: Option<bool>,
  pub rule: Option<CtPathRule>,
  pub actions: Option<Actions>,
  pub clips: Option<Clips>,
  pub stroke_color: Option<CtColor>,
  pub fill_color: Option<CtColor>,
  pub abbreviated_data: String,
}
#[derive(Clone, Debug)]
pub enum CellContentContentChoice {
  TextObject(Box<TextObject>),
  PathObject(Box<PathObject>),
  ImageObject(Box<ImageObject>),
  CompositeObject(Box<CompositeObject>),
  PageBlock(Box<PageBlock>),
}
///底纹单元。
#[derive(Clone, Debug, Default)]
pub struct CellContent {
  ///引用资源文件中缩略图图像的标识。
  pub thumbnail: Option<crate::schemas::definitions::StRefId>,
  pub xml_children: Vec<CellContentContentChoice>,
}
///底纹属性。
#[derive(Clone, Debug, Default)]
pub struct CtPattern {
  ///底纹单元的宽度。
  pub width: f64,
  ///底纹单元的高度。
  pub height: f64,
  ///X 方向底纹单元间距，默认值为底纹单元的宽度。若设定值小于底纹单元的宽度时，应按默认值处理。
  pub x_step: Option<f64>,
  ///Y 方向底纹单元间距，默认值为底纹单元的高度。若设定值小于底纹单元的高度时，应按默认值处理。
  pub y_step: Option<f64>,
  ///描述底纹单元的映像翻转方式，枚举值，默认值为 Normal。
  pub reflect_method: Option<CtPatternReflectMethod>,
  ///底纹单元起始绘制位置，可取值为 Page 或 Object，默认值为 Object。
  pub relative_to: Option<CtPatternRelativeTo>,
  ///底纹单元的变换矩阵，用于某些需要对底纹单元进行平移旋转变换的场合，默认为单位矩阵。底纹呈现时先做 XStep、YStep 排列，然后一起做 CTM 处理。
  pub ctm: Option<crate::schemas::definitions::StArray>,
  pub cell_content: CellContent,
}
///颜色段。
#[derive(Clone, Debug, Default)]
pub struct Segment {
  ///用于确定 StartPoint 和 EndPoint 中的各颜色的位置值，取值范围是 [0, 1.0]，各段颜色的 Position 值应根据颜色出现的顺序递增。
  pub position: Option<f64>,
  ///该段的颜色，应是基本颜色。
  pub color: CtColor,
}
///轴向渐变属性。
#[derive(Clone, Debug, Default)]
pub struct CtAxialShd {
  ///渐变绘制的方式，可选值为 Direct、Repeat、Reflect。默认值为 Direct。
  pub map_type: Option<CtAxialShdMapType>,
  ///轴线一个渐变区间的长度，当 MapType 的值不等于 Direct 时出现。默认值为轴线长度。
  pub map_unit: Option<f64>,
  ///轴线延长线方向是否继续绘制渐变。可选值为 0、1、2、3。
  pub extend: Option<CtAxialShdExtend>,
  ///轴线的起始点。
  pub start_point: crate::schemas::definitions::StPos,
  ///轴线的结束点。
  pub end_point: crate::schemas::definitions::StPos,
  ///颜色段，至少出现两个。
  pub segment: Vec<Segment>,
}
///径向渐变属性。
#[derive(Clone, Debug, Default)]
pub struct CtRadialShd {
  ///渐变绘制的方式，可选值为 Direct、Repeat、Reflect。默认值为 Direct。
  pub map_type: Option<CtRadialShdMapType>,
  ///渐变区间的长度，当 MapType 的值不等于 Direct 时出现。
  pub map_unit: Option<f64>,
  ///渐变椭圆的偏心率。
  pub eccentricity: Option<f64>,
  ///渐变椭圆的旋转角度。
  pub angle: Option<f64>,
  ///渐变起始点。
  pub start_point: crate::schemas::definitions::StPos,
  ///渐变起始半径。
  pub start_radius: Option<f64>,
  ///渐变结束点。
  pub end_point: crate::schemas::definitions::StPos,
  ///渐变结束半径。
  pub end_radius: f64,
  ///是否向外延长绘制渐变。可选值为 0、1、2、3。
  pub extend: Option<i32>,
  ///颜色段，至少出现两个。
  pub segment: Vec<Segment>,
}
///高洛德渐变控制点。
#[derive(Clone, Debug, Default)]
pub struct Point {
  ///控制点 X 坐标。
  pub x: f64,
  ///控制点 Y 坐标。
  pub y: f64,
  ///控制点边界标记。
  pub edge_flag: Option<PointEdgeFlag>,
  ///控制点颜色。
  pub color: CtColor,
}
///高洛德渐变属性。
#[derive(Clone, Debug, Default)]
pub struct CtGouraudShd {
  ///是否向外延长绘制渐变。
  pub extend: Option<i32>,
  ///渐变控制点。
  pub point: Vec<Point>,
  ///背景颜色。
  pub back_color: Option<CtColor>,
}
///格构高洛德渐变控制点。
#[derive(Clone, Debug, Default)]
pub struct CtLaGouraudShdPoint {
  ///控制点 X 坐标。
  pub x: Option<f64>,
  ///控制点 Y 坐标。
  pub y: Option<f64>,
  ///控制点颜色。
  pub color: CtColor,
}
///格构高洛德渐变属性。
#[derive(Clone, Debug, Default)]
pub struct CtLaGouraudShd {
  ///每行顶点数。
  pub vertices_per_row: i32,
  ///是否向外延长绘制渐变。
  pub extend: Option<i32>,
  ///控制点集合。
  pub point: Vec<CtLaGouraudShdPoint>,
  ///背景颜色。
  pub back_color: Option<CtColor>,
}
#[derive(Clone, Debug)]
pub enum CtColorContentChoice {
  Pattern(Box<CtPattern>),
  AxialShd(Box<CtAxialShd>),
  RadialShd(Box<CtRadialShd>),
  GouraudShd(Box<CtGouraudShd>),
  LaGourandShd(Box<CtLaGouraudShd>),
}
///颜色属性。
#[derive(Clone, Debug, Default)]
pub struct CtColor {
  ///颜色值，指定了当前颜色空间下各通道的取值。Value 的取值应符合“通道 1 通道 2 通道 3 …”格式。此属性不出现时，应采用 Index 属性从颜色空间的调色板中的取值。当二者都不出现时，该颜色各通道的值全部为 0。
  pub value: Option<crate::schemas::definitions::StArray>,
  ///调色板中颜色的编号，非负整数，将从当前颜色空间的调色板中取出相应索引的预定义颜色用来绘制。索引从 0 开始。
  pub index: Option<i32>,
  ///引用资源文件中颜色空间的标识。默认值为文档设定的颜色空间。
  pub color_space: Option<crate::schemas::definitions::StRefId>,
  ///颜色透明度，在 0~255 之间取值。默认为 255，表示完全不透明。
  pub alpha: Option<i32>,
  pub xml_children: Vec<CtColorContentChoice>,
}
#[derive(Clone, Debug, Default)]
pub struct Action(pub crate::schemas::definitions::CtAction);
impl From<crate::schemas::definitions::CtAction> for Action {
  fn from(value: crate::schemas::definitions::CtAction) -> Self {
    Self(value)
  }
}
impl From<Action> for crate::schemas::definitions::CtAction {
  fn from(value: Action) -> Self {
    value.0
  }
}
impl std::ops::Deref for Action {
  type Target = crate::schemas::definitions::CtAction;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for Action {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
#[derive(Clone, Debug, Default)]
pub struct Path(pub CtPath);
impl From<CtPath> for Path {
  fn from(value: CtPath) -> Self {
    Self(value)
  }
}
impl From<Path> for CtPath {
  fn from(value: Path) -> Self {
    value.0
  }
}
impl std::ops::Deref for Path {
  type Target = CtPath;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for Path {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
#[derive(Clone, Debug, Default)]
pub struct Text(pub CtText);
impl From<CtText> for Text {
  fn from(value: CtText) -> Self {
    Self(value)
  }
}
impl From<Text> for CtText {
  fn from(value: Text) -> Self {
    value.0
  }
}
impl std::ops::Deref for Text {
  type Target = CtText;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for Text {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
#[derive(Clone, Debug, Default)]
pub struct Clip(pub CtClip);
impl From<CtClip> for Clip {
  fn from(value: CtClip) -> Self {
    Self(value)
  }
}
impl From<Clip> for CtClip {
  fn from(value: Clip) -> Self {
    value.0
  }
}
impl std::ops::Deref for Clip {
  type Target = CtClip;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for Clip {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
#[derive(Clone, Debug, Default)]
pub struct FillColor(pub CtColor);
impl From<CtColor> for FillColor {
  fn from(value: CtColor) -> Self {
    Self(value)
  }
}
impl From<FillColor> for CtColor {
  fn from(value: FillColor) -> Self {
    value.0
  }
}
impl std::ops::Deref for FillColor {
  type Target = CtColor;
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
pub struct StrokeColor(pub CtColor);
impl From<CtColor> for StrokeColor {
  fn from(value: CtColor) -> Self {
    Self(value)
  }
}
impl From<StrokeColor> for CtColor {
  fn from(value: StrokeColor) -> Self {
    value.0
  }
}
impl std::ops::Deref for StrokeColor {
  type Target = CtColor;
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
pub struct CgTransform(pub CtCgTransform);
impl From<CtCgTransform> for CgTransform {
  fn from(value: CtCgTransform) -> Self {
    Self(value)
  }
}
impl From<CgTransform> for CtCgTransform {
  fn from(value: CgTransform) -> Self {
    value.0
  }
}
impl std::ops::Deref for CgTransform {
  type Target = CtCgTransform;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for CgTransform {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
#[derive(Clone, Debug, Default)]
pub struct BorderColor(pub CtColor);
impl From<CtColor> for BorderColor {
  fn from(value: CtColor) -> Self {
    Self(value)
  }
}
impl From<BorderColor> for CtColor {
  fn from(value: BorderColor) -> Self {
    value.0
  }
}
impl std::ops::Deref for BorderColor {
  type Target = CtColor;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for BorderColor {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
#[derive(Clone, Debug, Default)]
pub struct Color(pub CtColor);
impl From<CtColor> for Color {
  fn from(value: CtColor) -> Self {
    Self(value)
  }
}
impl From<Color> for CtColor {
  fn from(value: Color) -> Self {
    value.0
  }
}
impl std::ops::Deref for Color {
  type Target = CtColor;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for Color {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
#[derive(Clone, Debug, Default)]
pub struct BackColor(pub CtColor);
impl From<CtColor> for BackColor {
  fn from(value: CtColor) -> Self {
    Self(value)
  }
}
impl From<BackColor> for CtColor {
  fn from(value: BackColor) -> Self {
    value.0
  }
}
impl std::ops::Deref for BackColor {
  type Target = CtColor;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for BackColor {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
#[derive(Clone, Debug, Default)]
pub struct Pattern(pub CtPattern);
impl From<CtPattern> for Pattern {
  fn from(value: CtPattern) -> Self {
    Self(value)
  }
}
impl From<Pattern> for CtPattern {
  fn from(value: Pattern) -> Self {
    value.0
  }
}
impl std::ops::Deref for Pattern {
  type Target = CtPattern;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for Pattern {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
#[derive(Clone, Debug, Default)]
pub struct AxialShd(pub CtAxialShd);
impl From<CtAxialShd> for AxialShd {
  fn from(value: CtAxialShd) -> Self {
    Self(value)
  }
}
impl From<AxialShd> for CtAxialShd {
  fn from(value: AxialShd) -> Self {
    value.0
  }
}
impl std::ops::Deref for AxialShd {
  type Target = CtAxialShd;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for AxialShd {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
#[derive(Clone, Debug, Default)]
pub struct RadialShd(pub CtRadialShd);
impl From<CtRadialShd> for RadialShd {
  fn from(value: CtRadialShd) -> Self {
    Self(value)
  }
}
impl From<RadialShd> for CtRadialShd {
  fn from(value: RadialShd) -> Self {
    value.0
  }
}
impl std::ops::Deref for RadialShd {
  type Target = CtRadialShd;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for RadialShd {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
#[derive(Clone, Debug, Default)]
pub struct GouraudShd(pub CtGouraudShd);
impl From<CtGouraudShd> for GouraudShd {
  fn from(value: CtGouraudShd) -> Self {
    Self(value)
  }
}
impl From<GouraudShd> for CtGouraudShd {
  fn from(value: GouraudShd) -> Self {
    value.0
  }
}
impl std::ops::Deref for GouraudShd {
  type Target = CtGouraudShd;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for GouraudShd {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
#[derive(Clone, Debug, Default)]
pub struct LaGourandShd(pub CtLaGouraudShd);
impl From<CtLaGouraudShd> for LaGourandShd {
  fn from(value: CtLaGouraudShd) -> Self {
    Self(value)
  }
}
impl From<LaGourandShd> for CtLaGouraudShd {
  fn from(value: LaGourandShd) -> Self {
    value.0
  }
}
impl std::ops::Deref for LaGourandShd {
  type Target = CtLaGouraudShd;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for LaGourandShd {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
///模板页的默认图层类型，见表 15。
#[derive(Clone, Debug, Default)]
pub enum TemplateZOrder {
  #[default]
  Background,
  Foreground,
}
///层类型描述，预定义的值见表 15。默认为 Body。
#[derive(Clone, Debug, Default)]
pub enum CtLayerType {
  #[default]
  Body,
  Background,
  Foreground,
  Custom,
}
#[derive(Clone, Debug, Default)]
pub enum CtGraphicUnitCap {
  #[default]
  Butt,
  Round,
  Square,
}
#[derive(Clone, Debug, Default)]
pub enum CtGraphicUnitJoin {
  #[default]
  Miter,
  Round,
  Bevel,
}
///文字对象的粗细值。
#[derive(Clone, Debug, Default)]
pub enum CtTextWeight {
  #[default]
  _0,
  _100,
  _200,
  _300,
  _400,
  _500,
  _600,
  _700,
  _800,
  _900,
  _1000,
}
///图形的填充规则，当 Fill 属性存在时出现。可选值为 NonZero 和 Even-Odd。默认值为 NonZero。
#[derive(Clone, Debug, Default)]
pub enum CtPathRule {
  #[default]
  NonZero,
  _EvenOdd,
}
///描述底纹单元的映像翻转方式，枚举值，默认值为 Normal。
#[derive(Clone, Debug, Default)]
pub enum CtPatternReflectMethod {
  #[default]
  Normal,
  Row,
  Column,
  RowAndColumn,
}
///底纹单元起始绘制位置，可取值为 Page 或 Object，默认值为 Object。
#[derive(Clone, Debug, Default)]
pub enum CtPatternRelativeTo {
  #[default]
  Page,
  Object,
}
///渐变绘制的方式，可选值为 Direct、Repeat、Reflect。
#[derive(Clone, Debug, Default)]
pub enum CtAxialShdMapType {
  #[default]
  Direct,
  Repeat,
  Reflect,
}
///轴线延长线方向是否继续绘制渐变。
#[derive(Clone, Debug, Default)]
pub enum CtAxialShdExtend {
  #[default]
  _0,
  _1,
  _2,
  _3,
}
///渐变绘制的方式，可选值为 Direct、Repeat、Reflect。
#[derive(Clone, Debug, Default)]
pub enum CtRadialShdMapType {
  #[default]
  Direct,
  Repeat,
  Reflect,
}
///控制点边界标记。
#[derive(Clone, Debug, Default)]
pub enum PointEdgeFlag {
  #[default]
  _0,
  _1,
  _2,
}
