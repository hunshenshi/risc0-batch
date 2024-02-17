// Copyright 2023 RISC Zero, Inc.
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

#![no_main]
// #![no_std]

use risc0_zkvm::guest::env;
use serde_json::Value as JsonValue;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // Load the string from the host
    let a: String = env::read();
    env::log(&format!("input: {:?}", a));

    // the string is a json array
    let input_v: JsonValue = serde_json::from_str(&a).unwrap();
    let item_arr = input_v.as_array().unwrap();
 
    for item in item_arr {
        let item_str: JsonValue = serde_json::from_str(item.as_str().unwrap()).unwrap();
        env::log(&format!("public input parse error, Error: {:?}", item_str["public_input"].as_str().unwrap().to_string()));
    }
}
