# Performance Metrics Guide

## Overview

The SLOC counter tool includes comprehensive performance metrics logging (REQ-9.7) to help analyze tool performance, identify bottlenecks, and optimize processing of large codebases.

## Enabling Metrics

### Command Line Options

All commands support metrics logging through CLI flags:

```bash
# Enable metrics with default log file (sloc_metrics.log)
cargo run -- count src/ --enable-metrics

# Enable metrics with custom log file
cargo run -- count src/ --enable-metrics --metrics-file /path/to/custom_metrics.log

# Set performance summary threshold (default: 5 seconds)
cargo run -- count src/ --enable-metrics --perf-summary-threshold 10
```

### Configuration File

Enable metrics via configuration file:

```toml
# config.toml
[performance]
enable_metrics = true
metrics_file = "project_metrics.log"
default_threads = 8
chunk_size = 2000
```

Use with:

```bash
cargo run -- count src/ --config config.toml
```

## Unsupported Files (REQ-3.5.1, REQ-3.5.2, REQ-3.5.3)

Files with unsupported syntax (no language definition):

- **Are excluded from all statistics and metrics** (REQ-3.5.1)
- **Are not included in any line, file, or language counts** (REQ-3.5.2)
- **Are listed separately** in the console output and reports (REQ-3.5.3)

The metrics log contains timestamped entries showing:

```bash
=== SLOC Metrics Session Started ===
Operation: count
Timestamp: 2024-01-15 14:30:25 UTC
Args: paths=1, recursive=true, threads=8, format=Some(Json)

Tool version: 0.1.0
[0.001s] system_cpu_count: 8.000
[0.002s] system_available_parallelism: 8.000
[0.005s] config_load_time: 0.003
[0.008s] language_overrides_count: 0.000
[0.015s] path_collection_time: 0.007
[0.016s] total_files_to_process: 245.000
[0.017s] thread_count: 8.000
[0.234s] files_processed_successfully: 245.000
[0.235s] total_lines_processed: 45678.000
[0.236s] logical_lines_processed: 32145.000
[0.236s] comment_lines_processed: 4600.000
[0.237s] empty_lines_processed: 8945.000
[0.238s] overall_throughput_lines_per_sec: 195234.567
[0.239s] files_per_second: 1047.863
[0.245s] report_creation_time: 0.006
[0.250s] console_output_time: 0.005
[0.255s] total_files: 245.000
[0.256s] total_lines: 45678.000
[0.257s] elapsed_seconds: 0.255
[0.258s] lines_per_second: 179133.333
[0.259s] total_operation_time: 0.259
=== Session Completed ===
```

## Logged Metrics

> Note: Metrics and statistics do **not** include unsupported files (REQ-3.5.1, REQ-3.5.2). These files are only listed separately (REQ-3.5.3).

### System Information

- `system_cpu_count`: Number of CPU cores
- `system_available_parallelism`: Available parallel threads
- Tool version information

### Configuration and Setup

- `config_load_time`: Time to load configuration files
- `language_overrides_count`: Number of language overrides applied
- `thread_count`: Actual number of threads used

### File Processing

- `path_collection_time`: Time to collect and validate file paths
- `total_files_to_process`: Number of files to analyze
- `files_processed_successfully`: Successfully processed files
- `file_errors`: Number of file processing errors (if any)

### Performance Metrics

- `total_processing_time`: Core file processing time
- `overall_throughput_lines_per_sec`: Lines processed per second
- `files_per_second`: Files processed per second
- `large_file_throughput`: Throughput for files > 1000 lines
- `comment_lines_processed`: Total comment lines processed

### Per-File Metrics (for files taking > 1ms)

- `file_process_time_<filename>`: Individual file processing time

### Report Generation

- `report_creation_time`: Time to create report structure
- `checksum_calculation_time`: Time for checksum calculation (if enabled)
- `console_output_time`: Time to display console output
- `report_export_time`: Time to export report to file

### Memory Usage

- `memory_usage_estimate_bytes`: Estimated memory usage

## Use Cases

### Performance Optimization

Monitor throughput and identify slow files:

```bash
# Process large codebase with metrics
cargo run -- count /large/project --recursive --enable-metrics --threads 16

# Analyze metrics log
grep "throughput" sloc_metrics.log
grep "file_process_time" sloc_metrics.log | sort -k3 -n | tail -10
```

### Benchmarking

Compare performance across different configurations:

```bash
# Single-threaded
cargo run -- count src/ --threads 1 --enable-metrics --metrics-file single_thread.log

# Multi-threaded
cargo run -- count src/ --threads 8 --enable-metrics --metrics-file multi_thread.log

# Compare results
echo "Single-threaded:"
grep "elapsed_seconds" single_thread.log

echo "Multi-threaded:"
grep "elapsed_seconds" multi_thread.log
```

### Continuous Integration

Monitor tool performance in CI pipelines:

```bash
# In CI script
cargo run -- count . --recursive --enable-metrics --metrics-file ci_metrics.log

# Extract key metrics for monitoring
PROCESSING_TIME=$(grep "total_processing_time" ci_metrics.log | cut -d' ' -f3)
THROUGHPUT=$(grep "overall_throughput" ci_metrics.log | cut -d' ' -f3)

echo "Processing time: ${PROCESSING_TIME}s"
echo "Throughput: ${THROUGHPUT} lines/sec"
```

### Report Processing Metrics

All commands support metrics:

```bash
# Process existing report with metrics
cargo run -- process report.json --enable-metrics

# Compare reports with metrics
cargo run -- compare old_report.json new_report.json --enable-metrics

# Generate report with metrics
cargo run -- report src/ --output project_report.json --enable-metrics
```

### Performance Summary

For operations taking longer than the threshold (default 5 seconds) or processing more than 1000 files, the tool displays a performance summary:

```text
Performance Summary:
    Total time: 2.45s
    Files processed: 1,234
    Lines processed: 456,789
    Comment lines: 45,000
    Throughput: 186,444 lines/sec
    Metrics logged to: sloc_metrics.log
```

## Analyzing Metrics

### Common Patterns

1. **Slow File Processing**: Look for high `file_process_time_*` values
2. **Thread Efficiency**: Compare `thread_count` vs actual performance
3. **I/O Bottlenecks**: Check `path_collection_time` vs `total_processing_time`
4. **Comment Line Ratio**: Compare `comment_lines_processed` to total lines for documentation density
5. **Memory Issues**: Monitor `memory_usage_estimate_bytes` for large projects

### Performance Tuning

Based on metrics, you can optimize:

- **Thread Count**: Adjust `--threads` based on CPU cores and I/O patterns
- **Chunk Size**: Modify `chunk_size` in configuration for better parallelization
- **File Filtering**: Use specific paths instead of broad recursive scans

## Integration with External Tools

The metrics log format is designed for easy parsing by external monitoring tools:

```python
# Example Python script to parse metrics
import re
import json

def parse_metrics_log(filename):
    metrics = {}
    with open(filename, 'r') as f:
        for line in f:
            match = re.match(r'\[(\d+\.\d+)s\] ([^:]+): (\d+\.\d+)', line)
            if match:
                timestamp, metric_name, value = match.groups()
                metrics[metric_name] = {
                    'timestamp': float(timestamp),
                    'value': float(value)
                }
    return metrics

# Usage
metrics = parse_metrics_log('sloc_metrics.log')
print(f"Total processing time: {metrics['total_processing_time']['value']:.2f}s")
print(f"Throughput: {metrics['overall_throughput_lines_per_sec']['value']:.0f} lines/sec")
```

This comprehensive metrics system helps ensure the SLOC counter performs optimally across different environments and use cases.
