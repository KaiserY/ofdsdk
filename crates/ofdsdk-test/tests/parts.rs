use std::io::{Cursor, Read, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use ofdsdk_test::fixtures::{DOCUMENT_PREFERENCES_XML, PAGE_SIMPLE_TEXT_XML};

fn sample_path(name: &str) -> std::path::PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("samples/{name}"))
}

fn save_path(name: &str) -> std::path::PathBuf {
  std::env::temp_dir().join(format!(
    "ofdsdk_parts_{}_{}_{}",
    name,
    std::process::id(),
    SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_nanos()
  ))
}

const MINIMAL_OFD_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<ofd:OFD xmlns:ofd="http://www.ofdspec.org/2016" DocType="OFD" Version="1.0">
  <ofd:DocBody>
    <ofd:DocInfo>
      <ofd:DocID>minimal-doc-id</ofd:DocID>
    </ofd:DocInfo>
    <ofd:DocRoot>Doc_0/Document.xml</ofd:DocRoot>
  </ofd:DocBody>
</ofd:OFD>"#;

const MINIMAL_DOCUMENT_XML: &str = r#"<ofd:Document xmlns:ofd="http://www.ofdspec.org/2016">
<ofd:CommonData>
<ofd:MaxUnitID>10</ofd:MaxUnitID>
<ofd:PageArea>
<ofd:PhysicalBox>0 0 210 140</ofd:PhysicalBox>
</ofd:PageArea>
</ofd:CommonData>
<ofd:Pages>
<ofd:Page ID="1" BaseLoc="Pages/Page_0/Content.xml"/>
</ofd:Pages>
</ofd:Document>"#;

const MINIMAL_DOCUMENT_WITH_RES_XML: &str = r#"<ofd:Document xmlns:ofd="http://www.ofdspec.org/2016">
<ofd:CommonData>
<ofd:MaxUnitID>10</ofd:MaxUnitID>
<ofd:PageArea>
<ofd:PhysicalBox>0 0 210 140</ofd:PhysicalBox>
</ofd:PageArea>
<ofd:DocumentRes>DocumentRes.xml</ofd:DocumentRes>
</ofd:CommonData>
<ofd:Pages>
<ofd:Page ID="1" BaseLoc="Pages/Page_0/Content.xml"/>
</ofd:Pages>
</ofd:Document>"#;

const DOCUMENT_RES_WITH_MISSING_MEDIA_XML: &str = r#"<ofd:Res xmlns:ofd="http://www.ofdspec.org/2016" BaseLoc="Res">
<ofd:MultiMedias>
<ofd:MultiMedia ID="10" Type="Image" Format="PNG"><ofd:MediaFile>present.png</ofd:MediaFile></ofd:MultiMedia>
<ofd:MultiMedia ID="11" Type="Image" Format="PNG"><ofd:MediaFile>missing.png</ofd:MediaFile></ofd:MultiMedia>
</ofd:MultiMedias>
</ofd:Res>"#;

const PAGE_LAYER_WITHOUT_ID_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<ofd:Page xmlns:ofd="http://www.ofdspec.org/2016">
  <ofd:Content>
    <ofd:Layer Type="Body">
      <ofd:ImageObject ID="issue-qrcode" CTM="2.56 0 0 2.56 0 0" Boundary="154.72 24.72 2.56 2.56" ResourceID="55001"/>
    </ofd:Layer>
  </ofd:Content>
</ofd:Page>"#;

fn package_bytes(entries: &[(&str, &[u8])]) -> Vec<u8> {
  let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
  let options = zip::write::SimpleFileOptions::default();

  for (path, content) in entries {
    zip.start_file(path, options).unwrap();
    zip.write_all(content).unwrap();
  }

  zip.finish().unwrap().into_inner()
}

fn package_bytes_with_page(page_xml: &str) -> Vec<u8> {
  package_bytes(&[
    ("OFD.xml", MINIMAL_OFD_XML.as_bytes()),
    ("Doc_0/Document.xml", MINIMAL_DOCUMENT_XML.as_bytes()),
    ("Doc_0/Pages/Page_0/Content.xml", page_xml.as_bytes()),
  ])
}

#[test]
fn ofd_package_from_transparency_sample_file() {
  let sample_path = sample_path("透明度文字.ofd");

  let package = ofdsdk::parts::ofd_package::OfdPackage::new_from_file(&sample_path).unwrap();

  assert_eq!(package.root_element.doc_body.len(), 1);
  assert_eq!(package.documents.len(), 1);

  let document = &package.documents[0];
  assert_eq!(document.doc_body.doc_root, "Doc_0/Document.xml");
  assert_eq!(document.pages.len(), 1);
  assert!(document.template_pages.is_empty());
  assert_eq!(document.public_res.len(), 1);
  assert_eq!(document.document_res.len(), 1);
  assert!(document.annotations.is_some());
  assert!(document.signatures.is_none());
  assert!(document.versions.is_empty());

  let annotations = document.annotations.as_ref().unwrap();
  assert_eq!(annotations.annotations.len(), 1);

  let public_res = &document.public_res[0];
  assert!(public_res.public_res_font_files.is_empty());
  assert!(public_res.public_res_media_files.is_empty());

  let document_res = &document.document_res[0];
  assert!(document_res.document_res_font_files.is_empty());
  assert!(document_res.document_res_media_files.is_empty());

  let save_path = save_path("transparency.ofd");

  package.save_to_file(&save_path).unwrap();

  let saved_package = ofdsdk::parts::ofd_package::OfdPackage::new_from_file(&save_path).unwrap();
  assert_eq!(saved_package.documents.len(), 1);
  assert_eq!(saved_package.documents[0].pages.len(), 1);

  std::fs::remove_file(save_path).unwrap();
}

#[test]
fn ofd_package_save_minimal_constructed_package() {
  let root_element = MINIMAL_OFD_XML
    .parse::<ofdsdk::schemas::ofd::Ofd>()
    .unwrap();
  let doc_body = root_element.doc_body[0].clone();
  let document_root = DOCUMENT_PREFERENCES_XML
    .parse::<ofdsdk::schemas::document::Document>()
    .unwrap();
  let page_root = PAGE_SIMPLE_TEXT_XML
    .parse::<ofdsdk::schemas::page::Page>()
    .unwrap();

  let package = ofdsdk::parts::ofd_package::OfdPackage {
    inner_path: "OFD.xml".to_string(),
    other_parts: vec![],
    root_element,
    documents: vec![ofdsdk::parts::document::Document {
      inner_path: "Doc_0/Document.xml".to_string(),
      doc_body,
      root_element: document_root,
      pages: vec![ofdsdk::parts::page::Page {
        inner_path: "Doc_0/Pages/Page_0/Content.xml".to_string(),
        root_element: page_root,
        page_res: vec![],
      }],
      template_pages: vec![],
      annotations: None,
      attachments: None,
      custom_tags: None,
      extensions: None,
      public_res: vec![],
      document_res: vec![],
      signatures: None,
      versions: vec![],
    }],
  };

  let save_path = save_path("minimal_constructed.ofd");

  package.save_to_file(&save_path).unwrap();

  let saved_package = ofdsdk::parts::ofd_package::OfdPackage::new_from_file(&save_path).unwrap();
  assert_eq!(saved_package.root_element.doc_body.len(), 1);
  assert_eq!(saved_package.documents.len(), 1);

  let saved_document = &saved_package.documents[0];
  assert_eq!(saved_document.inner_path, "Doc_0/Document.xml");
  assert_eq!(saved_document.pages.len(), 1);
  assert!(saved_document.public_res.is_empty());
  assert!(saved_document.document_res.is_empty());

  let saved_page = &saved_document.pages[0];
  let layer = &saved_page.root_element.content.as_ref().unwrap().layer[0];
  match &layer.xml_children[0] {
    ofdsdk::schemas::page::LayerContentChoice::TextObject(text) => {
      assert_eq!(text.text_code[0].xml_value, "你好呀，OFD Reader&Writer！");
    }
    other => panic!("unexpected saved page child: {other:?}"),
  }

  std::fs::remove_file(save_path).unwrap();
}

#[test]
fn ofd_package_accepts_page_layer_without_id() {
  let package = ofdsdk::parts::ofd_package::OfdPackage::new(Cursor::new(package_bytes_with_page(
    PAGE_LAYER_WITHOUT_ID_XML,
  )))
  .unwrap();

  assert_eq!(package.documents.len(), 1);

  let document = &package.documents[0];
  assert_eq!(document.pages.len(), 1);

  let page = &document.pages[0].root_element;
  let layer = &page.content.as_ref().unwrap().layer[0];
  assert_eq!(layer.id, None);

  match &layer.xml_children[0] {
    ofdsdk::schemas::page::LayerContentChoice::ImageObject(image) => {
      assert_eq!(image.id, "issue-qrcode");
      assert_eq!(image.resource_id, 55001);
    }
    other => panic!("unexpected layer child: {other:?}"),
  }
}

#[test]
fn ofd_package_skips_missing_resource_blob_parts() {
  let package_bytes = package_bytes(&[
    ("OFD.xml", MINIMAL_OFD_XML.as_bytes()),
    (
      "Doc_0/Document.xml",
      MINIMAL_DOCUMENT_WITH_RES_XML.as_bytes(),
    ),
    (
      "Doc_0/Pages/Page_0/Content.xml",
      PAGE_SIMPLE_TEXT_XML.as_bytes(),
    ),
    (
      "Doc_0/DocumentRes.xml",
      DOCUMENT_RES_WITH_MISSING_MEDIA_XML.as_bytes(),
    ),
    ("Doc_0/Res/present.png", b"present image bytes"),
  ]);

  let package = ofdsdk::parts::ofd_package::OfdPackage::new(Cursor::new(package_bytes)).unwrap();
  let document_res = &package.documents[0].document_res[0];

  let multi_medias = match &document_res.root_element.xml_children[0] {
    ofdsdk::schemas::res::ResContentChoice::MultiMedias(value) => value,
    other => panic!("unexpected document res child: {other:?}"),
  };

  assert_eq!(multi_medias.multi_media.len(), 2);
  assert_eq!(document_res.document_res_media_files.len(), 1);
  assert_eq!(
    document_res.document_res_media_files[0].inner_path,
    "Doc_0/Res/present.png"
  );

  let mut saved = Cursor::new(Vec::new());
  package.save(&mut saved).unwrap();
  let saved_bytes = saved.into_inner();
  let saved_package =
    ofdsdk::parts::ofd_package::OfdPackage::new(Cursor::new(saved_bytes.clone())).unwrap();

  assert_eq!(saved_package.documents[0].document_res.len(), 1);
  assert_eq!(
    saved_package.documents[0].document_res[0]
      .document_res_media_files
      .len(),
    1
  );

  let mut saved_archive = zip::ZipArchive::new(Cursor::new(saved_bytes)).unwrap();
  assert!(saved_archive.by_name("Doc_0/Res/present.png").is_ok());
  assert!(saved_archive.by_name("Doc_0/Res/missing.png").is_err());
}

#[test]
fn ofd_package_still_requires_structural_page_part() {
  let package_bytes = package_bytes(&[
    ("OFD.xml", MINIMAL_OFD_XML.as_bytes()),
    ("Doc_0/Document.xml", MINIMAL_DOCUMENT_XML.as_bytes()),
  ]);

  let error = ofdsdk::parts::ofd_package::OfdPackage::new(Cursor::new(package_bytes)).unwrap_err();

  assert!(matches!(
    error,
    ofdsdk::common::SdkError::ZipError(zip::result::ZipError::FileNotFound)
  ));
}

#[test]
fn ofd_package_from_hello_world_sample_file() {
  let package =
    ofdsdk::parts::ofd_package::OfdPackage::new_from_file(sample_path("helloworld.ofd")).unwrap();

  assert_eq!(package.root_element.doc_body.len(), 1);
  assert_eq!(package.documents.len(), 1);

  let document = &package.documents[0];
  assert_eq!(document.pages.len(), 1);
  assert!(document.template_pages.is_empty());
  assert_eq!(document.public_res.len(), 1);
  assert!(document.document_res.is_empty());
  assert!(document.annotations.is_none());
  assert!(document.signatures.is_none());
  assert!(document.versions.is_empty());
}

#[test]
fn ofd_package_from_windows_encoded_resource_sample_file() {
  let package =
    ofdsdk::parts::ofd_package::OfdPackage::new_from_file(sample_path("chineseDir_windows.ofd"))
      .unwrap();

  assert_eq!(package.root_element.doc_body.len(), 1);
  assert_eq!(package.documents.len(), 1);

  let document = &package.documents[0];
  assert_eq!(document.pages.len(), 1);
  assert_eq!(document.public_res.len(), 1);
  assert!(document.document_res.is_empty());
  assert!(document.annotations.is_none());
  assert!(document.signatures.is_none());
  assert!(document.versions.is_empty());
  assert_eq!(package.other_parts.len(), 1);

  let other_part = &package.other_parts[0];
  assert!(other_part.inner_path.starts_with("Doc_0/Res/"));
  assert!(other_part.inner_path.ends_with(".txt"));
  assert!(!other_part.part_content.is_empty());

  let save_path = save_path("chinese_dir_windows.ofd");
  package.save_to_file(&save_path).unwrap();

  let mut archive = zip::ZipArchive::new(std::fs::File::open(&save_path).unwrap()).unwrap();
  let mut saved_content = Vec::new();
  archive
    .by_name(&other_part.inner_path)
    .unwrap()
    .read_to_end(&mut saved_content)
    .unwrap();
  assert_eq!(saved_content, other_part.part_content.as_ref());

  std::fs::remove_file(save_path).unwrap();
}

#[test]
fn ofd_package_from_999_sample_file() {
  let package =
    ofdsdk::parts::ofd_package::OfdPackage::new_from_file(sample_path("999.ofd")).unwrap();

  assert_eq!(package.root_element.doc_body.len(), 1);
  assert_eq!(package.documents.len(), 1);

  let document = &package.documents[0];
  assert_eq!(document.pages.len(), 5);
  assert_eq!(document.template_pages.len(), 5);
  assert_eq!(document.public_res.len(), 1);
  assert_eq!(document.document_res.len(), 1);
  assert!(document.annotations.is_some());
  assert!(document.attachments.is_some());
  assert!(document.custom_tags.is_some());
  assert!(document.signatures.is_some());
  assert!(document.versions.is_empty());

  let signatures = document.signatures.as_ref().unwrap();
  assert_eq!(signatures.root_element.signature.len(), 1);
  assert_eq!(signatures.signatures.len(), 1);

  let signature = &signatures.signatures[0];
  assert_eq!(
    signature.root_element.signed_info.references.check_method,
    "1.2.156.10197.1.401"
  );
  assert_eq!(
    signature
      .root_element
      .signed_info
      .references
      .reference
      .len(),
    20
  );
  assert!(!signature.signature_value.part_content.is_empty());

  let annotations = document.annotations.as_ref().unwrap();
  assert_eq!(annotations.annotations.len(), 1);
  assert_eq!(annotations.annotations[0].root_element.annot.len(), 1);
  assert_eq!(
    annotations.annotations[0].root_element.annot[0].creator,
    None
  );

  let custom_tags = document.custom_tags.as_ref().unwrap();
  assert_eq!(custom_tags.root_element.custom_tag.len(), 1);
  assert_eq!(custom_tags.root_element.custom_tag[0].name_space, None);
}

#[test]
fn ofd_package_from_zsbk_sample_file() {
  let package =
    ofdsdk::parts::ofd_package::OfdPackage::new_from_file(sample_path("zsbk.ofd")).unwrap();

  assert_eq!(package.root_element.doc_body.len(), 1);
  assert_eq!(package.documents.len(), 1);

  let document = &package.documents[0];
  assert_eq!(document.pages.len(), 2);
  assert_eq!(document.template_pages.len(), 2);
  assert_eq!(document.public_res.len(), 1);
  assert_eq!(document.document_res.len(), 1);
  assert!(document.annotations.is_none());
  assert!(document.attachments.is_none());
  assert!(document.custom_tags.is_none());
  assert!(document.signatures.is_some());
  assert!(document.versions.is_empty());

  let signatures = document.signatures.as_ref().unwrap();
  assert_eq!(signatures.root_element.signature.len(), 2);
  assert_eq!(signatures.signatures.len(), 2);
  assert!(
    signatures
      .signatures
      .iter()
      .all(|signature| !signature.signature_value.part_content.is_empty())
  );
}
