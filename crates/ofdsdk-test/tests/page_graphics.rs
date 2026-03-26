use ofdsdk::schemas::page::{
  CellContentContentChoice, CtAxialShd, CtColor, CtColorContentChoice, CtPattern, CtRadialShd,
  PathObject,
};

const COLOR_XML: &str =
  r#"<ofd:CT_Color xmlns:ofd="http://www.ofdspec.org/2016" Value="255 255 255" Alpha="255"/>"#;

const PATTERN_XML: &str = r#"<ofd:CT_Pattern xmlns:ofd="http://www.ofdspec.org/2016" Width="50" Height="50"><ofd:CellContent><ofd:TextObject Boundary="0 0 10 10" Font="3" Size="3" ID="4"><ofd:TextCode X="0" Y="3">A</ofd:TextCode></ofd:TextObject></ofd:CellContent></ofd:CT_Pattern>"#;

const AXIAL_SHD_XML: &str = r#"<ofd:CT_AxialShd xmlns:ofd="http://www.ofdspec.org/2016" MapType="Reflect" MapUnit="30" StartPoint="0 0" EndPoint="140 0"><ofd:Segment Position="0"><ofd:Color Value="255 255 0"/></ofd:Segment><ofd:Segment Position="1"><ofd:Color Value="0 0 255"/></ofd:Segment></ofd:CT_AxialShd>"#;

const RADIAL_SHD_XML: &str = r#"<ofd:CT_RadialShd xmlns:ofd="http://www.ofdspec.org/2016" Extend="1" StartPoint="40 70" StartRadius="10" EndPoint="140 70" EndRadius="50"><ofd:Segment Position="0"><ofd:Color Value="255 255 0"/></ofd:Segment><ofd:Segment Position="1"><ofd:Color Value="0 0 255"/></ofd:Segment></ofd:CT_RadialShd>"#;

const PATH_OBJECT_XML: &str = r#"<ofd:PathObject xmlns:ofd="http://www.ofdspec.org/2016" Boundary="10 10 140 40" Fill="true" ID="10005"><ofd:FillColor><ofd:AxialShd MapType="Reflect" MapUnit="30" StartPoint="0 0" EndPoint="140 0"><ofd:Segment Position="0"><ofd:Color Value="255 255 0"/></ofd:Segment><ofd:Segment Position="1"><ofd:Color Value="0 0 255"/></ofd:Segment></ofd:AxialShd></ofd:FillColor><ofd:AbbreviatedData>M 0 0 L 140 0 L 140 40 L 0 40 C</ofd:AbbreviatedData></ofd:PathObject>"#;

#[test]
fn color_round_trip() {
  let color: CtColor = COLOR_XML.parse().unwrap();

  assert_eq!(color.value.as_deref(), Some("255 255 255"));
  assert_eq!(color.alpha, Some(255));
  assert_eq!(color.xml_children.len(), 0);

  let serialized = color.to_xml().unwrap();
  let reparsed: CtColor = serialized.parse().unwrap();

  assert_eq!(reparsed.value.as_deref(), Some("255 255 255"));
  assert_eq!(reparsed.alpha, Some(255));
}

#[test]
fn pattern_round_trip() {
  let pattern: CtPattern = PATTERN_XML.parse().unwrap();

  assert_eq!(pattern.width, 50.0);
  assert_eq!(pattern.height, 50.0);
  assert_eq!(pattern.cell_content.xml_children.len(), 1);

  match &pattern.cell_content.xml_children[0] {
    CellContentContentChoice::TextObject(text) => {
      assert_eq!(text.boundary, "0 0 10 10");
      assert_eq!(text.font, 3);
      assert_eq!(text.size, 3.0);
      assert_eq!(text.id, 4);
      assert_eq!(text.text_code.len(), 1);
      assert_eq!(text.text_code[0].xml_value, "A");
    }
    other => panic!("unexpected pattern child: {other:?}"),
  }

  let serialized = pattern.to_xml().unwrap();
  let reparsed: CtPattern = serialized.parse().unwrap();

  assert_eq!(reparsed.width, 50.0);
  assert_eq!(reparsed.cell_content.xml_children.len(), 1);
}

#[test]
fn axial_shd_round_trip() {
  let axial_shd: CtAxialShd = AXIAL_SHD_XML.parse().unwrap();

  assert!(matches!(
    axial_shd.map_type,
    Some(ofdsdk::schemas::page::CtAxialShdMapType::Reflect)
  ));
  assert_eq!(axial_shd.map_unit, Some(30.0));
  assert_eq!(axial_shd.start_point, "0 0");
  assert_eq!(axial_shd.end_point, "140 0");
  assert_eq!(axial_shd.segment.len(), 2);
  assert_eq!(axial_shd.segment[0].position, Some(0.0));
  assert_eq!(axial_shd.segment[0].color.xml_children.len(), 0);

  let serialized = axial_shd.to_xml().unwrap();
  let reparsed: CtAxialShd = serialized.parse().unwrap();

  assert_eq!(reparsed.segment.len(), 2);
  assert_eq!(reparsed.segment[1].color.value.as_deref(), Some("0 0 255"));
}

#[test]
fn radial_shd_round_trip() {
  let radial_shd: CtRadialShd = RADIAL_SHD_XML.parse().unwrap();

  assert_eq!(radial_shd.extend, Some(1));
  assert_eq!(radial_shd.start_point, "40 70");
  assert_eq!(radial_shd.start_radius, Some(10.0));
  assert_eq!(radial_shd.end_point, "140 70");
  assert_eq!(radial_shd.end_radius, 50.0);
  assert_eq!(radial_shd.segment.len(), 2);

  let serialized = radial_shd.to_xml().unwrap();
  let reparsed: CtRadialShd = serialized.parse().unwrap();

  assert_eq!(
    reparsed.segment[0].color.value.as_deref(),
    Some("255 255 0")
  );
}

#[test]
fn path_object_fill_color_round_trip() {
  let path_object: PathObject = PATH_OBJECT_XML.parse().unwrap();

  assert_eq!(path_object.boundary, "10 10 140 40");
  assert_eq!(path_object.fill, Some(true));
  assert_eq!(path_object.id, 10005);
  assert_eq!(
    path_object.abbreviated_data,
    "M 0 0 L 140 0 L 140 40 L 0 40 C"
  );
  assert_eq!(
    path_object.fill_color.as_ref().unwrap().xml_children.len(),
    1
  );

  match &path_object.fill_color.as_ref().unwrap().xml_children[0] {
    CtColorContentChoice::AxialShd(axial_shd) => {
      assert_eq!(axial_shd.segment.len(), 2);
    }
    other => panic!("unexpected fill color child: {other:?}"),
  }

  let serialized = path_object.to_xml().unwrap();
  let reparsed: PathObject = serialized.parse().unwrap();

  assert_eq!(reparsed.id, 10005);
  match &reparsed.fill_color.as_ref().unwrap().xml_children[0] {
    CtColorContentChoice::AxialShd(axial_shd) => {
      assert!(matches!(
        axial_shd.map_type,
        Some(ofdsdk::schemas::page::CtAxialShdMapType::Reflect)
      ));
    }
    other => panic!("unexpected reparsed fill color child: {other:?}"),
  }
}
