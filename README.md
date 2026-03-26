# ofdsdk

[![crates.io](https://img.shields.io/crates/v/ofdsdk.svg)](https://crates.io/crates/ofdsdk)
[![docs](https://docs.rs/ofdsdk/badge.svg)](https://docs.rs/ofdsdk)

`ofdsdk` is a Rust SDK for OFD documents.

Chinese version: [README_cn.md](README_cn.md)

## Goals

- Generate strongly typed Rust structures from OFD XSD files.
- Provide stable XML deserialization and serialization.
- Provide OFD package reading and saving when the `parts` feature is enabled.
- Keep real-world compatibility differences in generated data, not scattered runtime special cases.

Current version: `0.1.1`.

## Capabilities

- Rust schema types generated from `schemas/`
- XML deserialization from strings and readers
- XML serialization
- OFD package reading and saving with the `parts` feature
- Sample-based integration coverage for OFD packages

This repository contains three crates:

- `crates/ofdsdk`: runtime library for consumers
- `crates/ofdsdk-build`: code generator
- `crates/ofdsdk-test`: integration tests and sample validation

## Installation

If you only need XML schema types:

```toml
[dependencies]
ofdsdk = "0.1.1"
```

If you also need to read `.ofd` archives:

```toml
[dependencies]
ofdsdk = { version = "0.1.1", features = ["parts"] }
```

The current minimum supported Rust version is `1.88`.

## Quick Start

### 1. Parse and write OFD XML

```rust
use ofdsdk::schemas::ofd::Ofd;

fn main() -> Result<(), ofdsdk::common::SdkError> {
  let xml = r#"<ofd:OFD xmlns:ofd="http://www.ofdspec.org/2016" DocType="OFD" Version="1.0">
<ofd:DocBody>
<ofd:DocInfo>
<ofd:DocID>doc-id</ofd:DocID>
</ofd:DocInfo>
<ofd:DocRoot>Doc_0/Document.xml</ofd:DocRoot>
</ofd:DocBody>
</ofd:OFD>"#;

  let ofd: Ofd = xml.parse()?;
  let out = ofd.to_xml().unwrap();

  assert!(out.contains("Doc_0/Document.xml"));
  Ok(())
}
```

### 2. Read an OFD package

```rust
use ofdsdk::parts::ofd_package::OfdPackage;

fn main() -> Result<(), ofdsdk::common::SdkError> {
  let package = OfdPackage::new_from_file("example.ofd")?;

  assert_eq!(package.documents.len(), 1);

  let document = &package.documents[0];
  println!("pages = {}", document.pages.len());
  println!("public_res = {}", document.public_res.len());

  Ok(())
}
```

## API Overview

Common entry points:

- `ofdsdk::schemas::*`
- `ofdsdk::deserializers::*`
- `ofdsdk::serializers::*`
- `ofdsdk::parts::*` when `parts` is enabled

In short:

- `schemas` contains the generated data types
- `deserializers` contains XML input logic
- `serializers` contains XML output logic
- `parts` contains OFD package assembly and loading logic

## Design Notes

### 1. `schemas/` is the source of truth

The OFD models are derived from the XSD files under `schemas/`.

The generation flow is:

1. Read the XSD files.
2. Generate `sdk_data/schemas/*.json`.
3. Generate `crates/ofdsdk/src/schemas/`.
4. Generate the matching deserializers, serializers, and parts code.

### 2. Compatibility is data, not ad hoc runtime branching

Compatibility rules live in:

- `sdk_data/compatibility.json`

The goal is not to accept everything blindly. The goal is to record known real-world differences explicitly and absorb them during generation.

### 3. `parts` describes package assembly

The `parts` metadata lives in:

- `sdk_data/parts/*.json`

It describes:

- the root type of each part
- how paths are resolved
- how child parts are assembled recursively
- which parts depend on surrounding context

## Current Status

`0.1.1` is suitable for:

- OFD schema mapping
- XML round-trip testing
- validation against a set of real sample packages

It is still best treated as a low-level, engineering-focused SDK rather than a generic OFD reader that promises to handle everything.

## Development

Common commands:

```bash
cargo test -p ofdsdk-build test_gen -- --ignored --nocapture
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p ofdsdk-test
cargo test --workspace
```

If you modify any of the following:

- `schemas/*.xsd`
- `sdk_data/compatibility.json`
- `sdk_data/parts/*.json`
- `crates/ofdsdk-build`

regenerate the code and review the generated diff.

## License

This project is dual-licensed under:

- MIT
- Apache-2.0

You may use it under either license.
