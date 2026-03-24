use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use ofdsdk::schemas::document::Document;
use ofdsdk_test::fixtures::DOCUMENT_PREFERENCES_XML;
use std::hint::black_box;
use std::io::{BufReader, Cursor};

fn bench_document_read_write(c: &mut Criterion) {
  let xml = DOCUMENT_PREFERENCES_XML;
  let parsed = xml.parse::<Document>().unwrap();
  let mut group = c.benchmark_group("document");

  group.throughput(Throughput::Bytes(xml.len() as u64));
  group.bench_with_input(BenchmarkId::new("read", "xml"), &xml, |b, xml| {
    b.iter(|| black_box(xml).parse::<Document>().unwrap())
  });

  group.bench_with_input(
    BenchmarkId::new("read_from_reader", "cursor"),
    &xml,
    |b, xml| b.iter(|| Document::from_reader(Cursor::new(black_box(xml.as_bytes()))).unwrap()),
  );

  group.bench_with_input(
    BenchmarkId::new("read_from_reader", "bufreader_cursor"),
    &xml,
    |b, xml| {
      b.iter(|| {
        Document::from_reader(BufReader::new(Cursor::new(black_box(xml.as_bytes())))).unwrap()
      })
    },
  );

  group.bench_with_input(
    BenchmarkId::new("write", "parsed_document"),
    &parsed,
    |b, document| b.iter(|| black_box(document).to_xml().unwrap()),
  );

  group.bench_with_input(BenchmarkId::new("round_trip", "xml"), &xml, |b, xml| {
    b.iter(|| {
      let document = black_box(xml).parse::<Document>().unwrap();
      let serialized = black_box(&document).to_xml().unwrap();
      black_box(serialized)
    })
  });

  group.finish();
}

criterion_group!(benches, bench_document_read_write);
criterion_main!(benches);
