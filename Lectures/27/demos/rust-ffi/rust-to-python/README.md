# Rust-to-Python FFI Demo

Rust binary embeds Python via PyO3 and calls Python code.

## Features

- **Rust → Python**: Rust imports `math`, calls `math.sqrt(16)`, runs `sum([1,2,3,4,5])`.

## Prerequisites

- Rust (rustc, cargo)
- Python 3.8+ with development headers (python3-dev)

## Build and Run

```bash
cargo run
```

PyO3 will auto-detect Python. If you have multiple Python versions, set `PYO3_PYTHON` (e.g. `PYO3_PYTHON=python3.10`).

## Testing

Run the binary; it exits 0 on success:

```bash
cargo run
```
