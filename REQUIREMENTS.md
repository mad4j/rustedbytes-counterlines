# CLI Source Code Line Counter — Specification

This document defines the requirements for a command-line interface (CLI) tool dedicated to counting source code lines in software projects.
The terms **SHALL**, **SHOULD**, and **MAY** are to be interpreted per [RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119).

---

## 1. Purpose

The tool **SHALL** count and report the following metrics:

- **Total Lines**: all lines in a file, including comments, code, and blank lines.
- **Logical Lines**: lines containing actual code statments, excluding comments and blank lines.
- **Empty Lines**: lines containing only whitespaces or comments containing only whitespaces.

The tool **SHALL** support multiple programming languages.
The tool **SHALL** provide console output.
The tool **SHALL** provide report-based output.
The tool **SHALL** enable report processing.
The tool **SHALL** enable report comparison.

---

## 2. Input Handling

- The tool **SHALL** accept one or more file and/or directory paths as input.
- The tool **SHALL** accept file paths with wildcards (e.g., `*.rs`, `*.py`).
- The tool **SHALL** recursively traverse directories if instructed to do.
- The tool **SHOULD** accept input lists via standard input (stdin).

---

## 3. Language Support

- The tool **SHALL** support multiple languages with distinct comment syntaxes.  
- The tool **SHALL** perform language detection based on file extension by default.
- The tool **SHOULD** allow new languages to be added without modifying the core logic.

---

## 4. Counting Rules

- The tool **SHALL** count total, logical, and empty lines.  
- The tool **SHALL** correctly identify single-line and multi-line comments per language.  
- If the language permits nested comments, the tool **SHALL** handle them correctly.  
- The tool **SHOULD** handle edge cases such as inline code mixed with comments, in this case the line counts as logical line.

---

## 5. Console Output

- The tool **SHALL** present a summary in a tabular format showing total counts.
- The tool **SHALL** present a summary in a tabular format showing per-language.  
- The summaries **SHOULD** be human-readable and aligned for clarity (e.g. including thosand separator and supporting two decimal digits if a fractional part is needed).

---

## 6. Report Generation

- The tool **SHALL** support report export in **JSON** format.
- The tool **SHALL** support report export in **XML** format.
- The tool **SHALL** support report export in **CSV** format.  
- Reports **SHALL** include:
  - Per-file statistics: path, language, total lines, logical lines, empty lines.
  - `generatedAt`: report generation timestamp, in RFC 339 / ISO 8601 format.
  - `reportFormatVersion`: version of the report format following the major.minor pattern(e.g., `"1.0"`).
- Report schemas **SHALL** remain consistent across formats.  
- Users **SHOULD** be able to customize output paths for report files.

---

## 7. Report Processing

- The tool **SHALL** compute global statistics from an existing report without re-scanning files.  
- The tool **SHALL** allow comparison of two existing reports, yielding:
  - Per-language deltas.
  - Global statistic deltas.
- Outputs of comparisons **SHALL** be available in console output.

---

## 8. CLI Interface

- The tool **SHALL** provide a command-line interface (CLI).
- The CLI **SHALL** provide help and usage information via `--help` or `-h` flags.
- The CLI **SHALL** support the following commands:
  - `count`: to count lines in specified files/directories.
  - `report`: to generate a report from counted lines.
  - `process`: to process an existing report.
  - `compare`: to compare two reports.

---

## 9. Non-Functional Requirements

- The tool **SHALL** run on major operating systems: Linux, macOS, and Windows.  
- It **SHALL** support UTF-8 (with or without BOM) and UTF-16 file encodings.  
- It **SHALL** produce deterministic output for identical inputs.  
- The tool **SHOULD** process large codebases (tens of thousands of files) efficiently (e.g. using parallel processing).
- The tool **SHOULD** provide progress indicators for long-running operations (e.g., using a progress bar).
- It **SHOULD** optimize memory use in case of large files.

---
