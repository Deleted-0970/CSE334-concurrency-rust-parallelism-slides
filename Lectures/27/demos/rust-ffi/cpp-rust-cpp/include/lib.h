#ifndef CPP_RUST_CPP_LIB_H
#define CPP_RUST_CPP_LIB_H

#include <cstdint>
#include <memory>
#include <string>

// Define Point before cxx does (lib.h is included first).
// Must match cxx bridge: struct Point { x: i32, y: i32 }
#ifndef CXXBRIDGE1_STRUCT_Point
#define CXXBRIDGE1_STRUCT_Point
struct Point final {
    std::int32_t x = 0;
    std::int32_t y = 0;
};
#endif

// C++ functions called by Rust
std::unique_ptr<std::string> greet(const std::string& name);
Point add_points(Point a, Point b);

#endif /* CPP_RUST_CPP_LIB_H */
