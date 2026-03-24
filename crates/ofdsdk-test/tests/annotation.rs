const XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<ofd:PageAnnot xmlns:ofd="http://www.ofdspec.org/2016"><ofd:Annot Type="Stamp" ID="62" Creator="Signature1" LastModDate="2019-05-27 15:37:52" ReadOnly="false" Subtype="PDFWidgetSign"><Parameters><Parameter Name="userinfo.userid">user_id_suwell_pdf2ofd</Parameter><Parameter Name="PDFWidgetObjNum">34</Parameter></Parameters><ofd:Appearance Boundary="166.8639 106.1897 42.6861 29.2806"><ofd:CompositeObject ID="66" CTM="0.3528 0 0 0.3528 0 0" Boundary="0 0 42.6861 29.2806" ResourceID="67"/></ofd:Appearance></ofd:Annot></ofd:PageAnnot>"#;

const WATERMARK_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<ofd:PageAnnot xmlns:ofd="http://www.ofdspec.org/2016"><ofd:Annot Type="Watermark" Creator="OFD R&amp;W" LastModDate="2024-09-29" ID="5"><ofd:Appearance Boundary="0 0 420 297"><ofd:TextObject Boundary="0 0 420 297" Font="6" Size="5.644" ID="7" Fill="true" Alpha="76" CTM="0.866 -0.500 0.500 0.866 -25.135 -13.206" DrawParam="8"><ofd:TextCode X="-11.281" Y="5.644" DeltaX="5.641 5.641 5.641">测试水印</ofd:TextCode></ofd:TextObject></ofd:Appearance></ofd:Annot><ofd:Annot Type="Watermark" Creator="OFD R&amp;W" LastModDate="2024-09-29" ID="9"><ofd:Appearance Boundary="0 0 420 297"><ofd:TextObject Boundary="0 0 420 297" Font="6" Size="5.644" ID="10" Fill="true" Alpha="76" CTM="0.866 -0.500 0.500 0.866 61.622 -13.206" DrawParam="8"><ofd:TextCode X="-11.281" Y="5.644" DeltaX="5.641 5.641 5.641">测试水印</ofd:TextCode></ofd:TextObject></ofd:Appearance></ofd:Annot><ofd:Annot Type="Watermark" Creator="OFD R&amp;W" LastModDate="2024-09-29" ID="11"><ofd:Appearance Boundary="0 0 420 297"><ofd:TextObject Boundary="0 0 420 297" Font="6" Size="5.644" ID="12" Fill="true" Alpha="76" CTM="0.866 -0.500 0.500 0.866 148.378 -13.206" DrawParam="8"><ofd:TextCode X="-11.281" Y="5.644" DeltaX="5.641 5.641 5.641">测试水印</ofd:TextCode></ofd:TextObject></ofd:Appearance></ofd:Annot><ofd:Annot Type="Watermark" Creator="OFD R&amp;W" LastModDate="2024-09-29" ID="13"><ofd:Appearance Boundary="0 0 420 297"><ofd:TextObject Boundary="0 0 420 297" Font="6" Size="5.644" ID="14" Fill="true" Alpha="76" CTM="0.866 -0.500 0.500 0.866 235.135 -13.206" DrawParam="8"><ofd:TextCode X="-11.281" Y="5.644" DeltaX="5.641 5.641 5.641">测试水印</ofd:TextCode></ofd:TextObject></ofd:Appearance></ofd:Annot><ofd:Annot Type="Watermark" Creator="OFD R&amp;W" LastModDate="2024-09-29" ID="15"><ofd:Appearance Boundary="0 0 420 297"><ofd:TextObject Boundary="0 0 420 297" Font="6" Size="5.644" ID="16" Fill="true" Alpha="76" CTM="0.866 -0.500 0.500 0.866 -25.135 67.647" DrawParam="8"><ofd:TextCode X="-11.281" Y="5.644" DeltaX="5.641 5.641 5.641">测试水印</ofd:TextCode></ofd:TextObject></ofd:Appearance></ofd:Annot><ofd:Annot Type="Watermark" Creator="OFD R&amp;W" LastModDate="2024-09-29" ID="17"><ofd:Appearance Boundary="0 0 420 297"><ofd:TextObject Boundary="0 0 420 297" Font="6" Size="5.644" ID="18" Fill="true" Alpha="76" CTM="0.866 -0.500 0.500 0.866 61.622 67.647" DrawParam="8"><ofd:TextCode X="-11.281" Y="5.644" DeltaX="5.641 5.641 5.641">测试水印</ofd:TextCode></ofd:TextObject></ofd:Appearance></ofd:Annot><ofd:Annot Type="Watermark" Creator="OFD R&amp;W" LastModDate="2024-09-29" ID="19"><ofd:Appearance Boundary="0 0 420 297"><ofd:TextObject Boundary="0 0 420 297" Font="6" Size="5.644" ID="20" Fill="true" Alpha="76" CTM="0.866 -0.500 0.500 0.866 148.378 67.647" DrawParam="8"><ofd:TextCode X="-11.281" Y="5.644" DeltaX="5.641 5.641 5.641">测试水印</ofd:TextCode></ofd:TextObject></ofd:Appearance></ofd:Annot><ofd:Annot Type="Watermark" Creator="OFD R&amp;W" LastModDate="2024-09-29" ID="21"><ofd:Appearance Boundary="0 0 420 297"><ofd:TextObject Boundary="0 0 420 297" Font="6" Size="5.644" ID="22" Fill="true" Alpha="76" CTM="0.866 -0.500 0.500 0.866 235.135 67.647" DrawParam="8"><ofd:TextCode X="-11.281" Y="5.644" DeltaX="5.641 5.641 5.641">测试水印</ofd:TextCode></ofd:TextObject></ofd:Appearance></ofd:Annot><ofd:Annot Type="Watermark" Creator="OFD R&amp;W" LastModDate="2024-09-29" ID="23"><ofd:Appearance Boundary="0 0 420 297"><ofd:TextObject Boundary="0 0 420 297" Font="6" Size="5.644" ID="24" Fill="true" Alpha="76" CTM="0.866 -0.500 0.500 0.866 -25.135 148.500" DrawParam="8"><ofd:TextCode X="-11.281" Y="5.644" DeltaX="5.641 5.641 5.641">测试水印</ofd:TextCode></ofd:TextObject></ofd:Appearance></ofd:Annot><ofd:Annot Type="Watermark" Creator="OFD R&amp;W" LastModDate="2024-09-29" ID="25"><ofd:Appearance Boundary="0 0 420 297"><ofd:TextObject Boundary="0 0 420 297" Font="6" Size="5.644" ID="26" Fill="true" Alpha="76" CTM="0.866 -0.500 0.500 0.866 61.622 148.500" DrawParam="8"><ofd:TextCode X="-11.281" Y="5.644" DeltaX="5.641 5.641 5.641">测试水印</ofd:TextCode></ofd:TextObject></ofd:Appearance></ofd:Annot><ofd:Annot Type="Watermark" Creator="OFD R&amp;W" LastModDate="2024-09-29" ID="27"><ofd:Appearance Boundary="0 0 420 297"><ofd:TextObject Boundary="0 0 420 297" Font="6" Size="5.644" ID="28" Fill="true" Alpha="76" CTM="0.866 -0.500 0.500 0.866 148.378 148.500" DrawParam="8"><ofd:TextCode X="-11.281" Y="5.644" DeltaX="5.641 5.641 5.641">测试水印</ofd:TextCode></ofd:TextObject></ofd:Appearance></ofd:Annot><ofd:Annot Type="Watermark" Creator="OFD R&amp;W" LastModDate="2024-09-29" ID="29"><ofd:Appearance Boundary="0 0 420 297"><ofd:TextObject Boundary="0 0 420 297" Font="6" Size="5.644" ID="30" Fill="true" Alpha="76" CTM="0.866 -0.500 0.500 0.866 235.135 148.500" DrawParam="8"><ofd:TextCode X="-11.281" Y="5.644" DeltaX="5.641 5.641 5.641">测试水印</ofd:TextCode></ofd:TextObject></ofd:Appearance></ofd:Annot><ofd:Annot Type="Watermark" Creator="OFD R&amp;W" LastModDate="2024-09-29" ID="31"><ofd:Appearance Boundary="0 0 420 297"><ofd:TextObject Boundary="0 0 420 297" Font="6" Size="5.644" ID="32" Fill="true" Alpha="76" CTM="0.866 -0.500 0.500 0.866 -25.135 229.353" DrawParam="8"><ofd:TextCode X="-11.281" Y="5.644" DeltaX="5.641 5.641 5.641">测试水印</ofd:TextCode></ofd:TextObject></ofd:Appearance></ofd:Annot><ofd:Annot Type="Watermark" Creator="OFD R&amp;W" LastModDate="2024-09-29" ID="33"><ofd:Appearance Boundary="0 0 420 297"><ofd:TextObject Boundary="0 0 420 297" Font="6" Size="5.644" ID="34" Fill="true" Alpha="76" CTM="0.866 -0.500 0.500 0.866 61.622 229.353" DrawParam="8"><ofd:TextCode X="-11.281" Y="5.644" DeltaX="5.641 5.641 5.641">测试水印</ofd:TextCode></ofd:TextObject></ofd:Appearance></ofd:Annot><ofd:Annot Type="Watermark" Creator="OFD R&amp;W" LastModDate="2024-09-29" ID="35"><ofd:Appearance Boundary="0 0 420 297"><ofd:TextObject Boundary="0 0 420 297" Font="6" Size="5.644" ID="36" Fill="true" Alpha="76" CTM="0.866 -0.500 0.500 0.866 148.378 229.353" DrawParam="8"><ofd:TextCode X="-11.281" Y="5.644" DeltaX="5.641 5.641 5.641">测试水印</ofd:TextCode></ofd:TextObject></ofd:Appearance></ofd:Annot><ofd:Annot Type="Watermark" Creator="OFD R&amp;W" LastModDate="2024-09-29" ID="37"><ofd:Appearance Boundary="0 0 420 297"><ofd:TextObject Boundary="0 0 420 297" Font="6" Size="5.644" ID="38" Fill="true" Alpha="76" CTM="0.866 -0.500 0.500 0.866 235.135 229.353" DrawParam="8"><ofd:TextCode X="-11.281" Y="5.644" DeltaX="5.641 5.641 5.641">测试水印</ofd:TextCode></ofd:TextObject></ofd:Appearance></ofd:Annot><ofd:Annot Type="Watermark" Creator="OFD R&amp;W" LastModDate="2024-09-29" ID="39"><ofd:Appearance Boundary="0 0 420 297"><ofd:TextObject Boundary="0 0 420 297" Font="6" Size="5.644" ID="40" Fill="true" Alpha="76" CTM="0.866 -0.500 0.500 0.866 -25.135 310.206" DrawParam="8"><ofd:TextCode X="-11.281" Y="5.644" DeltaX="5.641 5.641 5.641">测试水印</ofd:TextCode></ofd:TextObject></ofd:Appearance></ofd:Annot><ofd:Annot Type="Watermark" Creator="OFD R&amp;W" LastModDate="2024-09-29" ID="41"><ofd:Appearance Boundary="0 0 420 297"><ofd:TextObject Boundary="0 0 420 297" Font="6" Size="5.644" ID="42" Fill="true" Alpha="76" CTM="0.866 -0.500 0.500 0.866 61.622 310.206" DrawParam="8"><ofd:TextCode X="-11.281" Y="5.644" DeltaX="5.641 5.641 5.641">测试水印</ofd:TextCode></ofd:TextObject></ofd:Appearance></ofd:Annot><ofd:Annot Type="Watermark" Creator="OFD R&amp;W" LastModDate="2024-09-29" ID="43"><ofd:Appearance Boundary="0 0 420 297"><ofd:TextObject Boundary="0 0 420 297" Font="6" Size="5.644" ID="44" Fill="true" Alpha="76" CTM="0.866 -0.500 0.500 0.866 148.378 310.206" DrawParam="8"><ofd:TextCode X="-11.281" Y="5.644" DeltaX="5.641 5.641 5.641">测试水印</ofd:TextCode></ofd:TextObject></ofd:Appearance></ofd:Annot><ofd:Annot Type="Watermark" Creator="OFD R&amp;W" LastModDate="2024-09-29" ID="45"><ofd:Appearance Boundary="0 0 420 297"><ofd:TextObject Boundary="0 0 420 297" Font="6" Size="5.644" ID="46" Fill="true" Alpha="76" CTM="0.866 -0.500 0.500 0.866 235.135 310.206" DrawParam="8"><ofd:TextCode X="-11.281" Y="5.644" DeltaX="5.641 5.641 5.641">测试水印</ofd:TextCode></ofd:TextObject></ofd:Appearance></ofd:Annot></ofd:PageAnnot>"#;

#[test]
fn page_annot_round_trip() {
  let page_annot = XML
    .parse::<ofdsdk::schemas::annotation::PageAnnot>()
    .unwrap();

  assert_eq!(page_annot.annot.len(), 1);

  let annot = &page_annot.annot[0];
  assert_eq!(annot.id, 62);
  assert!(matches!(
    annot.r#type,
    ofdsdk::schemas::annotation::AnnotType::Stamp
  ));
  assert_eq!(annot.creator, "Signature1");
  assert_eq!(annot.last_mod_date, "2019-05-27 15:37:52");
  assert_eq!(annot.read_only, Some(false));
  assert_eq!(annot.subtype.as_deref(), Some("PDFWidgetSign"));

  let parameters = annot.parameters.as_ref().unwrap();
  assert_eq!(parameters.parameter.len(), 2);
  assert_eq!(parameters.parameter[0].name, "userinfo.userid");
  assert_eq!(parameters.parameter[0].xml_value, "user_id_suwell_pdf2ofd");
  assert_eq!(parameters.parameter[1].name, "PDFWidgetObjNum");
  assert_eq!(parameters.parameter[1].xml_value, "34");

  assert_eq!(
    annot.appearance.boundary.as_deref(),
    Some("166.8639 106.1897 42.6861 29.2806")
  );
  assert_eq!(annot.appearance.xml_children.len(), 1);

  match &annot.appearance.xml_children[0] {
    ofdsdk::schemas::annotation::AppearanceContentChoice::CompositeObject(object) => {
      assert_eq!(object.id, 66);
      assert_eq!(object.resource_id, 67);
      assert_eq!(object.boundary, "0 0 42.6861 29.2806");
      assert_eq!(object.ctm.as_deref(), Some("0.3528 0 0 0.3528 0 0"));
    }
    other => panic!("unexpected appearance child: {other:?}"),
  }

  let serialized = page_annot.to_xml().unwrap();

  assert_eq!(
    serialized,
    r#"<ofd:PageAnnot xmlns:ofd="http://www.ofdspec.org/2016"><ofd:Annot ID="62" Type="Stamp" Creator="Signature1" LastModDate="2019-05-27 15:37:52" Subtype="PDFWidgetSign" ReadOnly="false"><ofd:Parameters><ofd:Parameter Name="userinfo.userid">user_id_suwell_pdf2ofd</ofd:Parameter><ofd:Parameter Name="PDFWidgetObjNum">34</ofd:Parameter></ofd:Parameters><ofd:Appearance Boundary="166.8639 106.1897 42.6861 29.2806"><ofd:CompositeObject Boundary="0 0 42.6861 29.2806" CTM="0.3528 0 0 0.3528 0 0" ResourceID="67" ID="66"></ofd:CompositeObject></ofd:Appearance></ofd:Annot></ofd:PageAnnot>"#
  );

  let reparsed = serialized
    .parse::<ofdsdk::schemas::annotation::PageAnnot>()
    .unwrap();
  assert_eq!(reparsed.annot.len(), 1);
  assert_eq!(
    reparsed.annot[0].parameters.as_ref().unwrap().parameter[1].xml_value,
    "34"
  );
}

#[test]
fn page_annot_watermark_grid_round_trip() {
  let page_annot = WATERMARK_XML
    .parse::<ofdsdk::schemas::annotation::PageAnnot>()
    .unwrap();

  assert_eq!(page_annot.annot.len(), 20);

  let first = &page_annot.annot[0];
  assert!(matches!(
    first.r#type,
    ofdsdk::schemas::annotation::AnnotType::Watermark
  ));
  assert_eq!(first.id, 5);
  assert_eq!(first.creator, "OFD R&W");
  assert_eq!(first.last_mod_date, "2024-09-29");
  assert_eq!(first.appearance.boundary.as_deref(), Some("0 0 420 297"));
  assert_eq!(first.appearance.xml_children.len(), 1);

  match &first.appearance.xml_children[0] {
    ofdsdk::schemas::annotation::AppearanceContentChoice::TextObject(text) => {
      assert_eq!(text.id, 7);
      assert_eq!(text.boundary, "0 0 420 297");
      assert_eq!(text.font, 6);
      assert_eq!(text.size, 5.644);
      assert_eq!(text.fill, Some(true));
      assert_eq!(text.alpha, Some(76));
      assert_eq!(text.draw_param, Some(8));
      assert_eq!(
        text.ctm.as_deref(),
        Some("0.866 -0.500 0.500 0.866 -25.135 -13.206")
      );
      assert_eq!(text.text_code.len(), 1);
      assert_eq!(text.text_code[0].x, Some(-11.281));
      assert_eq!(text.text_code[0].y, Some(5.644));
      assert_eq!(
        text.text_code[0].delta_x.as_deref(),
        Some("5.641 5.641 5.641")
      );
      assert_eq!(text.text_code[0].xml_value, "测试水印");
    }
    other => panic!("unexpected appearance child: {other:?}"),
  }

  let last = &page_annot.annot[19];
  assert_eq!(last.id, 45);
  match &last.appearance.xml_children[0] {
    ofdsdk::schemas::annotation::AppearanceContentChoice::TextObject(text) => {
      assert_eq!(text.id, 46);
      assert_eq!(
        text.ctm.as_deref(),
        Some("0.866 -0.500 0.500 0.866 235.135 310.206")
      );
      assert_eq!(text.text_code[0].xml_value, "测试水印");
    }
    other => panic!("unexpected last appearance child: {other:?}"),
  }

  let serialized = page_annot.to_xml().unwrap();
  let reparsed = serialized
    .parse::<ofdsdk::schemas::annotation::PageAnnot>()
    .unwrap();

  assert_eq!(reparsed.annot.len(), 20);
  assert_eq!(reparsed.annot[0].creator, "OFD R&W");
  assert_eq!(reparsed.annot[19].id, 45);
}
