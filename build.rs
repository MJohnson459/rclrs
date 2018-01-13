extern crate cmake;
extern crate pkg_config;

use std::error::Error;

fn main() {
    // Check to see if rcl is already installed
    let rcl_pkg = pkg_config::Config::new().atleast_version("0.4.0").probe("rcl");
    match rcl_pkg {
        Ok(_v) => {
            println!("Found rcl installed...now what?");
            return;
        },
        Err(e) => {
            println!("Failed to find rcl installed: {}", e.description());
            return; //tmp
            install_rcl();
        },
    }



}

fn install_rcl() {
    println!("Attempting to build rcl from source");
    let dst = cmake::build("rcl");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=foo");
}
