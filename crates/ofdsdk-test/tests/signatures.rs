const XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<ofd:Signatures xmlns:ofd="http://www.ofdspec.org/2016"><ofd:MaxSignId>2</ofd:MaxSignId><ofd:Signature ID="1" BaseLoc="Sign_0/Signature.xml"/></ofd:Signatures>"#;

#[test]
fn signatures_round_trip() {
  let signatures = XML
    .parse::<ofdsdk::schemas::signatures::Signatures>()
    .unwrap();

  assert_eq!(signatures.max_sign_id.as_deref(), Some("2"));
  assert_eq!(signatures.signature.len(), 1);
  assert_eq!(signatures.signature[0].id, "1");
  assert_eq!(signatures.signature[0].base_loc, "Sign_0/Signature.xml");
  assert!(signatures.signature[0].r#type.is_none());

  let serialized = signatures.to_xml().unwrap();
  assert_eq!(
    serialized,
    r#"<ofd:Signatures xmlns:ofd="http://www.ofdspec.org/2016"><ofd:MaxSignId>2</ofd:MaxSignId><ofd:Signature ID="1" BaseLoc="Sign_0/Signature.xml"/></ofd:Signatures>"#
  );

  let reparsed = serialized
    .parse::<ofdsdk::schemas::signatures::Signatures>()
    .unwrap();
  assert_eq!(reparsed.max_sign_id.as_deref(), Some("2"));
  assert_eq!(reparsed.signature.len(), 1);
  assert_eq!(reparsed.signature[0].id, "1");
  assert_eq!(reparsed.signature[0].base_loc, "Sign_0/Signature.xml");
}
