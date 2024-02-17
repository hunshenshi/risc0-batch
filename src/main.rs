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

use range::range_than;
use range_methods::RANGE_ID;

use serde_json::Value as JsonValue;

fn main() {
    // origin json is 
    // [
    // "{\"private_input\":\"14\", \"public_input\":\"3,34\", \"receipt_type\":\"Snark\"}",
    // "{\"private_input\":\"14\", \"public_input\":\"3,34\", \"receipt_type\":\"Snark\"}"
    // ]
    let json_arr = "[
        \"{\\\"private_input\\\":\\\"14\\\", \\\"public_input\\\":\\\"3,34\\\", \\\"receipt_type\\\":\\\"Snark\\\"}\",
        \"{\\\"private_input\\\":\\\"14\\\", \\\"public_input\\\":\\\"3,34\\\", \\\"receipt_type\\\":\\\"Snark\\\"}\"
        ]";

    // println!("{}", json_arr);

    // let input_v: JsonValue = serde_json::from_str(json_arr).unwrap();
    // let item_arr = input_v.as_array().unwrap();

    // for item in item_arr {
    //     let item_str: JsonValue = serde_json::from_str(item.as_str().unwrap()).unwrap();
    //     // println!("{}", item)
    //     println!("{}", item_str["public_input"].as_str().unwrap().to_string());
    // }

    // Pick two numbers
    let (receipt, _) = range_than(json_arr.to_owned());

    // Here is where one would send 'receipt' over the network...

    // Verify receipt, panic if it's wrong
    receipt.verify(RANGE_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );
}
