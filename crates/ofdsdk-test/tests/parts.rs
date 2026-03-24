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
  assert_eq!(annotations.annotations[0].root_element.annot[0].creator, "");

  let custom_tags = document.custom_tags.as_ref().unwrap();
  assert_eq!(custom_tags.root_element.custom_tag.len(), 1);
  assert_eq!(custom_tags.root_element.custom_tag[0].name_space, "");
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
