//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

///模板页。
#[derive(Clone, Debug, Default)]
pub struct TemplatePage {
  ///模板页的标识，不能与已有标识重复。
  pub id: crate::schemas::definitions::StId,
  ///模板页名称。
  pub name: Option<String>,
  ///模板页的默认图层类型，其类型描述和呈现顺序与 Layer 中 Type 的描述和处理一致。
  pub z_order: Option<TemplatePageZOrder>,
  ///指向模板页内容描述文件。
  pub base_loc: crate::schemas::definitions::StLoc,
}
///文档公共数据。
#[derive(Clone, Debug, Default)]
pub struct CommonData {
  ///当前文档中所有对象使用标识的最大值，初始值为 0。MaxUnitID 主要用于文档编辑，在向文档中新增加一个对象时，需要分配一个新的标识，新标识取值宜为 MaxUnitID + 1，同时需要修改此 MaxUnitID 值。
  pub max_unit_id: crate::schemas::definitions::StId,
  ///指定该文档页面区域的默认大小和位置。
  pub page_area: crate::schemas::definitions::CtPageArea,
  ///公共资源序列，每个节点指向 OFD 包内的一个资源描述文档，资源部分的描述见 7.9，字型和颜色空间等宜在公共资源文件中描述。
  pub public_res: Vec<crate::schemas::definitions::StLoc>,
  ///文档资源序列，每个节点指向 OFD 包内的一个资源描述文档，资源部分的描述见 7.9，绘制参数、多媒体和矢量图像等宜在文档资源文件中描述。
  pub document_res: Vec<crate::schemas::definitions::StLoc>,
  ///模板页序列，为一系列模板页的集合，模板页内容结构和普通页相同，描述见 7.7。
  pub template_page: Vec<TemplatePage>,
  ///引用在资源文件中定义的颜色空间标识，有关颜色空间的描述见 8.3.1。如果此项不存在，采用 RGB 作为默认颜色空间。
  pub default_cs: Option<crate::schemas::definitions::StRefId>,
}
///页树节点。一个页树中可以包含一个或多个页节点，页顺序是根据页树进行前序遍历时叶节点的访问顺序。
#[derive(Clone, Debug, Default)]
pub struct Page {
  ///声明该页的标识，不能与已有标识重复。
  pub id: crate::schemas::definitions::StId,
  ///指向页对象描述文件。
  pub base_loc: crate::schemas::definitions::StLoc,
}
///页树。
#[derive(Clone, Debug, Default)]
pub struct Pages {
  pub page: Vec<Page>,
}
///大纲。
#[derive(Clone, Debug, Default)]
pub struct Outlines {
  pub outline_elem: Vec<CtOutlineElem>,
}
///文档关联的动作序列。
#[derive(Clone, Debug, Default)]
pub struct Actions {
  ///文档关联的动作，事件类型应为 DO（文档打开，见表 52 事件类型）。
  pub action: Vec<crate::schemas::definitions::CtAction>,
}
///文档的书签集，包含一组书签。
#[derive(Clone, Debug, Default)]
pub struct Bookmarks {
  ///文档的书签。
  pub bookmark: Vec<CtBookmark>,
}
///文档根节点。
#[derive(Clone, Debug, Default)]
pub struct Document {
  ///文档公共数据，定义了页面区域、公共资源等数据。
  pub common_data: CommonData,
  ///页树，有关页树的描述见 7.6。
  pub pages: Pages,
  ///大纲，有关大纲的描述见 7.8。
  pub outlines: Option<Outlines>,
  ///文档的权限声明。
  pub permissions: Option<CtPermission>,
  ///文档关联的动作序列，当存在多个 Action 对象时，所有动作依次执行。
  pub actions: Option<Actions>,
  ///文档的视图首选项。
  pub v_preferences: Option<CtVPreferences>,
  ///文档的书签集，包含一组书签。
  pub bookmarks: Option<Bookmarks>,
  ///指向注释列表文件，有关注释描述见第 15 章。
  pub annotations: Option<crate::schemas::definitions::StLoc>,
  ///指向自定义标引列表文件，有关自定义标引描述见第 16 章。
  pub custom_tags: Option<crate::schemas::definitions::StLoc>,
  ///指向附件列表文件。有关附件描述见第 20 章。
  pub attachments: Option<crate::schemas::definitions::StLoc>,
  ///指向扩展列表文件，有关扩展描述见第 17 章。
  pub extensions: Option<crate::schemas::definitions::StLoc>,
}
///打印权限设置。
#[derive(Clone, Debug, Default)]
pub struct Print {
  ///文档是否允许被打印。默认值为 true。
  pub printable: bool,
  ///打印份数，在 Printable 为 true 时有效，若 Printable 为 true 并且不设置 Copies 则打印份数不受限，若 Copies 的值为负值时，打印份数不受限，当 Copies 的值为 0 时，不允许打印，当 Copies 的值大于 0 时，则代表实际可打印的份数值。
  pub copies: Option<i32>,
}
///文档访问有效期。
#[derive(Clone, Debug, Default)]
pub struct ValidPeriod {
  ///有效期开始日期。
  pub start_date: Option<String>,
  ///有效期结束日期。
  pub end_date: Option<String>,
}
///文档权限声明。
#[derive(Clone, Debug, Default)]
pub struct CtPermission {
  ///是否允许编辑。默认值为 true。
  pub edit: Option<bool>,
  ///是否允许添加或修改标注。默认值为 true。
  pub annot: Option<bool>,
  ///是否允许导出。默认值为 true。
  pub export: Option<bool>,
  ///是否允许进行数字签名。默认值为 true。
  pub signature: Option<bool>,
  ///是否允许添加水印。默认值为 true。
  pub watermark: Option<bool>,
  ///是否允许截屏。默认值为 true。
  pub print_screen: Option<bool>,
  ///打印权限，其具体的权限和份数设置由其属性 Printable 及 Copies 控制，若不设置 Print 节点，则默认为可以打印，并且打印份数不受限制。
  pub print: Option<Print>,
  ///有效期，即此文档允许访问的期限，其具体期限取决于开始日期和结束日期，其中开始日期不能晚于结束日期，并且开始日期和结束日期至少出现一个。当不设置开始日期时，代表不限定开始日期，当不设置结束日期时代表不限定结束日期；当此不设置此节点时，表示开始日期和结束日期均不受限。
  pub valid_period: Option<ValidPeriod>,
}
#[derive(Clone, Debug)]
pub enum CtVPreferencesContentChoice {
  ZoomMode(Box<ZoomMode>),
  Zoom(Box<f64>),
}
///文档的视图首选项。
#[derive(Clone, Debug, Default)]
pub struct CtVPreferences {
  ///窗口模式，可取值如下：
  ///None——常规模式
  ///FullScreen——打开后全文显示
  ///UseOutlines——同时呈现文档大纲
  ///UseThumbs——同时呈现缩略图
  ///UseCustomTags——同时呈现语义结构
  ///UseLayers——同时呈现图层
  ///UseAttachments——同时呈现附件
  ///UseBookmarks——同时呈现书签
  ///默认值为 None。
  pub page_mode: Option<PageMode>,
  ///页面布局模式，可取值如下：
  ///OnePage——单页模式
  ///OneColumn——单列模式
  ///TwoPageL——对开模式
  ///TwoColumnL——对开连续模式
  ///TwoPageR——对开靠右模式
  ///TwoColumnR——对开连续靠右模式
  ///默认值为 OneColumn。
  pub page_layout: Option<PageLayout>,
  ///标题栏显示模式，可取值如下：
  ///FileName——文件名称
  ///DocTitle——呈现元数据中的 Title 属性
  ///默认值为 FileName。当设置为 DocTitle 但不存在 Title 属性时，按照 FileName 处理。
  pub tab_display: Option<TabDisplay>,
  ///是否隐藏工具栏。默认值为 false。
  pub hide_toolbar: Option<bool>,
  ///是否隐藏菜单栏。默认值为 false。
  pub hide_menubar: Option<bool>,
  ///是否隐藏主窗口之外的其他窗体组件。默认值为 false。
  pub hide_window_ui: Option<bool>,
  pub xml_children: Vec<CtVPreferencesContentChoice>,
}
///大纲节点。
#[derive(Clone, Debug, Default)]
pub struct CtOutlineElem {
  ///大纲节点标题。
  pub title: String,
  ///该节点下所有叶节点的数目参考值，应根据该节点下实际出现的子节点数为准。默认值为 0。
  pub count: Option<i32>,
  ///在有子节点存在时有效，如果为 true，表示该大纲在初始状态下展开子节点；如果为 false，则表示不展开。默认值为 true。
  pub expanded: Option<bool>,
  ///当此大纲节点被激活时将执行的动作序列。
  pub actions: Option<Actions>,
  ///该节点的子大纲节点。层层嵌套，形成树状结构。
  pub outline_elem: Vec<CtOutlineElem>,
}
#[derive(Clone, Debug, Default)]
pub struct CtBookmark {
  pub name: String,
  pub dest: crate::schemas::definitions::CtDest,
}
#[derive(Clone, Debug, Default)]
pub struct PageArea(pub crate::schemas::definitions::CtPageArea);
impl From<crate::schemas::definitions::CtPageArea> for PageArea {
  fn from(value: crate::schemas::definitions::CtPageArea) -> Self {
    Self(value)
  }
}
impl From<PageArea> for crate::schemas::definitions::CtPageArea {
  fn from(value: PageArea) -> Self {
    value.0
  }
}
impl std::ops::Deref for PageArea {
  type Target = crate::schemas::definitions::CtPageArea;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for PageArea {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
#[derive(Clone, Debug, Default)]
pub struct OutlineElem(pub CtOutlineElem);
impl From<CtOutlineElem> for OutlineElem {
  fn from(value: CtOutlineElem) -> Self {
    Self(value)
  }
}
impl From<OutlineElem> for CtOutlineElem {
  fn from(value: OutlineElem) -> Self {
    value.0
  }
}
impl std::ops::Deref for OutlineElem {
  type Target = CtOutlineElem;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for OutlineElem {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
#[derive(Clone, Debug, Default)]
pub struct Permissions(pub CtPermission);
impl From<CtPermission> for Permissions {
  fn from(value: CtPermission) -> Self {
    Self(value)
  }
}
impl From<Permissions> for CtPermission {
  fn from(value: Permissions) -> Self {
    value.0
  }
}
impl std::ops::Deref for Permissions {
  type Target = CtPermission;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for Permissions {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
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
pub struct VPreferences(pub CtVPreferences);
impl From<CtVPreferences> for VPreferences {
  fn from(value: CtVPreferences) -> Self {
    Self(value)
  }
}
impl From<VPreferences> for CtVPreferences {
  fn from(value: VPreferences) -> Self {
    value.0
  }
}
impl std::ops::Deref for VPreferences {
  type Target = CtVPreferences;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for VPreferences {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
///书签。
#[derive(Clone, Debug, Default)]
pub struct Bookmark(pub CtBookmark);
impl From<CtBookmark> for Bookmark {
  fn from(value: CtBookmark) -> Self {
    Self(value)
  }
}
impl From<Bookmark> for CtBookmark {
  fn from(value: Bookmark) -> Self {
    value.0
  }
}
impl std::ops::Deref for Bookmark {
  type Target = CtBookmark;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for Bookmark {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
#[derive(Clone, Debug, Default)]
pub struct Dest(pub crate::schemas::definitions::CtDest);
impl From<crate::schemas::definitions::CtDest> for Dest {
  fn from(value: crate::schemas::definitions::CtDest) -> Self {
    Self(value)
  }
}
impl From<Dest> for crate::schemas::definitions::CtDest {
  fn from(value: Dest) -> Self {
    value.0
  }
}
impl std::ops::Deref for Dest {
  type Target = crate::schemas::definitions::CtDest;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::ops::DerefMut for Dest {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
#[derive(Clone, Debug, Default)]
pub enum TemplatePageZOrder {
  #[default]
  Background,
  Foreground,
}
///窗口模式。
#[derive(Clone, Debug, Default)]
pub enum PageMode {
  #[default]
  None,
  FullScreen,
  UseOutlines,
  UseThumbs,
  UseCustomTags,
  UseLayers,
  UseAttatchs,
  UseBookmarks,
}
///页面布局模式。
#[derive(Clone, Debug, Default)]
pub enum PageLayout {
  #[default]
  OnePage,
  OneColumn,
  TwoPageL,
  TwoColumnL,
  TwoPageR,
  TwoColumnR,
}
///标题栏显示模式。
#[derive(Clone, Debug, Default)]
pub enum TabDisplay {
  #[default]
  DocTitle,
  FileName,
}
///自动缩放模式。
#[derive(Clone, Debug, Default)]
pub enum ZoomMode {
  #[default]
  Default,
  FitHeight,
  FitWidth,
  FitRect,
}
