// Copyright 2020-2024 Manta Network.
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

use super::*;
use crate as calamari_vesting;
use frame_support::{
    derive_impl, parameter_types,
    traits::{ConstU32, ConstU64},
};
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

pub type AccountId = u128;
pub type Balance = u128;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const ALICE_DEPOSIT: Balance = 10_000;

frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Timestamp: pallet_timestamp,
        Balances: pallet_balances,
        CalamariVesting: calamari_vesting,
    }
);

parameter_types! {
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(Weight::from_parts(1024, 0));
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
    type AccountData = pallet_balances::AccountData<Balance>;
    type AccountId = AccountId;
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockHashCount = ConstU64<250>;
    type BlockLength = ();
    type Block = Block;
    type BlockWeights = ();
    type RuntimeCall = RuntimeCall;
    type DbWeight = ();
    type RuntimeEvent = RuntimeEvent;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type Nonce = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type OnKilledAccount = ();
    type OnNewAccount = ();
    type OnSetCode = ();
    type RuntimeOrigin = RuntimeOrigin;
    type PalletInfo = PalletInfo;
    type SS58Prefix = ();
    type SystemWeightInfo = ();
    type Version = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_balances::Config for Test {
    type AccountStore = System;
    type Balance = Balance;
    type DustRemoval = ();
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ExistentialDeposit;
    type MaxLocks = ConstU32<10>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = ();
    type RuntimeHoldReason = RuntimeHoldReason;
    type RuntimeFreezeReason = RuntimeFreezeReason;
    type FreezeIdentifier = ();
    type MaxFreezes = ConstU32<1>;
    type MaxHolds = ConstU32<1>;
}

parameter_types! {
    pub const MinimumPeriod: u64 = 12_000 / 2;
}
impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

parameter_types! {
    pub const MinVestedTransfer: Balance = 2;
    pub static ExistentialDeposit: Balance = 1;
    pub const MaxScheduleLength: u32 = 6;
}
impl Config for Test {
    type Currency = Balances;
    type RuntimeEvent = RuntimeEvent;
    type Timestamp = Timestamp;
    type MinVestedTransfer = MinVestedTransfer;
    type MaxScheduleLength = MaxScheduleLength;
    type WeightInfo = ();
}

pub struct ExtBuilder {
    existential_deposit: Balance,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            existential_deposit: 1,
        }
    }
}

impl ExtBuilder {
    pub fn existential_deposit(mut self, existential_deposit: Balance) -> Self {
        self.existential_deposit = existential_deposit;
        self
    }

    pub fn build(self) -> sp_io::TestExternalities {
        EXISTENTIAL_DEPOSIT.with(|v| *v.borrow_mut() = self.existential_deposit);
        let mut t = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap();
        pallet_balances::GenesisConfig::<Test> {
            balances: vec![(ALICE, ALICE_DEPOSIT * self.existential_deposit)],
        }
        .assimilate_storage(&mut t)
        .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| {
            // Set current time more early than the first schedule.
            Timestamp::set_timestamp(VestingSchedule::<Test>::get()[0].1 * 1000 - 3 * 6000);
            System::set_block_number(1);
        });
        ext
    }
}

pub(crate) fn run_to_block(n: u64) {
    System::set_block_number(n);
}
