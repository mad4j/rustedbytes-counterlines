# RustedBytes CounterLines - SLOC Counter CLI Tool

RustedBytes CounterLines is a high-performance, multi-language source code line counter CLI tool written in Rust. It provides accurate line counting, supports multiple output formats (JSON, CSV, XML), and includes report processing and comparison capabilities.

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively

### Bootstrap, Build, and Test
- Ensure Rust toolchain is available: `rustc --version && cargo --version`
- Build debug version: `cargo build` -- takes ~84 seconds. NEVER CANCEL. Set timeout to 120+ seconds.
- Build release version: `cargo build --release` -- takes ~38 seconds. NEVER CANCEL. Set timeout to 60+ seconds.
- Run tests: `cargo test` -- takes ~9 seconds. Currently 0 unit tests defined.
- Check code formatting: `cargo fmt --check`
- Run linting: `cargo clippy --all-targets --all-features` -- shows 12 format string warnings but completes successfully.

### Build Time Expectations
- **CRITICAL**: Debug build takes ~84 seconds, release build takes ~38 seconds. NEVER CANCEL builds.
- **CRITICAL**: Use timeouts of 120+ seconds for debug builds and 60+ seconds for release builds.
- **CRITICAL**: Test suite runs in ~9 seconds but contains 0 tests, so build validation relies on successful compilation.

### Run the CLI Tool
- **ALWAYS** build the project first before running.
- Use release build for better performance: `./target/release/rustedbytes-counterlines`
- Use debug build for development: `./target/debug/rustedbytes-counterlines`
- Get help: `./target/release/rustedbytes-counterlines --help`
- Get version: `./target/release/rustedbytes-counterlines --version`

## Core CLI Commands

### Count Command
Count lines in files and directories:
```bash
# Count lines in specific files
./target/release/rustedbytes-counterlines count src/main.rs src/lib.rs

# Count with recursive directory traversal
./target/release/rustedbytes-counterlines count src/ --recursive

# Progress bar enabled by default. Use --no-progress to disable
./target/release/rustedbytes-counterlines count . --recursive --no-progress

# Enable performance metrics logging
./target/release/rustedbytes-counterlines count src/ --recursive --enable-metrics

# Customize thread count (default: number of CPU cores)
./target/release/rustedbytes-counterlines count src/ --recursive --threads 8
```

### Report Command
Generate structured reports:
```bash
# Generate JSON report
./target/release/rustedbytes-counterlines report src/ --recursive --format json --output report.json

# Generate CSV report  
./target/release/rustedbytes-counterlines report src/ --recursive --format csv --output report.csv

# Generate XML report (KNOWN ISSUE: XML serialization fails with "emitter error")
./target/release/rustedbytes-counterlines report src/ --recursive --format xml --output report.xml

# Include performance metrics in report generation
./target/release/rustedbytes-counterlines report src/ --recursive --format json --output report.json --enable-metrics
```

### Process Command
Process existing reports without re-scanning files:
```bash
# Display statistics from a report
./target/release/rustedbytes-counterlines process report.json

# Sort by different metrics
./target/release/rustedbytes-counterlines process report.json --sort logical

# Export processed results
./target/release/rustedbytes-counterlines process report.json --export summary.json --format json
```

### Compare Command
Compare two reports:
```bash
# Compare two reports
./target/release/rustedbytes-counterlines compare report_old.json report_new.json

# Export comparison results
./target/release/rustedbytes-counterlines compare report_old.json report_new.json --export comparison.json --format json
```

## Performance Characteristics

### Speed and Throughput
- Tool processes ~1.5-2.0 million lines per second
- Processing 12 source files (2,900 lines) takes <3ms
- Uses parallel processing (default: number of CPU cores)
- Progress indicators enabled by default for all operations

### Memory Usage
- Memory efficient: streams files instead of loading entirely into memory
- Deterministic output for identical inputs
- Optimized for large codebases (tens of thousands of files)

## Validation Scenarios

### Always Test These Scenarios After Changes
1. **Basic functionality**: Count lines in the src/ directory
   ```bash
   ./target/release/rustedbytes-counterlines count src/ --recursive
   ```

2. **Report generation**: Create and process a JSON report
   ```bash
   ./target/release/rustedbytes-counterlines report src/ --recursive --format json --output /tmp/test.json
   ./target/release/rustedbytes-counterlines process /tmp/test.json
   ```

3. **Comparison workflow**: Generate two reports and compare them
   ```bash
   ./target/release/rustedbytes-counterlines report src/ --recursive --format json --output /tmp/report1.json
   ./target/release/rustedbytes-counterlines report Cargo.toml --format json --output /tmp/report2.json
   ./target/release/rustedbytes-counterlines compare /tmp/report1.json /tmp/report2.json
   ```

4. **Performance features**: Test metrics and progress indicators
   ```bash
   ./target/release/rustedbytes-counterlines count src/ --recursive --enable-metrics
   ```

## Validation and Quality Assurance

### Always Run Before Committing
- Format check: `cargo fmt --check` -- should return exit code 0
- Linting: `cargo clippy --all-targets --all-features` -- shows 12 format warnings but should complete successfully
- Build validation: `cargo build && cargo build --release` -- both must succeed

### Expected Output Validation
- Console output includes formatted tables with thousands separators
- Reports include per-file statistics, language summaries, and global statistics
- JSON/CSV export creates valid files with proper structure
- XML export currently fails (known limitation)

## Known Issues and Limitations

### XML Export Issue
- XML report generation fails with "Serialization error: Writer: emitter error: last element name is not available"
- **DO NOT** rely on XML export functionality
- Use JSON or CSV formats instead

### Test Coverage
- Project has 0 unit tests defined
- Validation relies on successful compilation and manual testing
- Always manually test CLI functionality after changes

## Common Tasks

### Project Structure
```
.
├── Cargo.toml           # Rust project configuration
├── build.rs             # Build script for Windows resources  
├── src/                 # Source code directory
│   ├── main.rs         # CLI entry point
│   ├── cli.rs          # Command-line interface
│   ├── counter.rs      # Core counting logic
│   ├── language.rs     # Language detection and definitions
│   ├── output.rs       # Console output formatting
│   ├── report.rs       # Report generation
│   ├── processor.rs    # Report processing
│   ├── config.rs       # Configuration and metrics
│   ├── error.rs        # Error handling
│   ├── utils.rs        # Utility functions
│   └── config.toml     # Default configuration
├── assets/             # Application assets (icons)
├── resources/          # Additional resources
├── README.md           # Comprehensive documentation
├── REQUIREMENTS.md     # Detailed specifications
├── METRICS_USAGE.md    # Performance metrics guide
└── CHANGELOG.md        # Version history
```

### Quick Reference Commands
```bash
# Complete workflow example
cargo build --release                                                    # Build (38s)
./target/release/rustedbytes-counterlines count src/ --recursive        # Count lines (<3ms)
./target/release/rustedbytes-counterlines report src/ --recursive --format json --output report.json  # Generate report (<3ms)
./target/release/rustedbytes-counterlines process report.json           # Process report (<1ms)
cargo fmt --check && cargo clippy --all-targets --all-features          # Validation (9s)
```

### Output Formats
- **JSON**: Fully working, includes all metadata
- **CSV**: Fully working, compatible with spreadsheet applications  
- **XML**: Currently broken due to serialization issues
- **Console**: Rich formatted tables with color coding and alignment

### Performance Metrics
- Enable with `--enable-metrics` flag
- Logs to `sloc_metrics.log` by default
- Custom log file: `--metrics-file /path/to/custom.log`
- Includes timing, throughput, thread count, and system information

## Troubleshooting

### Build Issues
- Ensure Rust toolchain is up to date: `rustup update`
- Clean build artifacts: `cargo clean` then rebuild
- Check dependencies: `cargo check`

### Performance Issues  
- Use release build for production use
- Adjust thread count with `--threads N`
- Enable metrics to identify bottlenecks
- Avoid processing entire repository with build artifacts (use specific paths)

### Output Issues
- XML export is known to fail - use JSON or CSV instead
- Large directory scans may include unwanted files (use specific paths)
- Progress indicators enabled by default (use --no-progress to disable)

Remember: Always build first, test core functionality, and validate with the scenarios above before considering changes complete.