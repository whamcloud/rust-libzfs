// Copyright (c) 2018 Intel Corporation. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

extern crate bindgen;
extern crate pkg_config;

use std::env;

fn main() {
    let out_file = env::current_dir().unwrap().join("src").join("bindings.rs");

    env::set_var("LIBCLANG_PATH", "/opt/llvm-5.0.0/lib64/");

    pkg_config::Config::new().probe("libzfs").unwrap();
    println!("cargo:rustc-link-lib=zpool");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .constified_enum_module("boolean")
        .whitelisted_var("vdev_stat_t")
        .whitelisted_type("vdev_stat_t")
        .whitelisted_var("ZPOOL_MAXPROPLEN")
        .whitelisted_var("ZPOOL_CONFIG_POOL_NAME")
        .whitelisted_var("ZPOOL_CONFIG_TYPE")
        .whitelisted_var("ZPOOL_CONFIG_VDEV_TREE")
        .whitelisted_var("ZPOOL_CONFIG_CHILDREN")
        .whitelisted_var("ZPOOL_CONFIG_SPARES")
        .whitelisted_var("ZPOOL_CONFIG_L2CACHE")
        .whitelisted_var("ZPOOL_CONFIG_PATH")
        .whitelisted_var("ZPOOL_CONFIG_PHYS_PATH")
        .whitelisted_var("ZPOOL_CONFIG_DEVID")
        .whitelisted_var("ZPOOL_CONFIG_WHOLE_DISK")
        .whitelisted_var("ZPOOL_CONFIG_IS_LOG")
        .whitelisted_var("ZPOOL_CONFIG_HOSTID")
        .whitelisted_var("ZPOOL_CONFIG_HOSTNAME")
        .whitelisted_var("ZPOOL_CONFIG_GUID")
        .whitelisted_var("ZPOOL_CONFIG_AUX_STATE")
        .whitelisted_var("ZPOOL_CONFIG_VDEV_STATS")
        .whitelisted_var("VDEV_TYPE_ROOT")
        .whitelisted_var("VDEV_TYPE_MIRROR")
        .whitelisted_var("VDEV_TYPE_REPLACING")
        .whitelisted_var("VDEV_TYPE_RAIDZ")
        .whitelisted_var("VDEV_TYPE_DISK")
        .whitelisted_var("VDEV_TYPE_FILE")
        .whitelisted_var("VDEV_TYPE_MISSING")
        .whitelisted_var("VDEV_TYPE_HOLE")
        .whitelisted_var("VDEV_TYPE_SPARE")
        .whitelisted_var("VDEV_TYPE_LOG")
        .whitelisted_var("VDEV_TYPE_L2CACHE")
        .whitelisted_var("ZPROP_VALUE")
        .whitelisted_var("ZFS_MAXPROPLEN")
        .whitelisted_var("ZFS_MAX_DATASET_NAME_LEN")
        .whitelisted_type("zpool_prop_t")
        .constified_enum_module("zpool_prop_t")
        .whitelisted_type("pool_state_t")
        .constified_enum_module("pool_state")
        .bitfield_enum("zfs_type_t")
        .opaque_type("libzfs_handle_t")
        .hide_type("nvlist_t")
        .hide_type("nvlist")
        .whitelisted_function("libzfs_init")
        .whitelisted_function("libzfs_fini")
        .whitelisted_function("thread_init")
        .whitelisted_function("thread_fini")
        .whitelisted_function("zpool_import")
        .whitelisted_function("zpool_export")
        .whitelisted_function("zpool_find_import")
        .whitelisted_function("zpool_iter")
        .whitelisted_function("zpool_open_canfail")
        .whitelisted_function("zpool_close")
        .whitelisted_function("zpool_get_name")
        .whitelisted_function("zpool_get_state")
        .whitelisted_function("zpool_pool_state_to_name")
        .whitelisted_function("zpool_get_prop_int")
        .whitelisted_function("zpool_get_prop")
        .whitelisted_function("zpool_get_config")
        .whitelisted_function("zpool_get_handle")
        .whitelisted_function("zpool_state_to_name")
        .whitelisted_function("zfs_open")
        .whitelisted_function("zfs_close")
        .whitelisted_function("zfs_iter_filesystems")
        .whitelisted_function("zfs_get_name")
        .whitelisted_function("zfs_get_user_props")
        .whitelisted_function("zfs_get_type")
        .whitelisted_function("zfs_type_to_name")
        .whitelisted_function("zfs_path_to_zhandle")
        .whitelisted_function("zpool_disable_datasets")
        .whitelisted_function("libzfs_error_description")
        .whitelisted_function("zfs_prop_get")
        .whitelisted_function("zfs_expand_proplist")
        .whitelisted_function("zfs_prop_to_name")
        .whitelisted_function("zfs_validate_name")
        .whitelisted_function("zprop_free_list")
        .clang_arg("-I/usr/lib/gcc/x86_64-redhat-linux/4.8.2/include/")
        .clang_arg("-I/usr/src/zfs-0.7.7/lib/libspl/include/")
        .clang_arg("-I/usr/src/zfs-0.7.7/include/")
        .generate()
        .expect("Unable to generate bindings");

    // Write bindings to src.
    bindings
        .write_to_file(out_file)
        .expect("Couldn't write bindings!");
}
