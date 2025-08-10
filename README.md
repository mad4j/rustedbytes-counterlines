# SLOC Counter - Source Lines of Code CLI Tool

[![Crates.io](https://img.shields.io/crates/v/rustedbytes-counterlines.svg)](https://crates.io/crates/rustedbytes-counterlines)
[![Documentation](https://docs.rs/rustedbytes-counterlines/badge.svg)](https://docs.rs/rustedbytes-counterlines)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance, multi-language source code line counter written in Rust that fully implements the requirements specified in REQUIREMENTS.md.

- **Multi-language support** with extensible language definitions (REQ-3.1, REQ-3.3)
- **Excludes unsupported files** from statistics and lists them separately (REQ-3.5)
- **Accurate line counting** differentiating between total, logical, and empty lines (REQ-1.1)
- **Multiple output formats**: JSON, XML, CSV (REQ-6.1, REQ-6.2, REQ-6.3)
- **Report processing and comparison** without rescanning files (REQ-7.1, REQ-7.2)
- **Parallel processing** for large codebases (REQ-9.4)
- **Progress indicators** for long operations (REQ-9.5)
- **Cross-platform** support for Linux, macOS, and Windows (REQ-9.1)
- **UTF-8 and UTF-16** encoding support (REQ-9.2)

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/sloc-counter.git
cd sloc-counter

# Build the project
cargo build --release

# The binary will be available at target/release/sloc
```

## Usage

### Basic Commands

#### Count Lines (REQ-8.3)

```bash
# Count lines in specific files
sloc count src/main.rs src/lib.rs

# Count with wildcards (REQ-2.2)
sloc count "src/*.rs" "tests/*.rs"

# Recursive directory traversal (REQ-2.3)
sloc count src/ -r

# Read file list from stdin (REQ-2.4)
find . -name "*.rs" | sloc count --stdin
```

#### Generate Reports (REQ-8.3)

```bash
# Generate JSON report (REQ-6.1)
sloc report src/ -r -f json -o report.json

# Generate XML report (REQ-6.2)
sloc report src/ -r -f xml -o report.xml

# Generate CSV report (REQ-6.3)
sloc report src/ -r -f csv -o report.csv

# Include checksum (REQ-6.9)
sloc report src/ -r -f json -o report.json --checksum
```

#### Process Existing Reports (REQ-7.1)

```bash
# Process and display statistics from a report
sloc process report.json

# Sort by different metrics (REQ-5.4)
sloc process report.json --sort logical

# Export processed results
sloc process report.json --export summary.json -f json
```

#### Compare Reports (REQ-7.2)

```bash
# Compare two reports
sloc compare report_old.json report_new.json

# Export comparison (REQ-7.4)
sloc compare report_old.json report_new.json --export comparison.json -f json
```

### Advanced Options

#### Language Configuration

```bash
# Use custom language definitions (REQ-3.3)
sloc count src/ -r --config languages.toml

# Override language detection (REQ-3.4)
sloc count src/ --language-override "txt=python" --language-override "conf=yaml"

# Ignore preprocessor directives (REQ-4.5)
sloc count src/ --ignore-preprocessor
```

#### Performance Options

```bash
# Set parallel threads (REQ-9.4)
sloc count src/ -r -j 8

# Show progress bar (REQ-9.5)
sloc count src/ -r --progress
```

## Supported Languages & Unsupported Files

Built-in support for (REQ-3.1):

Files for which no language definition is available are **excluded from all statistics and summaries**. These files are **listed separately** in the console output and reports, so you can review which files were not counted.

- Rust (including nested comments)
- C/C++ (with preprocessor directives)
- Python (including docstrings)
- JavaScript/TypeScript
- Java
- Go
- Ruby
- Shell scripts
- SQL
- HTML
- CSS/SCSS/SASS
- YAML
- TOML
- And more...

## Line Counting Rules

The tool counts four types of lines (REQ-1.1):

1. **Total Lines**: All lines in a file
2. **Logical Lines**: Lines containing actual code (excluding comments, comment-only lines, and blanks)
3. **Comment Lines**: Lines containing only comments (excluding code and blank lines)
4. **Empty Lines**: Lines with only whitespace

### Special Features

- **Nested Comments** (REQ-4.3): Properly handles nested comments in languages like Rust, Scala, and Haskell
- **Mixed Lines** (REQ-4.4): Correctly identifies lines containing both code and comments
- **Preprocessor Directives** (REQ-4.5): Option to ignore or count preprocessor directives

## Report Format

Reports include (REQ-6.4, REQ-6.5, REQ-6.6):

- Per-file statistics (path, language, line counts: total, logical, comment, empty)
- Language summaries
- Global statistics
- Generation timestamp (RFC 3339/ISO 8601)
- Report format version
- Optional SHA256 checksum (REQ-6.9)

### Example JSON Report Structure

```json
{
  "reportFormatVersion": "1.0",
  "generatedAt": "2024-01-15T10:30:00Z",
  "files": [
    {
      "path": "src/main.rs",
      "language": "Rust",
      "totalLines": 150,
      "logicalLines": 120,
      "commentLines": 20,
      "emptyLines": 10
    }
  ],
  "languages": [
    {
      "language": "Rust",
      "fileCount": 10,
      "totalLines": 1500,
      "logicalLines": 1200,
      "commentLines": 200,
      "emptyLines": 100
    }
  ],
  "summary": {
    "totalFiles": 10,
    "totalLines": 1500,
    "logicalLines": 1200,
    "commentLines": 200,
    "emptyLines": 100,
    "languagesCount": 1
  },
  "checksum": "sha256_hash_here"
}
```

## Custom Language Configuration

Create a `languages.toml` file to add or modify language definitions (REQ-3.3):

```toml
[languages.mylang]
name = "MyLanguage"
extensions = ["ml", "mli"]
single_line_comment = ["//", "#"]
multi_line_comment = [
    { start = "/*", end = "*/" },
    { start = "(*", end = "*)" }
]
nested_comments = true
preprocessor_prefix = "#"
```

## Performance

- **Parallel Processing** (REQ-9.4): Utilizes multiple CPU cores via Rayon
- **Memory Efficient** (REQ-9.6): Streams files instead of loading them entirely
- **Progress Indicators** (REQ-9.5): Optional progress bars for large operations
- **Deterministic Output** (REQ-9.3): Consistent results for identical inputs

## Console Output

The tool provides formatted console output with (REQ-5.1, REQ-5.2, REQ-5.3):

- Color-coded tables
- Thousands separators
- Percentage calculations
- Sortable columns

### Example Output

```text
════════════════════════════════════════════════════════════════════════════════
Source Lines of Code (SLOC) Report
════════════════════════════════════════════════════════════════════════════════

Global Summary
────────────────────────────────────
┌──────────────┬──────────┐
│ Metric       │ Value    │
├──────────────┼──────────┤
│ Total Files  │ 25       │
│ Total Lines  │ 10,543   │
│ Logical Lines│ 8,234    │
│ Empty Lines  │ 2,309    │
│ Languages    │ 3        │
│ Code Density │ 78.10%   │
│ Empty Ratio  │ 21.90%   │
└──────────────┴──────────┘

Language Summary
────────────────────────────────────────────────────────────────────────────────
┌────────────┬───────┬─────────┬──────────┬───────┬───────────┐
│ Language   │ Files │ Total   │ Logical  │ Empty │ Density % │
├────────────┼───────┼─────────┼──────────┼───────┼───────────┤
│ Rust       │ 15    │ 7,234   │ 5,890    │ 1,344 │ 81.42     │
│ TOML       │ 5     │ 1,543   │ 1,234    │ 309   │ 79.97     │
│ Markdown   │ 5     │ 1,766   │ 1,110    │ 656   │ 62.85     │
└────────────┴───────┴─────────┴──────────┴───────┴───────────┘
```

## Error Handling

The tool provides clear error messages for (REQ-2.5):

- Invalid or inaccessible paths
- Unsupported file formats (listed separately, not counted; see REQ-3.5)
- Permission issues
- Encoding problems

## Contributing

Contributions are welcome! Please ensure that:

1. All existing tests pass
2. New features include tests
3. Code follows Rust conventions
4. Documentation is updated

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Requirements Compliance

This implementation fully complies with all requirements specified in REQUIREMENTS.md:

- ✅ Purpose (REQ-1.x): Complete line counting functionality, including comment lines
- ✅ Input Handling (REQ-2.x): Flexible input methods
- ✅ Language Support (REQ-3.x, REQ-3.5): Extensible multi-language support, unsupported files excluded from statistics and listed separately
- ✅ Counting Rules (REQ-4.x): Accurate line and comment classification
- ✅ Console Output (REQ-5.x): Formatted, sortable output
- ✅ Report Generation (REQ-6.x): Multiple export formats
- ✅ Report Processing (REQ-7.x): Compare and analyze reports
- ✅ CLI Interface (REQ-8.x): Intuitive command structure
- ✅ Non-Functional (REQ-9.x): Performance and compatibility
