//! C++ → Rust → C++ round-trip: C++ main calls Rust, Rust calls C++.
//! Shared struct Point used in both languages.

#[cxx::bridge]
mod ffi {
    /// Shared struct: usable from both C++ and Rust.
    struct Point {
        x: i32,
        y: i32,
    }

    extern "Rust" {
        /// C++ calls this; Rust uppercases input, then calls C++ greet().
        fn rust_process(input: &str) -> String;

        /// C++ passes two Points; Rust forwards to C++ add_points().
        fn rust_add_points(a: Point, b: Point) -> Point;
    }

    unsafe extern "C++" {
        include!("lib.h");

        fn greet(name: &CxxString) -> UniquePtr<CxxString>;
        fn add_points(a: Point, b: Point) -> Point;
    }
}

use cxx::let_cxx_string;

fn rust_process(input: &str) -> String {
    let upper = input.to_uppercase();
    let_cxx_string!(s = upper.as_str());
    let greeting = ffi::greet(&*s);
    greeting
        .as_ref()
        .map(|g| g.to_string_lossy().into_owned())
        .unwrap_or_default()
}

fn rust_add_points(a: ffi::Point, b: ffi::Point) -> ffi::Point {
    ffi::add_points(a, b)
}
