// Copyright (c) 2018 DDN. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

extern crate libzfs_sys as sys;
use std::ffi::CStr;

#[derive(Debug, PartialEq)]
pub struct ZpropList {
    head: *mut sys::zprop_list,
    pos: *mut sys::zprop_list_t,
}

impl ZpropList {
    pub fn new(pl: *mut sys::zprop_list_t) -> ZpropList {
        ZpropList { head: pl, pos: pl }
    }
}

impl Iterator for ZpropList {
    type Item = ZpropItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.is_null() {
            None
        } else {
            let out = Some(ZpropItem { raw: self.pos });
            self.pos = unsafe { (*self.pos).pl_next };
            out
        }
    }
}

impl Drop for ZpropList {
    fn drop(&mut self) {
        unsafe {
            sys::zprop_free_list(self.head);
        }
    }
}

pub struct ZpropItem {
    raw: *mut sys::zprop_list_t,
}

impl ZpropItem {
    pub fn prop(&self) -> sys::zfs_prop_t {
        unsafe { sys::to_zfs_prop_t((*self.raw).pl_prop).unwrap() }
    }
    pub fn user_prop(&self) -> &CStr {
        unsafe { CStr::from_ptr((*self.raw).pl_user_prop) }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Clone)]
pub struct ZProp {
    pub name: String,
    pub value: String,
}
