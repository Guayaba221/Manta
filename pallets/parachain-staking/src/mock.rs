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

//! Test utilities
use crate as pallet_parachain_staking;
use crate::{
    pallet, AwardedPts, Config, InflationInfo, Points, Range, COLLATOR_LOCK_ID, DELEGATOR_LOCK_ID,
};
use frame_support::{
    construct_runtime, derive_impl, parameter_types,
    traits::{Everything, LockIdentifier, OnFinalize, OnInitialize},
};
use manta_primitives::types::BlockNumber;
use sp_core::H256;
use sp_io;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage, Perbill, Percent,
};

pub type AccountId = u64;
pub type Balance = u128;

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
// WHITELIST: Remove Session and CollatorSelection after end of whitelist-period
construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Balances: pallet_balances,
        ParachainStaking: pallet_parachain_staking,
        BlockAuthor: block_author,
        Session: pallet_session,
        CollatorSelection: manta_collator_selection,
    }
);

parameter_types! {
    pub const BlockHashCount: BlockNumber = 250;
    pub const MaximumBlockWeight: u64 = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::one();
    pub const SS58Prefix: u8 = manta_primitives::constants::CALAMARI_SS58PREFIX;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
    type BaseCallFilter = Everything;
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type Nonce = u64;
    type Block = Block;
    type RuntimeCall = RuntimeCall;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type BlockWeights = ();
    type BlockLength = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}
parameter_types! {
    pub const ExistentialDeposit: u128 = 1;
}
impl pallet_balances::Config for Test {
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 4];
    type MaxLocks = ();
    type Balance = Balance;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type RuntimeHoldReason = RuntimeHoldReason;
    type RuntimeFreezeReason = RuntimeFreezeReason;
    type FreezeIdentifier = ();
    type MaxFreezes = ConstU32<1>;
    type MaxHolds = ConstU32<1>;
}
impl block_author::Config for Test {}
parameter_types! {
    pub const MinBlocksPerRound: u32 = 3;
    pub const DefaultBlocksPerRound: u32 = 5;
    pub const LeaveCandidatesDelay: u32 = 2;
    pub const CandidateBondLessDelay: u32 = 2;
    pub const LeaveDelegatorsDelay: u32 = 2;
    pub const RevokeDelegationDelay: u32 = 2;
    pub const DelegationBondLessDelay: u32 = 2;
    pub const RewardPaymentDelay: u32 = 2;
    pub const MinSelectedCandidates: u32 = 5;
    pub const MaxTopDelegationsPerCandidate: u32 = 4;
    pub const MaxBottomDelegationsPerCandidate: u32 = 4;
    pub const MaxDelegationsPerDelegator: u32 = 4;
    pub const DefaultCollatorCommission: Perbill = Perbill::from_percent(20);
    pub const DefaultParachainBondReservePercent: Percent = Percent::from_percent(30);
    pub const MinCollatorStk: u128 = 1;
    pub const MinNormalCandidateStk: u128 = 10;
    pub const MinWhitelistCandidateStk: u128 = 1; // WHITELIST - remove
    pub const MinDelegatorStk: u128 = 5;
    pub const MinDelegation: u128 = 3;
}
impl Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MonetaryGovernanceOrigin = frame_system::EnsureRoot<AccountId>;
    type MinBlocksPerRound = MinBlocksPerRound;
    type DefaultBlocksPerRound = DefaultBlocksPerRound;
    type LeaveCandidatesDelay = LeaveCandidatesDelay;
    type CandidateBondLessDelay = CandidateBondLessDelay;
    type LeaveDelegatorsDelay = LeaveDelegatorsDelay;
    type RevokeDelegationDelay = RevokeDelegationDelay;
    type DelegationBondLessDelay = DelegationBondLessDelay;
    type RewardPaymentDelay = RewardPaymentDelay;
    type MinSelectedCandidates = MinSelectedCandidates;
    type MaxTopDelegationsPerCandidate = MaxTopDelegationsPerCandidate;
    type MaxBottomDelegationsPerCandidate = MaxBottomDelegationsPerCandidate;
    type MaxDelegationsPerDelegator = MaxDelegationsPerDelegator;
    type DefaultCollatorCommission = DefaultCollatorCommission;
    type DefaultParachainBondReservePercent = DefaultParachainBondReservePercent;
    type MinCollatorStk = MinCollatorStk; // WHITELIST - remove
    type MinCandidateStk = MinNormalCandidateStk;
    type MinWhitelistCandidateStk = MinWhitelistCandidateStk; // WHITELIST - remove
    type MinDelegatorStk = MinDelegatorStk;
    type MinDelegation = MinDelegation;
    type BlockAuthor = BlockAuthor;
    type OnCollatorPayout = ();
    type OnNewRound = ();
    type WeightInfo = ();
}

use frame_support::traits::{ValidatorRegistration, ValidatorSet};
/// WHITELIST BEGIN TEMPORARY SECTION FOR TIGHTLY COUPLED COLLATOR_SELECTION/SESSION PALLETS
/// TODO: Remove after end of whitelist-period
use frame_support::{ord_parameter_types, PalletId};
use frame_system::EnsureSignedBy;
use manta_collator_selection::IdentityCollator;
use sp_runtime::traits::ConstU32;

pub struct IsRegistered;
impl ValidatorRegistration<u64> for IsRegistered {
    fn is_registered(id: &u64) -> bool {
        *id != 7u64
    }
}
impl ValidatorSet<u64> for IsRegistered {
    type ValidatorId = u64;
    type ValidatorIdOf = IdentityCollator;
    fn session_index() -> sp_staking::SessionIndex {
        Session::current_index()
    }
    fn validators() -> Vec<Self::ValidatorId> {
        Session::validators()
    }
}
parameter_types! {
    pub const PotId: PalletId = PalletId(*b"PotStake");
}
ord_parameter_types! {
    pub const RootAccount: u64 = 777;
}
impl manta_collator_selection::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type UpdateOrigin = EnsureSignedBy<RootAccount, u64>;
    type PotId = PotId;
    type MaxCandidates = ConstU32<20>;
    type MaxInvulnerables = ConstU32<20>;
    type ValidatorId = <Self as frame_system::Config>::AccountId;
    type ValidatorIdOf = manta_collator_selection::IdentityCollator;
    type AccountIdOf = manta_collator_selection::IdentityCollator;
    type ValidatorRegistration = IsRegistered;
    type WeightInfo = ();
    type CanAuthor = ();
}

use sp_runtime::{traits::OpaqueKeys, RuntimeAppPublic};
parameter_types! {
    pub static SessionHandlerCollators: Vec<u64> = Vec::new();
    pub static SessionChangeBlock: u64 = 0;
}

pub struct TestSessionHandler;
impl pallet_session::SessionHandler<u64> for TestSessionHandler {
    const KEY_TYPE_IDS: &'static [sp_runtime::KeyTypeId] = &[UintAuthorityId::ID];
    fn on_genesis_session<Ks: OpaqueKeys>(keys: &[(u64, Ks)]) {
        SessionHandlerCollators::set(keys.iter().map(|(a, _)| *a).collect::<Vec<_>>())
    }
    fn on_new_session<Ks: OpaqueKeys>(_: bool, keys: &[(u64, Ks)], _: &[(u64, Ks)]) {
        SessionChangeBlock::set(System::block_number() as u64);
        SessionHandlerCollators::set(keys.iter().map(|(a, _)| *a).collect::<Vec<_>>())
    }
    fn on_before_session_ending() {}
    fn on_disabled(_: u32) {}
}

sp_runtime::impl_opaque_keys! {
    pub struct MockSessionKeys {
        // a key for aura authoring
        pub aura: UintAuthorityId,
    }
}
use sp_runtime::testing::UintAuthorityId;
impl From<UintAuthorityId> for MockSessionKeys {
    fn from(aura: sp_runtime::testing::UintAuthorityId) -> Self {
        Self { aura }
    }
}

parameter_types! {
    pub const Offset: BlockNumber = 0;
    pub const Period: BlockNumber = 10;
}
impl pallet_session::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type ValidatorId = <Self as frame_system::Config>::AccountId;
    // we don't have stash and controller, thus we don't need the convert as well.
    type ValidatorIdOf = manta_collator_selection::IdentityCollator;
    type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
    type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
    type SessionManager = CollatorSelection;
    type SessionHandler = TestSessionHandler;
    type Keys = MockSessionKeys;
    type WeightInfo = ();
}
/// WHITELIST END TEMPORARY SECTION FOR TIGHTLY COUPLED COLLATOR_SELECTION/SESSION PALLETS

pub(crate) struct ExtBuilder {
    // endowed accounts with balances
    balances: Vec<(AccountId, Balance)>,
    // [collator, amount]
    collators: Vec<(AccountId, Balance)>,
    // [delegator, collator, delegation_amount]
    delegations: Vec<(AccountId, AccountId, Balance)>,
    // inflation config
    inflation: InflationInfo<Balance>,
}

impl Default for ExtBuilder {
    fn default() -> ExtBuilder {
        ExtBuilder {
            balances: vec![],
            delegations: vec![],
            collators: vec![],
            inflation: InflationInfo {
                expect: Range {
                    min: 700,
                    ideal: 700,
                    max: 700,
                },
                // not used
                annual: Range {
                    min: Perbill::from_percent(50),
                    ideal: Perbill::from_percent(50),
                    max: Perbill::from_percent(50),
                },
                // unrealistically high parameterization, only for testing
                round: Range {
                    min: Perbill::from_percent(5),
                    ideal: Perbill::from_percent(5),
                    max: Perbill::from_percent(5),
                },
            },
        }
    }
}

impl ExtBuilder {
    pub(crate) fn with_balances(mut self, balances: Vec<(AccountId, Balance)>) -> Self {
        self.balances = balances;
        self
    }

    pub(crate) fn with_candidates(mut self, collators: Vec<(AccountId, Balance)>) -> Self {
        self.collators = collators;
        self
    }

    pub(crate) fn with_delegations(
        mut self,
        delegations: Vec<(AccountId, AccountId, Balance)>,
    ) -> Self {
        self.delegations = delegations;
        self
    }

    #[allow(dead_code)]
    pub(crate) fn with_inflation(mut self, inflation: InflationInfo<Balance>) -> Self {
        self.inflation = inflation;
        self
    }

    pub(crate) fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .expect("Frame system builds valid default genesis config");

        pallet_balances::GenesisConfig::<Test> {
            balances: self.balances,
        }
        .assimilate_storage(&mut t)
        .expect("Pallet balances storage can be assimilated");
        pallet_parachain_staking::GenesisConfig::<Test> {
            candidates: self.collators,
            delegations: self.delegations,
            inflation_config: self.inflation,
        }
        .assimilate_storage(&mut t)
        .expect("Parachain Staking's storage can be assimilated");

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

/// Rolls forward one block. Returns the new block number.
pub(crate) fn roll_one_block() -> u64 {
    Balances::on_finalize(System::block_number());
    System::on_finalize(System::block_number());
    System::set_block_number(System::block_number() + 1);
    System::on_initialize(System::block_number());
    Balances::on_initialize(System::block_number());
    ParachainStaking::on_initialize(System::block_number());
    System::block_number()
}

/// Rolls to the desired block. Returns the number of blocks played.
pub(crate) fn roll_to(n: u64) -> u64 {
    let mut num_blocks = 0;
    let mut block = System::block_number();
    while block < n {
        block = roll_one_block();
        num_blocks += 1;
    }
    num_blocks
}

/// Rolls block-by-block to the beginning of the specified round.
/// This will complete the block in which the round change occurs.
/// Returns the number of blocks played.
pub(crate) fn roll_to_round_begin(round: u32) -> u64 {
    let block = (round - 1) * DefaultBlocksPerRound::get();
    roll_to(block.try_into().unwrap())
}

/// Rolls block-by-block to the end of the specified round.
/// The block following will be the one in which the specified round change occurs.
pub(crate) fn roll_to_round_end(round: u32) -> u64 {
    let block = round * DefaultBlocksPerRound::get() - 1;
    roll_to(block.try_into().unwrap())
}

pub(crate) fn last_event() -> RuntimeEvent {
    System::events().pop().expect("Event expected").event
}

pub(crate) fn events() -> Vec<pallet::Event<Test>> {
    System::events()
        .into_iter()
        .map(|r| r.event)
        .filter_map(|e| {
            if let RuntimeEvent::ParachainStaking(inner) = e {
                Some(inner)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

/// Assert input equal to the last event emitted
#[macro_export]
macro_rules! assert_last_event {
    ($event:expr) => {
        match &$event {
            e => assert_eq!(*e, crate::mock::last_event()),
        }
    };
}

/// Compares the system events with passed in events
/// Prints highlighted diff iff assert_eq fails
#[macro_export]
macro_rules! assert_eq_events {
    ($events:expr) => {
        match &$events {
            e => similar_asserts::assert_eq!(*e, crate::mock::events()),
        }
    };
}

/// Compares the last N system events with passed in events, where N is the length of events passed
/// in.
///
/// Prints highlighted diff iff assert_eq fails.
/// The last events from frame_system will be taken in order to match the number passed to this
/// macro. If there are insufficient events from frame_system, they will still be compared; the
/// output may or may not be helpful.
///
/// Examples:
/// If frame_system has events [A, B, C, D, E] and events [C, D, E] are passed in, the result would
/// be a successful match ([C, D, E] == [C, D, E]).
///
/// If frame_system has events [A, B, C, D] and events [B, C] are passed in, the result would be an
/// error and a hopefully-useful diff will be printed between [C, D] and [B, C].
///
/// Note that events are filtered to only match parachain-staking (see events()).
#[macro_export]
macro_rules! assert_eq_last_events {
    ($events:expr $(,)?) => {
        assert_tail_eq!($events, crate::mock::events());
    };
    ($events:expr, $($arg:tt)*) => {
        assert_tail_eq!($events, crate::mock::events(), $($arg)*);
    };
}

/// Assert that one array is equal to the tail of the other. A more generic and testable version of
/// assert_eq_last_events.
#[macro_export]
macro_rules! assert_tail_eq {
    ($tail:expr, $arr:expr $(,)?) => {
        if $tail.len() != 0 {
            // 0-length always passes

            if $tail.len() > $arr.len() {
                similar_asserts::assert_eq!($tail, $arr); // will fail
            }

            let len_diff = $arr.len() - $tail.len();
            similar_asserts::assert_eq!($tail, $arr[len_diff..]);
        }
    };
    ($tail:expr, $arr:expr, $($arg:tt)*) => {
        if $tail.len() != 0 {
            // 0-length always passes

            if $tail.len() > $arr.len() {
                similar_asserts::assert_eq!($tail, $arr, $($arg)*); // will fail
            }

            let len_diff = $arr.len() - $tail.len();
            similar_asserts::assert_eq!($tail, $arr[len_diff..], $($arg)*);
        }
    };
}

/// Panics if an event is not found in the system log of events
#[macro_export]
macro_rules! assert_event_emitted {
    ($event:expr) => {
        match &$event {
            e => {
                assert!(
                    crate::mock::events().iter().find(|x| *x == e).is_some(),
                    "Event {:?} was not found in events: \n {:?}",
                    e,
                    crate::mock::events()
                );
            }
        }
    };
}

/// Panics if an event is found in the system log of events
#[macro_export]
macro_rules! assert_event_not_emitted {
    ($event:expr) => {
        match &$event {
            e => {
                assert!(
                    crate::mock::events().iter().find(|x| *x == e).is_none(),
                    "Event {:?} was found in events: \n {:?}",
                    e,
                    crate::mock::events()
                );
            }
        }
    };
}

// Same storage changes as ParachainStaking::on_finalize
pub(crate) fn set_author(round: u32, acc: u64, pts: u32) {
    <Points<Test>>::mutate(round, |p| *p += pts);
    <AwardedPts<Test>>::mutate(round, acc, |p| *p += pts);
}

/// fn to query the lock amount
pub(crate) fn query_lock_amount(account_id: u64, id: LockIdentifier) -> Option<Balance> {
    for lock in Balances::locks(&account_id) {
        if lock.id == id {
            return Some(lock.amount);
        }
    }
    None
}

#[test]
fn geneses() {
    ExtBuilder::default()
        .with_balances(vec![
            (1, 1000),
            (2, 300),
            (3, 100),
            (4, 100),
            (5, 100),
            (6, 100),
            (7, 100),
            (8, 9),
            (9, 4),
        ])
        .with_candidates(vec![(1, 500), (2, 200)])
        .with_delegations(vec![(3, 1, 100), (4, 1, 100), (5, 2, 100), (6, 2, 100)])
        .build()
        .execute_with(|| {
            assert!(System::events().is_empty());
            // collators
            assert_eq!(
                ParachainStaking::get_collator_stakable_free_balance(&1),
                500
            );
            assert_eq!(query_lock_amount(1, COLLATOR_LOCK_ID), Some(500));
            assert!(ParachainStaking::is_candidate(&1));
            assert_eq!(query_lock_amount(2, COLLATOR_LOCK_ID), Some(200));
            assert_eq!(
                ParachainStaking::get_collator_stakable_free_balance(&2),
                100
            );
            assert!(ParachainStaking::is_candidate(&2));
            // delegators
            for x in 3..7 {
                assert!(ParachainStaking::is_delegator(&x));
                assert_eq!(ParachainStaking::get_delegator_stakable_free_balance(&x), 0);
                assert_eq!(query_lock_amount(x, DELEGATOR_LOCK_ID), Some(100));
            }
            // uninvolved
            for x in 7..10 {
                assert!(!ParachainStaking::is_delegator(&x));
            }
            // no delegator staking locks
            assert_eq!(query_lock_amount(7, DELEGATOR_LOCK_ID), None);
            assert_eq!(
                ParachainStaking::get_delegator_stakable_free_balance(&7),
                100
            );
            assert_eq!(query_lock_amount(8, DELEGATOR_LOCK_ID), None);
            assert_eq!(ParachainStaking::get_delegator_stakable_free_balance(&8), 9);
            assert_eq!(query_lock_amount(9, DELEGATOR_LOCK_ID), None);
            assert_eq!(ParachainStaking::get_delegator_stakable_free_balance(&9), 4);
            // no collator staking locks
            assert_eq!(
                ParachainStaking::get_collator_stakable_free_balance(&7),
                100
            );
            assert_eq!(ParachainStaking::get_collator_stakable_free_balance(&8), 9);
            assert_eq!(ParachainStaking::get_collator_stakable_free_balance(&9), 4);
        });
    ExtBuilder::default()
        .with_balances(vec![
            (1, 100),
            (2, 100),
            (3, 100),
            (4, 100),
            (5, 100),
            (6, 100),
            (7, 100),
            (8, 100),
            (9, 100),
            (10, 100),
        ])
        .with_candidates(vec![(1, 20), (2, 20), (3, 20), (4, 20), (5, 10)])
        .with_delegations(vec![
            (6, 1, 10),
            (7, 1, 10),
            (8, 2, 10),
            (9, 2, 10),
            (10, 1, 10),
        ])
        .build()
        .execute_with(|| {
            assert!(System::events().is_empty());
            // collators
            for x in 1..5 {
                assert!(ParachainStaking::is_candidate(&x));
                assert_eq!(query_lock_amount(x, COLLATOR_LOCK_ID), Some(20));
                assert_eq!(ParachainStaking::get_collator_stakable_free_balance(&x), 80);
            }
            assert!(ParachainStaking::is_candidate(&5));
            assert_eq!(query_lock_amount(5, COLLATOR_LOCK_ID), Some(10));
            assert_eq!(ParachainStaking::get_collator_stakable_free_balance(&5), 90);
            // delegators
            for x in 6..11 {
                assert!(ParachainStaking::is_delegator(&x));
                assert_eq!(query_lock_amount(x, DELEGATOR_LOCK_ID), Some(10));
                assert_eq!(
                    ParachainStaking::get_delegator_stakable_free_balance(&x),
                    90
                );
            }
        });
}

#[frame_support::pallet]
pub mod block_author {
    use super::*;
    use frame_support::{pallet_prelude::*, traits::Get};

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn block_author)]
    pub(super) type BlockAuthor<T> = StorageValue<_, AccountId, ValueQuery>;

    impl<T: Config> Get<AccountId> for Pallet<T> {
        fn get() -> AccountId {
            <BlockAuthor<T>>::get()
        }
    }
}

#[test]
fn roll_to_round_begin_works() {
    ExtBuilder::default().build().execute_with(|| {
        // these tests assume blocks-per-round of 5, as established by DefaultBlocksPerRound
        assert_eq!(System::block_number(), 1); // we start on block 1

        let num_blocks = roll_to_round_begin(1);
        assert_eq!(System::block_number(), 1); // no-op, we're already on this round
        assert_eq!(num_blocks, 0);

        let num_blocks = roll_to_round_begin(2);
        assert_eq!(System::block_number(), 5);
        assert_eq!(num_blocks, 4);

        let num_blocks = roll_to_round_begin(3);
        assert_eq!(System::block_number(), 10);
        assert_eq!(num_blocks, 5);
    });
}

#[test]
fn roll_to_round_end_works() {
    ExtBuilder::default().build().execute_with(|| {
        // these tests assume blocks-per-round of 5, as established by DefaultBlocksPerRound
        assert_eq!(System::block_number(), 1); // we start on block 1

        let num_blocks = roll_to_round_end(1);
        assert_eq!(System::block_number(), 4);
        assert_eq!(num_blocks, 3);

        let num_blocks = roll_to_round_end(2);
        assert_eq!(System::block_number(), 9);
        assert_eq!(num_blocks, 5);

        let num_blocks = roll_to_round_end(3);
        assert_eq!(System::block_number(), 14);
        assert_eq!(num_blocks, 5);
    });
}

#[test]
fn assert_tail_eq_works() {
    assert_tail_eq!(vec![1, 2], vec![0, 1, 2]);

    assert_tail_eq!(vec![1], vec![1]);

    assert_tail_eq!(
        vec![0u32; 0], // 0 length array
        vec![0u32; 1]  // 1-length array
    );

    assert_tail_eq!(vec![0u32, 0], vec![0u32, 0]);
}

#[test]
#[should_panic]
fn assert_tail_eq_panics_on_non_equal_tail() {
    assert_tail_eq!(vec![2, 2], vec![0, 1, 2]);
}

#[test]
#[should_panic]
fn assert_tail_eq_panics_on_empty_arr() {
    assert_tail_eq!(vec![2, 2], vec![0u32; 0]);
}

#[test]
#[should_panic]
fn assert_tail_eq_panics_on_longer_tail() {
    assert_tail_eq!(vec![1, 2, 3], vec![1, 2]);
}

#[test]
#[should_panic]
fn assert_tail_eq_panics_on_unequal_elements_same_length_array() {
    assert_tail_eq!(vec![1, 2, 3], vec![0, 1, 2]);
}
