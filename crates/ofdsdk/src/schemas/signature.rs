//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

///创建签名时所用的签章组件提供者信息。
#[derive(Clone, Debug, Default)]
pub struct Provider {
  ///创建签名时所用的签章组件的提供者名称。
  pub provider_name: String,
  ///创建签名时所用的签章组件的版本。
  pub version: Option<String>,
  ///创建签名时所用的签章组件的制造商。
  pub company: Option<String>,
}
///针对一个文件的摘要节点。
#[derive(Clone, Debug, Default)]
pub struct Reference {
  ///指向包内的文件，使用绝对路径。
  pub file_ref: crate::schemas::definitions::StLoc,
  ///对包内文件进行摘要计算，对所得的二进制摘要值进行 base64 编码所得结果。
  pub check_value: String,
}
///包内文件计算所得的摘要记录列表。
#[derive(Clone, Debug, Default)]
pub struct References {
  ///摘要方法，视应用场景的不同使用不同的摘要方法。用于各行业应用时，应使用符合该行业安全标准的算法。
  pub check_method: String,
  ///针对一个文件的摘要节点。
  pub reference: Vec<Reference>,
}
///签名外观节点属性。
#[derive(Clone, Debug, Default)]
pub struct StampAnnot {
  ///签章注释的标识。
  pub id: String,
  ///引用外观注释所在的页面的标识。
  pub page_ref: crate::schemas::definitions::StRefId,
  ///签章注释的外观外边框位置，可用于签章注释在页面内的定位。
  pub boundary: crate::schemas::definitions::StBox,
  ///签章注释的外观裁剪设置。
  pub clip: Option<crate::schemas::definitions::StBox>,
}
///电子印章信息。
#[derive(Clone, Debug, Default)]
pub struct Seal {
  ///指向包内的安全电子印章文件，遵循密码领域的相关规范。
  pub base_loc: crate::schemas::definitions::StLoc,
}
///签名要保护的原文及本次签名相关的信息。
#[derive(Clone, Debug, Default)]
pub struct SignedInfo {
  ///创建签名时所用的签章组件提供者信息。
  pub provider: Provider,
  ///签名方法，记录安全模块返回的签名算法代码，以便验证时使用。
  pub signature_method: Option<String>,
  ///签名时间，记录安全模块返回的签名时间，以便验证时使用。
  pub signature_date_time: Option<String>,
  ///包内文件计算所得的摘要记录列表。
  pub references: References,
  ///本签名关联的外观（用 OFD 中的注释来表示），该节点可出现多次。
  pub stamp_annot: Vec<StampAnnot>,
  ///电子印章信息。
  pub seal: Option<Seal>,
}
///签名描述文件的根节点。
#[derive(Clone, Debug, Default)]
pub struct Signature {
  ///签名要保护的原文及本次签名相关的信息。
  pub signed_info: SignedInfo,
  ///指向安全签名提供者所返回的针对签名描述文件计算所得的签名值文件。
  pub signed_value: crate::schemas::definitions::StLoc,
}
///摘要方法，视应用场景的不同使用不同的摘要方法。
#[derive(Clone, Debug, Default)]
pub enum ReferencesCheckMethod {
  #[default]
  Md5,
  Sha1,
}
