// Copyright (c) 2018 DDN. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! libzfs-types — Shared types for libzfs
//!

extern crate serde_derive;

use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
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
        path: PathBuf,
        dev_id: Option<String>,
        phys_path: Option<String>,
        whole_disk: Option<bool>,
        is_log: Option<bool>,
    },
    File {
        guid: Option<u64>,
        state: String,
        path: PathBuf,
        is_log: Option<bool>,
    },
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Clone)]
pub struct ZProp {
    pub name: String,
    pub value: String,
}

/// A Pool at a point in time
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pool {
    pub name: String,
    pub guid: u64,
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dataset {
    pub name: String,
    pub guid: String,
    pub kind: String,
    pub props: Vec<ZProp>,
}
