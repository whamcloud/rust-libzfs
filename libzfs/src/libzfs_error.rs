// Copyright (c) 2018 DDN. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use std::ffi::IntoStringError;
use std::io::Error;
use std::{error, fmt, result, str};

#[derive(Debug)]
pub enum LibZfsError {
    Io(::std::io::Error),
    IntoString(IntoStringError),
}

impl fmt::Display for LibZfsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LibZfsError::Io(ref err) => write!(f, "{}", err),
            LibZfsError::IntoString(ref err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for LibZfsError {
    fn description(&self) -> &str {
        match *self {
            LibZfsError::Io(ref err) => err.description(),
            LibZfsError::IntoString(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            LibZfsError::Io(ref err) => Some(err),
            LibZfsError::IntoString(ref err) => Some(err),
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
