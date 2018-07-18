// Copyright (c) 2018 DDN. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

extern crate libzfs_sys as sys;

use std::io::Error;
use std::ptr;
use std::ffi::{CStr, CString};
use nvpair;
use libzfs_error::{LibZfsError, Result};
use zprop_list::{ZProp, ZpropItem, ZpropList};

#[derive(Debug, PartialEq)]
pub struct Zfs {
    raw: *mut sys::zfs_handle_t,
}

impl Zfs {
    pub fn new(raw: *mut sys::zfs_handle_t) -> Zfs {
        Zfs { raw }
    }
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
    pub fn zfs_type(&self) -> sys::zfs_type_t {
        unsafe { sys::zfs_get_type(self.raw) }
    }
    pub fn zfs_type_name(&self) -> CString {
        let x = self.zfs_type();

        let s = unsafe { CStr::from_ptr(sys::zfs_type_to_name(x)) };

        s.to_owned()
    }
    pub fn prop_list(&self) -> Result<ZpropList> {
        let mut prop_list_ptr: *mut sys::zprop_list_t = ptr::null_mut();

        let code = unsafe {
            sys::zfs_expand_proplist(
                self.raw,
                &mut prop_list_ptr,
                sys::boolean::B_TRUE,
                sys::boolean::B_TRUE,
            )
        };

        match code {
            0 => Ok(ZpropList::new(prop_list_ptr)),
            x => Err(LibZfsError::Io(Error::from_raw_os_error(x))),
        }
    }
    pub fn props(&self) -> Result<(Vec<ZProp>)> {
        let buff_size = 319;
        let pl = self.prop_list()?;

        let xs = pl.filter_map(|x: ZpropItem| match x.prop() {
            sys::zfs_prop_t_ZFS_PROP_BAD => self.user_props()
                .lookup_nv_list(x.user_prop())
                .and_then(|nv| nv.lookup_string(sys::zprop_value()))
                .map(|v| ZProp {
                    name: x.user_prop().to_owned().into_string().unwrap(),
                    value: v.into_string().unwrap(),
                })
                .ok(),
            y => {
                let raw = CString::new("0".repeat(buff_size)).unwrap().into_raw();

                let ret = unsafe {
                    sys::zfs_prop_get(
                        self.raw,
                        y,
                        raw,
                        buff_size,
                        ptr::null_mut(),
                        ptr::null_mut(),
                        0,
                        sys::boolean::B_TRUE,
                    )
                };

                let out = unsafe { CString::from_raw(raw) };

                if ret == 0 {
                    let name = unsafe { CStr::from_ptr(sys::zfs_prop_to_name(x.prop())) };

                    Some(ZProp {
                        name: name.to_string_lossy().into_owned(),
                        value: out.into_string().unwrap(),
                    })
                } else {
                    None
                }
            }
        }).collect::<Vec<_>>();

        Ok(xs)
    }
}

impl Drop for Zfs {
    fn drop(&mut self) {
        unsafe { sys::zfs_close(self.raw) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    use std::str;
    use zprop_list::ZProp;
    use libzfs::Libzfs;
    use std::ffi::CString;

    fn zfs_by_name<F: Fn(&Zfs) -> ()>(name: &str, f: F) -> ()
    where
        F: panic::RefUnwindSafe,
    {
        let mut z = Libzfs::new();

        let pools_to_import = z.find_importable_pools();

        z.import_all(&pools_to_import)
            .expect("Could not import pools");

        let ds = z.dataset_by_name(name)
            .expect("could not get dataset by name");

        let result = panic::catch_unwind(|| {
            f(&ds);
        });

        result.unwrap();
    }

    #[test]
    fn dataset_type_name() {
        zfs_by_name("test/ds", |ds| {
            assert_eq!(ds.zfs_type_name(), CString::new("filesystem").unwrap());
        })
    }

    #[test]
    fn dataset_name() {
        zfs_by_name("test/ds", |ds| {
            assert_eq!(ds.name(), CString::new("test/ds").unwrap());
        });
    }

    #[test]
    fn dataset_props() {
        zfs_by_name("test/ds", |ds| {
            let props = ds.props().unwrap();

            assert_eq!(
                props
                    .into_iter()
                    .filter(|x| ![
                        "available".to_owned(),
                        "creation".to_owned(),
                        "guid".to_owned(),
                        "createtxg".to_owned()
                    ].contains(&x.name))
                    .collect::<Vec<ZProp>>(),
                vec![
                    ZProp {
                        name: "name".to_owned(),
                        value: "test/ds".to_owned(),
                    },
                    ZProp {
                        name: "type".to_owned(),
                        value: "filesystem".to_owned(),
                    },
                    ZProp {
                        name: "used".to_owned(),
                        value: "24576".to_owned(),
                    },
                    ZProp {
                        name: "referenced".to_owned(),
                        value: "24576".to_owned(),
                    },
                    ZProp {
                        name: "compressratio".to_owned(),
                        value: "1.00x".to_owned(),
                    },
                    ZProp {
                        name: "mounted".to_owned(),
                        value: "no".to_owned(),
                    },
                    ZProp {
                        name: "quota".to_owned(),
                        value: "0".to_owned(),
                    },
                    ZProp {
                        name: "reservation".to_owned(),
                        value: "0".to_owned(),
                    },
                    ZProp {
                        name: "recordsize".to_owned(),
                        value: "131072".to_owned(),
                    },
                    ZProp {
                        name: "mountpoint".to_owned(),
                        value: "/test/ds".to_owned(),
                    },
                    ZProp {
                        name: "sharenfs".to_owned(),
                        value: "off".to_owned(),
                    },
                    ZProp {
                        name: "checksum".to_owned(),
                        value: "on".to_owned(),
                    },
                    ZProp {
                        name: "compression".to_owned(),
                        value: "off".to_owned(),
                    },
                    ZProp {
                        name: "atime".to_owned(),
                        value: "on".to_owned(),
                    },
                    ZProp {
                        name: "devices".to_owned(),
                        value: "on".to_owned(),
                    },
                    ZProp {
                        name: "exec".to_owned(),
                        value: "on".to_owned(),
                    },
                    ZProp {
                        name: "setuid".to_owned(),
                        value: "on".to_owned(),
                    },
                    ZProp {
                        name: "readonly".to_owned(),
                        value: "off".to_owned(),
                    },
                    ZProp {
                        name: "zoned".to_owned(),
                        value: "off".to_owned(),
                    },
                    ZProp {
                        name: "snapdir".to_owned(),
                        value: "hidden".to_owned(),
                    },
                    ZProp {
                        name: "aclinherit".to_owned(),
                        value: "restricted".to_owned(),
                    },
                    ZProp {
                        name: "canmount".to_owned(),
                        value: "on".to_owned(),
                    },
                    ZProp {
                        name: "xattr".to_owned(),
                        value: "on".to_owned(),
                    },
                    ZProp {
                        name: "copies".to_owned(),
                        value: "1".to_owned(),
                    },
                    ZProp {
                        name: "version".to_owned(),
                        value: "5".to_owned(),
                    },
                    ZProp {
                        name: "utf8only".to_owned(),
                        value: "off".to_owned(),
                    },
                    ZProp {
                        name: "normalization".to_owned(),
                        value: "none".to_owned(),
                    },
                    ZProp {
                        name: "casesensitivity".to_owned(),
                        value: "sensitive".to_owned(),
                    },
                    ZProp {
                        name: "vscan".to_owned(),
                        value: "off".to_owned(),
                    },
                    ZProp {
                        name: "nbmand".to_owned(),
                        value: "off".to_owned(),
                    },
                    ZProp {
                        name: "sharesmb".to_owned(),
                        value: "off".to_owned(),
                    },
                    ZProp {
                        name: "refquota".to_owned(),
                        value: "0".to_owned(),
                    },
                    ZProp {
                        name: "refreservation".to_owned(),
                        value: "0".to_owned(),
                    },
                    ZProp {
                        name: "primarycache".to_owned(),
                        value: "all".to_owned(),
                    },
                    ZProp {
                        name: "secondarycache".to_owned(),
                        value: "all".to_owned(),
                    },
                    ZProp {
                        name: "usedbysnapshots".to_owned(),
                        value: "0".to_owned(),
                    },
                    ZProp {
                        name: "usedbydataset".to_owned(),
                        value: "24576".to_owned(),
                    },
                    ZProp {
                        name: "usedbychildren".to_owned(),
                        value: "0".to_owned(),
                    },
                    ZProp {
                        name: "usedbyrefreservation".to_owned(),
                        value: "0".to_owned(),
                    },
                    ZProp {
                        name: "logbias".to_owned(),
                        value: "latency".to_owned(),
                    },
                    ZProp {
                        name: "dedup".to_owned(),
                        value: "off".to_owned(),
                    },
                    ZProp {
                        name: "mlslabel".to_owned(),
                        value: "none".to_owned(),
                    },
                    ZProp {
                        name: "sync".to_owned(),
                        value: "standard".to_owned(),
                    },
                    ZProp {
                        name: "dnodesize".to_owned(),
                        value: "legacy".to_owned(),
                    },
                    ZProp {
                        name: "refcompressratio".to_owned(),
                        value: "1.00x".to_owned(),
                    },
                    ZProp {
                        name: "written".to_owned(),
                        value: "24576".to_owned(),
                    },
                    ZProp {
                        name: "logicalused".to_owned(),
                        value: "12288".to_owned(),
                    },
                    ZProp {
                        name: "logicalreferenced".to_owned(),
                        value: "12288".to_owned(),
                    },
                    ZProp {
                        name: "volmode".to_owned(),
                        value: "default".to_owned(),
                    },
                    ZProp {
                        name: "filesystem_limit".to_owned(),
                        value: "18446744073709551615".to_owned(),
                    },
                    ZProp {
                        name: "snapshot_limit".to_owned(),
                        value: "18446744073709551615".to_owned(),
                    },
                    ZProp {
                        name: "filesystem_count".to_owned(),
                        value: "18446744073709551615".to_owned(),
                    },
                    ZProp {
                        name: "snapshot_count".to_owned(),
                        value: "18446744073709551615".to_owned(),
                    },
                    ZProp {
                        name: "snapdev".to_owned(),
                        value: "hidden".to_owned(),
                    },
                    ZProp {
                        name: "acltype".to_owned(),
                        value: "off".to_owned(),
                    },
                    ZProp {
                        name: "context".to_owned(),
                        value: "none".to_owned(),
                    },
                    ZProp {
                        name: "fscontext".to_owned(),
                        value: "none".to_owned(),
                    },
                    ZProp {
                        name: "defcontext".to_owned(),
                        value: "none".to_owned(),
                    },
                    ZProp {
                        name: "rootcontext".to_owned(),
                        value: "none".to_owned(),
                    },
                    ZProp {
                        name: "relatime".to_owned(),
                        value: "off".to_owned(),
                    },
                    ZProp {
                        name: "redundant_metadata".to_owned(),
                        value: "all".to_owned(),
                    },
                    ZProp {
                        name: "overlay".to_owned(),
                        value: "off".to_owned(),
                    },
                    ZProp {
                        name: "lustre:mgsnode".to_owned(),
                        value: "10.14.82.0@tcp:10.14.82.1@tcp".to_owned(),
                    },
                ]
            )
        });
    }
}
