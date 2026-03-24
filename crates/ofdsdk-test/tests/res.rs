use ofdsdk::schemas::res::{CtMultiMediaType, Res, ResContentChoice};

const RES_XML: &str = r#"<ofd:Res xmlns:ofd="http://www.ofdspec.org/2016" BaseLoc="Res">
<ofd:MultiMedias>
<ofd:MultiMedia ID="6" Type="Image" Format="PNG">
<ofd:MediaFile>qrcode.png</ofd:MediaFile>
</ofd:MultiMedia>
</ofd:MultiMedias>
</ofd:Res>"#;

const RES_COLOR_FONT_XML: &str = r#"<ofd:Res xmlns:ofd="http://www.ofdspec.org/2016" BaseLoc="Res">
<ofd:ColorSpaces>
<ofd:ColorSpace ID="1" Type="RGB" BitsPerComponent="8"/>
</ofd:ColorSpaces>
<ofd:Fonts>
<ofd:Font ID="2" FontName="楷体" FamilyName="楷体"/>
<ofd:Font ID="3" FontName="宋体" FamilyName="宋体"/>
<ofd:Font ID="4" FontName="Courier New" FamilyName="Courier New"/>
<ofd:Font ID="5" FontName="黑体" FamilyName="黑体"/>
</ofd:Fonts>
</ofd:Res>"#;

const RES_DRAW_PARAM_XML: &str = r#"<ofd:Res xmlns:ofd="http://www.ofdspec.org/2016" BaseLoc="Res"><ofd:DrawParams><ofd:DrawParam ID="8"><ofd:FillColor Value="127 127 127"/><ofd:StrokeColor Value="0 0 0"/></ofd:DrawParam></ofd:DrawParams></ofd:Res>"#;

#[test]
fn res_round_trip() {
  let res: Res = RES_XML.parse().unwrap();

  assert_eq!(res.base_loc, "Res");
  assert_eq!(res.xml_children.len(), 1);

  let multi_medias = match &res.xml_children[0] {
    ResContentChoice::MultiMedias(value) => value,
    _ => panic!("expected MultiMedias"),
  };

  assert_eq!(multi_medias.multi_media.len(), 1);

  let multi_media = &multi_medias.multi_media[0];
  assert_eq!(multi_media.id, 6);
  assert!(matches!(multi_media.r#type, CtMultiMediaType::Image));
  assert_eq!(multi_media.format.as_deref(), Some("PNG"));
  assert_eq!(multi_media.media_file, "qrcode.png");

  let serialized = res.to_xml().unwrap();

  assert_eq!(
    serialized,
    r#"<ofd:Res xmlns:ofd="http://www.ofdspec.org/2016" BaseLoc="Res"><ofd:MultiMedias><ofd:MultiMedia Type="Image" Format="PNG" ID="6"><ofd:MediaFile>qrcode.png</ofd:MediaFile></ofd:MultiMedia></ofd:MultiMedias></ofd:Res>"#
  );

  let reparsed: Res = serialized.parse().unwrap();
  assert_eq!(reparsed.xml_children.len(), 1);
}

#[test]
fn res_color_spaces_and_fonts_round_trip() {
  let res: Res = RES_COLOR_FONT_XML.parse().unwrap();

  assert_eq!(res.base_loc, "Res");
  assert_eq!(res.xml_children.len(), 2);

  let color_spaces = match &res.xml_children[0] {
    ResContentChoice::ColorSpaces(value) => value,
    _ => panic!("expected ColorSpaces"),
  };
  assert_eq!(color_spaces.color_space.len(), 1);
  assert_eq!(color_spaces.color_space[0].id, 1);
  assert!(matches!(
    color_spaces.color_space[0].r#type,
    ofdsdk::schemas::res::CtColorSpaceType::Rgb
  ));
  assert_eq!(color_spaces.color_space[0].bits_per_component, Some(8));

  let fonts = match &res.xml_children[1] {
    ResContentChoice::Fonts(value) => value,
    _ => panic!("expected Fonts"),
  };
  assert_eq!(fonts.font.len(), 4);
  assert_eq!(fonts.font[0].font_name, "楷体");
  assert_eq!(fonts.font[1].family_name.as_deref(), Some("宋体"));
  assert_eq!(fonts.font[2].font_name, "Courier New");
  assert_eq!(fonts.font[3].id, 5);

  let serialized = res.to_xml().unwrap();

  assert_eq!(
    serialized,
    r#"<ofd:Res xmlns:ofd="http://www.ofdspec.org/2016" BaseLoc="Res"><ofd:ColorSpaces><ofd:ColorSpace Type="RGB" BitsPerComponent="8" ID="1"></ofd:ColorSpace></ofd:ColorSpaces><ofd:Fonts><ofd:Font FontName="楷体" FamilyName="楷体" ID="2"></ofd:Font><ofd:Font FontName="宋体" FamilyName="宋体" ID="3"></ofd:Font><ofd:Font FontName="Courier New" FamilyName="Courier New" ID="4"></ofd:Font><ofd:Font FontName="黑体" FamilyName="黑体" ID="5"></ofd:Font></ofd:Fonts></ofd:Res>"#
  );

  let reparsed: Res = serialized.parse().unwrap();
  assert_eq!(reparsed.xml_children.len(), 2);
}

#[test]
fn res_draw_params_round_trip() {
  let res: Res = RES_DRAW_PARAM_XML.parse().unwrap();

  assert_eq!(res.base_loc, "Res");
  assert_eq!(res.xml_children.len(), 1);

  let draw_params = match &res.xml_children[0] {
    ResContentChoice::DrawParams(value) => value,
    _ => panic!("expected DrawParams"),
  };

  assert_eq!(draw_params.draw_param.len(), 1);

  let draw_param = &draw_params.draw_param[0];
  assert_eq!(draw_param.id, 8);
  assert_eq!(
    draw_param.fill_color.as_ref().unwrap().value.as_deref(),
    Some("127 127 127")
  );
  assert_eq!(
    draw_param.fill_color.as_ref().unwrap().xml_children.len(),
    0
  );
  assert_eq!(
    draw_param.stroke_color.as_ref().unwrap().value.as_deref(),
    Some("0 0 0")
  );
  assert_eq!(
    draw_param.stroke_color.as_ref().unwrap().xml_children.len(),
    0
  );

  let serialized = res.to_xml().unwrap();

  assert_eq!(
    serialized,
    r#"<ofd:Res xmlns:ofd="http://www.ofdspec.org/2016" BaseLoc="Res"><ofd:DrawParams><ofd:DrawParam ID="8"><ofd:FillColor Value="127 127 127"></ofd:FillColor><ofd:StrokeColor Value="0 0 0"></ofd:StrokeColor></ofd:DrawParam></ofd:DrawParams></ofd:Res>"#
  );

  let reparsed: Res = serialized.parse().unwrap();
  let draw_params = match &reparsed.xml_children[0] {
    ResContentChoice::DrawParams(value) => value,
    _ => panic!("expected DrawParams"),
  };
  assert_eq!(
    draw_params.draw_param[0]
      .fill_color
      .as_ref()
      .unwrap()
      .value
      .as_deref(),
    Some("127 127 127")
  );
}
