// Copyright (c) 2018 DDN. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! libzfs-types — Shared types for libzfs
//!
extern crate serde_derive;

use serde_derive::{Deserialize, Serialize};

use std::{error, ffi::IntoStringError, fmt, io::Error, path::PathBuf, result};

#[derive(Debug)]
pub enum LibZfsError {
    Io(::std::io::Error),
    IntoString(IntoStringError),
    PoolNotFound(Option<String>, Option<u64>),
    ZfsNotFound(String),
}

impl fmt::Display for LibZfsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LibZfsError::Io(ref err) => write!(f, "{}", err),
            LibZfsError::IntoString(ref err) => write!(f, "{}", err),
            LibZfsError::PoolNotFound(ref pool, guid) => match (pool, guid) {
                (Some(pool), Some(guid)) => write!(
                    f,
                    "The pool: {} with guid: {} could not be found.",
                    pool, guid
                ),
                (Some(pool), None) => write!(f, "The pool: {} could not be found.", pool),
                (None, Some(guid)) => write!(f, "The pool with guid: {} could not be found.", guid),
                (None, None) => write!(f, "The pool could not be found."),
            },
            LibZfsError::ZfsNotFound(ref err) => {
                write!(f, "The zfs object {} could not be found", err)
            }
        }
    }
}

impl error::Error for LibZfsError {
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            LibZfsError::Io(ref err) => Some(err),
            LibZfsError::IntoString(ref err) => Some(err),
            LibZfsError::PoolNotFound(_, _) => None,
            LibZfsError::ZfsNotFound(_) => None,
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

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone, PartialOrd, Ord)]
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

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Clone, PartialOrd, Ord)]
pub struct ZProp {
    pub name: String,
    pub value: String,
}

/// A Pool at a point in time
#[derive(Debug, Serialize, PartialEq, Deserialize, Clone)]
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
#[derive(Debug, Serialize, PartialEq, Deserialize, Clone)]
pub struct Dataset {
    pub name: String,
    pub guid: String,
    pub kind: String,
    pub props: Vec<ZProp>,
}
