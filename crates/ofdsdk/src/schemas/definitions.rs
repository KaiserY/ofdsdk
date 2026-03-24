//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

///跳转的目标区域。
#[derive(Clone, Debug, Default)]
pub struct CtDest {
  ///声明目标区域的描述方法，可取值为 XYZ、Fit、FitH、FitV、FitR。
  pub r#type: CtDestType,
  ///引用跳转目标页面的标识。
  pub page_id: StRefId,
  ///目标区域左上角 x 坐标。默认为 0。
  pub left: Option<f64>,
  ///目标区域左上角 y 坐标。默认为 0。
  pub top: Option<f64>,
  ///目标区域右下角 x 坐标。
  pub right: Option<f64>,
  ///目标区域右下角 y 坐标。
  pub bottom: Option<f64>,
  ///目标区域页面缩放比例，为 0 或不出现则按照当前缩放比例跳转，可取值范围 [0.1, 64.0]。
  pub zoom: Option<f64>,
}
///页面区域。
#[derive(Clone, Debug, Default)]
pub struct CtPageArea {
  ///页面物理区域，左上角的坐标为页面空间坐标系的原点。
  pub physical_box: StBox,
  ///显示区域，页面内容实际显示或打印输出的区域，位于页面物理区域内，包含页眉、页脚、版心等内容。
  ///[例外处理]如果显示区域不完全位于页面物理区域内，页面物理区域外的部分则被忽略。如果显示区域完全位于页面物理区域外，则该页为空白页。
  pub application_box: Option<StBox>,
  ///版心区域，即文件的正文区域，位于显示区域内。左上角的坐标决定了其在显示区域内的位置。
  ///[例外处理]如果版心区域不完全位于显示区域内，显示区域外的部分则被忽略。如果版心区域完全位于显示区域外，则版心内容不被绘制。
  pub content_box: Option<StBox>,
  ///出血区域，即超出设备性能限制的额外出血区域，位于页面物理区域外。不出现时，默认值为页面物理区域。
  ///[例外处理]如果出血区域不完全位于页面物理区域外，页面物理区域内的部分则被忽略。如果出血区域完全位于页面物理区域内，出血区域无效。
  pub bleed_box: Option<StBox>,
}
///跳转的目标书签。
#[derive(Clone, Debug, Default)]
pub struct Bookmark {
  ///目标书签的名称，引用文档书签定义中的名称。
  pub name: String,
}
#[derive(Clone, Debug)]
pub enum GotoContentChoice {
  Dest(Box<CtDest>),
  Bookmark(Box<Bookmark>),
}
///本文档内的跳转。
#[derive(Clone, Debug, Default)]
pub struct Goto {
  pub xml_children: Vec<GotoContentChoice>,
}
///打开或访问一个 URI 链接。
#[derive(Clone, Debug, Default)]
pub struct Uri {
  ///目标 URI 的位置。
  pub uri: String,
  ///Base URI，用于相对地址。
  pub base: Option<String>,
  pub target: Option<String>,
}
///打开本文档附件。
#[derive(Clone, Debug, Default)]
pub struct GotoA {
  ///附件的标识。
  pub attach_id: String,
  ///是否在新窗口中打开。
  pub new_window: Option<bool>,
}
///播放一段音频。
#[derive(Clone, Debug, Default)]
pub struct Sound {
  ///引用资源文件中的音频资源标识。
  pub resource_id: StRefId,
  ///播放的音量，取值范围 [0, 100]。默认值为 100。
  pub volume: Option<i32>,
  ///此音频是否需要循环播放。如果此属性为 true，则 Synchronous 值无效。默认为 false。
  pub repeat: Option<bool>,
  ///是否同步播放。true 表示后续动作应等待此音频播放结束后才能开始，false 表示立刻返回并开始下一个动作。默认值为 false。
  pub synchronous: Option<bool>,
}
///播放一段视频。
#[derive(Clone, Debug, Default)]
pub struct Movie {
  ///引用资源文件中定义的视频资源标识。
  pub resource_id: StRefId,
  ///放映参数，见表 59。默认值为 Play。
  pub operator: Option<MovieOperator>,
}
#[derive(Clone, Debug)]
pub enum CtActionContentChoice {
  Goto(Box<Goto>),
  Uri(Box<Uri>),
  GotoA(Box<GotoA>),
  Sound(Box<Sound>),
  Movie(Box<Movie>),
}
///动作类型属性。
#[derive(Clone, Debug, Default)]
pub struct CtAction {
  ///事件类型，触发动作的条件，事件的具体类型见表 52。
  pub event: CtActionEvent,
  ///指定多个复杂区域为该链接对象的启动区域，不出现时以所在图元或页面的外接矩形作为启动区域，见 9.3。
  pub region: Option<CtRegion>,
  pub xml_children: Vec<CtActionContentChoice>,
}
///从当前点移动到新的当前点。
#[derive(Clone, Debug, Default)]
pub struct Move {
  ///移动后新的当前绘制点。
  pub point1: StPos,
}
///从当前点连接一条到指定点的线段，并将当前点移动到指定点。
#[derive(Clone, Debug, Default)]
pub struct Line {
  ///线段的结束点。
  pub point1: StPos,
}
///从当前点连接一条到 Point2 的二次贝塞尔曲线，并将当前点移动到 Point2，此贝塞尔曲线使用 Point1 作为其控制点。
#[derive(Clone, Debug, Default)]
pub struct QuadraticBezier {
  ///二次贝塞尔曲线的控制点。
  pub point1: StPos,
  ///二次贝塞尔曲线的结束点，下一路径的起始点。
  pub point2: StPos,
}
///从当前点连接一条到 Point3 的三次贝塞尔曲线，并将当前点移动到 Point3，使用 Point1 和 Point2 作为控制点。
#[derive(Clone, Debug, Default)]
pub struct CubicBezier {
  ///三次贝塞尔曲线的第一个控制点。
  pub point1: Option<StPos>,
  ///三次贝塞尔曲线的第二个控制点。
  pub point2: Option<StPos>,
  ///三次贝塞尔曲线的结束点，下一路径的起始点。
  pub point3: StPos,
}
///从当前点连接一条到 EndPoint 点的圆弧，并将当前点移动到 EndPoint 点。
#[derive(Clone, Debug, Default)]
pub struct Arc {
  ///弧线方向是否为顺时针。true 表示由圆弧起始点到结束点是顺时针旋转，false 表示由圆弧起始点到结束点是逆时针旋转。
  pub sweep_direction: bool,
  ///是否是大圆弧。true 表示此线型对应的为度数大于 180° 的弧，false 表示对应度数小于 180° 的弧。对于恰好 180° 的弧，此属性不被参考，可由 SweepDirection 属性确定圆弧的形状。
  pub large_arc: bool,
  ///表示按 EllipseSize 绘制的椭圆在当前坐标系下旋转的角度，正值为顺时针，负值为逆时针。角度大于 360° 时以 360 取模。
  pub rotation_angle: f64,
  ///形如 [200 100] 的数组，2 个正浮点数值依次对应椭圆的长、短轴长度，较大的一个为长轴。数组长度超过 2 时只取前两个数值；数组长度为 1 时认为这是一个圆，该数值为圆半径；数组前两个数值中有一个为 0，或者数组为空时，圆弧退化为一条从当前点到 EndPoint 的线段；数组数值为负值时取其绝对值。
  pub ellipse_size: StArray,
  ///圆弧的结束点，下个路径的起始点，不能与当前的绘制起始点为同一位置。
  pub end_point: StPos,
}
///自动闭合到当前分路径的起始点，并以该点为当前点。
#[derive(Clone, Debug, Default)]
pub struct Close {}
#[derive(Clone, Debug)]
pub enum AreaContentChoice {
  Move(Box<Move>),
  Line(Box<Line>),
  QuadraticBezier(Box<QuadraticBezier>),
  CubicBezier(Box<CubicBezier>),
  Arc(Box<Arc>),
  Close(Box<Close>),
}
#[derive(Clone, Debug, Default)]
pub struct Area {
  pub start: StPos,
  pub xml_children: Vec<AreaContentChoice>,
}
///指定多个复杂区域为该链接对象的启动区域，不出现时以所在图元或页面的外接矩形作为启动区域。
#[derive(Clone, Debug, Default)]
pub struct CtRegion {
  ///区域集合。
  pub area: Vec<Area>,
}
pub type StId = u32;
pub type StRefId = u32;
pub type StLoc = String;
pub type StArray = String;
pub type StPos = String;
pub type StBox = String;
#[derive(Clone, Debug, Default)]
pub struct Region(pub CtRegion);
impl From<CtRegion> for Region {
  fn from(value: CtRegion) -> Self {
    Self(value)
  }
}
impl From<Region> for CtRegion {
  fn from(value: Region) -> Self {
    value.0
  }
}
impl std::ops::Deref for Region {
  type Target = CtRegion;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for Region {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
#[derive(Clone, Debug, Default)]
pub struct Dest(pub CtDest);
impl From<CtDest> for Dest {
  fn from(value: CtDest) -> Self {
    Self(value)
  }
}
impl From<Dest> for CtDest {
  fn from(value: Dest) -> Self {
    value.0
  }
}
impl std::ops::Deref for Dest {
  type Target = CtDest;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for Dest {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
///目标区域的描述方法。
#[derive(Clone, Debug, Default)]
pub enum CtDestType {
  #[default]
  Xyz,
  Fit,
  FitH,
  FitV,
  FitR,
}
///放映参数。
#[derive(Clone, Debug, Default)]
pub enum MovieOperator {
  #[default]
  Play,
  Stop,
  Pause,
  Resume,
}
///事件类型。
#[derive(Clone, Debug, Default)]
pub enum CtActionEvent {
  #[default]
  Do,
  Po,
  Click,
}
