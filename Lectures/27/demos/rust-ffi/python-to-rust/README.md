# Python-to-Rust FFI Demo

Python imports Rust PyO3 extension and calls Rust functions.

## Features

- **Python → Rust**: `add`, `fibonacci`, `process_string` — Python calls Rust for performance/safety.

## Prerequisites

- Rust (rustc, cargo)
- Python 3.8+
- maturin: `pip install maturin`

## Build and Run

```bash
maturin develop
python examples/demo.py
```

## Testing

```bash
maturin develop
pip install -e ".[dev]"
pytest tests/ -v
```
