// Copyright (c) 2018 DDN. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! libzfs — Rusty wrapper around libzfs-sys.
//!

extern crate serde_derive;

#[macro_use]
extern crate foreign_types;

#[macro_use]
extern crate lazy_static;

extern crate libzfs_sys as sys;

pub extern crate libzfs_types;

mod nvpair;

pub use libzfs_types::*;

pub mod vdev;
pub use vdev::VDev;

pub mod zprop_list;
pub use zprop_list::ZProp;

pub mod zfs;
pub use zfs::Zfs;

pub mod zpool;
pub use zpool::Zpool;

pub mod libzfs;
pub use libzfs::Libzfs;

pub mod state;
pub use state::*;
