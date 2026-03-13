fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("cpp/lib.cc")
        .include("include")
        .std("c++14")
        .compile("cpp-rust-cpp");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=cpp/lib.cc");
    println!("cargo:rerun-if-changed=include/lib.h");
}
