use ofdsdk::schemas::extensions::{CtExtensionContentChoice, Extensions};

const EXTENSIONS_XML: &str = r#"<ofd:Extensions xmlns:ofd="http://www.ofdspec.org/2016">
<ofd:Extension AppName="ofdrw" Company="ofdrw.org" AppVersion="1.0.0" Date="2024-03-26" RefId="724">
<ofd:Property Name="P1" Type="string">value1</ofd:Property>
<ofd:Data>plain data</ofd:Data>
<ofd:ExtendData>Res/ExtensionData.xml</ofd:ExtendData>
</ofd:Extension>
</ofd:Extensions>"#;

#[test]
fn extensions_round_trip() {
  let extensions: Extensions = EXTENSIONS_XML.parse().unwrap();

  assert_eq!(extensions.extension.len(), 1);

  let extension = &extensions.extension[0];
  assert_eq!(extension.app_name, "ofdrw");
  assert_eq!(extension.company.as_deref(), Some("ofdrw.org"));
  assert_eq!(extension.app_version.as_deref(), Some("1.0.0"));
  assert_eq!(extension.date.as_deref(), Some("2024-03-26"));
  assert_eq!(extension.ref_id, 724);
  assert_eq!(extension.xml_children.len(), 3);

  match &extension.xml_children[0] {
    CtExtensionContentChoice::Property(property) => {
      assert_eq!(property.name, "P1");
      assert_eq!(property.r#type.as_deref(), Some("string"));
      assert_eq!(property.xml_value, "value1");
    }
    other => panic!("unexpected extension child: {other:?}"),
  }

  match &extension.xml_children[1] {
    CtExtensionContentChoice::Data(data) => {
      assert_eq!(data.as_str(), "plain data");
    }
    other => panic!("unexpected extension child: {other:?}"),
  }

  match &extension.xml_children[2] {
    CtExtensionContentChoice::ExtendData(path) => {
      assert_eq!(path.as_str(), "Res/ExtensionData.xml");
    }
    other => panic!("unexpected extension child: {other:?}"),
  }

  let serialized = extensions.to_xml().unwrap();

  assert_eq!(
    serialized,
    r#"<ofd:Extensions xmlns:ofd="http://www.ofdspec.org/2016"><ofd:Extension AppName="ofdrw" Company="ofdrw.org" AppVersion="1.0.0" Date="2024-03-26" RefId="724"><ofd:Property Name="P1" Type="string">value1</ofd:Property><ofd:Data>plain data</ofd:Data><ofd:ExtendData>Res/ExtensionData.xml</ofd:ExtendData></ofd:Extension></ofd:Extensions>"#
  );

  let reparsed: Extensions = serialized.parse().unwrap();
  assert_eq!(reparsed.extension.len(), 1);
  assert_eq!(reparsed.extension[0].xml_children.len(), 3);
}
