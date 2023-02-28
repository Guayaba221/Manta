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

//! Autogenerated weights for pallet_preimage
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
// --pallet=pallet_preimage
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/pallet_preimage.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;
use manta_primitives::constants::RocksDbWeight;

/// Weight functions needed for pallet_preimage.
pub trait WeightInfo {
    fn note_preimage(s: u32, ) -> Weight;
    fn note_requested_preimage(s: u32, ) -> Weight;
    fn note_no_deposit_preimage(s: u32, ) -> Weight;
    fn unnote_preimage() -> Weight;
    fn unnote_no_deposit_preimage() -> Weight;
    fn request_preimage() -> Weight;
    fn request_no_deposit_preimage() -> Weight;
    fn request_unnoted_preimage() -> Weight;
    fn request_requested_preimage() -> Weight;
    fn unrequest_preimage() -> Weight;
    fn unrequest_unnoted_preimage() -> Weight;
    fn unrequest_multi_referenced_preimage() -> Weight;
}

/// Weights for pallet_preimage using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_preimage::WeightInfo for SubstrateWeight<T> {
    // Storage: Preimage PreimageFor (r:1 w:1)
    // Storage: Preimage StatusFor (r:1 w:1)
    fn note_preimage(s: u32, ) -> Weight {
        (0 as Weight)
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: Preimage PreimageFor (r:1 w:1)
    // Storage: Preimage StatusFor (r:1 w:0)
    fn note_requested_preimage(s: u32, ) -> Weight {
        (0 as Weight)
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Preimage PreimageFor (r:1 w:1)
    // Storage: Preimage StatusFor (r:1 w:0)
    fn note_no_deposit_preimage(s: u32, ) -> Weight {
        (0 as Weight)
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Preimage StatusFor (r:1 w:1)
    // Storage: Preimage PreimageFor (r:0 w:1)
    fn unnote_preimage() -> Weight {
        (36_208_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: Preimage StatusFor (r:1 w:1)
    // Storage: Preimage PreimageFor (r:0 w:1)
    fn unnote_no_deposit_preimage() -> Weight {
        (22_670_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: Preimage StatusFor (r:1 w:1)
    fn request_preimage() -> Weight {
        (35_218_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Preimage StatusFor (r:1 w:1)
    fn request_no_deposit_preimage() -> Weight {
        (21_461_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Preimage StatusFor (r:1 w:1)
    fn request_unnoted_preimage() -> Weight {
        (17_370_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Preimage StatusFor (r:1 w:1)
    fn request_requested_preimage() -> Weight {
        (7_247_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Preimage StatusFor (r:1 w:1)
    // Storage: Preimage PreimageFor (r:0 w:1)
    fn unrequest_preimage() -> Weight {
        (21_803_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: Preimage StatusFor (r:1 w:1)
    // Storage: Preimage PreimageFor (r:0 w:1)
    fn unrequest_unnoted_preimage() -> Weight {
        (17_345_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: Preimage StatusFor (r:1 w:1)
    fn unrequest_multi_referenced_preimage() -> Weight {
        (7_187_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    // Storage: Preimage PreimageFor (r:1 w:1)
    // Storage: Preimage StatusFor (r:1 w:1)
    fn note_preimage(s: u32, ) -> Weight {
        (0 as Weight)
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(s as u64))
            .saturating_add(RocksDbWeight::get().reads(2 as u64))
            .saturating_add(RocksDbWeight::get().writes(2 as u64))
    }
    // Storage: Preimage PreimageFor (r:1 w:1)
    // Storage: Preimage StatusFor (r:1 w:0)
    fn note_requested_preimage(s: u32, ) -> Weight {
        (0 as Weight)
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(s as u64))
            .saturating_add(RocksDbWeight::get().reads(2 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: Preimage PreimageFor (r:1 w:1)
    // Storage: Preimage StatusFor (r:1 w:0)
    fn note_no_deposit_preimage(s: u32, ) -> Weight {
        (0 as Weight)
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(s as u64))
            .saturating_add(RocksDbWeight::get().reads(2 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: Preimage StatusFor (r:1 w:1)
    // Storage: Preimage PreimageFor (r:0 w:1)
    fn unnote_preimage() -> Weight {
        (36_208_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().writes(2 as u64))
    }
    // Storage: Preimage StatusFor (r:1 w:1)
    // Storage: Preimage PreimageFor (r:0 w:1)
    fn unnote_no_deposit_preimage() -> Weight {
        (22_670_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().writes(2 as u64))
    }
    // Storage: Preimage StatusFor (r:1 w:1)
    fn request_preimage() -> Weight {
        (35_218_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: Preimage StatusFor (r:1 w:1)
    fn request_no_deposit_preimage() -> Weight {
        (21_461_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: Preimage StatusFor (r:1 w:1)
    fn request_unnoted_preimage() -> Weight {
        (17_370_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: Preimage StatusFor (r:1 w:1)
    fn request_requested_preimage() -> Weight {
        (7_247_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: Preimage StatusFor (r:1 w:1)
    // Storage: Preimage PreimageFor (r:0 w:1)
    fn unrequest_preimage() -> Weight {
        (21_803_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().writes(2 as u64))
    }
    // Storage: Preimage StatusFor (r:1 w:1)
    // Storage: Preimage PreimageFor (r:0 w:1)
    fn unrequest_unnoted_preimage() -> Weight {
        (17_345_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().writes(2 as u64))
    }
    // Storage: Preimage StatusFor (r:1 w:1)
    fn unrequest_multi_referenced_preimage() -> Weight {
        (7_187_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
}
