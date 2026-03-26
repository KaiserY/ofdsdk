use ofdsdk::schemas::signature::{ReferencesCheckMethod, Signature};

const SIGNATURE_XML: &str = r#"<ofd:Signature xmlns:ofd="http://www.ofdspec.org/2016">
<ofd:SignedInfo>
<ofd:Provider ProviderName="ofdrw" Version="1.0.0" Company="ofdrw.org"/>
<ofd:SignatureMethod>SM2</ofd:SignatureMethod>
<ofd:SignatureDateTime>2024-03-26 12:00:00</ofd:SignatureDateTime>
<ofd:References CheckMethod="SHA1">
<ofd:Reference FileRef="/Doc_0/Document.xml">
<ofd:CheckValue>YWJjZA==</ofd:CheckValue>
</ofd:Reference>
</ofd:References>
<ofd:StampAnnot ID="seal-1" PageRef="7" Boundary="0 0 40 40" Clip="1 1 2 2"/>
<ofd:Seal><ofd:BaseLoc>Seal.esl</ofd:BaseLoc></ofd:Seal>
</ofd:SignedInfo>
<ofd:SignedValue>signature-value.dat</ofd:SignedValue>
</ofd:Signature>"#;

const SIGNATURE_DEFAULT_CHECK_METHOD_XML: &str = r#"<ofd:Signature xmlns:ofd="http://www.ofdspec.org/2016">
<ofd:SignedInfo>
<ofd:Provider ProviderName="ofdrw"/>
<ofd:References>
<ofd:Reference FileRef="/Doc_0/OFD.xml">
<ofd:CheckValue>AAAA</ofd:CheckValue>
</ofd:Reference>
</ofd:References>
</ofd:SignedInfo>
<ofd:SignedValue>signed-value.dat</ofd:SignedValue>
</ofd:Signature>"#;

#[test]
fn signature_round_trip() {
  let signature: Signature = SIGNATURE_XML.parse().unwrap();

  assert_eq!(signature.signed_info.provider.provider_name, "ofdrw");
  assert_eq!(
    signature.signed_info.provider.version.as_deref(),
    Some("1.0.0")
  );
  assert_eq!(
    signature.signed_info.provider.company.as_deref(),
    Some("ofdrw.org")
  );
  assert_eq!(
    signature.signed_info.signature_method.as_deref(),
    Some("SM2")
  );
  assert_eq!(
    signature.signed_info.signature_date_time.as_deref(),
    Some("2024-03-26 12:00:00")
  );
  assert_eq!(signature.signed_info.references.check_method, "SHA1");
  assert_eq!(signature.signed_info.references.reference.len(), 1);
  assert_eq!(
    signature.signed_info.references.reference[0].file_ref,
    "/Doc_0/Document.xml"
  );
  assert_eq!(
    signature.signed_info.references.reference[0].check_value,
    "YWJjZA=="
  );
  assert_eq!(signature.signed_info.stamp_annot.len(), 1);
  assert_eq!(signature.signed_info.stamp_annot[0].id, "seal-1");
  assert_eq!(signature.signed_info.stamp_annot[0].page_ref, 7);
  assert_eq!(signature.signed_info.stamp_annot[0].boundary, "0 0 40 40");
  assert_eq!(
    signature.signed_info.stamp_annot[0].clip.as_deref(),
    Some("1 1 2 2")
  );
  assert_eq!(
    signature.signed_info.seal.as_ref().unwrap().base_loc,
    "Seal.esl"
  );
  assert_eq!(signature.signed_value, "signature-value.dat");

  let serialized = signature.to_xml().unwrap();
  let reparsed: Signature = serialized.parse().unwrap();

  assert_eq!(reparsed.signed_info.references.check_method, "SHA1");
  assert_eq!(reparsed.signed_info.stamp_annot.len(), 1);
  assert_eq!(reparsed.signed_value, "signature-value.dat");
}

#[test]
fn signature_default_check_method_round_trip() {
  let signature: Signature = SIGNATURE_DEFAULT_CHECK_METHOD_XML.parse().unwrap();

  assert_eq!(
    signature.signed_info.references.check_method,
    ReferencesCheckMethod::Md5.to_string()
  );
  assert_eq!(signature.signed_info.references.reference.len(), 1);

  let serialized = signature.to_xml().unwrap();
  let reparsed: Signature = serialized.parse().unwrap();

  assert_eq!(
    reparsed.signed_info.references.check_method,
    ReferencesCheckMethod::Md5.to_string()
  );
}
