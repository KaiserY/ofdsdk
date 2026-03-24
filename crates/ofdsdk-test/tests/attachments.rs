use ofdsdk::schemas::attachments::Attachments;

const ATTACHMENTS_XML: &str = r#"<ofd:Attachments xmlns:ofd="http://www.ofdspec.org/2016">
<ofd:Attachment ID="8" Name="original_invoice" Format="xml" CreationDate="2020-08-17T19:13:09 " Size="30.9297" Visible="false">
<ofd:FileLoc>original_invoice.xml</ofd:FileLoc>
</ofd:Attachment>
</ofd:Attachments>"#;

#[test]
fn attachments_round_trip() {
  let attachments: Attachments = ATTACHMENTS_XML.parse().unwrap();

  assert_eq!(attachments.attachment.len(), 1);

  let attachment = &attachments.attachment[0];
  assert_eq!(attachment.id, "8");
  assert_eq!(attachment.name, "original_invoice");
  assert_eq!(attachment.format.as_deref(), Some("xml"));
  assert_eq!(
    attachment.creation_date.as_deref(),
    Some("2020-08-17T19:13:09 ")
  );
  assert_eq!(attachment.size, Some(30.9297));
  assert_eq!(attachment.visible, Some(false));
  assert_eq!(attachment.file_loc, "original_invoice.xml");

  let serialized = attachments.to_xml().unwrap();

  assert_eq!(
    serialized,
    r#"<ofd:Attachments xmlns:ofd="http://www.ofdspec.org/2016"><ofd:Attachment ID="8" Name="original_invoice" Format="xml" CreationDate="2020-08-17T19:13:09 " Size="30.9297" Visible="false"><ofd:FileLoc>original_invoice.xml</ofd:FileLoc></ofd:Attachment></ofd:Attachments>"#
  );

  let reparsed: Attachments = serialized.parse().unwrap();
  assert_eq!(reparsed.attachment.len(), 1);
  assert_eq!(reparsed.attachment[0].file_loc, "original_invoice.xml");
}
