//! Rust-to-Python FFI demo: Rust binary embeds Python and calls it.

use pyo3::prelude::*;
use pyo3::types::PyList;

fn main() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        println!("Rust calling Python:");

        // Call math.sqrt(16.0)
        let math = py.import_bound("math")?;
        let sqrt_fn = math.getattr("sqrt")?;
        let result: f64 = sqrt_fn.call1((16.0,))?.extract()?;
        println!("  math.sqrt(16.0) = {}", result);

        // Call builtins.sum([1, 2, 3, 4, 5])
        let builtins = py.import_bound("builtins")?;
        let sum_fn = builtins.getattr("sum")?;
        let list = PyList::new_bound(py, [1i64, 2, 3, 4, 5]);
        let sum_result: i64 = sum_fn.call1((list,))?.extract()?;
        println!("  sum([1, 2, 3, 4, 5]) = {}", sum_result);

        println!("\nAll Rust->Python FFI demos completed.");
        Ok(())
    })
}
