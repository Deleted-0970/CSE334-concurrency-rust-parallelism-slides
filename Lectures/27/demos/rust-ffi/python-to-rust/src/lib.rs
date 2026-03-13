//! Python-to-Rust FFI demo: Python imports and calls Rust extension module.

use pyo3::prelude::*;

/// Python → Rust: Add two integers.
#[pyfunction]
fn add(a: i64, b: i64) -> PyResult<i64> {
    Ok(a + b)
}

/// Python → Rust: Compute Fibonacci number.
#[pyfunction]
fn fibonacci(n: u32) -> PyResult<u64> {
    match n {
        0 => Ok(0),
        1 => Ok(1),
        n => {
            let mut a = 0u64;
            let mut b = 1u64;
            for _ in 2..=n {
                let tmp = a + b;
                a = b;
                b = tmp;
            }
            Ok(b)
        }
    }
}

/// Python → Rust: Process a string (uppercase and reverse as demo).
#[pyfunction]
fn process_string(s: &str) -> PyResult<String> {
    Ok(s.to_uppercase().chars().rev().collect())
}

#[pymodule]
fn python_to_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    m.add_function(wrap_pyfunction!(process_string, m)?)?;
    Ok(())
}
