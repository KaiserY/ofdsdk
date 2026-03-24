use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use ofdsdk::schemas::page::Page;
use ofdsdk_test::fixtures::PAGE_XML;
use std::hint::black_box;
use std::io::{BufReader, Cursor};

fn bench_page_read_write(c: &mut Criterion) {
  let xml = PAGE_XML;
  let parsed = xml.parse::<Page>().unwrap();
  let mut group = c.benchmark_group("page");

  group.throughput(Throughput::Bytes(xml.len() as u64));
  group.bench_with_input(BenchmarkId::new("read", "xml"), &xml, |b, xml| {
    b.iter(|| black_box(xml).parse::<Page>().unwrap())
  });

  group.bench_with_input(
    BenchmarkId::new("read_from_reader", "cursor"),
    &xml,
    |b, xml| b.iter(|| Page::from_reader(Cursor::new(black_box(xml.as_bytes()))).unwrap()),
  );

  group.bench_with_input(
    BenchmarkId::new("read_from_reader", "bufreader_cursor"),
    &xml,
    |b, xml| {
      b.iter(|| Page::from_reader(BufReader::new(Cursor::new(black_box(xml.as_bytes())))).unwrap())
    },
  );

  group.bench_with_input(
    BenchmarkId::new("write", "parsed_page"),
    &parsed,
    |b, page| b.iter(|| black_box(page).to_xml().unwrap()),
  );

  group.bench_with_input(BenchmarkId::new("round_trip", "xml"), &xml, |b, xml| {
    b.iter(|| {
      let page = black_box(xml).parse::<Page>().unwrap();
      let serialized = black_box(&page).to_xml().unwrap();
      black_box(serialized)
    })
  });

  group.finish();
}

criterion_group!(benches, bench_page_read_write);
criterion_main!(benches);
