use std::collections::BTreeMap;
use std::io::{Cursor, Read};
use std::path::Path;

use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, XmlVersion};
use zip::ZipArchive;

#[derive(Clone, Debug, PartialEq, Eq)]
struct XmlElement {
  name: String,
  attrs: Vec<(String, String)>,
  children: Vec<XmlNode>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum XmlNode {
  Element(XmlElement),
  Text(String),
}

#[derive(Debug)]
struct XmlFrame {
  name: String,
  attrs: Vec<(String, String)>,
  children: Vec<XmlNode>,
}

struct KnownFailure {
  file: &'static str,
  reason: &'static str,
  issue: u32,
}

const KNOWN_FAILURES: &[KnownFailure] = &[];

#[test]
fn sample_packages_round_trip() {
  let sample_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("samples");
  let mut sample_paths = std::fs::read_dir(&sample_dir)
    .unwrap_or_else(|err| panic!("failed to read sample dir {}: {err}", sample_dir.display()))
    .map(|entry| {
      entry
        .unwrap_or_else(|err| panic!("failed to read sample dir entry: {err}"))
        .path()
    })
    .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("ofd"))
    .collect::<Vec<_>>();
  sample_paths.sort();

  let mut failures = Vec::new();
  let mut unexpected_successes = Vec::new();
  let mut passed = 0usize;
  let mut known_failed = 0usize;

  for path in sample_paths {
    let file_name = sample_name(&path);
    let known_failure = known_failure(&file_name);

    match (assert_sample_round_trip(&path), known_failure) {
      (Ok(()), None) => {
        passed += 1;
      }
      (Ok(()), Some(known_failure)) => {
        unexpected_successes.push(format!(
          "UNEXPECTED SUCCESS: {}\n  Was expected to fail: {}\n  Remove this entry from KNOWN_FAILURES.",
          known_failure.file, known_failure.reason
        ));
      }
      (Err(errors), None) => {
        failures.push(format!("{file_name}\n{}", format_errors(&errors)));
      }
      (Err(errors), Some(known_failure)) => {
        let issue = if known_failure.issue > 0 {
          format!("issue #{}", known_failure.issue)
        } else {
          "no issue filed yet".to_string()
        };
        println!(
          "KNOWN FAILURE: {} - {} ({issue})",
          known_failure.file, known_failure.reason
        );
        if std::env::var_os("OFDSDK_ROUND_TRIP_DETAILS").is_some() {
          print!("{}", format_errors(&errors));
        }
        known_failed += 1;
      }
    }
  }

  assert!(
    unexpected_successes.is_empty() && failures.is_empty(),
    "sample package round-trip mismatch\n{}",
    unexpected_successes
      .into_iter()
      .chain(failures)
      .collect::<Vec<_>>()
      .join("\n")
  );

  println!("sample package round-trip: {passed} passed, {known_failed} known failures");
}

fn assert_sample_round_trip(path: &Path) -> Result<(), Vec<String>> {
  let file_name = sample_name(path);
  let original_bytes = std::fs::read(path)
    .unwrap_or_else(|err| panic!("failed to read source sample {}: {err}", path.display()));
  let package = ofdsdk::parts::ofd_package::OfdPackage::new_from_file(path)
    .unwrap_or_else(|err| panic!("failed to open source sample {file_name}: {err:?}"));

  let mut buffer = Cursor::new(Vec::new());
  package
    .save(&mut buffer)
    .unwrap_or_else(|err| panic!("failed to save source sample {file_name}: {err:?}"));
  let roundtripped_bytes = buffer.into_inner();

  ofdsdk::parts::ofd_package::OfdPackage::new(Cursor::new(roundtripped_bytes.clone()))
    .unwrap_or_else(|err| panic!("failed to reopen saved sample {file_name}: {err:?}"));

  zip_equivalence_errors(&original_bytes, &roundtripped_bytes, &file_name)
}

fn zip_equivalence_errors(
  original: &[u8],
  roundtripped: &[u8],
  file_name: &str,
) -> Result<(), Vec<String>> {
  let original = read_zip_entries(original, file_name, "original");
  let roundtripped = read_zip_entries(roundtripped, file_name, "roundtripped");
  let mut errors = Vec::new();

  for name in original.keys() {
    if !roundtripped.contains_key(name) {
      errors.push(format!("missing zip entry: {name}"));
    }
  }

  for name in roundtripped.keys() {
    if !original.contains_key(name) {
      errors.push(format!("extra zip entry: {name}"));
    }
  }

  for (name, original_bytes) in &original {
    let Some(roundtripped_bytes) = roundtripped.get(name) else {
      continue;
    };

    if is_xml_entry(name) {
      errors.extend(xml_equivalence_errors(
        original_bytes,
        roundtripped_bytes,
        file_name,
        name,
      ));
    } else if original_bytes != roundtripped_bytes {
      errors.push(format!(
        "{name}: binary mismatch: original {} bytes, roundtripped {} bytes",
        original_bytes.len(),
        roundtripped_bytes.len()
      ));
    }
  }

  if errors.is_empty() {
    Ok(())
  } else {
    Err(errors)
  }
}

fn read_zip_entries(bytes: &[u8], file_name: &str, label: &str) -> BTreeMap<String, Vec<u8>> {
  let mut archive = ZipArchive::new(Cursor::new(bytes))
    .unwrap_or_else(|err| panic!("failed to open {label} zip for {file_name}: {err}"));
  let mut entries = BTreeMap::new();

  for idx in 0..archive.len() {
    let mut file = archive
      .by_index(idx)
      .unwrap_or_else(|err| panic!("failed to read {label} zip entry {idx}: {err}"));
    if file.is_dir() {
      continue;
    }

    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap_or_else(|err| {
      panic!(
        "failed to read {label} zip entry {} for {file_name}: {err}",
        file.name()
      )
    });
    entries.insert(file.name().to_string(), data);
  }

  entries
}

fn xml_equivalence_errors(
  original: &[u8],
  roundtripped: &[u8],
  file_name: &str,
  entry_name: &str,
) -> Vec<String> {
  let original_xml = canonicalize_xml(original, file_name, entry_name, "original");
  let roundtripped_xml = canonicalize_xml(roundtripped, file_name, entry_name, "roundtripped");
  compare_xml_nodes(
    &XmlNode::Element(original_xml),
    &XmlNode::Element(roundtripped_xml),
    entry_name,
  )
}

fn canonicalize_xml(bytes: &[u8], file_name: &str, entry_name: &str, label: &str) -> XmlElement {
  let mut reader = Reader::from_reader(Cursor::new(bytes));
  reader.config_mut().trim_text(false);
  let mut buf = Vec::new();
  let mut stack: Vec<XmlFrame> = Vec::new();

  loop {
    match reader.read_event_into(&mut buf).unwrap_or_else(|err| {
      panic!("failed to parse {label} XML {entry_name} in {file_name}: {err}")
    }) {
      Event::Start(e) => stack.push(frame_from_start(&reader, &e)),
      Event::Empty(e) => push_node(
        &mut stack,
        XmlNode::Element(element_from_start(&reader, &e)),
      ),
      Event::Text(text) => {
        let value = text
          .xml10_content()
          .unwrap_or_else(|err| panic!("failed to decode text in {entry_name}: {err}"))
          .into_owned();
        if !value.trim().is_empty() {
          push_node(&mut stack, XmlNode::Text(value));
        }
      }
      Event::CData(text) => {
        let value = text
          .decode()
          .unwrap_or_else(|err| panic!("failed to decode CDATA in {entry_name}: {err}"))
          .into_owned();
        if !value.trim().is_empty() {
          push_node(&mut stack, XmlNode::Text(value));
        }
      }
      Event::End(_) => {
        let frame = stack
          .pop()
          .unwrap_or_else(|| panic!("unexpected end tag in {entry_name}"));
        let element = XmlElement {
          name: frame.name,
          attrs: frame.attrs,
          children: frame.children,
        };
        if stack.is_empty() {
          return element;
        }
        push_node(&mut stack, XmlNode::Element(element));
      }
      Event::Eof => panic!("missing root element in {entry_name}"),
      _ => {}
    }
    buf.clear();
  }
}

fn frame_from_start(reader: &Reader<Cursor<&[u8]>>, e: &BytesStart<'_>) -> XmlFrame {
  XmlFrame {
    name: local_name(e.name().as_ref()),
    attrs: attrs_from_start(reader, e),
    children: Vec::new(),
  }
}

fn element_from_start(reader: &Reader<Cursor<&[u8]>>, e: &BytesStart<'_>) -> XmlElement {
  XmlElement {
    name: local_name(e.name().as_ref()),
    attrs: attrs_from_start(reader, e),
    children: Vec::new(),
  }
}

fn attrs_from_start(reader: &Reader<Cursor<&[u8]>>, e: &BytesStart<'_>) -> Vec<(String, String)> {
  let mut attrs = e
    .attributes()
    .with_checks(false)
    .filter_map(|attr| {
      let attr = attr.unwrap_or_else(|err| panic!("failed to read XML attribute: {err}"));
      let raw_name = std::str::from_utf8(attr.key.as_ref())
        .unwrap_or_else(|err| panic!("invalid XML attribute name: {err}"));
      if raw_name == "xmlns" || raw_name.starts_with("xmlns:") {
        return None;
      }
      let name = local_name(attr.key.as_ref());
      let value = attr
        .decoded_and_normalized_value(XmlVersion::Implicit1_0, reader.decoder())
        .unwrap_or_else(|err| panic!("failed to decode XML attribute {name}: {err}"))
        .into_owned();
      Some((name, value))
    })
    .collect::<Vec<_>>();
  attrs.sort();
  attrs
}

fn push_node(stack: &mut [XmlFrame], node: XmlNode) {
  let Some(frame) = stack.last_mut() else {
    panic!("XML node found before root element");
  };
  frame.children.push(node);
}

fn compare_xml_nodes(original: &XmlNode, roundtripped: &XmlNode, path: &str) -> Vec<String> {
  match (original, roundtripped) {
    (XmlNode::Text(original), XmlNode::Text(roundtripped)) => {
      if xml_value_equivalent(original, roundtripped) {
        Vec::new()
      } else {
        vec![format!(
          "{path}: text mismatch: original {original:?}, roundtripped {roundtripped:?}"
        )]
      }
    }
    (XmlNode::Element(original), XmlNode::Element(roundtripped)) => {
      compare_xml_elements(original, roundtripped, path)
    }
    (original, roundtripped) => vec![format!(
      "{path}: node kind mismatch: original {}, roundtripped {}",
      node_kind(original),
      node_kind(roundtripped)
    )],
  }
}

fn compare_xml_elements(
  original: &XmlElement,
  roundtripped: &XmlElement,
  path: &str,
) -> Vec<String> {
  let current_path = format!("{path}/{}", original.name);
  let mut errors = Vec::new();

  if original.name != roundtripped.name {
    errors.push(format!(
      "{path}: element name mismatch: original {}, roundtripped {}",
      original.name, roundtripped.name
    ));
    return errors;
  }

  errors.extend(compare_attrs(
    &original.attrs,
    &roundtripped.attrs,
    &current_path,
  ));
  errors.extend(compare_children(
    &original.children,
    &roundtripped.children,
    &current_path,
  ));
  errors
}

fn compare_attrs(
  original: &[(String, String)],
  roundtripped: &[(String, String)],
  path: &str,
) -> Vec<String> {
  let original = original.iter().cloned().collect::<BTreeMap<_, _>>();
  let roundtripped = roundtripped.iter().cloned().collect::<BTreeMap<_, _>>();
  let mut errors = Vec::new();

  for (name, value) in &original {
    match roundtripped.get(name) {
      Some(roundtripped_value) if xml_value_equivalent(value, roundtripped_value) => {}
      Some(roundtripped_value) => errors.push(format!(
        "{path}: attr {name} mismatch: original {value:?}, roundtripped {roundtripped_value:?}"
      )),
      None => errors.push(format!(
        "{path}: missing attr in roundtripped XML: {name}={value:?}"
      )),
    }
  }

  for (name, value) in &roundtripped {
    if !original.contains_key(name) {
      errors.push(format!(
        "{path}: extra attr in roundtripped XML: {name}={value:?}"
      ));
    }
  }

  errors
}

fn xml_value_equivalent(original: &str, roundtripped: &str) -> bool {
  original == roundtripped || numeric_lexical_equivalent(original, roundtripped)
}

fn numeric_lexical_equivalent(original: &str, roundtripped: &str) -> bool {
  if !is_decimal_lexical(original) && !is_decimal_lexical(roundtripped) {
    return false;
  }

  let Ok(original) = original.parse::<f64>() else {
    return false;
  };
  let Ok(roundtripped) = roundtripped.parse::<f64>() else {
    return false;
  };

  original.is_finite() && roundtripped.is_finite() && original == roundtripped
}

fn is_decimal_lexical(value: &str) -> bool {
  value.contains('.') || value.contains('e') || value.contains('E')
}

fn compare_children(original: &[XmlNode], roundtripped: &[XmlNode], path: &str) -> Vec<String> {
  if has_compatible_canonical_child_order(path) {
    return compare_children_unordered(original, roundtripped, path);
  }

  let mut errors = Vec::new();
  let len = original.len().max(roundtripped.len());

  for idx in 0..len {
    match (original.get(idx), roundtripped.get(idx)) {
      (Some(original), Some(roundtripped)) => {
        errors.extend(compare_xml_nodes(
          original,
          roundtripped,
          &format!("{path}[{idx}]"),
        ));
      }
      (Some(original), None) => errors.push(format!(
        "{path}: missing child in roundtripped XML: {}",
        node_summary(original)
      )),
      (None, Some(roundtripped)) => errors.push(format!(
        "{path}: extra child in roundtripped XML: {}",
        node_summary(roundtripped)
      )),
      (None, None) => {}
    }
  }

  errors
}

fn compare_children_unordered(
  original: &[XmlNode],
  roundtripped: &[XmlNode],
  path: &str,
) -> Vec<String> {
  let mut errors = Vec::new();
  let mut matched = vec![false; roundtripped.len()];

  for original_node in original {
    let Some(match_index) =
      roundtripped
        .iter()
        .enumerate()
        .position(|(index, roundtripped_node)| {
          !matched[index] && compare_xml_nodes(original_node, roundtripped_node, path).is_empty()
        })
    else {
      errors.push(format!(
        "{path}: missing child in roundtripped XML: {}",
        node_summary(original_node)
      ));
      continue;
    };

    matched[match_index] = true;
  }

  for (index, roundtripped_node) in roundtripped.iter().enumerate() {
    if !matched[index] {
      errors.push(format!(
        "{path}: extra child in roundtripped XML: {}",
        node_summary(roundtripped_node)
      ));
    }
  }

  errors
}

fn has_compatible_canonical_child_order(path: &str) -> bool {
  matches!(
    path.rsplit('/').next().unwrap_or(path),
    "Document" | "CommonData" | "Page" | "PathObject" | "DocInfo"
  )
}

fn local_name(name: &[u8]) -> String {
  let name = std::str::from_utf8(name).unwrap_or_else(|err| panic!("invalid XML name: {err}"));
  name
    .rsplit_once(':')
    .map(|(_, local)| local)
    .unwrap_or(name)
    .to_string()
}

fn is_xml_entry(name: &str) -> bool {
  name.ends_with(".xml")
}

fn sample_name(path: &Path) -> String {
  path
    .file_name()
    .and_then(|name| name.to_str())
    .unwrap_or_else(|| panic!("invalid sample path {}", path.display()))
    .to_string()
}

fn known_failure(file_name: &str) -> Option<&'static KnownFailure> {
  KNOWN_FAILURES
    .iter()
    .find(|known_failure| known_failure.file == file_name)
}

fn format_errors(errors: &[String]) -> String {
  const MAX_ERRORS: usize = 80;
  let mut out = String::new();

  for error in errors.iter().take(MAX_ERRORS) {
    out.push_str("- ");
    out.push_str(error);
    out.push('\n');
  }

  if errors.len() > MAX_ERRORS {
    out.push_str(&format!(
      "- ... {} additional mismatches omitted\n",
      errors.len() - MAX_ERRORS
    ));
  }

  out
}

fn node_kind(node: &XmlNode) -> &'static str {
  match node {
    XmlNode::Element(_) => "element",
    XmlNode::Text(_) => "text",
  }
}

fn node_summary(node: &XmlNode) -> String {
  match node {
    XmlNode::Element(element) => format!("<{}>", element.name),
    XmlNode::Text(text) => format!("text {text:?}"),
  }
}
