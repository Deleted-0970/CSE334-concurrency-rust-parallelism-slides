#include "cpp_rust_cpp/src/lib.rs.h"  // defines Point, includes lib.h with declarations
#include <sstream>

std::unique_ptr<std::string> greet(const std::string& name) {
    std::ostringstream ss;
    ss << "Hello from C++, " << name << "!";
    return std::make_unique<std::string>(ss.str());
}

Point add_points(Point a, Point b) {
    return Point{a.x + b.x, a.y + b.y};
}
