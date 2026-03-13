# Build Rust cdylib and C++ executable for cpp-rust-cpp demo
# C++ main -> Rust -> C++ round-trip with shared struct Point

RUST_LIB := target/debug/libcpp_rust_cpp.so
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
  RUST_LIB := target/debug/libcpp_rust_cpp.dylib
endif

CXX_INC := $(shell find target/debug/build/cpp_rust_cpp-*/out/cxxbridge/include -type d 2>/dev/null | head -1)
CXX_BRIDGE_A := $(shell find target/debug/build/cpp_rust_cpp-*/out -name "libcpp-rust-cpp.a" 2>/dev/null | head -1)
CXX_RUNTIME_A := $(shell find target/debug/build/cxx-*/out -name "libcxxbridge1.a" 2>/dev/null | head -1)

.PHONY: all clean run test

all: cpp_demo

$(RUST_LIB):
	cargo build

cpp_demo: $(RUST_LIB) cpp/main.cc
	$(CXX) -std=c++14 -o cpp_demo cpp/main.cc \
		-I $(CXX_INC) -I include \
		$(CXX_BRIDGE_A) $(CXX_RUNTIME_A) \
		-L target/debug -lcpp_rust_cpp -lpthread -ldl \
		-Wl,-rpath,target/debug

run: cpp_demo
	./cpp_demo

test: cpp_demo
	@./cpp_demo | grep -q 'rust_process("world")' && \
	 ./cpp_demo | grep -q 'rust_add_points' && \
	 ./cpp_demo | grep -q 'C++->Rust->C++' && \
	 echo "Tests passed"

clean:
	rm -f cpp_demo
	cargo clean
