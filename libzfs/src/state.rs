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
use libzfs_error::Result;
use vdev::VDev;
use zfs::Zfs;
use zpool::Zpool;
use zprop_list::ZProp;

/// A Pool at a point in time
#[derive(Debug, Serialize, Deserialize)]
pub struct Pool {
    pub name: String,
    pub guid: String,
    pub health: String,
    pub hostname: String,
    pub hostid: Option<u64>,
    pub state: String,
    pub readonly: bool,
    pub size: String,
    pub vdev: VDev,
    pub props: Vec<ZProp>,
    pub datasets: Vec<Dataset>,
}

/// A Dataset at a point in time
#[derive(Debug, Serialize, Deserialize)]
pub struct Dataset {
    pub name: String,
    pub guid: String,
    pub kind: String,
    pub props: Vec<ZProp>,
}

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
        guid: p.guid().to_string(),
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
/// The outer `Option` represents failure to find the pool,
/// The inner `Result` represents failure during the conversion.
pub fn get_pool_by_name(pool_name: &str) -> Option<Result<Pool>> {
    let mut libzfs = Libzfs::new();

    libzfs
        .pool_by_name(&pool_name)
        .map(|x| convert_to_js_pool(&x))
}

/// Given a dataset name, try to find it and convert it to a `Dataset`.
/// The outer `Option` represents failure to find the dataset,
/// The inner `Result` represents failure during the conversion.
pub fn get_dataset_by_name(ds_name: &str) -> Option<Result<Dataset>> {
    let mut libzfs = Libzfs::new();

    libzfs
        .dataset_by_name(&ds_name)
        .map(|x| convert_to_dataset(&x))
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
