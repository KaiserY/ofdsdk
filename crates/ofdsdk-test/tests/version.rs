use ofdsdk::schemas::ofd::Versions;
use ofdsdk::schemas::version::DocVersion;

const VERSIONS_XML: &str = r#"<ofd:Versions xmlns:ofd="http://www.ofdspec.org/2016">
<ofd:Version ID="1" Index="1" Current="true" BaseLoc="Versions/Version_1.xml"/>
<ofd:Version ID="2" Index="2" BaseLoc="Versions/Version_2.xml"/>
</ofd:Versions>"#;

const DOC_VERSION_XML: &str = r#"<ofd:DocVersion xmlns:ofd="http://www.ofdspec.org/2016" ID="77" Version="1.0" Name="MyVer" CreationDate="2024-03-26">
<ofd:FileList>
<ofd:File ID="12">Res/VersionFile_1.xml</ofd:File>
<ofd:File ID="13">Res/VersionFile_2.xml</ofd:File>
</ofd:FileList>
<ofd:DocRoot>Res/DocRoot.xml</ofd:DocRoot>
</ofd:DocVersion>"#;

#[test]
fn versions_round_trip() {
  let versions: Versions = VERSIONS_XML.parse().unwrap();

  assert_eq!(versions.version.len(), 2);
  assert_eq!(versions.version[0].id, "1");
  assert_eq!(versions.version[0].index, 1);
  assert_eq!(versions.version[0].current, Some(true));
  assert_eq!(versions.version[0].base_loc, "Versions/Version_1.xml");
  assert_eq!(versions.version[1].base_loc, "Versions/Version_2.xml");

  let serialized = versions.to_xml().unwrap();
  let reparsed: Versions = serialized.parse().unwrap();

  assert_eq!(reparsed.version.len(), 2);
  assert_eq!(reparsed.version[1].id, "2");
}

#[test]
fn doc_version_round_trip() {
  let doc_version: DocVersion = DOC_VERSION_XML.parse().unwrap();

  assert_eq!(doc_version.id, "77");
  assert_eq!(doc_version.version.as_deref(), Some("1.0"));
  assert_eq!(doc_version.name.as_deref(), Some("MyVer"));
  assert_eq!(doc_version.creation_date.as_deref(), Some("2024-03-26"));
  assert_eq!(doc_version.file_list.file.len(), 2);
  assert_eq!(doc_version.file_list.file[0].id, "12");
  assert_eq!(
    doc_version.file_list.file[0].xml_value,
    "Res/VersionFile_1.xml"
  );
  assert_eq!(doc_version.doc_root, "Res/DocRoot.xml");

  let serialized = doc_version.to_xml().unwrap();
  assert_eq!(
    serialized,
    r#"<ofd:DocVersion xmlns:ofd="http://www.ofdspec.org/2016" ID="77" Version="1.0" Name="MyVer" CreationDate="2024-03-26"><ofd:FileList><ofd:File ID="12">Res/VersionFile_1.xml</ofd:File><ofd:File ID="13">Res/VersionFile_2.xml</ofd:File></ofd:FileList><ofd:DocRoot>Res/DocRoot.xml</ofd:DocRoot></ofd:DocVersion>"#
  );

  let reparsed: DocVersion = serialized.parse().unwrap();
  assert_eq!(reparsed.file_list.file.len(), 2);
  assert_eq!(reparsed.doc_root, "Res/DocRoot.xml");
}
