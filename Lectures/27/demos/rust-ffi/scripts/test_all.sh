#!/usr/bin/env bash
# Run all FFI project tests.
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$ROOT_DIR"

echo "=== c-to-rust ==="
cd c-to-rust && make test && cd ..

echo ""
echo "=== rust-to-c ==="
cd rust-to-c && cargo test && cd ..

echo ""
echo "=== cpp-rust-cpp ==="
cd cpp-rust-cpp && make test && cd ..

echo ""
echo "=== python-to-rust ==="
cd python-to-rust
if [ ! -d .venv ]; then
    python3 -m venv .venv
fi
source .venv/bin/activate
maturin develop --quiet
pip install pytest -q 2>/dev/null || true
pytest tests/ -v
cd ..

echo ""
echo "=== rust-to-python ==="
cd rust-to-python && cargo run && cd ..

echo ""
echo "All tests passed."
