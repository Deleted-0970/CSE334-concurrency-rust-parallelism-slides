# False sharing demo

Section 7 live demo.

Build with
```
cargo build --release
```
and profile with
```
perf stat -d ./target/release/false_sharing_demo
```
and look at the number of cache misses.

Then, uncomment `#[repr(align(64))]` to add padding, and rerun.