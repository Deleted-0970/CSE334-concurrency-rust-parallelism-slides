# C-to-Rust FFI Demo

C executable calls Rust cdylib via `extern "C"`, demonstrating unidirectional FFI.

## Features

- **C → Rust**: C main links against Rust library, calls `rust_multiply`, `rust_concat_strings`, `rust_greet`, `rust_point_sum`
- **String handling**: Uses `CStr::from_ptr()` when reading, `CString` when creating strings for C
- **Shared types**: `Point` struct with `#[repr(C)]` passed across the boundary

## Prerequisites

- Rust (rustc, cargo)
- C compiler (gcc or clang)

## Build and Run

```bash
make
make run
```

Or manually:

```bash
cargo build
gcc -o demo c/main.c -I c -L target/debug -lc_to_rust -Wl,-rpath,target/debug
./demo
```

## Testing

```bash
make test
```
