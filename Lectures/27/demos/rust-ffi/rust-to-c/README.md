# Rust-to-C FFI Demo

Rust binary calls C functions via `extern "C"`, demonstrating unidirectional FFI.

## Features

- **Rust → C**: Rust calls `c_add` and `c_greet` (defined in `c/external.c`)
- **String handling**: Uses `CStr::from_ptr()` when reading C strings, `CString` when passing to C, `libc::free` for C-allocated strings

## Prerequisites

- Rust (rustc, cargo)
- C compiler (gcc or clang)

## Build and Run

```bash
cargo run
```

## Testing

```bash
cargo test
```
