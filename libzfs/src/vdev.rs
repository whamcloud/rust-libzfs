// Copyright (c) 2018 DDN. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

extern crate libzfs_sys as sys;

use libzfs_error::{LibZfsError, Result};
use nvpair;
use std::ffi::CStr;
use std::io::{Error, ErrorKind};

/// Represents vdevs
/// The enum starts at Root and is recursive.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum VDev {
    Mirror {
        children: Vec<VDev>,
        is_log: Option<bool>,
    },
    RaidZ {
        children: Vec<VDev>,
    },
    Replacing {
        children: Vec<VDev>,
    },
    Root {
        children: Vec<VDev>,
        spares: Vec<VDev>,
        cache: Vec<VDev>,
    },
    Disk {
        guid: Option<u64>,
        state: String,
        path: String,
        dev_id: Option<String>,
        phys_path: Option<String>,
        whole_disk: Option<bool>,
        is_log: Option<bool>,
    },
    File {
        guid: Option<u64>,
        state: String,
        path: String,
        is_log: Option<bool>,
    },
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

    fn get_spares(tree: &nvpair::NvList) -> Result<Vec<VDev>> {
        let spares = tree.lookup_nv_list_array(sys::zpool_config_spares());

        match spares {
            Ok(x) => x.iter().map(enumerate_vdev_tree).collect(),
            Err(_) => Ok(vec![]),
        }
    }

    fn get_cache(tree: &nvpair::NvList) -> Result<Vec<VDev>> {
        let cache = tree.lookup_nv_list_array(sys::zpool_config_l2cache());

        match cache {
            Ok(x) => x.iter().map(enumerate_vdev_tree).collect(),
            Err(_) => Ok(vec![]),
        }
    }

    fn lookup_tree_str(tree: &nvpair::NvList, name: String) -> Result<Option<String>> {
        let x = tree.lookup_string(name);

        match x {
            Ok(x) => Ok(Some(x.into_string()?)),
            Err(_) => Ok(None),
        }
    }

    fn lookup_is_log(tree: &nvpair::NvList) -> Option<bool> {
        tree.lookup_uint64(sys::zpool_config_is_log())
            .map(|x| x == 1)
            .ok()
    }

    fn lookup_guid(tree: &nvpair::NvList) -> Option<u64> {
        tree.lookup_uint64(sys::zpool_config_guid()).ok()
    }

    fn lookup_state(tree: &nvpair::NvList) -> Result<String> {
        let vdev_stats = tree
            .lookup_uint64_array(sys::zpool_config_vdev_stats())
            .map(sys::to_vdev_stat)?;

        let state = unsafe {
            let s = sys::zpool_state_to_name(
                sys::to_vdev_state(vdev_stats.vs_state as u32).ok_or(Error::new(
                    ErrorKind::NotFound,
                    "vs_state not in enum range",
                ))?,
                sys::to_vdev_aux(vdev_stats.vs_aux as u32)
                    .ok_or(Error::new(ErrorKind::NotFound, "vs_aux not in enum range"))?,
            );

            CStr::from_ptr(s)
        };

        state.to_owned().into_string().map_err(LibZfsError::from)
    }

    match x {
        x if x == sys::VDEV_TYPE_DISK => {
            let path = tree
                .lookup_string(sys::zpool_config_path())?
                .into_string()?;
            let dev_id = lookup_tree_str(tree, sys::zpool_config_dev_id())?;
            let phys_path = lookup_tree_str(tree, sys::zpool_config_phys_path())?;
            let whole_disk = tree
                .lookup_uint64(sys::zpool_config_whole_disk())
                .map(|x| x == 1)
                .ok();

            Ok(VDev::Disk {
                guid: lookup_guid(tree),
                state: lookup_state(tree)?,
                path,
                dev_id,
                phys_path,
                whole_disk,
                is_log: lookup_is_log(tree),
            })
        }
        x if x == sys::VDEV_TYPE_FILE => {
            let path = tree
                .lookup_string(sys::zpool_config_path())?
                .into_string()?;

            Ok(VDev::File {
                guid: lookup_guid(tree),
                state: lookup_state(tree)?,
                path,
                is_log: lookup_is_log(tree),
            })
        }
        x if x == sys::VDEV_TYPE_MIRROR => {
            let children = get_children(tree)?;
            let is_log = tree
                .lookup_uint64(sys::zpool_config_is_log())
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
        x if x == sys::VDEV_TYPE_ROOT => {
            let children = get_children(tree)?;

            let spares = get_spares(tree)?;

            let cache = get_cache(tree)?;

            Ok(VDev::Root {
                children,
                spares,
                cache,
            })
        }
        _ => Err(LibZfsError::Io(Error::new(
            ErrorKind::NotFound,
            "hit unknown vdev type",
        ))),
    }
}
