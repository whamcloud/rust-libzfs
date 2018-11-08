// Copyright (c) 2018 DDN. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Get the current state of ZFS on a node
//!
//! Uses a `Libzfs` handle to get state at that point.
//! Collects into a struct that can be serialized using `serde`.
//!

use std::io;

use libzfs::Libzfs;
use libzfs_error::{LibZfsError, Result};
use libzfs_types::{Dataset, Pool};
use zfs::Zfs;
use zpool::Zpool;

/// Takes a Zfs reference and converts it into a
/// `Dataset`
fn convert_to_dataset(x: &Zfs) -> Result<Dataset> {
    let props = x.props()?;

    let guid = props
        .iter()
        .find(|x| x.name == "guid")
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "could not find guid in props"))
        .map(|x| x.value.clone())?;

    Ok(Dataset {
        name: x.name().into_string()?,
        kind: x.zfs_type_name().into_string()?,
        guid,
        props,
    })
}

/// Takes a Zpool reference and converts it into a
/// `Pool`
fn convert_to_js_pool(p: &Zpool) -> Result<Pool> {
    let xs: Vec<Dataset> = p
        .datasets()?
        .iter()
        .map(convert_to_dataset)
        .collect::<Result<_>>()?;

    let hostname = p.hostname()?;

    let hostid = p.hostid().ok();

    let health = p.health()?;

    Ok(Pool {
        name: p.name().into_string()?,
        health: health.into_string()?,
        guid: p.guid(),
        hostname: hostname.into_string()?,
        hostid,
        state: p.state_name().into_string()?,
        readonly: p.read_only(),
        size: p.size().to_string(),
        props: vec![],
        vdev: p.vdev_tree()?,
        datasets: xs,
    })
}

/// Given a pool name, try to find it and convert it to a `Pool`.
/// The `Result` represents failure to find or convert the pool.
pub fn get_pool_by_name(pool_name: &str) -> Result<Pool> {
    let mut libzfs = Libzfs::new();

    libzfs
        .pool_by_name(&pool_name)
        .ok_or_else(|| LibZfsError::PoolNotFound(Some(pool_name.to_string()), None))
        .and_then(|x| convert_to_js_pool(&x))
}

/// Given a pool name and guid try to find it and convert it to a `Pool`.
/// The `Result` represents failure to find or convert the pool.
pub fn get_pool_by_name_and_guid(pool_name: &str, guid: u64) -> Result<Pool> {
    let mut libzfs = Libzfs::new();

    libzfs
        .pool_by_name(&pool_name)
        .filter(|x| x.guid() == guid)
        .ok_or_else(|| LibZfsError::PoolNotFound(Some(pool_name.to_string()), Some(guid)))
        .and_then(|x| convert_to_js_pool(&x))
}

/// Given a dataset name, try to find it and convert it to a `Dataset`.
/// The `Result` represents failure to find or convert the dataset.
pub fn get_dataset_by_name(ds_name: &str) -> Result<Dataset> {
    let mut libzfs = Libzfs::new();

    libzfs
        .dataset_by_name(&ds_name)
        .ok_or_else(|| LibZfsError::ZfsNotFound(ds_name.to_string()))
        .and_then(|x| convert_to_dataset(&x))
}

/// Return all imported pools on this node.
/// Returns `Err` if any imported pool fails conversion to `Pool`.
pub fn get_imported_pools() -> Result<Vec<Pool>> {
    let mut libzfs = Libzfs::new();
    libzfs
        .get_imported_pools()?
        .iter()
        .map(convert_to_js_pool)
        .collect()
}
