//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

///签名属性。
#[derive(Clone, Debug, Default)]
pub struct Signature {
  ///签名或签章的标识。
  pub id: String,
  ///签名节点的类型，目前规定了两个可选值，Seal 表示是安全签章，Sign 表示是纯数字签名。
  pub r#type: Option<SignatureType>,
  ///指向包内的签名描述文件。
  pub base_loc: crate::schemas::definitions::StLoc,
}
///签名列表根节点属性。
#[derive(Clone, Debug, Default)]
pub struct Signatures {
  ///安全标识的最大值，作用与文档入口文件 Document.xml 中的 MaxID 相同，为了避免在签名时影响文档入口文件，采用了与 ST_ID 不一样的 ID 编码方式。推荐使用“sNNN”的编码方式，NNN 从 1 开始。
  pub max_sign_id: Option<String>,
  ///数字签名或安全签章在列表中的注册信息，一次签名或签章对应一个节点。
  pub signature: Vec<Signature>,
}
///签名节点的类型。Seal 表示安全签章，Sign 表示纯数字签名。
#[derive(Clone, Debug, Default)]
pub enum SignatureType {
  #[default]
  Seal,
  Sign,
}
