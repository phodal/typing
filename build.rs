extern crate cmake;

use cmake::Config;
use std::path::PathBuf;


#[cfg(target_os = "macos")]
fn get_config() -> PathBuf {
    Config::new("native/basic").build()
}

#[cfg(target_os = "macos")]
fn print_config() {
    println!("cargo:rustc-link-lib=dylib=c++");
    println!("cargo:rustc-link-lib=framework=Cocoa");
    println!("cargo:rustc-link-lib=framework=IOKit");
}
//
// fn main() {
//     let path = get_config();
//     println!("cargo:rustc-link-search=native={}", path.display());
//     println!("cargo:rustc-link-lib=static=doubler");
//     print_config();
// }


fn main() {
    let dst = Config::new("native/basic")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=doubler");
}
