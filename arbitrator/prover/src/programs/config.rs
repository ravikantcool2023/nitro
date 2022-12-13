// Copyright 2022, Offchain Labs, Inc.
// For license information, see https://github.com/nitro/blob/master/LICENSE

use wasmer::{wasmparser::Operator, CompilerConfig, Store};
use wasmer_compiler_singlepass::Singlepass;

type Pricing = fn(&Operator) -> u64;

#[repr(C)]
pub struct PolyglotConfig {
    pub costs: Pricing,
    pub start_gas: u64,
}

impl Default for PolyglotConfig {
    fn default() -> Self {
        let costs = |_: &Operator| 0;
        Self {
            costs,
            start_gas: 0,
        }
    }
}

impl PolyglotConfig {
    pub fn new(costs: Pricing, start_gas: u64) -> Self {
        Self { costs, start_gas }
    }

    pub fn store(&self) -> Store {
        let mut compiler = Singlepass::new();
        compiler.canonicalize_nans(true);
        compiler.enable_verifier();
        Store::new(compiler)
    }
}
