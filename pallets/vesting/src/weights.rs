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

//! Autogenerated weights for calamari_vesting
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-08, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("calamari-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/manta
// benchmark
// pallet
// --chain=calamari-dev
// --steps=50
// --repeat=20
// --pallet=calamari_vesting
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/calamari_vesting.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for calamari_vesting.
pub trait WeightInfo {
    fn update_vesting_schedule() -> Weight;
    fn vest() -> Weight;
    fn vested_transfer() -> Weight;
}

/// Weights for calamari_vesting using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    // Storage: CalamariVesting VestingSchedule (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    fn update_vesting_schedule() -> Weight {
        (18_103_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: CalamariVesting VestingSchedule (r:1 w:0)
    // Storage: CalamariVesting VestingBalances (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn vest() -> Weight {
        (37_818_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(5 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: CalamariVesting VestingBalances (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: CalamariVesting VestingSchedule (r:1 w:0)
    // Storage: Balances Locks (r:1 w:1)
    fn vested_transfer() -> Weight {
        (66_814_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(5 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    // Storage: CalamariVesting VestingSchedule (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    fn update_vesting_schedule() -> Weight {
        (18_103_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: CalamariVesting VestingSchedule (r:1 w:0)
    // Storage: CalamariVesting VestingBalances (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn vest() -> Weight {
        (37_818_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(5 as u64))
            .saturating_add(RocksDbWeight::get().writes(3 as u64))
    }
    // Storage: CalamariVesting VestingBalances (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: CalamariVesting VestingSchedule (r:1 w:0)
    // Storage: Balances Locks (r:1 w:1)
    fn vested_transfer() -> Weight {
        (66_814_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(5 as u64))
            .saturating_add(RocksDbWeight::get().writes(3 as u64))
    }
}
