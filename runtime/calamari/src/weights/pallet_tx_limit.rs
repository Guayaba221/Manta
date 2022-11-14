// Copyright 2020-2022 Manta Network.
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

//! Autogenerated weights for pallet_tx_limit
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-14, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("calamari-local"), DB CACHE: 1024

// Executed Command:
// target/debug/manta
// benchmark
// pallet
// --chain=calamari-local
// --pallet=pallet_tx_limit
// --extrinsic=*
// --execution=Wasm
// --wasm-execution=Compiled
// --heap-pages=4096
// --repeat=20
// --steps=50
// --template=.github/resources/frame-weight-template.hbs
// --output=pallet_tx_limit.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_tx_limit.
pub trait WeightInfo {
    fn set_asset_limit() -> Weight;
    fn unset_asset_limit() -> Weight;
}

/// Weights for pallet_tx_limit using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_tx_limit::WeightInfo for SubstrateWeight<T> {
    // Storage: TransactionLimit AssetLimits (r:0 w:1)
    fn set_asset_limit() -> Weight {
        (194_000_000 as Weight)
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: TransactionLimit AssetLimits (r:0 w:1)
    fn unset_asset_limit() -> Weight {
        (196_000_000 as Weight)
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    // Storage: TransactionLimit AssetLimits (r:0 w:1)
    fn set_asset_limit() -> Weight {
        (194_000_000 as Weight)
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: TransactionLimit AssetLimits (r:0 w:1)
    fn unset_asset_limit() -> Weight {
        (196_000_000 as Weight)
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
}
