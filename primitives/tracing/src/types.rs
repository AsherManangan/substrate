// This file is part of Substrate.

// Copyright (C) 2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::{
    vec::Vec
};
use sp_runtime_interface::pass_by::PassByCodec;
use codec::{Encode, Decode};


#[derive(Encode, Decode, PassByCodec)]
pub enum WasmLevel {
	ERROR,
	WARN,
	INFO,
	DEBUG,
	TRACE
}

#[derive(Encode, Decode, PassByCodec)]
pub enum WasmFieldValue {
	I64(i64),
	U64(u64),
	Bool(bool),
	Str(Vec<u8>),
	Debug(Vec<u8>),
	Encoded(Vec<u8>),
}

pub type WasmFields = Vec<Vec<u8>>;
pub type WasmValues = Vec<(Vec<u8>, WasmFieldValue)>;

#[derive(Encode, Decode, PassByCodec)]
pub struct WasmMetadata {
    pub name: Vec<u8>,
    pub target: Vec<u8>,
    pub level: WasmLevel,
    pub file: Vec<u8>,
    pub line: u32,
	pub module_path: Vec<u8>,
    pub is_span: bool,
    pub fields: WasmFields,
}

#[derive(Encode, Decode, PassByCodec)]
pub struct WasmAttributes {
	pub parent_id: Option<u64>,
	pub fields: WasmValues,
}

#[derive(Encode, Decode, PassByCodec)]
pub struct WasmEvent {
    pub parent: Option<u64>,
    pub metadata: WasmMetadata,
	pub fields: WasmValues,
}

#[derive(Encode, Decode, PassByCodec)]
pub struct WasmRecord;
