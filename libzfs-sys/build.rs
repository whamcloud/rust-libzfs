// Copyright (c) 2018 DDN. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

extern crate bindgen;
extern crate pkg_config;

use std::env;

fn main() {
    let out_file = env::current_dir().unwrap().join("src").join("bindings.rs");

    env::set_var("LIBCLANG_PATH", "/opt/llvm-5.0.0/lib64/");

    pkg_config::Config::new()
        .atleast_version("0.7.9")
        .probe("libzfs")
        .unwrap();
    println!("cargo:rustc-link-lib=zpool");

    // Skip building if bindings already exist.
    // If you want to rebuild, delete the bindings file.
    if out_file.exists() {
        return;
    }

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .constified_enum_module("boolean")
        .whitelist_var("vdev_stat_t")
        .whitelist_type("vdev_stat_t")
        .whitelist_var("ZPOOL_MAXPROPLEN")
        .whitelist_var("ZPOOL_CONFIG_POOL_NAME")
        .whitelist_var("ZPOOL_CONFIG_TYPE")
        .whitelist_var("ZPOOL_CONFIG_VDEV_TREE")
        .whitelist_var("ZPOOL_CONFIG_CHILDREN")
        .whitelist_var("ZPOOL_CONFIG_SPARES")
        .whitelist_var("ZPOOL_CONFIG_L2CACHE")
        .whitelist_var("ZPOOL_CONFIG_PATH")
        .whitelist_var("ZPOOL_CONFIG_PHYS_PATH")
        .whitelist_var("ZPOOL_CONFIG_DEVID")
        .whitelist_var("ZPOOL_CONFIG_WHOLE_DISK")
        .whitelist_var("ZPOOL_CONFIG_IS_LOG")
        .whitelist_var("ZPOOL_CONFIG_HOSTID")
        .whitelist_var("ZPOOL_CONFIG_HOSTNAME")
        .whitelist_var("ZPOOL_CONFIG_GUID")
        .whitelist_var("ZPOOL_CONFIG_AUX_STATE")
        .whitelist_var("ZPOOL_CONFIG_VDEV_STATS")
        .whitelist_var("VDEV_TYPE_ROOT")
        .whitelist_var("VDEV_TYPE_MIRROR")
        .whitelist_var("VDEV_TYPE_REPLACING")
        .whitelist_var("VDEV_TYPE_RAIDZ")
        .whitelist_var("VDEV_TYPE_DISK")
        .whitelist_var("VDEV_TYPE_FILE")
        .whitelist_var("VDEV_TYPE_MISSING")
        .whitelist_var("VDEV_TYPE_HOLE")
        .whitelist_var("VDEV_TYPE_SPARE")
        .whitelist_var("VDEV_TYPE_LOG")
        .whitelist_var("VDEV_TYPE_L2CACHE")
        .whitelist_var("ZPROP_VALUE")
        .whitelist_var("ZFS_MAXPROPLEN")
        .whitelist_var("ZFS_MAX_DATASET_NAME_LEN")
        .whitelist_type("zpool_prop_t")
        .constified_enum_module("zpool_prop_t")
        .whitelist_type("pool_state_t")
        .constified_enum_module("pool_state")
        .bitfield_enum("zfs_type_t")
        .opaque_type("libzfs_handle_t")
        .blacklist_type("nvlist_t")
        .blacklist_type("nvlist")
        .whitelist_function("libzfs_init")
        .whitelist_function("libzfs_fini")
        .whitelist_function("thread_init")
        .whitelist_function("thread_fini")
        .whitelist_function("zpool_import")
        .whitelist_function("zpool_export")
        .whitelist_function("zpool_find_import")
        .whitelist_function("zpool_iter")
        .whitelist_function("zpool_open_canfail")
        .whitelist_function("zpool_close")
        .whitelist_function("zpool_get_name")
        .whitelist_function("zpool_get_state")
        .whitelist_function("zpool_pool_state_to_name")
        .whitelist_function("zpool_get_prop_int")
        .whitelist_function("zpool_get_prop")
        .whitelist_function("zpool_get_config")
        .whitelist_function("zpool_get_handle")
        .whitelist_function("zpool_state_to_name")
        .whitelist_function("zfs_open")
        .whitelist_function("zfs_close")
        .whitelist_function("zfs_iter_filesystems")
        .whitelist_function("zfs_get_name")
        .whitelist_function("zfs_get_user_props")
        .whitelist_function("zfs_get_type")
        .whitelist_function("zfs_type_to_name")
        .whitelist_function("zfs_path_to_zhandle")
        .whitelist_function("zpool_disable_datasets")
        .whitelist_function("libzfs_error_description")
        .whitelist_function("zfs_prop_get")
        .whitelist_function("zfs_expand_proplist")
        .whitelist_function("zfs_prop_to_name")
        .whitelist_function("zfs_validate_name")
        .whitelist_function("zprop_free_list")
        .clang_arg("-I/usr/lib/gcc/x86_64-redhat-linux/4.8.2/include/")
        .clang_arg("-I/usr/src/zfs-0.7.11/lib/libspl/include/")
        .clang_arg("-I/usr/src/zfs-0.7.11/include/")
        .generate()
        .expect("Unable to generate bindings");

    // Write bindings to src.
    bindings
        .write_to_file(out_file)
        .expect("Couldn't write bindings!");
}
