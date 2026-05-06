# Change Log

## 0.2.0

### 新增

- 增加 OFD 样例包 round-trip 测试，覆盖读取、内存写回、再次读取与 ZIP entry 对比
- 为生成的 schema 类型保留未建模 XML 属性和未建模 XML 子节点
- 为 `parts` 包级读写保留未建模 ZIP entry，减少保存后丢失原包内容的风险
- 增加兼容性规则，支持将已知现实样例中缺失的属性或子节点生成成 `Option`

### 兼容性

- `Annot.Creator`、`Annot.LastModDate`、`CustomTag.NameSpace` 现在按可选属性处理
- `Annot.Appearance` 现在按可选子节点处理，用于兼容缺失批注外观定义的样例
- XML round-trip 对比放宽了部分数值 lexical 表达差异，例如 `3.0` 与 `3`
- 对已知现实样例中与 XSD canonical 顺序不一致的子节点顺序进行测试侧兼容

### 说明

- 本版本重点提升真实 OFD 样例的读写保真度
- 未建模 XML 内容会尽量保留并按原 slot 写回，但 schema 已建模 child 的 canonical 写回顺序仍以生成模型为准
- 未建模 ZIP entry 会随包级保存写回，root-level preservation 的 API 位置后续仍可能继续整理

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
