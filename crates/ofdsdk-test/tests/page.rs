use ofdsdk_test::fixtures::{PAGE_SIMPLE_TEXT_XML, PAGE_XML};

#[test]
fn page_round_trip() {
  let page = PAGE_XML.parse::<ofdsdk::schemas::page::Page>().unwrap();

  assert_eq!(page.template.len(), 1);
  assert_eq!(page.template[0].template_id, 9);
  assert!(matches!(
    page.template[0].z_order,
    Some(ofdsdk::schemas::page::TemplateZOrder::Background)
  ));

  let area = page.area.as_ref().unwrap();
  assert_eq!(area.physical_box, "0 0 210 140");

  let content = page.content.as_ref().unwrap();
  assert_eq!(content.layer.len(), 1);

  let layer = &content.layer[0];
  assert_eq!(layer.id, 67);
  assert_eq!(layer.xml_children.len(), 23);

  match &layer.xml_children[0] {
    ofdsdk::schemas::page::LayerContentChoice::ImageObject(image) => {
      assert_eq!(image.id, 68);
      assert_eq!(image.resource_id, 6);
      assert_eq!(image.boundary, "8.5 3.5 20 20");
      assert_eq!(image.ctm.as_deref(), Some("20 0 0 20 0 0"));
    }
    other => panic!("unexpected first child: {other:?}"),
  }

  match &layer.xml_children[1] {
    ofdsdk::schemas::page::LayerContentChoice::TextObject(text) => {
      assert_eq!(text.id, 69);
      assert_eq!(text.font, 3);
      assert_eq!(text.size, 3.175);
      assert_eq!(text.text_code.len(), 1);
      assert_eq!(text.text_code[0].xml_value, "050001700111");
    }
    other => panic!("unexpected second child: {other:?}"),
  }

  match layer.xml_children.last().unwrap() {
    ofdsdk::schemas::page::LayerContentChoice::TextObject(text) => {
      assert_eq!(text.id, 90);
      assert_eq!(text.text_code[0].xml_value, "600.00");
    }
    other => panic!("unexpected last child: {other:?}"),
  }

  let serialized = page.to_xml().unwrap();
  let reparsed = serialized.parse::<ofdsdk::schemas::page::Page>().unwrap();

  assert_eq!(reparsed.template.len(), 1);
  assert_eq!(reparsed.area.as_ref().unwrap().physical_box, "0 0 210 140");
  assert_eq!(
    reparsed.content.as_ref().unwrap().layer[0]
      .xml_children
      .len(),
    23
  );
}

#[test]
fn page_simple_text_round_trip() {
  let page = PAGE_SIMPLE_TEXT_XML
    .parse::<ofdsdk::schemas::page::Page>()
    .unwrap();

  let content = page.content.as_ref().unwrap();
  assert_eq!(content.layer.len(), 1);

  let layer = &content.layer[0];
  assert_eq!(layer.id, 2);
  assert_eq!(layer.xml_children.len(), 1);

  match &layer.xml_children[0] {
    ofdsdk::schemas::page::LayerContentChoice::TextObject(text) => {
      assert_eq!(text.id, 4);
      assert_eq!(text.boundary, "31.7 25.4 40.5 5");
      assert_eq!(text.font, 3);
      assert_eq!(text.size, 3.0);
      assert_eq!(text.text_code.len(), 1);
      assert_eq!(text.text_code[0].x, Some(0.0));
      assert_eq!(text.text_code[0].y, Some(3.0));
      assert_eq!(
        text.text_code[0].delta_x.as_deref(),
        Some("3 3 3 3 1.5 1.5 1.5 1.5 1.5 1.5 1.5 1.5 1.5 1.5 1.5 1.5 1.5 1.5 1.5 1.5 1.5")
      );
      assert_eq!(text.text_code[0].xml_value, "你好呀，OFD Reader&Writer！");
    }
    other => panic!("unexpected layer child: {other:?}"),
  }

  let serialized = page.to_xml().unwrap();

  assert_eq!(
    serialized,
    r#"<ofd:Page xmlns:ofd="http://www.ofdspec.org/2016"><ofd:Content><ofd:Layer ID="2"><ofd:TextObject Boundary="31.7 25.4 40.5 5" Font="3" Size="3" ID="4"><ofd:TextCode X="0" Y="3" DeltaX="3 3 3 3 1.5 1.5 1.5 1.5 1.5 1.5 1.5 1.5 1.5 1.5 1.5 1.5 1.5 1.5 1.5 1.5 1.5">你好呀，OFD Reader&amp;Writer！</ofd:TextCode></ofd:TextObject></ofd:Layer></ofd:Content></ofd:Page>"#
  );

  let reparsed = serialized.parse::<ofdsdk::schemas::page::Page>().unwrap();
  assert_eq!(reparsed.content.as_ref().unwrap().layer[0].id, 2);
  match &reparsed.content.as_ref().unwrap().layer[0].xml_children[0] {
    ofdsdk::schemas::page::LayerContentChoice::TextObject(text) => {
      assert_eq!(text.text_code[0].xml_value, "你好呀，OFD Reader&Writer！");
    }
    other => panic!("unexpected reparsed layer child: {other:?}"),
  }
}
