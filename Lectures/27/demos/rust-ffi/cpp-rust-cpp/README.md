# C++ → Rust → C++ Round-Trip

C++ main calls Rust, which then calls back into C++. Demonstrates shared structs (`Point`) between both languages.

## Flow

1. **C++ main** calls `rust_process("world")` and `rust_add_points(a, b)`
2. **Rust** `rust_process`: uppercases input, calls C++ `greet()`, returns greeting
3. **Rust** `rust_add_points`: forwards to C++ `add_points(Point, Point)`
4. **Shared struct** `Point { x, y }` is defined in the cxx bridge and used in both directions

## Prerequisites

- Rust (rustc, cargo)
- C++ compiler (g++ or clang++)

## Build and Run

```bash
make
make run
```

## Testing

```bash
make test
```
