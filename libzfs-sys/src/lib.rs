// Copyright (c) 2018 DDN. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![crate_name = "libzfs_sys"]

//! libzfs_sys — Rust bindings to libzfs2.
//!
//! ## Overview
//!
//! Bindings created using [rust bindgen](https://github.com/rust-lang-nursery/rust-bindgen) and written
//! to the src dir. To rebuild bindings run `cargo build`.
//!
//! ## ZFS version
//! These bindings were compiled against ZFS 0.7.11. As `libzfs` is not a stable interface,
//! they should only be used against this version.
//!
//! ## OS
//!
//! These bindings were compiled on Centos 7.5.x. They are likely to work against other
//! OS, but make sure to test first.
//!

extern crate nvpair_sys;
use nvpair_sys::*;
include!("bindings.rs");

fn utf8_to_string(bytes: &[u8]) -> String {
    String::from_utf8(bytes.to_vec()).unwrap()
}

pub fn zpool_config_vdev_tree() -> String {
    utf8_to_string(ZPOOL_CONFIG_VDEV_TREE)
}

pub fn zpool_config_type() -> String {
    utf8_to_string(ZPOOL_CONFIG_TYPE)
}

pub fn zpool_config_children() -> String {
    utf8_to_string(ZPOOL_CONFIG_CHILDREN)
}

pub fn zpool_config_spares() -> String {
    utf8_to_string(ZPOOL_CONFIG_SPARES)
}

pub fn zpool_config_l2cache() -> String {
    utf8_to_string(ZPOOL_CONFIG_L2CACHE)
}

pub fn zpool_config_path() -> String {
    utf8_to_string(ZPOOL_CONFIG_PATH)
}

pub fn zpool_config_dev_id() -> String {
    utf8_to_string(ZPOOL_CONFIG_DEVID)
}

pub fn zpool_config_phys_path() -> String {
    utf8_to_string(ZPOOL_CONFIG_PHYS_PATH)
}

pub fn zpool_config_is_log() -> String {
    utf8_to_string(ZPOOL_CONFIG_IS_LOG)
}

pub fn zpool_config_whole_disk() -> String {
    utf8_to_string(ZPOOL_CONFIG_WHOLE_DISK)
}

pub fn zpool_config_hostid() -> String {
    utf8_to_string(ZPOOL_CONFIG_HOSTID)
}

pub fn zpool_config_hostname() -> String {
    utf8_to_string(ZPOOL_CONFIG_HOSTNAME)
}

pub fn zpool_config_guid() -> String {
    utf8_to_string(ZPOOL_CONFIG_GUID)
}

pub fn zprop_value() -> String {
    utf8_to_string(ZPROP_VALUE)
}

pub fn zpool_config_vdev_stats() -> String {
    utf8_to_string(ZPOOL_CONFIG_VDEV_STATS)
}

pub fn zfs_type_dataset() -> zfs_type_t {
    zfs_type_t::ZFS_TYPE_FILESYSTEM | zfs_type_t::ZFS_TYPE_VOLUME | zfs_type_t::ZFS_TYPE_SNAPSHOT
}

/// Converts a `Vec<u64>` to `vdev_stat_t`
pub fn to_vdev_stat(mut xs: Vec<u64>) -> vdev_stat_t {
    xs.shrink_to_fit();

    unsafe { std::ptr::read(xs.as_ptr() as *const _) }
}

/// Converts a `u32` to `Option<vdev_state_t>`
pub fn to_vdev_state(n: u32) -> Option<vdev_state_t> {
    if n <= 7 {
        Some(unsafe { std::mem::transmute(n) })
    } else {
        None
    }
}

/// Converts an `i32` to `Option<zfs_prop_t>`
pub fn to_zfs_prop_t(n: i32) -> Option<zfs_prop_t> {
    if n >= -1 && n <= 82 {
        Some(unsafe { std::mem::transmute(n) })
    } else {
        None
    }
}

/// Converts a `u32` to `Option<vdev_aux_t>`
pub fn to_vdev_aux(n: u32) -> Option<vdev_aux_t> {
    if n <= 18 {
        Some(unsafe { std::mem::transmute(n) })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString};
    use std::mem;
    use std::os::raw::{c_int, c_void};
    use std::ptr;

    fn create_libzfs_handle() -> *mut libzfs_handle_t {
        unsafe { libzfs_init() }
    }

    fn destroy_libzfs_handle(h: *mut libzfs_handle_t) {
        unsafe { libzfs_fini(h) }
    }

    fn imported_pools(h: *mut libzfs_handle_t) -> Vec<String> {
        unsafe extern "C" fn callback(handle: *mut zpool_handle_t, state: *mut c_void) -> c_int {
            let s = CStr::from_ptr((*handle).zpool_name.as_ptr());
            let s = s.to_owned().into_string().unwrap();

            let state = &mut *(state as *mut Vec<String>);
            state.push(s);

            zpool_close(handle);

            0
        }

        let mut state: Vec<String> = Vec::new();
        let state_ptr: *mut c_void = &mut state as *mut _ as *mut c_void;

        let code = unsafe { zpool_iter(h, Some(callback), state_ptr) };

        assert_eq!(code, 0);

        state
    }

    fn open_pool_canfail(h: *mut libzfs_handle_t, name: &str) -> *mut zpool_handle_t {
        unsafe {
            let pool_name = CString::new(name).unwrap();

            zpool_open_canfail(h, pool_name.as_ptr())
        }
    }

    fn export_pool(zpool_h: *mut zpool_handle_t) {
        let r = unsafe { zpool_export(zpool_h, boolean::B_FALSE, ptr::null_mut()) };

        assert_eq!(r, 0);
    }

    #[test]
    fn open_close_handle() {
        let h = create_libzfs_handle();
        destroy_libzfs_handle(h);
    }

    #[test]
    fn pool_search_import_list_export() {
        let h = create_libzfs_handle();

        // Pool may have been imported elsewhere.
        // Make sure we export first.
        if imported_pools(h).len() > 0 {
            let zpool_h = open_pool_canfail(h, "test");

            export_pool(zpool_h);
        }

        let (nvl, nvp) = unsafe {
            thread_init();
            let nvl = zpool_find_import(h, 0, ptr::null_mut());
            thread_fini();

            let nvp = nvlist_next_nvpair(nvl, ptr::null_mut());

            (nvl, nvp)
        };

        let name = unsafe {
            CStr::from_ptr(nvpair_name(nvp))
                .to_owned()
                .into_string()
                .unwrap()
        };
        assert_eq!(name, "test");

        let mut config = ptr::null_mut();
        let mut elem = ptr::null_mut();

        unsafe {
            loop {
                elem = nvlist_next_nvpair(nvl, elem);

                if elem == ptr::null_mut() {
                    break;
                }

                assert_eq!(nvpair_value_nvlist(elem, &mut config), 0);
            }
        }

        let code = unsafe { zpool_import(h, config, ptr::null(), ptr::null_mut()) };
        assert_eq!(code, 0);

        unsafe { nvlist_free(nvl) };

        let state = imported_pools(h);

        assert_eq!(state, vec!["test"]);

        let zpool_h = open_pool_canfail(h, "test");

        let ds_h = unsafe {
            let dsName = CString::new("test/ds").unwrap();

            let zfs_type_t(zfs_type) = zfs_type_dataset();

            zfs_open(h, dsName.as_ptr(), zfs_type as ::std::os::raw::c_int)
        };

        unsafe {
            let mut prop_list_ptr: *mut zprop_list_t = std::ptr::null_mut();

            let code =
                zfs_expand_proplist(ds_h, &mut prop_list_ptr, boolean::B_TRUE, boolean::B_TRUE);

            assert_eq!(code, 0);

            let user_props = zfs_get_user_props(ds_h);

            let mut pl_p = prop_list_ptr;

            while !pl_p.is_null() {
                let zfs_prop = to_zfs_prop_t((*pl_p).pl_prop).unwrap();

                if zfs_prop != zfs_prop_t_ZFS_PROP_BAD {
                    let raw = CString::new("0".repeat(319)).unwrap().into_raw();

                    let ret = zfs_prop_get(
                        ds_h,
                        zfs_prop,
                        raw,
                        319,
                        ptr::null_mut(),
                        ptr::null_mut(),
                        0,
                        boolean::B_TRUE,
                    );

                    let _ = CString::from_raw(raw);

                    if ret != 0 {
                        pl_p = (*pl_p).pl_next;
                        continue;
                    }
                } else {
                    let mut nv = ptr::null_mut();

                    let ret = nvlist_lookup_nvlist(user_props, (*pl_p).pl_user_prop, &mut nv);

                    assert_eq!(ret, 0);

                    let mut n = mem::uninitialized();

                    let v = CStr::from_bytes_with_nul(ZPROP_VALUE).unwrap();

                    let r2 = nvlist_lookup_string(nv as *mut _, v.as_ptr(), &mut n);

                    assert_eq!(r2, 0);
                }

                pl_p = (*pl_p).pl_next;
            }

            zprop_free_list(prop_list_ptr);
        };

        export_pool(zpool_h);

        destroy_libzfs_handle(h);
    }
}
