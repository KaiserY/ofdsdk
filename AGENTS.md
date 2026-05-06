# Repository Guidelines

## Agent Harness
Start from local evidence. Use `rg`/`rg --files` first, read only the files needed for the task, and keep summaries diff-based rather than conversation-based. Do not paste large generated snippets or broad search output back into the conversation unless the user asks.

Run commands from the repository root. Cargo generation, format, test, clippy, and bench commands must run sequentially in the default `target/` directory; do not set `CARGO_TARGET_DIR`. If Cargo reports a target lock, wait for Cargo rather than probing processes.

This Rust workspace has three crates:

- `crates/ofdsdk`: runtime crate and public API. Its public entry point is `src/lib.rs`. Generated schema/deserializer/serializer/parts output lives under `src/schemas/`, `src/deserializers/`, `src/serializers/`, `src/parts/`, plus `src/schemas.rs`, `src/deserializers.rs`, `src/serializers.rs`, and `src/parts.rs`. Shared runtime helpers live in `src/common.rs` and `src/common/`.
- `crates/ofdsdk-build`: generator and checked-in input model code. It reads XML Schema files from `schemas/`, emits intermediate data into `sdk_data/`, and generates Rust source into `crates/ofdsdk/src/`.
- `crates/ofdsdk-test`: integration tests, OFD package samples, fixtures, and performance benches. It depends only on `crates/ofdsdk`.

The runtime feature is `parts`; it enables ZIP/package support. There is no default feature declaration, so use explicit feature checks only when the code or test actually depends on package behavior.

## Generated Data
Treat `schemas/` as the source of truth for schema modeling. Changes to XSD files usually require regeneration through the build crate tests. Only implement or maintain XSD syntax that actually appears under `schemas/`. If a construct does not occur in `schemas/`, do not add handling for it beyond what is required to fail fast or stay out of the active path.

Treat checked-in `sdk_data/schemas/*.json` and `sdk_data/schema_comments/*.json` as generated intermediate schema data. Treat `sdk_data/parts/*.json` and `sdk_data/compatibility.json` as hand-maintained generator inputs; do not regenerate them from Rust code.

Avoid editing generated runtime files directly unless also changing generator/input data and intentionally regenerating output. Generated runtime files include `crates/ofdsdk/src/schemas/`, `crates/ofdsdk/src/deserializers/`, `crates/ofdsdk/src/serializers/`, `crates/ofdsdk/src/parts/`, `crates/ofdsdk/src/schemas.rs`, `crates/ofdsdk/src/deserializers.rs`, `crates/ofdsdk/src/serializers.rs`, and `crates/ofdsdk/src/parts.rs`. After `test_gen`, run `cargo fmt --all` before reviewing diffs; do not revert generated churn before checking the formatted diff.

Keep compatibility behavior in `sdk_data/compatibility.json` and the build crate. Prefer targeted compatibility overrides over weakening XSD-derived models globally.

## Commands
- `cargo build --workspace`: build all crates.
- `cargo test -p ofdsdk-build test_gen -- --ignored --nocapture`: regenerate `sdk_data/` and runtime generated code from `schemas/` and hand-maintained generator inputs.
- `cargo test -p ofdsdk-test`: fast integration lane for runtime XML round trips and OFD package samples.
- `cargo test --workspace`: default full test lane.
- `cargo fmt --all`: format.
- `cargo clippy --workspace --all-targets -- -D warnings`: clippy lane.
- `cargo bench -p ofdsdk-test --bench page`: page read, write, and round-trip benchmarks.
- `cargo bench -p ofdsdk-test --bench document`: document read, write, and round-trip benchmarks for `Document` parsing hot paths such as `VPreferences`.

Validation loop:

- `cargo test -p ofdsdk-build test_gen -- --ignored --nocapture`
- `cargo fmt --all`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test -p ofdsdk-test`
- `cargo test --workspace`
- `cargo fmt --all`

Benchmarks are separate and should be run only when checking performance-sensitive changes:

- `cargo bench -p ofdsdk-test --bench page`
- `cargo bench -p ofdsdk-test --bench document`

For runtime XML iteration, start with the targeted file under `crates/ofdsdk-test/tests/`, then run `cargo test -p ofdsdk-test`. For package/parts behavior, run `cargo test -p ofdsdk-test parts -- --nocapture` and review OFD samples under `crates/ofdsdk-test/samples/`. Add broader lanes when the change touches generator code, shared runtime behavior, package behavior, compatibility data, or generated output.

## Testing Rules
Place tests near the behavior they protect:

- Schema/simple XML round trips: the matching file under `crates/ofdsdk-test/tests/`, such as `page.rs`, `document.rs`, `res.rs`, `ofd.rs`, or `simple` schema-specific tests already present.
- Graphics behavior: `crates/ofdsdk-test/tests/page_graphics.rs`.
- Package/parts behavior: `crates/ofdsdk-test/tests/parts.rs`, using public APIs and saved package contents.
- Signature, attachment, annotation, custom tag, extension, and version behavior: the matching schema or package test file under `crates/ofdsdk-test/tests/`.
- Performance-sensitive page parsing or serialization: `cargo bench -p ofdsdk-test --bench page`.
- Performance-sensitive document parsing or serialization: `cargo bench -p ofdsdk-test --bench document`.

Testing is centered on the generator path in `crates/ofdsdk-build/src/lib.rs`. The code generation test `test_gen` is ignored by default, so run it first when schema or generator logic changes. Since generated code is committed, treat `test_gen` as the first verification step and run `cargo fmt --all` before inspecting generated diffs. Add unit tests close to the code they exercise using `#[cfg(test)] mod tests`.

When changing schema parsing or code generation, review the generated diff in `sdk_data/schemas/`, `sdk_data/schema_comments/`, `crates/ofdsdk/src/schemas/`, `crates/ofdsdk/src/deserializers/`, `crates/ofdsdk/src/serializers/`, and `crates/ofdsdk/src/parts/`.

When changing `sdk_data/parts/*.json`, `sdk_data/compatibility.json`, or the parts generator, verify the sample-based OFD package tests in `crates/ofdsdk-test/tests/parts.rs` and review the generated diff in `crates/ofdsdk/src/parts/`.

## Code Style
Follow `.editorconfig`: UTF-8, LF line endings, final newline, and spaces with 2-space indentation by default. Keep Rust `rustfmt`-clean. Use snake_case for modules and functions, PascalCase for Rust types, and mirror schema-derived filenames in lowercase with underscores, for example `custom_tags.rs`.

Prefer small, focused modules. Do not write code comments. Most runtime code is generated by the build crate, so prioritize generator-friendly structure over ad hoc hand-written patterns. Keep hand-written logic in `crates/ofdsdk-build` or small generic runtime helpers. Avoid editing generated runtime files unless you are also updating the generator or generator inputs.

For the `parts` API, keep the public surface aligned with OFD package concepts and established constructors/read/write entry points already present in `crates/ofdsdk/src/parts/`. Do not expose raw storage, relationship sets, generated factories, or internal caches just to satisfy a narrow test.

When benchmarking, evaluate the relevant bench as a whole and compare surrounding trends rather than relying on a single noisy metric.

## Commit Guidelines
Recent history uses short subjects like `WIP: 0.1.0`. For new work, keep commit subjects short, imperative, and scoped, for example `Generate annotation schema modules` or `Tighten OFD package path decoding`.

When generating a commit message, base it on repository state, not the latest chat turn. Inspect `git status --short`, `git diff --stat`, and relevant `git diff` hunks. If changes are staged, also inspect `git diff --cached --stat` and `git diff --cached`. Do not prefix the copyable commit message with explanatory text such as whether it covers staged or unstaged changes; if that context is useful, place it after the commit message under a separate non-copyable note.

Summarize the coherent change set visible in the diff. Distinguish documentation-only edits from code, metadata, generated output, fixtures, and tests. Mention generated churn only when intended. Do not fold unrelated worktree edits into the message unless the user asks for a message covering all changes.

Generate a commit message when needed, but do not create a commit unless the user explicitly confirms.

In pull requests, include:

- a brief summary of the functional change,
- confirmation that you ran the validation loop and any relevant standalone benchmarks,
- notes on regenerated files when schema or generator logic changed.
