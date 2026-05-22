# Publishing m5unified-rs crates

The workspace contains two publishable crates:

1. `m5unified-sys` - raw unsafe C ABI bindings and native shim files.
2. `m5unified` - safe Rust wrapper that depends on `m5unified-sys`.

Publish order matters. `m5unified` cannot be packaged against crates.io until the exact `m5unified-sys` version exists in the crates.io index.

## Preflight

Check the package metadata and docs before publishing:

```bash
cargo metadata --no-deps --format-version 1
cargo doc --workspace --no-deps
```

Run the full host verification suite:

```bash
python3 tools/check_examples_manifest.py
python3 tools/check_no_sys_in_examples.py
bash scripts/check-host.sh
```

Package the sys crate and inspect both crate payloads:

```bash
cargo package -p m5unified-sys
cargo package -p m5unified-sys --list
cargo package -p m5unified --list
```

Run the sys crate dry-run before the first upload:

```bash
cargo publish -p m5unified-sys --dry-run
```

`cargo package -p m5unified` and `cargo publish -p m5unified --dry-run` can fail before `m5unified-sys` is actually published because crates.io cannot resolve the not-yet-published dependency. That is expected for a first release of both crates. Use `cargo package -p m5unified --list` before the sys upload only to inspect the eventual payload.

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

## First-release notes

- Confirm both crate names are still available before uploading.
- Do not publish from unreviewed local changes unless the final package
  contents have been inspected with `cargo package --list`.
- Crates.io releases are immutable; fix any README, license, repository, and
  docs.rs metadata before the final `cargo publish`.
