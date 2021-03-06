//  rpc-perf - RPC Performance Testing
//  Copyright 2015 Twitter, Inc
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

mod buffer;
pub mod config;
mod consts;
mod gen;
mod parse;
mod testutil;

use cfgtypes::{tools, CResult, ProtocolConfig, Style};
use rand::random;
use std::collections::BTreeMap;
use toml::Value;

#[derive(Clone, Debug)]
pub struct Parameter {
    pub id: Option<i16>,
    pub seed: usize,
    pub size: usize,
    pub num: u64,
    pub style: Style,
    pub regenerate: bool,
    pub value: Tvalue,
}

impl Parameter {
    pub fn regen(&mut self) {
        if self.regenerate && self.style == Style::Random {
            self.value.regen(self.size, self.num)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tvalue {
    Stop,
    Void,
    Bool(bool),
    Byte(u8),
    Double(f64),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    String(String),
    Struct,
    Map,
    Set,
    List(String, i32),
}

impl Tvalue {
    fn regen(&mut self, size: usize, num: u64) {
        match *self {
            Tvalue::Bool(ref mut v) => *v = random::<bool>(),
            Tvalue::Byte(ref mut v) => *v = random::<u8>(),
            Tvalue::Double(ref mut v) => *v = random::<f64>(),
            Tvalue::Int16(ref mut v) => *v = random::<i16>(),
            Tvalue::Int32(ref mut v) => *v = random::<i32>(),
            Tvalue::Int64(ref mut v) => *v = random::<i64>(),
            Tvalue::String(ref mut v) => *v = tools::random_string(size, num),
            _ => {}
        }
    }
}

pub fn load_config(table: &BTreeMap<String, Value>) -> CResult<ProtocolConfig> {
    config::load_config(table)
}
