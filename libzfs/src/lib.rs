// Copyright (c) 2017 Intel Corporation. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

extern crate libzfs_sys as sys;
// extern crate nvpair;
use std::os::raw::{c_int, c_void};
use std::ffi::{CStr, CString, IntoStringError};
use std::{error, fmt, ptr, result, str};
use std::io::{Error, ErrorKind};
use nvpair::ForeignType;

#[macro_use]
extern crate foreign_types;
mod nvpair;

#[macro_use]
extern crate serde_derive;

#[derive(Debug)]
pub enum LibZfsError {
    Io(::std::io::Error),
    IntoString(IntoStringError),
}

impl fmt::Display for LibZfsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LibZfsError::Io(ref err) => write!(f, "{}", err),
            LibZfsError::IntoString(ref err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for LibZfsError {
    fn description(&self) -> &str {
        match *self {
            LibZfsError::Io(ref err) => err.description(),
            LibZfsError::IntoString(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            LibZfsError::Io(ref err) => Some(err),
            LibZfsError::IntoString(ref err) => Some(err),
        }
    }
}

impl From<Error> for LibZfsError {
    fn from(err: Error) -> Self {
        LibZfsError::Io(err)
    }
}

impl From<IntoStringError> for LibZfsError {
    fn from(err: IntoStringError) -> Self {
        LibZfsError::IntoString(err)
    }
}

pub type Result<T> = result::Result<T, LibZfsError>;

/// Represents vdevs
/// The enum starts at Root and is recursive.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
        guid: Option<String>,
        state: String,
        path: String,
        dev_id: Option<String>,
        phys_path: Option<String>,
        whole_disk: Option<bool>,
        is_log: Option<bool>,
    },
    File { path: String },
}

#[derive(Debug, PartialEq)]
pub struct Zfs {
    raw: *mut sys::zfs_handle_t,
}

impl Zfs {
    pub fn name(&self) -> CString {
        let s = unsafe { CStr::from_ptr(sys::zfs_get_name(self.raw)) };
        s.to_owned()
    }
    pub fn user_props(&self) -> &mut nvpair::NvListRef {
        unsafe {
            let x = sys::zfs_get_user_props(self.raw);
            nvpair::NvListRef::from_mut_ptr(x)
        }
    }
    pub fn props(&self) -> &mut nvpair::NvListRef {
        unsafe {
            let x = (*self.raw).zfs_props;
            nvpair::NvListRef::from_mut_ptr(x)
        }
    }
    pub fn zfs_type(&self) -> sys::zfs_type_t {
        unsafe { sys::zfs_get_type(self.raw) }
    }
    pub fn zfs_type_name(&self) -> CString {
        let x = self.zfs_type();

        let s = unsafe { CStr::from_ptr(sys::zfs_type_to_name(x)) };

        s.to_owned()
    }
    pub fn lookup_string_prop(&self, name: &str) -> Option<String> {
        let props: Result<String> = self.props()
            .lookup_nv_list(name)
            .map_err(LibZfsError::from)
            .and_then(|x| {
                x.lookup_string(sys::zfs_value()).map_err(LibZfsError::from)
            })
            .and_then(|x| x.into_string().map_err(LibZfsError::from));

        props.ok()
    }
    pub fn lookup_uint64_prop(&self, name: &str) -> Option<u64> {
        let props: Result<u64> = self.props()
            .lookup_nv_list(name)
            .map_err(LibZfsError::from)
            .and_then(|x| {
                x.lookup_uint64(sys::zfs_value()).map_err(LibZfsError::from)
            });

        props.ok()
    }
}

impl Drop for Zfs {
    fn drop(&mut self) {
        unsafe { sys::zfs_close(self.raw) }
    }
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
                sys::boolean_B_FALSE,
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
        format!("{:#x}", self.guid())
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
            let x = sys::zfs_open(h, name, zfs_type as ::std::os::raw::c_int);
            let _ = CString::from_raw(name);
            assert!(!x.is_null(), "zfs_handle_t is null");
            x
        };

        let ds = Zfs { raw: x };

        unsafe extern "C" fn callback(handle: *mut sys::zfs_handle_t, state: *mut c_void) -> c_int {
            let state = &mut *(state as *mut Vec<Zfs>);

            state.push(Zfs { raw: handle });

            0
        }

        let mut state: Vec<Zfs> = Vec::new();
        let state_ptr: *mut c_void = &mut state as *mut _ as *mut c_void;
        let code = unsafe { sys::zfs_iter_filesystems(ds.raw, Some(callback), state_ptr) };

        match code {
            0 => Ok(state),
            x => Err(LibZfsError::Io(Error::from_raw_os_error(x))),
        }
    }
    pub fn disable_datasets(&self) -> Result<()> {
        let code = unsafe { sys::zpool_disable_datasets(self.raw, sys::boolean_B_FALSE) };

        match code {
            0 => Ok(()),
            e => Err(LibZfsError::Io(Error::from_raw_os_error(e))),
        }
    }
    pub fn export(&self) -> Result<()> {

        let code = unsafe { sys::zpool_export(self.raw, sys::boolean_B_FALSE, ptr::null_mut()) };

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

pub fn enumerate_vdev_tree(tree: &nvpair::NvList) -> Result<VDev> {
    let tmp = tree.lookup_string(sys::zpool_config_type())?;
    let x = tmp.as_bytes_with_nul();

    fn get_children(tree: &nvpair::NvList) -> Result<Vec<VDev>> {
        tree.lookup_nv_list_array(sys::zpool_config_children())?
            .iter()
            .map(enumerate_vdev_tree)
            .collect()
    }

    fn lookup_tree_str(tree: &nvpair::NvList, name: String) -> Result<Option<String>> {
        let x = tree.lookup_string(name);

        match x {
            Ok(x) => Ok(Some(x.into_string()?)),
            Err(_) => Ok(None),
        }
    }

    match x {
        x if x == sys::VDEV_TYPE_DISK => {
            let guid = tree.lookup_uint64(sys::zpool_config_guid())
                .map(|x| format!("{:#x}", x))
                .ok();
            let path = tree.lookup_string(sys::zpool_config_path())?.into_string()?;
            let dev_id = lookup_tree_str(tree, sys::zpool_config_dev_id())?;
            let phys_path = lookup_tree_str(tree, sys::zpool_config_phys_path())?;
            let is_log = tree.lookup_uint64(sys::zpool_config_is_log())
                .map(|x| x == 1)
                .ok();
            let whole_disk = tree.lookup_uint64(sys::zpool_config_whole_disk())
                .map(|x| x == 1)
                .ok();

            let vdev_stats = tree.lookup_uint64_array(sys::zpool_config_vdev_stats())
                .map(sys::to_vdev_stat)?;

            let state = unsafe {
                let s = sys::zpool_state_to_name(
                    sys::to_vdev_state(vdev_stats.vs_state as u32).ok_or(
                        Error::new(
                            ErrorKind::NotFound,
                            "vs_state not in enum range",
                        ),
                    )?,
                    sys::to_vdev_aux(vdev_stats.vs_aux as u32).ok_or(
                        Error::new(
                            ErrorKind::NotFound,
                            "vs_aux not in enum range",
                        ),
                    )?,
                );

                CStr::from_ptr(s)
            };

            Ok(VDev::Disk {
                guid,
                state: state.to_owned().into_string()?,
                path,
                dev_id,
                phys_path,
                whole_disk,
                is_log,
            })
        }
        x if x == sys::VDEV_TYPE_FILE => {
            let path = tree.lookup_string(sys::zpool_config_path())?.into_string()?;

            Ok(VDev::File { path })
        }
        x if x == sys::VDEV_TYPE_MIRROR => {
            let children = get_children(tree)?;
            let is_log = tree.lookup_uint64(sys::zpool_config_is_log())
                .map(|x| x == 1)
                .ok();

            Ok(VDev::Mirror { children, is_log })
        }
        x if x == sys::VDEV_TYPE_RAIDZ => {
            let children = get_children(tree)?;

            Ok(VDev::RaidZ { children })
        }
        x if x == sys::VDEV_TYPE_REPLACING => {
            let children = get_children(tree)?;

            Ok(VDev::Replacing { children })
        }
        x if x == sys::VDEV_TYPE_SPARE => {
            let children = get_children(tree)?;

            Ok(VDev::Spare { children })
        }
        x if x == sys::VDEV_TYPE_ROOT => {
            let children = get_children(tree)?;

            Ok(VDev::Root { children })
        }
        _ => Err(LibZfsError::Io(
            Error::new(ErrorKind::NotFound, "hit unknown vdev type"),
        )),
    }
}

pub struct Libzfs {
    raw: *mut sys::libzfs_handle_t,
}

impl Libzfs {
    pub fn new() -> Libzfs {
        Libzfs { raw: unsafe { sys::libzfs_init() } }
    }
    pub fn pool_by_name(&mut self, name: &str) -> Option<Zpool> {
        unsafe {
            let pool_name = CString::new(name).unwrap();

            let pool = sys::zpool_open_canfail(self.raw, pool_name.as_ptr());

            if pool.is_null() {
                None
            } else {
                Some(Zpool { raw: pool })
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
                Some(Zfs { raw: ds })
            }
        }
    }
    pub fn find_importable_pools(&mut self) -> nvpair::NvList {
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
            })
            .collect()
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

            state.push(Zpool { raw: handle });

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

    #[test]
    fn import_check_methods_export() {
        let mut z = Libzfs::new();

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

        assert_eq!(test_pool.health().unwrap(), CString::new("ONLINE").unwrap());

        assert_eq!(test_pool.state_name(), CString::new("ACTIVE").unwrap());

        assert_eq!(test_pool.size(), 532575944704);

        assert_eq!(test_pool.read_only(), false);

        assert_eq!(
            test_pool.hostname().unwrap(),
            CString::new("localhost.localdomain").unwrap()
        );

        let disks = match test_pool.vdev_tree().expect("could not fetch vdev tree") {
            VDev::Root { children } => children,
            _ => panic!("did not find root device"),
        };

        let (whole_disk, state) = match disks[0] {
            VDev::Disk {
                whole_disk,
                ref state,
                ..
            } => (whole_disk, state),
            _ => panic!("did not find disk"),
        };

        assert_eq!(whole_disk, Some(true));

        assert_eq!(state, "ONLINE");

        let datasets = test_pool.datasets().expect("could not fetch datasets");

        let test_dataset = datasets
            .iter()
            .find(|x| x.name() == CString::new("test/ds").unwrap())
            .expect("did not find test dataset");

        assert_eq!(
            test_dataset.zfs_type_name(),
            CString::new("filesystem").unwrap()
        );

        z.export_all(&imported_pools).expect(
            "could not export pools",
        );
    }
}
