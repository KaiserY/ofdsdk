use ofdsdk::schemas::ofd::{Ofd, OfdDocType};

const OFD_XML: &str = r#"<ofd:OFD xmlns:ofd="http://www.ofdspec.org/2016" DocType="OFD" Version="1.0">
<ofd:DocBody>
<ofd:DocInfo>
<ofd:DocID>050001700111_12235358</ofd:DocID>
<ofd:Author>Huhuang Software</ofd:Author>
<ofd:CreationDate>2020-08-17</ofd:CreationDate>
<ofd:ModDate>2020-08-17</ofd:ModDate>
<ofd:Creator>Huhuang Software</ofd:Creator>
<ofd:CustomDatas>
<ofd:CustomData Name="native-producer">HuhuangSDK</ofd:CustomData>
<ofd:CustomData Name="producer-version">v1.0.0518.2</ofd:CustomData>
<ofd:CustomData Name="价税合计">10600.00</ofd:CustomData>
<ofd:CustomData Name="发票代码">050001700111</ofd:CustomData>
<ofd:CustomData Name="发票号码">12235358</ofd:CustomData>
<ofd:CustomData Name="合计金额">10000.00</ofd:CustomData>
<ofd:CustomData Name="开票日期">2020年08月05日</ofd:CustomData>
<ofd:CustomData Name="校验码">16630282980991510014</ofd:CustomData>
<ofd:CustomData Name="购买方名称">个人</ofd:CustomData>
<ofd:CustomData Name="销售方名称">财天下科技有限公司</ofd:CustomData>
<ofd:CustomData Name="销售方纳税人识别号">91500102563638729E</ofd:CustomData>
</ofd:CustomDatas>
</ofd:DocInfo>
<ofd:DocRoot>Doc_0/Document.xml</ofd:DocRoot>
<ofd:Signatures>/Doc_0/Signs/Signatures.xml</ofd:Signatures>
</ofd:DocBody>
</ofd:OFD>"#;

const OFD_XML_LEGACY_NS: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<ofd:OFD xmlns:ofd="http://www.ofdspec.org" DocType="OFD" Version="1.0">
  <ofd:DocBody>
    <ofd:DocInfo>
      <ofd:DocID>6b2ee2c019b211eb80002b8300002b83</ofd:DocID>
      <ofd:Title>未命名</ofd:Title>
      <ofd:Author>Administrator</ofd:Author>
      <ofd:CreationDate>2020-10-29</ofd:CreationDate>
      <ofd:ModDate>2020-10-29</ofd:ModDate>
      <ofd:Creator>Suwell-pdf2ofd</ofd:Creator>
      <ofd:CreatorVersion>1.0.19.0910</ofd:CreatorVersion>
      <ofd:CustomDatas>
        <ofd:CustomData Name="VersionDetail">12447-Sep 10 2019, 17:41:44</ofd:CustomData>
      </ofd:CustomDatas>
    </ofd:DocInfo>
    <ofd:DocRoot>Doc_0/Document.xml</ofd:DocRoot>
  </ofd:DocBody>
</ofd:OFD>"#;

#[test]
fn ofd_round_trip() {
  let ofd: Ofd = OFD_XML.parse().unwrap();

  assert_eq!(ofd.version, "1.0");
  assert!(matches!(ofd.doc_type, OfdDocType::Ofd));
  assert_eq!(ofd.doc_body.len(), 1);

  let doc_body = &ofd.doc_body[0];
  assert_eq!(doc_body.doc_root, "Doc_0/Document.xml");
  assert_eq!(
    doc_body.signatures.as_deref(),
    Some("/Doc_0/Signs/Signatures.xml")
  );

  let doc_info = &doc_body.doc_info;
  assert_eq!(doc_info.doc_id, "050001700111_12235358");
  assert_eq!(doc_info.author.as_deref(), Some("Huhuang Software"));
  assert_eq!(doc_info.creation_date.as_deref(), Some("2020-08-17"));
  assert_eq!(doc_info.mod_date.as_deref(), Some("2020-08-17"));
  assert_eq!(doc_info.creator.as_deref(), Some("Huhuang Software"));

  let custom_datas = doc_info.custom_datas.as_ref().unwrap();
  assert_eq!(custom_datas.custom_data.len(), 11);
  assert_eq!(custom_datas.custom_data[0].name, "native-producer");
  assert_eq!(custom_datas.custom_data[0].xml_value, "HuhuangSDK");
  assert_eq!(custom_datas.custom_data[10].name, "销售方纳税人识别号");
  assert_eq!(custom_datas.custom_data[10].xml_value, "91500102563638729E");

  let serialized = ofd.to_xml().unwrap();

  assert_eq!(
    serialized,
    r#"<ofd:OFD xmlns:ofd="http://www.ofdspec.org/2016" Version="1.0" DocType="OFD"><ofd:DocBody><ofd:DocInfo><ofd:DocID>050001700111_12235358</ofd:DocID><ofd:Author>Huhuang Software</ofd:Author><ofd:CreationDate>2020-08-17</ofd:CreationDate><ofd:ModDate>2020-08-17</ofd:ModDate><ofd:Creator>Huhuang Software</ofd:Creator><ofd:CustomDatas><ofd:CustomData Name="native-producer">HuhuangSDK</ofd:CustomData><ofd:CustomData Name="producer-version">v1.0.0518.2</ofd:CustomData><ofd:CustomData Name="价税合计">10600.00</ofd:CustomData><ofd:CustomData Name="发票代码">050001700111</ofd:CustomData><ofd:CustomData Name="发票号码">12235358</ofd:CustomData><ofd:CustomData Name="合计金额">10000.00</ofd:CustomData><ofd:CustomData Name="开票日期">2020年08月05日</ofd:CustomData><ofd:CustomData Name="校验码">16630282980991510014</ofd:CustomData><ofd:CustomData Name="购买方名称">个人</ofd:CustomData><ofd:CustomData Name="销售方名称">财天下科技有限公司</ofd:CustomData><ofd:CustomData Name="销售方纳税人识别号">91500102563638729E</ofd:CustomData></ofd:CustomDatas></ofd:DocInfo><ofd:DocRoot>Doc_0/Document.xml</ofd:DocRoot><ofd:Signatures>/Doc_0/Signs/Signatures.xml</ofd:Signatures></ofd:DocBody></ofd:OFD>"#
  );

  let reparsed: Ofd = serialized.parse().unwrap();
  assert_eq!(reparsed.doc_body.len(), 1);
  assert_eq!(
    reparsed.doc_body[0]
      .doc_info
      .custom_datas
      .as_ref()
      .unwrap()
      .custom_data[6]
      .xml_value,
    "2020年08月05日"
  );
}

#[test]
fn ofd_legacy_namespace_round_trip() {
  let ofd: Ofd = OFD_XML_LEGACY_NS.parse().unwrap();

  assert_eq!(ofd.version, "1.0");
  assert!(matches!(ofd.doc_type, OfdDocType::Ofd));
  assert_eq!(ofd.doc_body.len(), 1);

  let doc_body = &ofd.doc_body[0];
  assert_eq!(doc_body.doc_root, "Doc_0/Document.xml");
  assert!(doc_body.signatures.is_none());

  let doc_info = &doc_body.doc_info;
  assert_eq!(doc_info.doc_id, "6b2ee2c019b211eb80002b8300002b83");
  assert_eq!(doc_info.title.as_deref(), Some("未命名"));
  assert_eq!(doc_info.author.as_deref(), Some("Administrator"));
  assert_eq!(doc_info.creation_date.as_deref(), Some("2020-10-29"));
  assert_eq!(doc_info.mod_date.as_deref(), Some("2020-10-29"));
  assert_eq!(doc_info.creator.as_deref(), Some("Suwell-pdf2ofd"));
  assert_eq!(doc_info.creator_version.as_deref(), Some("1.0.19.0910"));

  let custom_datas = doc_info.custom_datas.as_ref().unwrap();
  assert_eq!(custom_datas.custom_data.len(), 1);
  assert_eq!(custom_datas.custom_data[0].name, "VersionDetail");
  assert_eq!(
    custom_datas.custom_data[0].xml_value,
    "12447-Sep 10 2019, 17:41:44"
  );

  let serialized = ofd.to_xml().unwrap();

  assert_eq!(
    serialized,
    r#"<ofd:OFD xmlns:ofd="http://www.ofdspec.org/2016" Version="1.0" DocType="OFD"><ofd:DocBody><ofd:DocInfo><ofd:DocID>6b2ee2c019b211eb80002b8300002b83</ofd:DocID><ofd:Title>未命名</ofd:Title><ofd:Author>Administrator</ofd:Author><ofd:CreationDate>2020-10-29</ofd:CreationDate><ofd:ModDate>2020-10-29</ofd:ModDate><ofd:Creator>Suwell-pdf2ofd</ofd:Creator><ofd:CreatorVersion>1.0.19.0910</ofd:CreatorVersion><ofd:CustomDatas><ofd:CustomData Name="VersionDetail">12447-Sep 10 2019, 17:41:44</ofd:CustomData></ofd:CustomDatas></ofd:DocInfo><ofd:DocRoot>Doc_0/Document.xml</ofd:DocRoot></ofd:DocBody></ofd:OFD>"#
  );
}
