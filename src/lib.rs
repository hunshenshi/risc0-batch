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

#[doc = include_str!("../README.md")]
use range_methods::RANGE_ELF;
use risc0_zkvm::{
    default_prover,
    serde::{from_slice, to_vec},
    ExecutorEnv, Receipt,
};

// Compare them inside the ZKP
pub fn range_than(a: String) -> (Receipt, String) {
    let env = ExecutorEnv::builder()
        // Send a & b to the guest
        // .add_input(&to_vec(&a).unwrap())
        // .add_input(&to_vec(&b).unwrap())
        .write_slice(&to_vec(&a).unwrap())
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    // let receipt = prover.prove_elf(env, RANGE_ELF).unwrap();
    let receipt = prover.prove(env, RANGE_ELF).unwrap();

    println!("{:?}", receipt);

    (receipt, "c".to_owned())
}
