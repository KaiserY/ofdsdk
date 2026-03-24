# Change Log

## 0.1.0

首个公开版本。

### 新增

- 基于 OFD XSD 生成的 Rust 强类型 schema
- XML 反序列化与序列化能力
- `parts` 特性下的 OFD 包读取与保存能力
- 生成器 crate `ofdsdk-build`
- 样例驱动的集成测试与 benchmark

### 说明

- 当前版本定位为强类型 OFD SDK
- 兼容策略以生成数据和显式规则为主，不以运行时宽容读取为目标
- `crates.io` 仅发布 `ofdsdk` crate，`ofdsdk-build` 和 `ofdsdk-test` 不对外发布
