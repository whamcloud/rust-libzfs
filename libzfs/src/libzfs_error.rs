// Copyright (c) 2018 DDN. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use std::ffi::IntoStringError;
use std::io::Error;
use std::{error, fmt, result};

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
