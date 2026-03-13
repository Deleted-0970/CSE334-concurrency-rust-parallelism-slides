#!/usr/bin/env python3
"""Demo script: Python calls Rust extension module."""

import python_to_rust

print("Python calling Rust:")
print(f"  add(1, 2) = {python_to_rust.add(1, 2)}")
print(f"  fibonacci(10) = {python_to_rust.fibonacci(10)}")
print(f"  process_string('hello') = {python_to_rust.process_string('hello')}")

print("\nAll Python->Rust FFI demos completed.")
