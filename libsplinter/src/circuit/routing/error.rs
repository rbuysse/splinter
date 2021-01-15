// Copyright 2018-2021 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Types for errors that can be raised by `RoutingTableReader` and `RoutingTableWriter` traits

use std::error::Error;
use std::fmt;

use crate::error::{InternalError, InvalidStateError};

#[derive(Debug)]
pub enum RoutingTableReaderError {
    /// Represents errors internal to the function.
    InternalError(InternalError),
    /// Represents when an operation cannot be completed because the state of the underlying
    /// struct is inconsistent.
    InvalidStateError(InvalidStateError),
}

impl RoutingTableReaderError {
    /// Reduces the `RoutingTableReaderError to the display string
    ///
    /// If the error is `InternalError` and includes a source, the debug format will be logged to
    /// provide information that may be lost on the conversion.
    pub fn reduce_to_string(self) -> String {
        match self {
            RoutingTableReaderError::InternalError(err) => err.reduce_to_string(),
            _ => self.to_string(),
        }
    }
}

impl Error for RoutingTableReaderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RoutingTableReaderError::InternalError(err) => Some(err),
            RoutingTableReaderError::InvalidStateError(err) => Some(err),
        }
    }
}

impl fmt::Display for RoutingTableReaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RoutingTableReaderError::InternalError(err) => write!(f, "{}", err),
            RoutingTableReaderError::InvalidStateError(err) => write!(f, "{}", err),
        }
    }
}
