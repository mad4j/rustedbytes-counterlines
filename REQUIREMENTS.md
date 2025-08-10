
# CLI Source Code Line Counter — Requirement Specification

This document defines the requirements for a command-line interface (CLI) tool dedicated to counting source code lines in software projects.  
The terms **SHALL**, **SHOULD**, and **MAY** are to be interpreted per [RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119).

---

## 1. Purpose

- **REQ-1.1** — The tool **SHALL** count and report the following metrics:  
  - **Total Lines**: all lines in a file, including comments, code, and blank lines.  
  - **Logical Lines**: lines containing actual code statements, excluding comments, comment-only lines, and blank lines.  
  - **Comment Lines**: lines containing only comments (excluding code and blank lines).  
  - **Empty Lines**: lines containing only whitespace.

- **REQ-1.2** — The tool **SHALL** support multiple programming languages.

- **REQ-1.3** — The tool **SHALL** provide output to the console.

- **REQ-1.4** — The tool **SHALL** support output in structured report formats.

- **REQ-1.5** — The tool **SHALL** enable post-processing of reports.

- **REQ-1.6** — The tool **SHALL** enable comparison between two reports.

---

## 2. Input Handling

- **REQ-2.1** — The tool **SHALL** accept one or more file and/or directory paths as input.

- **REQ-2.2** — The tool **SHALL** accept file paths with wildcards (e.g., `examples/*.rs`, `*.py`).

- **REQ-2.3** — The tool **SHALL** recursively traverse directories when explicitly instructed.

- **REQ-2.4** — The tool **SHOULD** accept input lists via standard input (stdin).

- **REQ-2.5** — The tool **SHOULD** provide error messages for invalid or inaccessible paths.

---

## 3. Language Support

- **REQ-3.1** — The tool **SHALL** support multiple programming languages, each with distinct comment syntaxes.

- **REQ-3.2** — The tool **SHALL** perform language detection based on file extensions by default.

- **REQ-3.3** — The tool **SHOULD** allow new language definitions to be added without modifying the core logic (e.g., via configuration files or plugins).

- **REQ-3.4** — The tool **MAY** allow users to override detected language for specific files.

- **REQ-3.5** — The tool **SHALL** exclude from statistics any files whose syntax is not supported (i.e., for which no language definition is available). These files SHALL NOT be included in line counts or language summaries, but SHALL be listed separately in the output and reports.

---

## 4. Counting Rules

- **REQ-4.1** — The tool **SHALL** count total, logical, and empty lines for each processed file.

- **REQ-4.2** — The tool **SHALL** correctly identify single-line and multi-line comments according to the language syntax.

- **REQ-4.3** — If the language supports nested comments, the tool **SHALL** handle them correctly.

- **REQ-4.4** — The tool **SHOULD** correctly handle lines that contain both code and comments, counting them as logical lines.

- **REQ-4.5** — The tool **MAY** ignore language-specific preprocessor directives if configured to do so.

---

## 5. Console Output

- **REQ-5.1** — The tool **SHALL** present a summary table showing total counts across all files.

- **REQ-5.2** — The tool **SHALL** present a summary table showing counts per language.

- **REQ-5.3** — The summaries **SHOULD** be human-readable, properly aligned, and formatted with thousands separators and up to two decimal places if needed.

- **REQ-5.4** — The tool **MAY** allow users to sort console output by metric (e.g., total lines, logical lines).

---

## 6. Report Generation

- **REQ-6.1** — The tool **SHALL** support exporting reports in **JSON** format.

- **REQ-6.2** — The tool **SHALL** support exporting reports in **XML** format.

- **REQ-6.3** — The tool **SHALL** support exporting reports in **CSV** format.

- **REQ-6.4** — Each report **SHALL** include per-file statistics: file path, detected language, total lines, logical lines, empty lines.

- **REQ-6.5** — Each report **SHALL** include a `generatedAt` field with the timestamp in RFC 3339 / ISO 8601 format.

- **REQ-6.6** — Each report **SHALL** include a `reportFormatVersion` field following the `major.minor` pattern (e.g., `"1.0"`).

- **REQ-6.7** — Report schemas **SHALL** remain consistent across all export formats.

- **REQ-6.8** — Users **SHOULD** be able to customize output paths for report files.

- **REQ-6.9** — The tool **MAY** include an optional checksum or signature to ensure report integrity.

---

## 7. Report Processing

- **REQ-7.1** — The tool **SHALL** compute global statistics from an existing report without rescanning source files.

- **REQ-7.2** — The tool **SHALL** allow comparison between two reports, producing:  
  - Per-language deltas.  
  - Global statistics deltas.

- **REQ-7.3** — The comparison results **SHALL** be available in console output.

- **REQ-7.4** — The tool **MAY** support exporting comparison results in the same formats as normal reports.

---

## 8. CLI Interface

- **REQ-8.1** — The tool **SHALL** provide a command-line interface (CLI).

- **REQ-8.2** — The CLI **SHALL** display help and usage information via `--help` or `-h`.

- **REQ-8.3** — The CLI **SHALL** support the following commands:  
  - `count` — Count lines in specified files/directories.  
  - `report` — Generate a report from counted lines.  
  - `process` — Process an existing report.  
  - `compare` — Compare two reports.

- **REQ-8.4** — The CLI **MAY** provide command aliases for faster usage.

---

## 9. Non-Functional Requirements

- **REQ-9.1** — The tool **SHALL** run on Linux, macOS, and Windows.

- **REQ-9.2** — The tool **SHALL** support UTF-8 (with or without BOM) and UTF-16 file encodings.

- **REQ-9.3** — The tool **SHALL** produce deterministic output for identical inputs.

- **REQ-9.4** — The tool **SHOULD** efficiently process large codebases (tens of thousands of files), possibly using parallel processing.

- **REQ-9.5** — The tool **SHOULD** display progress indicators for long-running operations (e.g., a progress bar).

- **REQ-9.6** — The tool **SHOULD** optimize memory usage when processing large files.

- **REQ-9.7** — The tool **MAY** log performance metrics for diagnostic purposes.

---
