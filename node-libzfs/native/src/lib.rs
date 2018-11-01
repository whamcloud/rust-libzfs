// Copyright (c) 2018 DDN. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

extern crate libzfs;
#[macro_use]
extern crate neon;
extern crate neon_serde;

#[macro_use]
extern crate serde_derive;

use libzfs::{Libzfs, VDev, ZProp, Zfs, Zpool};
use neon::js::error::{JsError, Kind};
use neon::js::{JsNull, JsString, JsValue};
use neon::vm::{Call, JsResult, Throw};
use std::ffi::CString;

#[derive(Serialize, Debug, Deserialize)]
struct Pool {
    name: String,
    guid: String,
    health: String,
    hostname: String,
    hostid: Option<u64>,
    state: String,
    readonly: bool,
    size: String,
    vdev: VDev,
    props: Vec<ZProp>,
    datasets: Vec<JsDataset>,
}

#[derive(Serialize, Debug, Deserialize)]
struct JsDataset {
    name: String,
    guid: String,
    kind: String,
    props: Vec<ZProp>,
}

fn convert_to_js_dataset(x: &Zfs) -> Result<JsDataset, Throw> {
    let props = x
        .props()
        .or_else(|_| JsError::throw(Kind::Error, "Could not enumerate props"))?;

    let guid = props.iter().find(|x| x.name == "guid").map_or_else(
        || JsError::throw(Kind::Error, "Could not find ds guid"),
        |x| Ok(x.value.clone()),
    )?;

    Ok(JsDataset {
        name: c_string_to_string(x.name())?,
        kind: c_string_to_string(x.zfs_type_name())?,
        guid,
        props,
    })
}

fn c_string_to_string(x: CString) -> Result<String, Throw> {
    let s = x.into_string();

    s.or_else(|_| JsError::throw(Kind::SyntaxError, "Could not convert CString into String."))
}

fn convert_to_js_pool(p: &Zpool) -> Result<Pool, Throw> {
    let xs = p
        .datasets()
        .or_else(|_| JsError::throw(Kind::Error, "Could not fetch datasets"))?
        .iter()
        .map(|x| convert_to_js_dataset(x))
        .collect::<Result<Vec<JsDataset>, Throw>>()?;

    let hostname = p
        .hostname()
        .or_else(|_| JsError::throw(Kind::Error, "Could not get hostname"))?;

    let hostid = p.hostid().ok();

    let health = p
        .health()
        .or_else(|_| JsError::throw(Kind::Error, "Could not get health"))?;

    Ok(Pool {
        name: c_string_to_string(p.name())?,
        health: c_string_to_string(health)?,
        guid: p.guid().to_string(),
        hostname: c_string_to_string(hostname)?,
        hostid,
        state: c_string_to_string(p.state_name())?,
        readonly: p.read_only(),
        size: p.size().to_string(),
        props: vec![],
        vdev: p
            .vdev_tree()
            .or_else(|_| JsError::throw(Kind::Error, "Could not enumerate vdev tree"))?,
        datasets: xs,
    })
}

fn get_pool_by_name(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let mut libzfs = Libzfs::new();

    let pool_name = call
        .arguments
        .require(scope, 0)?
        .check::<JsString>()?
        .value();

    let p = libzfs.pool_by_name(&pool_name);

    match p {
        Some(x) => {
            let value = convert_to_js_pool(&x)?;

            let js_value = neon_serde::to_value(scope, &value)?;

            Ok(js_value)
        }
        None => Ok(JsNull::new().upcast()),
    }
}

fn get_dataset_by_name(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let mut libzfs = Libzfs::new();

    let ds_name = call
        .arguments
        .require(scope, 0)?
        .check::<JsString>()?
        .value();

    let ds = libzfs.dataset_by_name(&ds_name);

    match ds {
        Some(x) => {
            let value = convert_to_js_dataset(&x)?;

            let js_value = neon_serde::to_value(scope, &value)?;

            Ok(js_value)
        }
        None => Ok(JsNull::new().upcast()),
    }
}

fn get_imported_pools(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let mut libzfs = Libzfs::new();
    let pools = libzfs
        .get_imported_pools()
        .unwrap()
        .iter()
        .map(convert_to_js_pool)
        .collect::<Result<Vec<Pool>, Throw>>()?;

    let arr = neon_serde::to_value(scope, &pools)?;

    Ok(arr)
}

register_module!(m, {
    m.export("getImportedPools", get_imported_pools)?;
    m.export("getPoolByName", get_pool_by_name)?;
    m.export("getDatasetByName", get_dataset_by_name)?;
    Ok(())
});
