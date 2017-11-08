// Copyright (c) 2017 Intel Corporation. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    env::set_var("LIBCLANG_PATH", "/opt/llvm-5.0.0/lib64/");

    pkg_config::Config::new().probe("libzfs").unwrap();
    println!("cargo:rustc-link-lib=zpool");


    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .opaque_type("libzfs_handle_t")
        .hide_type("nvlist_t")
        .hide_type("nvlist")
        .constified_enum("boolean")
        .whitelisted_function("libzfs_init")
        .whitelisted_function("libzfs_fini")
        .whitelisted_function("thread_init")
        .whitelisted_function("thread_fini")
        .whitelisted_function("zpool_import")
        .whitelisted_function("zpool_export")
        .whitelisted_function("zpool_find_import")
        .whitelisted_function("zpool_iter")
        .whitelisted_function("zpool_open_canfail")
        .clang_arg("-I/usr/lib/gcc/x86_64-redhat-linux/4.8.2/include/")
        .clang_arg("-I/usr/src/zfs-0.7.1/lib/libspl/include/")
        .clang_arg("-I/usr/src/zfs-0.7.1/include/")
        .generate()
        .expect("Unable to generate bindings");


    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
