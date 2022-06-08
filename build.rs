fn main() {
    println!("cargo:rerun-if-changed=memory.ld");
    println!("cargo:rerun-if-changed=build.rs");
}
