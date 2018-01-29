// Copyright (c) 2017 Intel Corporation. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

extern crate libzfs;
#[macro_use]
extern crate neon;
extern crate neon_serde;

#[macro_use]
extern crate serde_derive;

use std::ffi::CString;
use neon::vm::{Call, JsResult, Throw};
use neon::js::{JsNull, JsString, JsNumber, JsValue};
use neon::js::error::{JsError, Kind};
use libzfs::{Zfs, Libzfs, VDev, Zpool};

#[derive(Serialize, Debug, Deserialize)]
struct Pool {
    name: String,
    uid: String,
    hostname: String,
    hostid: Option<u64>,
    state: String,
    size: u64,
    vdev: VDev,
    datasets: Vec<JsDataset>,
}

#[derive(Serialize, Debug, Deserialize)]
struct JsDataset {
    name: String,
    kind: String,
}

fn convert_to_js_dataset(x: &Zfs) -> Result<JsDataset, Throw> {
    Ok(JsDataset {
        name: c_string_to_string(x.name())?,
        kind: c_string_to_string(x.zfs_type_name())?,
    })
}

fn c_string_to_string(x: CString) -> Result<String, Throw> {
    let s = x.into_string();

    s.or_else(|_| {
        JsError::throw(Kind::SyntaxError, "Could not convert CString into String.")
    })
}

fn convert_to_js_pool(p: &Zpool) -> Result<Pool, Throw> {
    let xs = p.datasets()
        .or_else(|_| JsError::throw(Kind::Error, "Could not fetch datasets"))?
        .iter()
        .map(|x| convert_to_js_dataset(x))
        .collect::<Result<Vec<JsDataset>, Throw>>()?;


    let hostname = p.hostname().or_else(|_| {
        JsError::throw(Kind::Error, "Could not get hostname")
    })?;

    let hostid = p.hostid().ok();

    Ok(Pool {
        name: c_string_to_string(p.name())?,
        uid: p.guid_hex().to_uppercase(),
        hostname: c_string_to_string(hostname)?,
        hostid,
        state: c_string_to_string(p.state_name())?,
        size: p.size(),
        vdev: p.vdev_tree().or_else(|_| {
            JsError::throw(Kind::Error, "Could not enumerate vdev tree")
        })?,
        datasets: xs,
    })
}

fn get_dataset_string_prop(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let mut libzfs = Libzfs::new();

    let ds_name = call.arguments
        .require(scope, 0)?
        .check::<JsString>()?
        .value();

    let prop_name = call.arguments
        .require(scope, 1)?
        .check::<JsString>()?
        .value();

    let x: Option<String> = libzfs.dataset_by_name(&ds_name).and_then(|x| {
        x.lookup_string_prop(&prop_name)
    });

    match x {
        Some(y) => Ok(JsString::new(scope, &y).unwrap().upcast()),
        None => Ok(JsNull::new().upcast()),
    }
}

fn get_dataset_uint64_prop(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let mut libzfs = Libzfs::new();

    let ds_name = call.arguments
        .require(scope, 0)?
        .check::<JsString>()?
        .value();

    let prop_name = call.arguments
        .require(scope, 1)?
        .check::<JsString>()?
        .value();

    let x: Option<u64> = libzfs.dataset_by_name(&ds_name).and_then(|x| {
        x.lookup_uint64_prop(&prop_name)
    });

    match x {
        Some(y) => Ok(JsNumber::new(scope, y as f64).upcast()),
        None => Ok(JsNull::new().upcast()),
    }
}

fn get_pool_by_name(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let mut libzfs = Libzfs::new();

    let pool_name = call.arguments
        .require(scope, 0)?
        .check::<JsString>()?
        .value();

    let p = libzfs.pool_by_name(&pool_name);

    match p {
        Some(x) => {
            let value = convert_to_js_pool(&x)?;

            let js_value = neon_serde::to_value(&value, scope)?;

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

    let arr = neon_serde::to_value(&pools, scope)?;

    Ok(arr)
}

register_module!(m, {
    m.export("getImportedPools", get_imported_pools)?;
    m.export("getPoolByName", get_pool_by_name)?;
    m.export("getDatasetStringProp", get_dataset_string_prop)?;
    m.export("getDatasetUint64Prop", get_dataset_uint64_prop)?;
    Ok(())
});
