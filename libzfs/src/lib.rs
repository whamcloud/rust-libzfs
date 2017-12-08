// Copyright (c) 2017 Intel Corporation. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

extern crate libzfs_sys as sys;
extern crate nvpair;
use std::ptr;
use std::os::raw::{c_int, c_void};
use std::ffi::{CStr, CString};
use std::str;
use std::io;
use std::io::{Error, ErrorKind};

#[derive(Debug, PartialEq)]
pub enum VDev {
    Mirror {
        children: Vec<VDev>,
        is_log: Option<bool>,
    },
    RaidZ { children: Vec<VDev> },
    Replacing { children: Vec<VDev> },
    Spare { children: Vec<VDev> },
    Root { children: Vec<VDev> },
    Disk {
        path: CString,
        devid: Option<CString>,
        phys_path: Option<CString>,
        whole_disk: Option<bool>,
        is_log: Option<bool>,
    },
    File { path: CString },
}

#[derive(Debug, PartialEq)]
pub struct Pool {
    pub name: CString,
    pub state: sys::pool_state_t,
    pub pool_guid: String,
    pub vdev_tree: VDev,
    pub read_only: bool,
}

#[derive(Debug, PartialEq)]
pub struct Dataset {
    raw: *mut sys::zfs_handle_t,
}

impl Dataset {
    pub fn name(&self) -> CString {
        let s = unsafe { CStr::from_ptr(sys::zfs_get_name(self.raw)) };
        s.to_owned()
    }
    pub fn user_props(&self) -> nvpair::NvList {
        unsafe {
            let x = sys::zfs_get_user_props(self.raw);
            nvpair::NvList::from_ptr(x, false)
        }
    }
}

impl Drop for Dataset {
    fn drop(&mut self) {
        unsafe { sys::zfs_close(self.raw) }
    }
}

fn utf8_to_str(bytes: &[u8]) -> &str {
    str::from_utf8(bytes).unwrap()
}

#[derive(Debug, PartialEq)]
pub struct Zpool {
    raw: *mut sys::zpool_handle_t,
}

impl Zpool {
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
    pub fn guid(&self) -> u64 {
        self.prop_int(sys::zpool_prop_t::ZPOOL_PROP_GUID)
    }
    pub fn guid_hex(&self) -> String {
        format!("{:#x}", self.guid())
    }
    pub fn size(&self) -> u64 {
        self.prop_int(sys::zpool_prop_t::ZPOOL_PROP_SIZE)
    }
    pub fn read_only(&self) -> bool {
        self.prop_int(sys::zpool_prop_t::ZPOOL_PROP_READONLY) != 0
    }
    pub fn get_config(&self) -> nvpair::NvList {
        unsafe {
            let x = sys::zpool_get_config(self.raw, ptr::null_mut());
            assert!(!x.is_null(), "config pointer is null");
            nvpair::NvList::from_ptr(x, false)
        }
    }
    pub fn vdev_tree(&self) -> io::Result<VDev> {
        let config = self.get_config();

        let zpool_config_vdev_tree = utf8_to_str(sys::ZPOOL_CONFIG_VDEV_TREE);

        let tree = config.lookup_nv_list(zpool_config_vdev_tree);

        tree.and_then(|ref x| enumerate_vdev_tree(x))
    }
    pub fn datasets(&self) -> io::Result<Vec<Dataset>> {
        let x = unsafe {
            let name = self.name().into_raw();
            let h = sys::zpool_get_handle(self.raw);
            let x = sys::zfs_open(h, name, sys::zfs_type_dataset());
            let _ = CString::from_raw(name);
            assert!(!x.is_null(), "zfs_handle_t is null");
            x
        };

        let ds = Dataset { raw: x };

        unsafe extern "C" fn callback(handle: *mut sys::zfs_handle_t, state: *mut c_void) -> c_int {
            let state = &mut *(state as *mut Vec<Dataset>);

            state.push(Dataset { raw: handle });

            0
        }

        let mut state: Vec<Dataset> = Vec::new();
        let state_ptr: *mut c_void = &mut state as *mut _ as *mut c_void;
        let code = unsafe { sys::zfs_iter_filesystems(ds.raw, Some(callback), state_ptr) };

        match code {
            0 => Ok(state),
            x => Err(io::Error::from_raw_os_error(x)),
        }
    }
}

pub fn enumerate_vdev_tree(tree: &nvpair::NvList) -> io::Result<VDev> {
    let zpool_config_type = utf8_to_str(sys::ZPOOL_CONFIG_TYPE);
    let tmp = tree.lookup_string(zpool_config_type)?;
    let x = tmp.as_bytes_with_nul();

    fn get_children(tree: &nvpair::NvList) -> io::Result<Vec<VDev>> {
        let zpool_config_children = utf8_to_str(sys::ZPOOL_CONFIG_CHILDREN);

        tree.lookup_nv_list_array(zpool_config_children)?
            .iter()
            .map(|x| enumerate_vdev_tree(x))
            .collect()
    }

    match x {
        ref x if x == sys::VDEV_TYPE_DISK => {
            let path = tree.lookup_string(utf8_to_str(sys::ZPOOL_CONFIG_PATH))?;
            let devid = tree.lookup_string(utf8_to_str(sys::ZPOOL_CONFIG_DEVID))
                .ok();
            let phys_path = tree.lookup_string(utf8_to_str(sys::ZPOOL_CONFIG_PHYS_PATH))
                .ok();
            let is_log = tree.lookup_uint64(utf8_to_str(sys::ZPOOL_CONFIG_IS_LOG))
                .map(|x| x == 1)
                .ok();
            let whole_disk = tree.lookup_uint64(utf8_to_str(sys::ZPOOL_CONFIG_WHOLE_DISK))
                .ok()
                .map(|x| x == 1);

            Ok(VDev::Disk {
                path,
                devid,
                phys_path,
                whole_disk,
                is_log,
            })
        }
        ref x if x == sys::VDEV_TYPE_FILE => {
            let path = tree.lookup_string("path")?;

            Ok(VDev::File { path })
        }
        ref x if x == sys::VDEV_TYPE_MIRROR => {
            let children = get_children(tree)?;
            let is_log = tree.lookup_uint64(utf8_to_str(sys::ZPOOL_CONFIG_IS_LOG))
                .map(|x| x == 1)
                .ok();

            Ok(VDev::Mirror { children, is_log })
        }
        ref x if x == sys::VDEV_TYPE_RAIDZ => {
            let children = get_children(tree)?;

            Ok(VDev::RaidZ { children })
        }
        ref x if x == sys::VDEV_TYPE_REPLACING => {
            let children = get_children(tree)?;

            Ok(VDev::Replacing { children })
        }
        ref x if x == sys::VDEV_TYPE_SPARE => {
            let children = get_children(tree)?;

            Ok(VDev::Spare { children })
        }
        ref x if x == sys::VDEV_TYPE_ROOT => {
            let children = get_children(tree)?;

            Ok(VDev::Root { children })
        }
        _ => Err(Error::new(ErrorKind::NotFound, "hit unknown vdev type")),
    }
}

impl Drop for Zpool {
    fn drop(&mut self) {
        unsafe { sys::zpool_close(self.raw) }
    }
}

pub struct Zfs {
    raw: *mut sys::libzfs_handle_t,
}

impl Zfs {
    pub fn new() -> Zfs {
        Zfs { raw: unsafe { sys::libzfs_init() } }
    }
    pub fn find_importable_pools(&mut self) -> nvpair::NvList {
        unsafe {
            sys::thread_init();
            let x = sys::zpool_find_import(self.raw, 0, ptr::null_mut());
            sys::thread_fini();

            nvpair::NvList::from_ptr(x, true)
        }
    }
    pub fn import_all(&mut self, nvl: &nvpair::NvList) -> io::Result<Vec<()>> {
        nvl.iter()
            .map(|x| {
                let nvl2 = x.value_nv_list()?;

                let code = unsafe {
                    sys::zpool_import(self.raw, nvl2.as_ptr(), ptr::null(), ptr::null_mut())
                };

                match code {
                    0 => Ok(()),
                    x => Err(io::Error::from_raw_os_error(x)),
                }
            })
            .collect()
    }
    pub fn export_all(&mut self, pools: &Vec<Zpool>) -> io::Result<Vec<()>> {
        pools
            .iter()
            .map(|x| {
                let code =
                    unsafe { sys::zpool_export(x.raw, sys::boolean_B_FALSE, ptr::null_mut()) };

                match code {
                    0 => Ok(()),
                    x => Err(io::Error::from_raw_os_error(x)),
                }
            })
            .collect()
    }
    pub fn get_imported_pools(&mut self) -> io::Result<Vec<Zpool>> {
        unsafe extern "C" fn callback(
            handle: *mut sys::zpool_handle_t,
            state: *mut c_void,
        ) -> c_int {
            let state = &mut *(state as *mut Vec<Zpool>);

            state.push(Zpool { raw: handle });

            0
        }
        let mut state: Vec<Zpool> = Vec::new();
        let state_ptr: *mut c_void = &mut state as *mut _ as *mut c_void;
        let code = unsafe { sys::zpool_iter(self.raw, Some(callback), state_ptr) };

        match code {
            0 => Ok(state),
            x => Err(io::Error::from_raw_os_error(x)),
        }
    }
}

impl Drop for Zfs {
    fn drop(&mut self) {
        unsafe { sys::libzfs_fini(self.raw) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_close_handle() {
        Zfs::new();
    }

    #[test]
    fn import_check_methods_export() {
        let mut z = Zfs::new();

        let pools_to_import = z.find_importable_pools();

        z.import_all(&pools_to_import).expect(
            "could not import pools",
        );

        let imported_pools = z.get_imported_pools().expect(
            "could not get imported pools",
        );

        let test_pool = imported_pools
            .iter()
            .find(|x| x.name() == CString::new("test").unwrap())
            .expect("did not find test pool");

        assert_eq!(test_pool.state_name(), CString::new("ACTIVE").unwrap());

        assert_eq!(test_pool.size(), 532575944704);

        assert_eq!(test_pool.read_only(), false);

        let disks = match test_pool.vdev_tree().expect("could not fetch vdev tree") {
            VDev::Root { children } => children,
            _ => panic!("did not find root device"),
        };

        let whole_disk = match disks[0] {
            VDev::Disk { whole_disk, .. } => whole_disk,
            _ => panic!("did not find disk"),
        };

        assert_eq!(whole_disk, Some(true));

        z.export_all(&imported_pools).expect(
            "could not export pools",
        );
    }
}
