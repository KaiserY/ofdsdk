use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Schema {
  pub target_namespace: String,
  pub types: Vec<Struct>,
  pub elements: Vec<Element>,
  pub enums: Vec<Enum>,
  pub aliases: Vec<Alias>,
  pub module_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum CodegenTypeKind {
  #[default]
  String,
  Bool,
  F64,
  I32,
  U32,
  Enum,
  Struct,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum ElementApiKind {
  #[default]
  None,
  EnumAlias,
  StructWrapper,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Element {
  pub name: String,
  pub ident: String,
  pub xml_name: String,
  pub r#type: String,
  #[serde(skip_serializing)]
  pub resolved_type: String,
  #[serde(skip_serializing)]
  pub type_kind: CodegenTypeKind,
  #[serde(skip_serializing)]
  pub is_struct: bool,
  #[serde(skip_serializing)]
  pub is_public_wrapper: bool,
  #[serde(skip_serializing)]
  pub api_kind: ElementApiKind,
  pub is_top_level: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Struct {
  pub name: String,
  pub ident: String,
  pub xml_name: Option<String>,
  #[serde(skip_serializing)]
  pub resolved_xml_name: String,
  pub attributes: Vec<Attribute>,
  pub sequences: Vec<Child>,
  pub children: Vec<Child>,

  #[serde(skip)]
  pub module_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Attribute {
  pub name: String,
  pub ident: String,
  pub r#type: String,
  #[serde(skip_serializing)]
  pub resolved_type: String,
  #[serde(skip_serializing)]
  pub type_kind: CodegenTypeKind,
  pub is_option: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Child {
  pub name: String,
  pub xml_name: Option<String>,
  #[serde(skip_serializing)]
  pub resolved_xml_name: String,
  pub ident: String,
  pub r#type: String,
  #[serde(skip_serializing)]
  pub resolved_type: String,
  #[serde(skip_serializing)]
  pub type_kind: CodegenTypeKind,
  #[serde(skip_serializing)]
  pub is_struct: bool,
  pub source_module_name: Option<String>,
  pub is_option: bool,
  pub is_vec: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Enum {
  pub name: String,
  pub ident: String,
  pub variants: Vec<Variant>,
  pub default_index: usize,

  #[serde(skip)]
  pub module_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Variant {
  pub ident: String,
  pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Alias {
  pub name: String,
  pub ident: String,
  pub r#type: String,

  #[serde(skip)]
  pub module_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct PartDefinition {
  pub name: String,
  pub base: PartBase,
  pub content: Option<PartContent>,
  pub schema_module: Option<String>,
  pub root_element: Option<String>,
  pub context_from: Option<PartSource>,
  pub path: PartPath,
  pub children: Vec<PartChild>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum PartBase {
  #[default]
  OfdPart,
  OfdPackage,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum PartContent {
  #[default]
  Xml,
  Blob,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "PascalCase", rename_all_fields = "PascalCase")]
pub enum PartPath {
  #[default]
  ReferenceField,
  Fixed {
    path: String,
  },
  ReferenceFieldWithBase {
    base_field_path: Vec<String>,
  },
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct PartSource {
  pub parent: String,
  pub field_path: Vec<String>,
  pub is_vec: bool,
  pub source_root: PartSourceRoot,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct PartChild {
  pub api_name: String,
  pub name: String,
  pub from: Option<PartSource>,
  pub context_from: Option<PartSource>,
  pub min_occurs_is_non_zero: bool,
  pub max_occurs_great_than_one: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum PartSourceRoot {
  #[default]
  RootElement,
  ContextElement,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct CompatibilityConfig {
  pub rules: Vec<CompatibilityRule>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaCommentsConfig {
  pub entries: Vec<SchemaComment>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaComment {
  pub schema: String,
  #[serde(rename = "Type")]
  pub type_name: String,
  pub field: Option<String>,
  pub docs: Vec<SchemaDocBlock>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaDocBlock {
  pub kind: SchemaDocBlockKind,
  pub text: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum SchemaDocBlockKind {
  #[default]
  Description,
  Example,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct CompatibilityRule {
  pub schema: String,
  #[serde(rename = "Type")]
  pub type_name: String,
  pub field: Option<String>,
  pub action: CompatibilityAction,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase", rename_all_fields = "PascalCase")]
pub enum CompatibilityAction {
  AllowMissingAttribute { default_value: String },
  EnumValueAlias { input: String, canonical: String },
  TreatAsString { default_value: String },
}

impl Default for CompatibilityAction {
  fn default() -> Self {
    Self::AllowMissingAttribute {
      default_value: String::new(),
    }
  }
}
