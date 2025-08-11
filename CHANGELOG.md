# Changelog

All notable changes to this project will be documented in this file.

<!-- markdownlint-disable MD024 -->

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Standard Keep a Changelog section template to facilitate future releases. (commit d4d7d45)
- Version comparison links at the bottom of the file for quick navigation between tags. (commit d4d7d45)

### Changed

- Corrected the note about removal of `utils.rs`: the file is kept as an empty stub for temporary compatibility and will be removed in the next minor release. (commit d4d7d45)
- Progress bar documentation aligned with current logic. (PR #6)

### Fixed

- Fixed incorrect URLs in Cargo manifest. (PR #2)
- Clarified and expanded CLI progress bar documentation (enable/disable behavior). (PR #4)

## [0.2.0] - 2025-08-10

### Added

- **Comment Lines metric**: Added the "Comment Lines" metric alongside Total, Logical, and Empty Lines (REQ-1.1).
- **Unsupported files handling**: Files with unsupported syntax are now excluded from all statistics (REQ-3.5.1, REQ-3.5.2) and are listed separately in output and reports (REQ-3.5.3).
- **Percent column**: Added a percent (%) column in global and language summary tables, with % symbol and calculation relative to the total (REQ-5.1, REQ-5.2).
- **Numeric alignment**: All numeric values in tables are now right-aligned and formatted with thousands separators (REQ-5.3).
- **CLI flag --details**: Added the --details flag to show per-file statistics and unsupported file list only if requested (REQ-8.3).
- **Human readable performance**: Performance (throughput) is now shown in human readable format using the external crate `human_format` (REQ-9.7).
- **Requirements traceability**: Improved requirements traceability with REQ-xx references in code comments and documentation.

### Changed

- Updated documentation (README.md, REQUIREMENTS.md, METRICS_USAGE.md) to reflect new metrics, unsupported file handling, new CLI flag, and traceability.
- Refactored output code to support alignment and advanced formatting.
- Replaced custom number formatting function with the `human_format` crate.

### Fixed

- Fixed minor bugs in percent handling and exclusion of unsupported files from statistics.

### Removed

- Removed the unused `utils.rs` module.

## [0.1.0] - 2024-08-09

### Added

- Initial release of rustedbytes-counterlines
- Multi-language source code line counting with support for 15+ programming languages
- Three types of line counting: total, logical, and empty lines
- Multiple output formats: JSON, XML, CSV
- Report processing and comparison without rescanning files
- Parallel processing for large codebases using Rayon
- Progress indicators for long operations
- Cross-platform support (Linux, macOS, Windows)
- UTF-8 and UTF-16 encoding support
- Extensible language definitions via TOML configuration files
- Language override support for custom file extensions
- Nested comment support for languages like Rust and Scala
- Mixed line detection (lines containing both code and comments)
- Preprocessor directive handling
- SHA256 checksums for report integrity
- Performance metrics logging
- Colored console output with formatted tables
- Deterministic output for consistent results

### Languages Supported

- Rust (with nested comments)
- C/C++ (with preprocessor directives)
- Python (including docstrings)
- JavaScript/TypeScript
- Java
- Go
- Ruby
- Shell scripts (bash, zsh, sh)
- SQL
- HTML
- CSS/SCSS/SASS
- YAML
- TOML
- And more via configuration files

### CLI Commands

- `sloc count` - Count lines in files/directories
- `sloc report` - Generate reports in various formats
- `sloc process` - Process existing reports
- `sloc compare` - Compare two reports

### Technical Features

- REQ-1.x: Complete line counting functionality
- REQ-2.x: Flexible input methods (files, directories, wildcards, stdin)
- REQ-3.x: Extensible multi-language support with custom configurations
- REQ-4.x: Accurate line classification including nested comments
- REQ-5.x: Formatted, sortable console output with colors and tables
- REQ-6.x: Multiple export formats with timestamps and checksums
- REQ-7.x: Report comparison and analysis capabilities
- REQ-8.x: Intuitive command-line interface
- REQ-9.x: Performance optimization and cross-platform compatibility

<!-- Version comparison links -->
[Unreleased]: https://github.com/daniele-olmisani/rustedbytes-counterlines/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/daniele-olmisani/rustedbytes-counterlines/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/daniele-olmisani/rustedbytes-counterlines/releases/tag/v0.1.0
