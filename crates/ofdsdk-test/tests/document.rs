use ofdsdk::schemas::document::Document;

const DOCUMENT_XML: &str = r#"<ofd:Document xmlns:ofd="http://www.ofdspec.org/2016">
<ofd:CommonData>
<ofd:MaxUnitID>789</ofd:MaxUnitID>
<ofd:PageArea>
<ofd:PhysicalBox>0 0 210 140</ofd:PhysicalBox>
</ofd:PageArea>
<ofd:PublicRes>PublicRes.xml</ofd:PublicRes>
<ofd:DocumentRes>DocumentRes.xml</ofd:DocumentRes>
<ofd:TemplatePage ID="9" BaseLoc="Tpls/Tpl_0/Content.xml"/>
<ofd:TemplatePage ID="91" BaseLoc="Tpls/Tpl_1/Content.xml"/>
<ofd:TemplatePage ID="270" BaseLoc="Tpls/Tpl_2/Content.xml"/>
<ofd:TemplatePage ID="449" BaseLoc="Tpls/Tpl_3/Content.xml"/>
<ofd:TemplatePage ID="628" BaseLoc="Tpls/Tpl_4/Content.xml"/>
</ofd:CommonData>
<ofd:Pages>
<ofd:Page ID="10" BaseLoc="Pages/Page_0/Content.xml"/>
<ofd:Page ID="92" BaseLoc="Pages/Page_1/Content.xml"/>
<ofd:Page ID="271" BaseLoc="Pages/Page_2/Content.xml"/>
<ofd:Page ID="450" BaseLoc="Pages/Page_3/Content.xml"/>
<ofd:Page ID="629" BaseLoc="Pages/Page_4/Content.xml"/>
</ofd:Pages>
<ofd:Annotations>Annots/Annotations.xml</ofd:Annotations>
<ofd:Attachments>Attachs/Attachments.xml</ofd:Attachments>
<ofd:CustomTags>Tags/CustomTags.xml</ofd:CustomTags>
</ofd:Document>"#;

const DOCUMENT_VPREFERENCES_XML: &str = r#"<ofd:Document xmlns:ofd="http://www.ofdspec.org/2016">
<ofd:CommonData>
<ofd:MaxUnitID>10</ofd:MaxUnitID>
<ofd:PageArea>
<ofd:PhysicalBox>0 0 210 140</ofd:PhysicalBox>
</ofd:PageArea>
</ofd:CommonData>
<ofd:Pages>
<ofd:Page ID="1" BaseLoc="Pages/Page_0/Content.xml"/>
</ofd:Pages>
<ofd:VPreferences>
<ofd:PageMode>UseThumbs</ofd:PageMode>
<ofd:PageLayout>OneColumn</ofd:PageLayout>
<ofd:TabDisplay>DocTitle</ofd:TabDisplay>
<ofd:ZoomMode>FitWidth</ofd:ZoomMode>
<ofd:Zoom>3.5</ofd:Zoom>
</ofd:VPreferences>
</ofd:Document>"#;

#[test]
fn document_round_trip() {
  let document: Document = DOCUMENT_XML.parse().unwrap();

  assert_eq!(document.common_data.max_unit_id, 789);
  assert_eq!(document.common_data.page_area.physical_box, "0 0 210 140");
  assert_eq!(document.common_data.public_res, vec!["PublicRes.xml"]);
  assert_eq!(document.common_data.document_res, vec!["DocumentRes.xml"]);
  assert_eq!(document.common_data.template_page.len(), 5);
  assert_eq!(document.common_data.template_page[0].id, 9);
  assert_eq!(
    document.common_data.template_page[4].base_loc,
    "Tpls/Tpl_4/Content.xml"
  );

  assert_eq!(document.pages.page.len(), 5);
  assert_eq!(document.pages.page[0].id, 10);
  assert_eq!(document.pages.page[4].base_loc, "Pages/Page_4/Content.xml");

  assert_eq!(
    document.annotations.as_deref(),
    Some("Annots/Annotations.xml")
  );
  assert_eq!(
    document.attachments.as_deref(),
    Some("Attachs/Attachments.xml")
  );
  assert_eq!(document.custom_tags.as_deref(), Some("Tags/CustomTags.xml"));

  let serialized = document.to_xml().unwrap();

  assert_eq!(
    serialized,
    r#"<ofd:Document xmlns:ofd="http://www.ofdspec.org/2016"><ofd:CommonData><ofd:MaxUnitID>789</ofd:MaxUnitID><ofd:PageArea><ofd:PhysicalBox>0 0 210 140</ofd:PhysicalBox></ofd:PageArea><ofd:PublicRes>PublicRes.xml</ofd:PublicRes><ofd:DocumentRes>DocumentRes.xml</ofd:DocumentRes><ofd:TemplatePage ID="9" BaseLoc="Tpls/Tpl_0/Content.xml"/><ofd:TemplatePage ID="91" BaseLoc="Tpls/Tpl_1/Content.xml"/><ofd:TemplatePage ID="270" BaseLoc="Tpls/Tpl_2/Content.xml"/><ofd:TemplatePage ID="449" BaseLoc="Tpls/Tpl_3/Content.xml"/><ofd:TemplatePage ID="628" BaseLoc="Tpls/Tpl_4/Content.xml"/></ofd:CommonData><ofd:Pages><ofd:Page ID="10" BaseLoc="Pages/Page_0/Content.xml"/><ofd:Page ID="92" BaseLoc="Pages/Page_1/Content.xml"/><ofd:Page ID="271" BaseLoc="Pages/Page_2/Content.xml"/><ofd:Page ID="450" BaseLoc="Pages/Page_3/Content.xml"/><ofd:Page ID="629" BaseLoc="Pages/Page_4/Content.xml"/></ofd:Pages><ofd:Annotations>Annots/Annotations.xml</ofd:Annotations><ofd:CustomTags>Tags/CustomTags.xml</ofd:CustomTags><ofd:Attachments>Attachs/Attachments.xml</ofd:Attachments></ofd:Document>"#
  );

  let reparsed: Document = serialized.parse().unwrap();
  assert_eq!(reparsed.pages.page.len(), 5);
  assert_eq!(
    reparsed.common_data.template_page[2].base_loc,
    "Tpls/Tpl_2/Content.xml"
  );
}

#[test]
fn document_vpreferences_round_trip() {
  let document: Document = DOCUMENT_VPREFERENCES_XML.parse().unwrap();

  let v_preferences = document.v_preferences.as_ref().unwrap();
  assert!(matches!(
    v_preferences.page_mode,
    Some(ofdsdk::schemas::document::PageMode::UseThumbs)
  ));
  assert!(matches!(
    v_preferences.page_layout,
    Some(ofdsdk::schemas::document::PageLayout::OneColumn)
  ));
  assert!(matches!(
    v_preferences.tab_display,
    Some(ofdsdk::schemas::document::TabDisplay::DocTitle)
  ));
  assert_eq!(v_preferences.xml_children.len(), 2);

  match &v_preferences.xml_children[0] {
    ofdsdk::schemas::document::CtVPreferencesContentChoice::ZoomMode(mode) => {
      assert!(matches!(
        **mode,
        ofdsdk::schemas::document::ZoomMode::FitWidth
      ));
    }
    other => panic!("unexpected vpreferences child: {other:?}"),
  }

  match &v_preferences.xml_children[1] {
    ofdsdk::schemas::document::CtVPreferencesContentChoice::Zoom(value) => {
      assert_eq!(**value, 3.5);
    }
    other => panic!("unexpected vpreferences child: {other:?}"),
  }

  let serialized = document.to_xml().unwrap();
  let reparsed: Document = serialized.parse().unwrap();

  let reparsed_v_preferences = reparsed.v_preferences.as_ref().unwrap();
  assert_eq!(reparsed_v_preferences.xml_children.len(), 2);
  match &reparsed_v_preferences.xml_children[1] {
    ofdsdk::schemas::document::CtVPreferencesContentChoice::Zoom(value) => {
      assert_eq!(**value, 3.5);
    }
    other => panic!("unexpected reparsed vpreferences child: {other:?}"),
  }
}
