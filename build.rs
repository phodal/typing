extern crate cmake;

use cmake::Config;
use std::path::PathBuf;

#[cfg(target_os = "macos")]
fn get_config() -> PathBuf {
    Config::new("native/macos").build()
}

#[cfg(target_os = "macos")]
fn print_config() {
    println!("cargo:rustc-link-lib=static=objbridge");
    println!("cargo:rustc-link-lib=dylib=c++");
    println!("cargo:rustc-link-lib=framework=Cocoa");
    println!("cargo:rustc-link-lib=framework=IOKit");
}

fn main() {
    let dest = get_config();
    println!("cargo:rustc-link-search=native={}", dest.display());
    print_config();
}
