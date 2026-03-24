use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Schema {
  #[serde(rename = "@id")]
  pub id: Option<String>,
  #[serde(rename = "@xmlns")]
  pub xmlns: Option<String>,
  #[serde(rename = "@targetNamespace")]
  pub target_namespace: Option<String>,
  #[serde(rename = "@elementFormDefault")]
  pub element_form_default: Option<String>,
  #[serde(rename = "@attributeFormDefault")]
  pub attribute_form_default: Option<String>,
  #[serde(rename = "@blockDefault")]
  pub block_default: Option<String>,
  #[serde(rename = "@finalDefault")]
  pub final_default: Option<String>,
  #[serde(rename = "@version")]
  pub version: Option<String>,

  #[serde(rename = "$value")]
  pub contents: Vec<SchemaContentChoice>,

  #[serde(skip)]
  pub module_name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SchemaContentChoice {
  #[serde(rename = "include")]
  Include(Include),
  #[serde(rename = "simpleType")]
  SimpleType(SimpleType),
  #[serde(rename = "complexType")]
  ComplexType(ComplexType),
  #[serde(rename = "element")]
  Element(Box<Element>),
  #[serde(rename = "attribute")]
  Attribute(Box<Attribute>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Include {
  #[serde(rename = "@id")]
  pub id: Option<String>,

  #[serde(rename = "@schemaLocation")]
  pub schema_location: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Choice {
  #[serde(rename = "@id")]
  pub id: Option<String>,
  #[serde(rename = "@minOccurs")]
  pub min_occurs: Option<String>,
  #[serde(rename = "@maxOccurs")]
  pub max_occurs: Option<String>,
  #[serde(default, rename = "$value")]
  pub contents: Vec<ChoiceContentChoice>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sequence {
  #[serde(rename = "@id")]
  pub id: Option<String>,
  #[serde(rename = "@minOccurs")]
  pub min_occurs: Option<String>,
  #[serde(rename = "@maxOccurs")]
  pub max_occurs: Option<String>,
  #[serde(default, rename = "$value")]
  pub contents: Vec<SequenceContentChoice>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SequenceContentChoice {
  #[serde(rename = "element")]
  Element(Box<Element>),
  #[serde(rename = "choice")]
  Choice(Choice),
  #[serde(rename = "sequence")]
  Sequence(Sequence),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ChoiceContentChoice {
  #[serde(rename = "element")]
  Element(Box<Element>),
  #[serde(rename = "sequence")]
  Sequence(Sequence),
  #[serde(rename = "choice")]
  Choice(Choice),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Attribute {
  #[serde(rename = "@id")]
  pub id: Option<String>,
  #[serde(rename = "@name")]
  pub name: Option<String>,
  #[serde(rename = "@default")]
  pub default: Option<String>,
  #[serde(rename = "@fixed")]
  pub fixed: Option<String>,
  #[serde(rename = "@form")]
  pub form: Option<String>,
  #[serde(rename = "@type")]
  pub r#type: Option<String>,
  #[serde(rename = "@use")]
  pub r#use: Option<String>,
  #[serde(rename = "@ref")]
  pub r#ref: Option<String>,
  #[serde(rename = "$value")]
  pub contents: Option<SimpleType>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimpleContent {
  #[serde(rename = "@id")]
  pub id: Option<String>,
  #[serde(rename = "$value")]
  pub contents: Option<SimpleContentContentChoice>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SimpleContentContentChoice {
  #[serde(rename = "restriction")]
  Restriction(Restriction),
  #[serde(rename = "extension")]
  Extension(Extension),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimpleType {
  #[serde(rename = "@id")]
  pub id: Option<String>,
  #[serde(rename = "@name")]
  pub name: Option<String>,
  #[serde(rename = "@final")]
  pub r#final: Option<String>,
  #[serde(rename = "$value")]
  pub contents: Option<SimpleTypeContentChoice>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SimpleTypeContentChoice {
  #[serde(rename = "restriction")]
  Restriction(Restriction),
  #[serde(rename = "union")]
  Union(Union),
  #[serde(rename = "list")]
  List(List),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Union {
  #[serde(rename = "@id")]
  pub id: Option<String>,
  #[serde(rename = "@memberTypes")]
  pub member_types: Option<String>,
  #[serde(rename = "$value")]
  pub contents: Option<Vec<SimpleType>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct List {
  #[serde(rename = "@id")]
  pub id: Option<String>,
  #[serde(rename = "@itemType")]
  pub item_type: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ComplexType {
  #[serde(rename = "@id")]
  pub id: Option<String>,
  #[serde(rename = "@name")]
  pub name: Option<String>,
  #[serde(rename = "@final")]
  pub r#final: Option<String>,
  #[serde(rename = "@abstract")]
  pub r#abstract: Option<bool>,
  #[serde(rename = "@block")]
  pub block: Option<bool>,
  #[serde(rename = "@mixed")]
  pub mixed: Option<bool>,
  #[serde(default, rename = "$value")]
  pub contents: Vec<ComplexTypeContentChoice>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ComplexTypeContentChoice {
  #[serde(rename = "sequence")]
  Sequence(Sequence),
  #[serde(rename = "attribute")]
  Attribute(Box<Attribute>),
  #[serde(rename = "choice")]
  Choice(Choice),
  #[serde(rename = "simpleContent")]
  SimpleContent(SimpleContent),
  #[serde(rename = "complexContent")]
  ComplexContent(ComplexContent),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ComplexContent {
  #[serde(rename = "@id")]
  pub id: Option<String>,
  #[serde(rename = "@mixed")]
  pub mixed: Option<bool>,
  #[serde(rename = "$value")]
  pub contents: Option<ComplexContentContentChoice>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ComplexContentContentChoice {
  #[serde(rename = "restriction")]
  Restriction(Restriction),
  #[serde(rename = "extension")]
  Extension(Extension),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Extension {
  #[serde(rename = "@id")]
  pub id: Option<String>,
  #[serde(rename = "@base")]
  pub base: String,
  #[serde(default, rename = "$value")]
  pub contents: Vec<ExtensionContentChoice>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ExtensionContentChoice {
  #[serde(rename = "sequence")]
  Sequence(Sequence),
  #[serde(rename = "choice")]
  Choice(Choice),
  #[serde(rename = "attribute")]
  Attribute(Box<Attribute>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Element {
  #[serde(rename = "@id")]
  pub id: Option<String>,
  #[serde(rename = "@name")]
  pub name: Option<String>,
  #[serde(rename = "@abstract")]
  pub r#abstract: Option<bool>,
  #[serde(rename = "@block")]
  pub block: Option<String>,
  #[serde(rename = "@default")]
  pub default: Option<String>,
  #[serde(rename = "@substitutionGroup")]
  pub substitution_group: Option<String>,
  #[serde(rename = "@final")]
  pub r#final: Option<String>,
  #[serde(rename = "@fixed")]
  pub fixed: Option<String>,
  #[serde(rename = "@form")]
  pub form: Option<String>,
  #[serde(rename = "@minOccurs")]
  pub min_occurs: Option<String>,
  #[serde(rename = "@maxOccurs")]
  pub max_occurs: Option<String>,
  #[serde(rename = "@type")]
  pub r#type: Option<String>,
  #[serde(rename = "@nillable")]
  pub nillable: Option<bool>,

  #[serde(rename = "$value")]
  pub contents: Option<ElementContentChoice>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ElementContentChoice {
  #[serde(rename = "simpleType")]
  SimpleType(SimpleType),
  #[serde(rename = "complexType")]
  ComplexType(ComplexType),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Restriction {
  #[serde(rename = "@id")]
  pub id: Option<String>,
  #[serde(rename = "@base")]
  pub base: String,
  #[serde(default, rename = "$value")]
  pub contents: Vec<RestrictionContentChoice>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum RestrictionContentChoice {
  #[serde(rename = "enumeration")]
  Enumeration(Enumeration),
  #[serde(rename = "pattern")]
  Pattern(Pattern),
  #[serde(rename = "length")]
  Length(Length),
  #[serde(rename = "minLength")]
  MinLength(MinLength),
  #[serde(rename = "maxLength")]
  MaxLength(MaxLength),
  #[serde(rename = "minInclusive")]
  MinInclusive(MinInclusive),
  #[serde(rename = "maxInclusive")]
  MaxInclusive(MaxInclusive),
  #[serde(rename = "minExclusive")]
  MinExclusive(MinExclusive),
  #[serde(rename = "maxExclusive")]
  MaxExclusive(MaxExclusive),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Enumeration {
  #[serde(rename = "@value")]
  pub value: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Pattern {
  #[serde(rename = "@value")]
  pub value: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Length {
  #[serde(rename = "@value")]
  pub value: String,
  #[serde(rename = "@fixed")]
  pub fixed: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MinLength {
  #[serde(rename = "@value")]
  pub value: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MaxLength {
  #[serde(rename = "@value")]
  pub value: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MinInclusive {
  #[serde(rename = "@value")]
  pub value: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MaxInclusive {
  #[serde(rename = "@value")]
  pub value: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MinExclusive {
  #[serde(rename = "@value")]
  pub value: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MaxExclusive {
  #[serde(rename = "@value")]
  pub value: String,
}
