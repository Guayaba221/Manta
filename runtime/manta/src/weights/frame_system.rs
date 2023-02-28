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

//! Autogenerated weights for frame_system
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
// --pallet=frame_system
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/frame_system.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;
use manta_primitives::constants::RocksDbWeight;

/// Weight functions needed for frame_system.
pub trait WeightInfo {
    fn remark(b: u32, ) -> Weight;
    fn remark_with_event(b: u32, ) -> Weight;
    fn set_heap_pages() -> Weight;
    fn set_storage(i: u32, ) -> Weight;
    fn kill_storage(i: u32, ) -> Weight;
    fn kill_prefix(p: u32, ) -> Weight;
}

/// Weights for frame_system using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> frame_system::WeightInfo for SubstrateWeight<T> {
    fn remark(b: u32, ) -> Weight {
        (11_364_000 as Weight)
            // Standard Error: 0
            .saturating_add((1_000 as Weight).saturating_mul(b as u64))
    }
    fn remark_with_event(b: u32, ) -> Weight {
        (0 as Weight)
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(b as u64))
    }
    // Storage: System Digest (r:1 w:1)
    // Storage: unknown [0x3a686561707061676573] (r:0 w:1)
    fn set_heap_pages() -> Weight {
        (5_617_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: Skipped Metadata (r:0 w:0)
    fn set_storage(i: u32, ) -> Weight {
        (0 as Weight)
            // Standard Error: 1_000
            .saturating_add((707_000 as Weight).saturating_mul(i as u64))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as u64)))
    }
    // Storage: Skipped Metadata (r:0 w:0)
    fn kill_storage(i: u32, ) -> Weight {
        (0 as Weight)
            // Standard Error: 1_000
            .saturating_add((595_000 as Weight).saturating_mul(i as u64))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as u64)))
    }
    // Storage: Skipped Metadata (r:0 w:0)
    fn kill_prefix(p: u32, ) -> Weight {
        (0 as Weight)
            // Standard Error: 2_000
            .saturating_add((1_228_000 as Weight).saturating_mul(p as u64))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(p as u64)))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn remark(b: u32, ) -> Weight {
        (11_364_000 as Weight)
            // Standard Error: 0
            .saturating_add((1_000 as Weight).saturating_mul(b as u64))
    }
    fn remark_with_event(b: u32, ) -> Weight {
        (0 as Weight)
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(b as u64))
    }
    // Storage: System Digest (r:1 w:1)
    // Storage: unknown [0x3a686561707061676573] (r:0 w:1)
    fn set_heap_pages() -> Weight {
        (5_617_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().writes(2 as u64))
    }
    // Storage: Skipped Metadata (r:0 w:0)
    fn set_storage(i: u32, ) -> Weight {
        (0 as Weight)
            // Standard Error: 1_000
            .saturating_add((707_000 as Weight).saturating_mul(i as u64))
            .saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(i as u64)))
    }
    // Storage: Skipped Metadata (r:0 w:0)
    fn kill_storage(i: u32, ) -> Weight {
        (0 as Weight)
            // Standard Error: 1_000
            .saturating_add((595_000 as Weight).saturating_mul(i as u64))
            .saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(i as u64)))
    }
    // Storage: Skipped Metadata (r:0 w:0)
    fn kill_prefix(p: u32, ) -> Weight {
        (0 as Weight)
            // Standard Error: 2_000
            .saturating_add((1_228_000 as Weight).saturating_mul(p as u64))
            .saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(p as u64)))
    }
}
