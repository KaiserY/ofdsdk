use ofdsdk::schemas::custom_tags::CustomTags;

const CUSTOM_TAGS_XML: &str = r#"<ofd:CustomTags xmlns:ofd="http://www.ofdspec.org/2016">
<ofd:CustomTag>
<ofd:SchemaLoc>schemas/custom-tags.xsd</ofd:SchemaLoc>
<ofd:FileLoc>Tags/CustomTags.xml</ofd:FileLoc>
</ofd:CustomTag>
</ofd:CustomTags>"#;

#[test]
fn custom_tags_round_trip() {
  let custom_tags: CustomTags = CUSTOM_TAGS_XML.parse().unwrap();

  assert_eq!(custom_tags.custom_tag.len(), 1);

  let tag = &custom_tags.custom_tag[0];
  assert_eq!(tag.name_space, "");
  assert_eq!(tag.schema_loc.as_deref(), Some("schemas/custom-tags.xsd"));
  assert_eq!(tag.file_loc, "Tags/CustomTags.xml");

  let serialized = custom_tags.to_xml().unwrap();

  assert_eq!(
    serialized,
    r#"<ofd:CustomTags xmlns:ofd="http://www.ofdspec.org/2016"><ofd:CustomTag NameSpace=""><ofd:SchemaLoc>schemas/custom-tags.xsd</ofd:SchemaLoc><ofd:FileLoc>Tags/CustomTags.xml</ofd:FileLoc></ofd:CustomTag></ofd:CustomTags>"#
  );

  let reparsed: CustomTags = serialized.parse().unwrap();
  assert_eq!(reparsed.custom_tag.len(), 1);
  assert_eq!(reparsed.custom_tag[0].name_space, "");
}
