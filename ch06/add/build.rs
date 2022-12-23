// SPDX-License-Identifier: GPL-2.0
fn main() {
    std::process::Command::new("wat2wasm")
        .args(["src/add.wat", "-o", "add.wasm"])
        .status()
        .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/add.wast");
}
