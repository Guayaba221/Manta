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

//! Test utilities

#![cfg(test)]

use crate::{self as pallet_balances, decl_tests, Config, Pallet};
use frame_support::{
    dispatch::DispatchInfo,
    parameter_types,
    traits::{ConstU32, ConstU64, ConstU8, StorageMapShim},
    weights::{IdentityFee, Weight},
};
use pallet_transaction_payment::CurrencyAdapter;
use sp_core::H256;
use sp_io;
use sp_runtime::{testing::Header, traits::IdentityLookup};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub struct Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        TransactionPayment: pallet_transaction_payment::{Pallet, Storage, Event<T>},
    }
);

parameter_types! {
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(
            frame_support::weights::Weight::from_ref_time(1024).set_proof_size(u64::MAX),
        );
    pub static ExistentialDeposit: u64 = 0;
}
impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = BlockWeights;
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type Index = u64;
    type BlockNumber = u64;
    type RuntimeCall = RuntimeCall;
    type Hash = H256;
    type Hashing = ::sp_runtime::traits::BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_transaction_payment::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeTransaction = CurrencyAdapter<Pallet<Test>, ()>;
    type OperationalFeeMultiplier = ConstU8<5>;
    type WeightToFee = IdentityFee<u64>;
    type LengthToFee = IdentityFee<u64>;
    type FeeMultiplierUpdate = ();
}

pub struct MockNativeBarrier;
impl orml_traits::xcm_transfer::NativeBarrier<u64, u64> for MockNativeBarrier {
    fn update_xcm_native_transfers(_account_id: &u64, _amount: u64) {}
    fn ensure_xcm_transfer_limit_not_exceeded(
        _account_id: &u64,
        _amount: u64,
    ) -> frame_support::dispatch::DispatchResult {
        Ok(())
    }
}

impl orml_traits::xcm_transfer::NativeChecker<u64> for MockNativeBarrier {
    fn is_native(_currency_id: &u64) -> bool {
        true
    }
}

impl Config for Test {
    type Balance = u64;
    type DustRemoval = ();
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore =
        StorageMapShim<super::Account<Test>, system::Provider<Test>, u64, super::AccountData<u64>>;
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ConstU32<2>;
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = ();
    type NativeBarrierType = MockNativeBarrier;
}

pub struct ExtBuilder {
    existential_deposit: u64,
    monied: bool,
}
impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            existential_deposit: 1,
            monied: false,
        }
    }
}
impl ExtBuilder {
    pub fn existential_deposit(mut self, existential_deposit: u64) -> Self {
        self.existential_deposit = existential_deposit;
        self
    }
    pub fn monied(mut self, monied: bool) -> Self {
        self.monied = monied;
        if self.existential_deposit == 0 {
            self.existential_deposit = 1;
        }
        self
    }
    pub fn set_associated_consts(&self) {
        EXISTENTIAL_DEPOSIT.with(|v| *v.borrow_mut() = self.existential_deposit);
    }
    pub fn build(self) -> sp_io::TestExternalities {
        self.set_associated_consts();
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();
        pallet_balances::GenesisConfig::<Test> {
            balances: if self.monied {
                vec![
                    (1, 10 * self.existential_deposit),
                    (2, 20 * self.existential_deposit),
                    (3, 30 * self.existential_deposit),
                    (4, 40 * self.existential_deposit),
                    (12, 10 * self.existential_deposit),
                ]
            } else {
                vec![]
            },
        }
        .assimilate_storage(&mut t)
        .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

decl_tests! { Test, ExtBuilder, EXISTENTIAL_DEPOSIT }

#[test]
fn emit_events_with_no_existential_deposit_suicide_with_dust() {
    <ExtBuilder>::default()
        .existential_deposit(2)
        .build()
        .execute_with(|| {
            assert_ok!(Balances::set_balance(RawOrigin::Root.into(), 1, 100, 0));

            assert_eq!(
                events(),
                [
                    RuntimeEvent::System(system::Event::NewAccount { account: 1 }),
                    RuntimeEvent::Balances(crate::Event::Endowed {
                        account: 1,
                        free_balance: 100
                    }),
                    RuntimeEvent::Balances(crate::Event::BalanceSet {
                        who: 1,
                        free: 100,
                        reserved: 0
                    }),
                ]
            );

            let res = Balances::slash(&1, 98);
            assert_eq!(res, (NegativeImbalance::new(98), 0));

            // no events
            assert_eq!(
                events(),
                [RuntimeEvent::Balances(crate::Event::Slashed {
                    who: 1,
                    amount: 98
                })]
            );

            let res = Balances::slash(&1, 1);
            assert_eq!(res, (NegativeImbalance::new(1), 0));

            assert_eq!(
                events(),
                [
                    RuntimeEvent::System(system::Event::KilledAccount { account: 1 }),
                    RuntimeEvent::Balances(crate::Event::DustLost {
                        account: 1,
                        amount: 1
                    }),
                    RuntimeEvent::Balances(crate::Event::Slashed { who: 1, amount: 1 })
                ]
            );
        });
}
