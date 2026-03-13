# Build Rust library and C executable for c-to-rust demo
# Prerequisites: Rust, gcc/clang

RUST_LIB := target/debug/libc_to_rust.so
# On macOS: libc_to_rust.dylib
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
  RUST_LIB := target/debug/libc_to_rust.dylib
endif

.PHONY: all clean run test

all: demo

# Build Rust cdylib first
$(RUST_LIB):
	cargo build

# Build C executable, linking Rust library
demo: $(RUST_LIB) c/main.c c/lib.h
	$(CC) -o demo c/main.c -I c -L target/debug -lc_to_rust -Wl,-rpath,target/debug

run: demo
	./demo

test: demo
	./demo | diff - expected_output.txt && echo "Tests passed"

clean:
	rm -f demo
	cargo clean
