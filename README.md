# ofdsdk

[![crates.io](https://img.shields.io/crates/v/ofdsdk.svg)](https://crates.io/crates/ofdsdk)
[![docs](https://docs.rs/ofdsdk/badge.svg)](https://docs.rs/ofdsdk)

`ofdsdk` 是一个面向 Rust 的 OFD SDK。

项目目标：

- 基于 OFD XSD 生成强类型 Rust 结构
- 提供稳定的 XML 反序列化与序列化能力
- 在启用 `parts` 特性后，提供 OFD 包级别的读取与保存能力
- 将现实世界中的兼容性差异收敛到生成数据，而不是在运行时散落特判

当前版本为 `0.1.0`。

## 当前能力

已提供：

- 基于 `schemas/` 生成的 Rust schema 类型
- XML 字符串与 Reader 的反序列化
- XML 序列化输出
- `parts` 特性下的 OFD 压缩包读取与保存
- 基于样例文件的 OFD 包级测试覆盖

当前仓库包含三个 crate：

- `crates/ofdsdk`：运行时库，对外使用
- `crates/ofdsdk-build`：代码生成器
- `crates/ofdsdk-test`：集成测试与样例验证

## 安装

如果只需要 XML schema 类型：

```toml
[dependencies]
ofdsdk = "0.1.0"
```

如果还需要读取 `.ofd` 压缩包：

```toml
[dependencies]
ofdsdk = { version = "0.1.0", features = ["parts"] }
```

当前最低 Rust 版本为 `1.88`。

## 快速开始

### 1. 解析和写回 OFD XML

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

### 2. 读取 OFD 包

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

## API 概览

常用入口：

- `ofdsdk::schemas::*`
- `ofdsdk::deserializers::*`
- `ofdsdk::serializers::*`
- `ofdsdk::parts::*`（需要启用 `parts`）

其中：

- `schemas` 提供生成出的结构定义
- `deserializers` 提供 XML 读入逻辑
- `serializers` 提供 XML 输出逻辑
- `parts` 提供 OFD 包内部部件的装配读取

## 设计说明

### 1. `schemas/` 是真源

仓库中的 `schemas/*.xsd` 是模型来源。

生成流程大致为：

1. 读取 XSD
2. 生成 `sdk_data/schemas/*.json`
3. 生成 `crates/ofdsdk/src/schemas/`
4. 生成对应的反序列化、序列化和 parts 代码

### 2. compatibility 不是运行时特判集合

兼容策略集中放在：

- `sdk_data/compatibility.json`

它的目标不是无限制放宽输入，而是将已确认的现实差异显式记录，并在生成阶段统一吸收。

### 3. `parts` 是包装配模型

`parts` 相关定义位于：

- `sdk_data/parts/*.json`

它描述的是：

- 一个 part 的根类型是什么
- 路径如何取得
- 子 part 如何递归装配
- 哪些 part 之间存在上下文关联

## 当前状态

`0.1.0` 已经可以用于：

- OFD schema 结构映射
- XML round-trip
- 一批真实样例的 OFD 包读取验证

但仍然建议把它看作一个偏底层、偏工程化的 SDK，而不是“拿来即读一切 OFD”的通用阅读器。

## 开发

常用命令：

```bash
cargo test -p ofdsdk-build test_gen -- --ignored --nocapture
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p ofdsdk-test
cargo test --workspace
```

如果修改了：

- `schemas/*.xsd`
- `sdk_data/compatibility.json`
- `sdk_data/parts/*.json`
- `crates/ofdsdk-build`

建议重新执行生成测试并检查生成结果。

## 许可证

本项目使用双许可证：

- MIT
- Apache-2.0

你可以任选其一使用。
