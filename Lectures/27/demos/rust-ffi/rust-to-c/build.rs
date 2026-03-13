fn main() {
    cc::Build::new()
        .file("c/external.c")
        .compile("external");

    println!("cargo:rerun-if-changed=c/external.c");
}
