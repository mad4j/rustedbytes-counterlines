# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
