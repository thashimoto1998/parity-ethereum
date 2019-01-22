// Copyright 2015-2018 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

extern crate byteorder;
extern crate ethcore_logger;
extern crate ethereum_types;
extern crate parity_wasm;
extern crate libc;
extern crate log;
extern crate vm;
extern crate wasmi;
extern crate pwasm_utils as wasm_utils;
extern crate wasm_exec_common;

pub mod env;
pub mod parser;
pub mod runtime;
pub mod interpreter;