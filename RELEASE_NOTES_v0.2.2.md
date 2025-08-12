# rustedbytes-counterlines v0.2.2 (2025-08-12)

## Summary

Maintenance + quality release adding automatic output file naming parity between `count` and `report`, dependency refresh, and several fixes/optimizations.

## Added

- Auto-generated output filename when `--format/-f` is specified without `--output/-o` for `count`; now also applied consistently to `report`.
- Config key `defaults.output_file` to customize base auto-generated filename (default: `sloc-report`).

## Changed

- Dependencies bumped to latest compatible versions.
- Documentation links unified to canonical `mad4j/rustedbytes-counterlines` repository.

## Fixed

- Removed outdated documentation badge in README.
- XML serialization now works after upgrading `serde-xml-rs` to 0.8.1 (PR #8).
- Added optimized Cargo profiles & CPU-specific flags (PR #10).
- Corrected multi-line comment counting discrepancies (PR #12).

## Upgrade Notes

No breaking changes. Existing CLI usage remains valid. To leverage auto-naming just omit `-o/--output` while providing `-f/--format`.

## Links

- Changelog entry: see `CHANGELOG.md` under 0.2.2.
- Diff: <https://github.com/mad4j/rustedbytes-counterlines/compare/v0.2.1...v0.2.2>

## Verification Checklist

- [x] Version bumped in `Cargo.toml`
- [x] Changelog updated & comparison links fixed
- [x] README repository URL corrected
- [x] Build (debug + release) succeeds
- [ ] Crates.io publish (pending token) – run `cargo publish` once token is configured

---
Signed-off-by: Release Automation (GitHub Copilot)
