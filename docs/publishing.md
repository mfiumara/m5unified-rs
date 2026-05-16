# Publishing m5unified-rs crates

The workspace contains two publishable crates:

1. `m5unified-sys` — raw unsafe C ABI bindings and native shim files.
2. `m5unified` — safe Rust wrapper that depends on `m5unified-sys`.

Publish order matters. `m5unified` cannot be packaged against crates.io until the exact `m5unified-sys` version exists in the crates.io index.

## Preflight

```bash
python3 tools/check_examples_manifest.py
python3 tools/check_no_sys_in_examples.py
cargo test --workspace
cargo package -p m5unified-sys --allow-dirty
cargo publish -p m5unified-sys --dry-run --allow-dirty
```

`cargo package -p m5unified` / `cargo publish -p m5unified --dry-run` will fail before `m5unified-sys` is actually published because crates.io cannot resolve the not-yet-published dependency. This is expected.

## Publish

```bash
cargo publish -p m5unified-sys
# Wait for crates.io index propagation, usually 1-5 minutes.
cargo publish -p m5unified --dry-run
cargo publish -p m5unified
```

## Versioning rule

When bumping versions, bump both crates together unless only `m5unified-sys` changed and the high-level crate does not need the new sys API.

If `m5unified` uses a new raw symbol, publish `m5unified-sys` first and update the workspace dependency version before publishing `m5unified`.
