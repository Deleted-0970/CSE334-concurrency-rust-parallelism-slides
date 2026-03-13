//! Rust-to-C FFI demo: Rust binary calls C.

use rust_to_c::{call_c_add, call_c_greet};

fn main() {
    println!("Rust calling C:");
    println!("  c_add(2, 3) = {}", call_c_add(2, 3));
    println!("  c_greet(\"World\") = {}", call_c_greet("World").unwrap());

    println!("\nAll Rust->C FFI demos completed.");
}
