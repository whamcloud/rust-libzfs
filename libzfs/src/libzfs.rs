// Copyright (c) 2018 DDN. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

extern crate libzfs_sys as sys;

use libzfs_error::{LibZfsError, Result};
use nvpair;
use nvpair::ForeignType;
use std::ffi::CString;
use std::io::Error;
use std::os::raw::{c_int, c_void};
use std::ptr;
use std::sync::Mutex;
use zfs::Zfs;
use zpool::Zpool;

lazy_static! {
    pub static ref LOCK: Mutex<()> = Mutex::new(());
}

pub struct Libzfs {
    raw: *mut sys::libzfs_handle_t,
}

impl Default for Libzfs {
    fn default() -> Self {
        Libzfs::new()
    }
}

impl Libzfs {
    pub fn new() -> Libzfs {
        Libzfs {
            raw: unsafe { sys::libzfs_init() },
        }
    }
    pub fn pool_by_name(&mut self, name: &str) -> Option<Zpool> {
        unsafe {
            let pool_name = CString::new(name).unwrap();

            let pool = sys::zpool_open_canfail(self.raw, pool_name.as_ptr());

            if pool.is_null() {
                None
            } else {
                Some(Zpool::new(pool))
            }
        }
    }
    pub fn dataset_by_name(&mut self, name: &str) -> Option<Zfs> {
        unsafe {
            let x = CString::new(name).unwrap();
            let name = x.into_raw();

            let ds = sys::zfs_path_to_zhandle(self.raw, name, sys::zfs_type_dataset());
            let _ = CString::from_raw(name);

            if ds.is_null() {
                None
            } else {
                Some(Zfs::new(ds))
            }
        }
    }
    pub fn find_importable_pools(&mut self) -> nvpair::NvList {
        let _l = LOCK.lock().unwrap();
        unsafe {
            sys::thread_init();
            let x = sys::zpool_find_import(self.raw, 0, ptr::null_mut());
            sys::thread_fini();

            nvpair::NvList::from_ptr(x)
        }
    }
    pub fn import_all(&mut self, nvl: &nvpair::NvList) -> Result<Vec<()>> {
        nvl.iter()
            .map(|x| {
                let nvl2 = x.value_nv_list()?;

                let code = unsafe {
                    sys::zpool_import(
                        self.raw,
                        nvl2.as_ptr() as *mut _,
                        ptr::null(),
                        ptr::null_mut(),
                    )
                };

                match code {
                    0 => Ok(()),
                    x => Err(LibZfsError::Io(Error::from_raw_os_error(x))),
                }
            }).collect()
    }
    pub fn export_all(&mut self, pools: &[Zpool]) -> Result<Vec<()>> {
        pools
            .iter()
            .map(|x| x.disable_datasets().and_then(|_| x.export()))
            .collect()
    }
    pub fn get_imported_pools(&mut self) -> Result<Vec<Zpool>> {
        unsafe extern "C" fn callback(
            handle: *mut sys::zpool_handle_t,
            state: *mut c_void,
        ) -> c_int {
            let state = &mut *(state as *mut Vec<Zpool>);

            state.push(Zpool::new(handle));

            0
        }
        let mut state: Vec<Zpool> = Vec::new();
        let state_ptr: *mut c_void = &mut state as *mut _ as *mut c_void;
        let code = unsafe { sys::zpool_iter(self.raw, Some(callback), state_ptr) };

        match code {
            0 => Ok(state),
            x => Err(LibZfsError::Io(Error::from_raw_os_error(x))),
        }
    }
}

impl Drop for Libzfs {
    fn drop(&mut self) {
        unsafe { sys::libzfs_fini(self.raw) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_close_handle() {
        Libzfs::new();
    }
}
