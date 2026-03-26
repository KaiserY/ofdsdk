# Change Log

## 0.1.1

### 新增

- 补充更多 XML 级回归测试，覆盖 `Signature`、`CustomTags`、`Extensions`、`Versions`、`DocVersion`、`VPreferences`、`Page` 图元与颜色结构
- 增加英文 `README.md`，并新增中文 `README_cn.md`
- 更新发布文本到 `0.1.1`

### 性能

- 优化 XML 反序列化路径，降低 reader 侧重复处理开销

### 说明

- 这次发布继续强化 XML round-trip 和兼容性回归覆盖
- `README.md` 现在是英文版，中文说明保留在 `README_cn.md`

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
