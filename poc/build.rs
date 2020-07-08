extern crate cmake;

use cmake::Config;
use std::path::PathBuf;


#[cfg(target_os = "macos")]
fn get_config() -> PathBuf {
    Config::new("/Users/fdhuang/repractise/typing/native/objbridge").build()
}

#[cfg(target_os = "macos")]
fn print_config() {
    println!("cargo:rustc-link-lib=dylib=c++");
    println!("cargo:rustc-link-lib=static=objbridge.a");
    println!("cargo:rustc-link-search=native=/Users/fdhuang/repractise/typing/native/objbridge");
    println!("cargo:rustc-link-lib=framework=Cocoa");
    println!("cargo:rustc-link-lib=framework=IOKit");
}

fn main() {
    let path = get_config();
    println!("cargo:rustc-link-search=native={}", path.display());
    print_config();
}
