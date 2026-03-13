#include "cpp_rust_cpp/src/lib.rs.h"
#include <iostream>

int main() {
    std::cout << "C++ main -> Rust -> C++ round-trip" << std::endl;

    // C++ calls Rust; Rust calls C++ greet()
    auto greeting = rust_process("world");
    std::cout << "  rust_process(\"world\") = " << static_cast<std::string>(greeting)
              << std::endl;

    // C++ passes shared Points to Rust; Rust calls C++ add_points()
    Point a{10, 20};
    Point b{3, 4};
    Point sum = rust_add_points(a, b);
    std::cout << "  rust_add_points(Point{10,20}, Point{3,4}) = Point{"
              << sum.x << "," << sum.y << "}" << std::endl;

    std::cout << "\nAll C++->Rust->C++ demos completed." << std::endl;
    return 0;
}
