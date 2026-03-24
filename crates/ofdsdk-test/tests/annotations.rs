const XML: &str = r#"<ofd:Annotations xmlns:ofd="http://www.ofdspec.org/2016">
<ofd:Page PageID="10">
<ofd:FileLoc>Page_0/Annotation.xml</ofd:FileLoc>
</ofd:Page>
</ofd:Annotations>"#;

#[test]
fn annotations_round_trip() {
  let annotations = XML
    .parse::<ofdsdk::schemas::annotations::Annotations>()
    .unwrap();

  assert_eq!(annotations.page.len(), 1);

  let page = &annotations.page[0];
  assert_eq!(page.page_id, 10);
  assert_eq!(page.file_loc, "Page_0/Annotation.xml");

  let serialized = annotations.to_xml().unwrap();
  assert_eq!(
    serialized,
    r#"<ofd:Annotations xmlns:ofd="http://www.ofdspec.org/2016"><ofd:Page PageID="10"><ofd:FileLoc>Page_0/Annotation.xml</ofd:FileLoc></ofd:Page></ofd:Annotations>"#
  );

  let reparsed = serialized
    .parse::<ofdsdk::schemas::annotations::Annotations>()
    .unwrap();
  assert_eq!(reparsed.page.len(), 1);
  assert_eq!(reparsed.page[0].page_id, 10);
  assert_eq!(reparsed.page[0].file_loc, "Page_0/Annotation.xml");
}
