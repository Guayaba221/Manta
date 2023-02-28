// Copyright 2020-2023 Manta Network.
// This file is part of Manta.
//
// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_multisig
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("manta"), DB CACHE: 1024

// Executed Command:
// ./target/production/manta
// benchmark
// pallet
// --chain=manta
// --steps=50
// --repeat=20
// --pallet=pallet_multisig
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/pallet_multisig.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;
use manta_primitives::constants::RocksDbWeight;

/// Weight functions needed for pallet_multisig.
pub trait WeightInfo {
    fn as_multi_threshold_1(z: u32, ) -> Weight;
    fn as_multi_create(s: u32, z: u32, ) -> Weight;
    fn as_multi_create_store(s: u32, z: u32, ) -> Weight;
    fn as_multi_approve(s: u32, z: u32, ) -> Weight;
    fn as_multi_approve_store(s: u32, z: u32, ) -> Weight;
    fn as_multi_complete(s: u32, z: u32, ) -> Weight;
    fn approve_as_multi_create(s: u32, ) -> Weight;
    fn approve_as_multi_approve(s: u32, ) -> Weight;
    fn approve_as_multi_complete(s: u32, ) -> Weight;
    fn cancel_as_multi(s: u32, ) -> Weight;
}

/// Weights for pallet_multisig using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for SubstrateWeight<T> {
    fn as_multi_threshold_1(z: u32, ) -> Weight {
        (21_367_000 as Weight)
            // Standard Error: 0
            .saturating_add((1_000 as Weight).saturating_mul(z as u64))
    }
    // Storage: Multisig Multisigs (r:1 w:1)
    // Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
    fn as_multi_create(s: u32, z: u32, ) -> Weight {
        (38_960_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((106_000 as Weight).saturating_mul(s as u64))
            // Standard Error: 0
            .saturating_add((1_000 as Weight).saturating_mul(z as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Multisig Multisigs (r:1 w:1)
    // Storage: Multisig Calls (r:1 w:1)
    // Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
    fn as_multi_create_store(s: u32, z: u32, ) -> Weight {
        (41_571_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((89_000 as Weight).saturating_mul(s as u64))
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(z as u64))
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: Multisig Multisigs (r:1 w:1)
    fn as_multi_approve(s: u32, z: u32, ) -> Weight {
        (26_100_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((90_000 as Weight).saturating_mul(s as u64))
            // Standard Error: 0
            .saturating_add((1_000 as Weight).saturating_mul(z as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Multisig Multisigs (r:1 w:1)
    // Storage: Multisig Calls (r:1 w:1)
    fn as_multi_approve_store(s: u32, z: u32, ) -> Weight {
        (42_101_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((91_000 as Weight).saturating_mul(s as u64))
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(z as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: Multisig Multisigs (r:1 w:1)
    // Storage: Multisig Calls (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn as_multi_complete(s: u32, z: u32, ) -> Weight {
        (51_142_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((128_000 as Weight).saturating_mul(s as u64))
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(z as u64))
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: Multisig Multisigs (r:1 w:1)
    // Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
    fn approve_as_multi_create(s: u32, ) -> Weight {
        (34_650_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((125_000 as Weight).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Multisig Multisigs (r:1 w:1)
    // Storage: Multisig Calls (r:1 w:0)
    fn approve_as_multi_approve(s: u32, ) -> Weight {
        (21_755_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((119_000 as Weight).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Multisig Multisigs (r:1 w:1)
    // Storage: Multisig Calls (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn approve_as_multi_complete(s: u32, ) -> Weight {
        (64_361_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((164_000 as Weight).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: Multisig Multisigs (r:1 w:1)
    // Storage: Multisig Calls (r:1 w:1)
    fn cancel_as_multi(s: u32, ) -> Weight {
        (51_254_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((139_000 as Weight).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn as_multi_threshold_1(z: u32, ) -> Weight {
        (21_367_000 as Weight)
            // Standard Error: 0
            .saturating_add((1_000 as Weight).saturating_mul(z as u64))
    }
    // Storage: Multisig Multisigs (r:1 w:1)
    // Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
    fn as_multi_create(s: u32, z: u32, ) -> Weight {
        (38_960_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((106_000 as Weight).saturating_mul(s as u64))
            // Standard Error: 0
            .saturating_add((1_000 as Weight).saturating_mul(z as u64))
            .saturating_add(RocksDbWeight::get().reads(2 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: Multisig Multisigs (r:1 w:1)
    // Storage: Multisig Calls (r:1 w:1)
    // Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
    fn as_multi_create_store(s: u32, z: u32, ) -> Weight {
        (41_571_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((89_000 as Weight).saturating_mul(s as u64))
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(z as u64))
            .saturating_add(RocksDbWeight::get().reads(3 as u64))
            .saturating_add(RocksDbWeight::get().writes(2 as u64))
    }
    // Storage: Multisig Multisigs (r:1 w:1)
    fn as_multi_approve(s: u32, z: u32, ) -> Weight {
        (26_100_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((90_000 as Weight).saturating_mul(s as u64))
            // Standard Error: 0
            .saturating_add((1_000 as Weight).saturating_mul(z as u64))
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: Multisig Multisigs (r:1 w:1)
    // Storage: Multisig Calls (r:1 w:1)
    fn as_multi_approve_store(s: u32, z: u32, ) -> Weight {
        (42_101_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((91_000 as Weight).saturating_mul(s as u64))
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(z as u64))
            .saturating_add(RocksDbWeight::get().reads(2 as u64))
            .saturating_add(RocksDbWeight::get().writes(2 as u64))
    }
    // Storage: Multisig Multisigs (r:1 w:1)
    // Storage: Multisig Calls (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn as_multi_complete(s: u32, z: u32, ) -> Weight {
        (51_142_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((128_000 as Weight).saturating_mul(s as u64))
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(z as u64))
            .saturating_add(RocksDbWeight::get().reads(3 as u64))
            .saturating_add(RocksDbWeight::get().writes(3 as u64))
    }
    // Storage: Multisig Multisigs (r:1 w:1)
    // Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
    fn approve_as_multi_create(s: u32, ) -> Weight {
        (34_650_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((125_000 as Weight).saturating_mul(s as u64))
            .saturating_add(RocksDbWeight::get().reads(2 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: Multisig Multisigs (r:1 w:1)
    // Storage: Multisig Calls (r:1 w:0)
    fn approve_as_multi_approve(s: u32, ) -> Weight {
        (21_755_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((119_000 as Weight).saturating_mul(s as u64))
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: Multisig Multisigs (r:1 w:1)
    // Storage: Multisig Calls (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn approve_as_multi_complete(s: u32, ) -> Weight {
        (64_361_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((164_000 as Weight).saturating_mul(s as u64))
            .saturating_add(RocksDbWeight::get().reads(3 as u64))
            .saturating_add(RocksDbWeight::get().writes(3 as u64))
    }
    // Storage: Multisig Multisigs (r:1 w:1)
    // Storage: Multisig Calls (r:1 w:1)
    fn cancel_as_multi(s: u32, ) -> Weight {
        (51_254_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((139_000 as Weight).saturating_mul(s as u64))
            .saturating_add(RocksDbWeight::get().reads(2 as u64))
            .saturating_add(RocksDbWeight::get().writes(2 as u64))
    }
}
