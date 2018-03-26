// Copyright (c) 2018 Intel Corporation. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

extern crate libzfs_sys as sys;

use std::ptr;
use nvpair;
use zfs::Zfs;
use std::io::Error;
use std::os::raw::{c_int, c_void};
use std::ffi::{CStr, CString};
use libzfs_error::{LibZfsError, Result};
use vdev::{enumerate_vdev_tree, VDev};

#[derive(Debug, PartialEq)]
pub struct Zpool {
    raw: *mut sys::zpool_handle_t,
}

impl Zpool {
    pub fn new(raw: *mut sys::zpool_handle_t) -> Zpool {
        Zpool { raw }
    }
    pub fn name(&self) -> CString {
        let s = unsafe { CStr::from_ptr(sys::zpool_get_name(self.raw)) };
        s.to_owned()
    }
    pub fn state(&self) -> sys::pool_state_t {
        let state = unsafe { sys::zpool_get_state(self.raw) };
        state as sys::pool_state_t
    }
    pub fn state_name(&self) -> CString {
        let state = self.state();

        let name = unsafe {
            let x = sys::zpool_pool_state_to_name(state);

            CStr::from_ptr(x)
        };

        name.to_owned()
    }
    pub fn prop_int(&self, prop: sys::zpool_prop_t::Type) -> u64 {
        unsafe { sys::zpool_get_prop_int(self.raw, prop, ptr::null_mut()) }
    }
    pub fn prop_str(&self, prop: sys::zpool_prop_t::Type) -> Result<CString> {
        let s = String::with_capacity(sys::ZPOOL_MAXPROPLEN as usize);
        let c_string = CString::new(s).unwrap();
        let raw = c_string.into_raw();

        unsafe {
            let r = sys::zpool_get_prop(
                self.raw,
                prop,
                raw,
                sys::ZPOOL_MAXPROPLEN as usize,
                ptr::null_mut(),
                sys::boolean::B_FALSE,
            );

            let out = CString::from_raw(raw);

            if r != 0 {
                Err(::std::io::Error::from_raw_os_error(r))?
            } else {
                Ok(out)
            }
        }
    }
    pub fn health(&self) -> Result<CString> {
        self.prop_str(sys::zpool_prop_t::ZPOOL_PROP_HEALTH)
    }
    pub fn hostname(&self) -> Result<CString> {
        let config = self.get_config();

        let s = config.lookup_string(sys::zpool_config_hostname())?;

        Ok(s)
    }
    pub fn hostid(&self) -> Result<u64> {
        let s = self.get_config().lookup_uint64(sys::zpool_config_hostid())?;

        Ok(s)
    }
    pub fn guid(&self) -> u64 {
        self.prop_int(sys::zpool_prop_t::ZPOOL_PROP_GUID)
    }
    pub fn guid_hex(&self) -> String {
        format!("{:#018X}", self.guid())
    }
    pub fn size(&self) -> u64 {
        self.prop_int(sys::zpool_prop_t::ZPOOL_PROP_SIZE)
    }
    pub fn read_only(&self) -> bool {
        self.prop_int(sys::zpool_prop_t::ZPOOL_PROP_READONLY) != 0
    }
    pub fn get_config(&self) -> &mut nvpair::NvListRef {
        unsafe {
            let x = sys::zpool_get_config(self.raw, ptr::null_mut());
            assert!(!x.is_null(), "config pointer is null");
            nvpair::NvListRef::from_mut_ptr(x)
        }
    }
    pub fn vdev_tree(&self) -> Result<VDev> {
        let config = self.get_config();

        let tree = config.lookup_nv_list(sys::zpool_config_vdev_tree())?;

        enumerate_vdev_tree(&tree)
    }
    pub fn datasets(&self) -> Result<Vec<Zfs>> {
        let sys::zfs_type_t(zfs_type) = sys::zfs_type_dataset();

        let x = unsafe {
            let name = self.name().into_raw();
            let h = sys::zpool_get_handle(self.raw);
            let x = sys::zfs_open(h, name, zfs_type as c_int);
            let _ = CString::from_raw(name);
            assert!(!x.is_null(), "zfs_handle_t is null");
            x
        };

        let _ds = Zfs::new(x);

        unsafe extern "C" fn callback(handle: *mut sys::zfs_handle_t, state: *mut c_void) -> c_int {
            let state = &mut *(state as *mut Vec<Zfs>);

            state.push(Zfs::new(handle));

            0
        }

        let mut state: Vec<Zfs> = Vec::new();
        let state_ptr: *mut c_void = &mut state as *mut _ as *mut c_void;
        let code = unsafe { sys::zfs_iter_filesystems(x, Some(callback), state_ptr) };

        match code {
            0 => Ok(state),
            x => Err(LibZfsError::Io(Error::from_raw_os_error(x))),
        }
    }
    pub fn disable_datasets(&self) -> Result<()> {
        let code = unsafe { sys::zpool_disable_datasets(self.raw, sys::boolean::B_FALSE) };

        match code {
            0 => Ok(()),
            e => Err(LibZfsError::Io(Error::from_raw_os_error(e))),
        }
    }
    pub fn export(&self) -> Result<()> {
        let code = unsafe { sys::zpool_export(self.raw, sys::boolean::B_FALSE, ptr::null_mut()) };

        match code {
            0 => Ok(()),
            e => Err(LibZfsError::Io(Error::from_raw_os_error(e))),
        }
    }
}

impl Drop for Zpool {
    fn drop(&mut self) {
        unsafe { sys::zpool_close(self.raw) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    use std::str;
    use libzfs::Libzfs;
    use std::ffi::CString;

    fn test_pools<F: Fn(&Vec<Zpool>) -> ()>(f: F) -> ()
    where
        F: panic::RefUnwindSafe,
    {
        let mut z = Libzfs::new();

        let pools_to_import = z.find_importable_pools();

        z.import_all(&pools_to_import)
            .expect("could not import pools");

        let pools = z.get_imported_pools()
            .expect("could not fetch imported pools");

        let result = panic::catch_unwind(|| {
            f(&pools);
        });

        result.unwrap();
    }

    fn pool_by_name<F: Fn(&Zpool) -> ()>(name: &str, f: F) -> ()
    where
        F: panic::RefUnwindSafe,
    {
        test_pools(|xs| {
            let x = xs.iter()
                .find(|x| x.name() == CString::new(name).unwrap())
                .expect("did not find test pool");

            f(x);
        });
    }

    #[test]
    fn import_get_pool_len() {
        test_pools(|xs| assert_eq!(xs.len(), 1));
    }

    #[test]
    fn get_pool_health() {
        pool_by_name("test", |p| {
            assert_eq!(
                p.health().expect("could not fetch pool health"),
                CString::new("ONLINE").unwrap()
            )
        })
    }

    #[test]
    fn get_pool_state() {
        pool_by_name("test", |p| {
            assert_eq!(p.state_name(), CString::new("ACTIVE").unwrap())
        })
    }

    #[test]
    fn get_pool_size() {
        pool_by_name("test", |p| assert_eq!(p.size(), 83886080))
    }

    #[test]
    fn get_pool_read_only() {
        pool_by_name("test", |p| assert_eq!(p.read_only(), false))
    }

    #[test]
    fn get_pool_hostname() {
        pool_by_name("test", |p| {
            assert_eq!(
                p.hostname().expect("could not get hostname"),
                CString::new("localhost.localdomain").unwrap()
            )
        })
    }

    #[test]
    fn get_pool_hostid() {
        pool_by_name("test", |p| assert!(p.hostid().is_ok()))
    }

    #[test]
    fn test_vdev_tree() {
        pool_by_name("test", |p| {
            let (mirror, cache_vdevs, spare_vdevs) = match p.vdev_tree().unwrap() {
                VDev::Root {
                    children,
                    cache,
                    spares,
                } => (children, cache, spares),
                _ => panic!("did not find root device"),
            };

            let mirror_vdevs = match mirror[0] {
                VDev::Mirror { ref children, .. } => children,
                _ => panic!("did not find mirror"),
            };

            match mirror_vdevs[0] {
                VDev::Disk {
                    ref guid,
                    ref state,
                    ref path,
                    ref dev_id,
                    ref phys_path,
                    whole_disk,
                    is_log,
                } => {
                    assert!(guid.is_some());
                    assert_eq!(state, "ONLINE");
                    assert_eq!(path, "/dev/sdb1");
                    assert!(dev_id.is_some());
                    assert!(phys_path.is_some());
                    assert_eq!(whole_disk, Some(true));
                    assert!(is_log.is_none());
                }
                _ => panic!("did not find disk"),
            };

            match mirror_vdevs[1] {
                VDev::Disk {
                    ref guid,
                    ref state,
                    ref path,
                    ref dev_id,
                    ref phys_path,
                    whole_disk,
                    is_log,
                } => {
                    assert!(guid.is_some());
                    assert_eq!(state, "ONLINE");
                    assert_eq!(path, "/dev/sdc1");
                    assert!(dev_id.is_some());
                    assert!(phys_path.is_some());
                    assert_eq!(whole_disk, Some(true));
                    assert!(is_log.is_none());
                }
                _ => panic!("did not find disk"),
            };

            match cache_vdevs[0] {
                VDev::Disk {
                    ref guid,
                    ref state,
                    ref path,
                    ref dev_id,
                    ref phys_path,
                    whole_disk,
                    is_log,
                } => {
                    assert!(guid.is_some());
                    assert_eq!(state, "ONLINE");
                    assert_eq!(path, "/dev/sdd1");
                    assert!(dev_id.is_some());
                    assert!(phys_path.is_some());
                    assert_eq!(whole_disk, Some(true));
                    assert!(is_log.is_none());
                }
                _ => panic!("did not find disk"),
            };

            match spare_vdevs[0] {
                VDev::Disk {
                    ref guid,
                    ref state,
                    ref path,
                    ref dev_id,
                    ref phys_path,
                    whole_disk,
                    is_log,
                } => {
                    assert!(guid.is_some());
                    assert_eq!(state, "ONLINE");
                    assert_eq!(path, "/dev/sde1");
                    assert!(dev_id.is_some());
                    assert!(phys_path.is_some());
                    assert_eq!(whole_disk, Some(true));
                    assert!(is_log.is_none());
                }
                _ => panic!("did not find disk"),
            };

            match spare_vdevs[1] {
                VDev::Disk {
                    ref guid,
                    ref state,
                    ref path,
                    ref dev_id,
                    ref phys_path,
                    whole_disk,
                    is_log,
                } => {
                    assert!(guid.is_some());
                    assert_eq!(state, "ONLINE");
                    assert_eq!(path, "/dev/sdf1");
                    assert!(dev_id.is_some());
                    assert!(phys_path.is_some());
                    assert_eq!(whole_disk, Some(true));
                    assert!(is_log.is_none());
                }
                _ => panic!("did not find disk"),
            };
        })
    }
}
