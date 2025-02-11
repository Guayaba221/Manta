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

use crate::{
    assert_last_event,
    mock::{
        roll_one_block, roll_to, roll_to_round_begin, roll_to_round_end, AccountId, Assets,
        Balance, Balances, ExtBuilder, Farming, Lottery, ParachainStaking, RuntimeOrigin as Origin,
        System, Test, ALICE, BOB, CHARLIE, DAVE, DELEGATOR1, DELEGATOR2, DELEGATOR3, DELEGATOR4,
        DELEGATOR5, DELEGATOR6, DELEGATOR7, DELEGATOR8, EVE, INIT_JUMBO_AMOUNT,
        INIT_V_MANTA_AMOUNT, JUMBO_ID, POOL_ID, V_MANTA_ID,
    },
    Config, Error, FarmingParameters,
};

use frame_support::{assert_noop, assert_ok, traits::Currency};
use frame_system::RawOrigin;
use sp_runtime::TokenError;

const UNIT: Balance = 1_000_000_000_000;
const HIGH_BALANCE: Balance = 1_000_000_000 * UNIT;

#[test]
fn call_manager_extrinsics_as_normal_user_should_not_work() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            Lottery::start_lottery(Origin::signed(1)), // Somebody who is not T::ManageOrigin
            sp_runtime::DispatchError::BadOrigin
        );
        assert_noop!(
            Lottery::stop_lottery(Origin::signed(1)), // Somebody who is not T::ManageOrigin
            sp_runtime::DispatchError::BadOrigin
        );
        assert_noop!(
            Lottery::draw_lottery(Origin::signed(1)), // Somebody who is not T::ManageOrigin
            sp_runtime::DispatchError::BadOrigin
        );
        assert_noop!(
            Lottery::process_matured_withdrawals(Origin::signed(1)), // Somebody who is not T::ManageOrigin
            sp_runtime::DispatchError::BadOrigin
        );
        assert_noop!(
            Lottery::liquidate_lottery(Origin::signed(1)), // Somebody who is not T::ManageOrigin
            sp_runtime::DispatchError::BadOrigin
        );
        assert_noop!(
            Lottery::rebalance_stake(Origin::signed(1)), // Somebody who is not T::ManageOrigin
            sp_runtime::DispatchError::BadOrigin
        );
    });
}

#[test]
fn starting_lottery_without_gas_should_not_work() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            Lottery::start_lottery(RawOrigin::Root.into()),
            Error::<Test>::PotBalanceBelowGasReserve
        );
    });
}

#[test]
fn starting_funded_lottery_should_work() {
    ExtBuilder::default()
        .with_funded_lottery_account(HIGH_BALANCE)
        .build()
        .execute_with(|| {
            assert_ok!(Lottery::start_lottery(RawOrigin::Root.into()));
            assert_last_event!(crate::mock::RuntimeEvent::Lottery(
                crate::Event::LotteryStarted
            ));
            assert_noop!(
                Lottery::start_lottery(RawOrigin::Root.into()),
                Error::<Test>::LotteryIsRunning
            ); // Ensure double-starting fails
        });
}

#[test]
fn restarting_funded_lottery_should_work() {
    ExtBuilder::default()
        .with_funded_lottery_account(HIGH_BALANCE)
        .build()
        .execute_with(|| {
            assert_ok!(Lottery::start_lottery(RawOrigin::Root.into()));
            assert_ok!(Lottery::stop_lottery(RawOrigin::Root.into()));
            assert_last_event!(crate::mock::RuntimeEvent::Lottery(
                crate::Event::LotteryStopped
            ));
            assert_noop!(
                Lottery::stop_lottery(RawOrigin::Root.into()),
                Error::<Test>::LotteryNotStarted
            ); // Ensure double-stopping fails
            assert_ok!(Lottery::start_lottery(RawOrigin::Root.into()));
            assert_last_event!(crate::mock::RuntimeEvent::Lottery(
                crate::Event::LotteryStarted
            ));
        });
}

#[test]
fn depositing_and_withdrawing_in_freezeout_should_not_work() {
    let balance = 300_000_000 * UNIT;
    ExtBuilder::default()
        .with_balances(vec![(ALICE, HIGH_BALANCE), (BOB, HIGH_BALANCE)])
        .with_candidates(vec![(BOB, balance)])
        .with_funded_lottery_account(HIGH_BALANCE)
        .build()
        .execute_with(|| {
            assert!(HIGH_BALANCE > balance);
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), balance));
            assert_eq!(Lottery::sum_of_deposits(), balance);
            assert_eq!(Lottery::total_pot(), balance);
            assert_ok!(Lottery::start_lottery(RawOrigin::Root.into(),));
            assert!(Lottery::not_in_drawing_freezeout());
            roll_to(
                Lottery::next_drawing_at().unwrap()
                    - <u32 as Into<u64>>::into(<Test as Config>::DrawingFreezeout::get()),
            );
            assert!(!Lottery::not_in_drawing_freezeout());
            assert_noop!(
                Lottery::deposit(Origin::signed(ALICE), balance),
                Error::<Test>::TooCloseToDrawing
            );
            assert_noop!(
                Lottery::request_withdraw(Origin::signed(ALICE), balance),
                Error::<Test>::TooCloseToDrawing
            );
            assert_eq!(Lottery::sum_of_deposits(), balance);
            assert_eq!(Lottery::total_pot(), balance);
        });
}

#[test]
fn depositing_and_withdrawing_should_work() {
    let balance = 500_000_000 * UNIT;
    ExtBuilder::default()
        .with_balances(vec![(ALICE, HIGH_BALANCE), (BOB, HIGH_BALANCE)])
        .with_candidates(vec![(BOB, balance)])
        .with_funded_lottery_account(HIGH_BALANCE)
        .build()
        .execute_with(|| {
            assert!(HIGH_BALANCE > balance);
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), balance));
            assert_last_event!(crate::mock::RuntimeEvent::Lottery(
                crate::Event::Deposited {
                    account: ALICE,
                    amount: balance
                }
            ));
            assert_eq!(Lottery::active_balance_per_user(ALICE), balance);
            assert_eq!(Lottery::sum_of_deposits(), balance);
            assert_eq!(Lottery::total_pot(), balance);

            assert_ok!(Lottery::request_withdraw(Origin::signed(ALICE), balance));
            assert_last_event!(crate::mock::RuntimeEvent::Lottery(
                crate::Event::ScheduledWithdraw {
                    account: ALICE,
                    amount: balance
                }
            ));
            assert_eq!(Lottery::sum_of_deposits(), balance);
            assert_eq!(Lottery::active_balance_per_user(ALICE), 0);
            assert_eq!(Lottery::total_pot(), 0);
            assert_eq!(Lottery::withdrawal_request_queue().len(), 1);
            assert_eq!(Lottery::surplus_unstaking_balance(), 0);
        });
}

#[test]
fn depositing_and_withdrawing_partial_in_one_block_should_work() {
    let balance = 500_000_000 * UNIT;
    let half_balance = 250_000_000 * UNIT;
    ExtBuilder::default()
        .with_balances(vec![(ALICE, HIGH_BALANCE), (BOB, HIGH_BALANCE)])
        .with_candidates(vec![(BOB, balance)])
        .with_funded_lottery_account(HIGH_BALANCE)
        .build()
        .execute_with(|| {
            assert!(HIGH_BALANCE > balance);
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), balance));
            assert_eq!(Lottery::active_balance_per_user(ALICE), balance);
            assert_eq!(Lottery::sum_of_deposits(), balance);
            assert_eq!(Lottery::total_pot(), balance);
            assert_eq!(Lottery::staked_collators(BOB), balance);
            assert_eq!(crate::UnstakingCollators::<Test>::get().len(), 0);

            assert_ok!(Lottery::request_withdraw(
                Origin::signed(ALICE),
                half_balance
            ));
            assert_eq!(Lottery::sum_of_deposits(), balance);
            assert_eq!(Lottery::active_balance_per_user(ALICE), half_balance);
            assert_eq!(Lottery::total_pot(), half_balance);
            assert_eq!(Lottery::withdrawal_request_queue().len(), 1);
            assert_eq!(Lottery::surplus_unstaking_balance(), half_balance);
            assert_eq!(crate::UnstakingCollators::<Test>::get().len(), 1);
            assert_eq!(Lottery::staked_collators(BOB), balance);

            assert_ok!(Lottery::request_withdraw(
                Origin::signed(ALICE),
                half_balance
            ));
            assert_eq!(Lottery::sum_of_deposits(), balance);
            assert_eq!(Lottery::active_balance_per_user(ALICE), 0);
            assert_eq!(Lottery::total_pot(), 0);
            assert_eq!(Lottery::withdrawal_request_queue().len(), 2);
            assert_eq!(Lottery::surplus_unstaking_balance(), 0);
            assert_eq!(Lottery::staked_collators(BOB), balance);
            assert_eq!(crate::UnstakingCollators::<Test>::get().len(), 1);

            assert_noop!(
                Lottery::request_withdraw(Origin::signed(ALICE), half_balance),
                Error::<Test>::NoDepositForAccount
            );

            assert_noop!(
                Lottery::deposit(Origin::signed(ALICE), half_balance),
                Error::<Test>::NoCollatorForDeposit
            );
            assert_noop!(
                Lottery::deposit(Origin::signed(BOB), half_balance),
                Error::<Test>::NoCollatorForDeposit
            );
        });
}

#[test]
fn processing_withdrawing_leaves_correct_balance_with_user() {
    let balance = 500_000_000 * UNIT;
    ExtBuilder::default()
        .with_balances(vec![(ALICE, HIGH_BALANCE), (BOB, HIGH_BALANCE)])
        .with_candidates(vec![(BOB, balance)])
        .with_funded_lottery_account(HIGH_BALANCE)
        .build()
        .execute_with(|| {
            assert!(HIGH_BALANCE > balance);
            let alice_starting_balance = Balances::free_balance(ALICE);
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), balance));
            assert_eq!(
                Balances::free_balance(ALICE),
                alice_starting_balance - balance
            );
            assert_eq!(Lottery::sum_of_deposits(), balance);
            assert_eq!(Lottery::total_pot(), balance);

            assert_ok!(Lottery::request_withdraw(Origin::signed(ALICE), balance));
            assert_eq!(Lottery::sum_of_deposits(), balance);
            assert_eq!(Lottery::total_pot(), 0);
            let alice_balance_after_request = Balances::free_balance(ALICE);
            assert_eq!(
                alice_balance_after_request,
                alice_starting_balance - balance
            );
            assert_eq!(Lottery::withdrawal_request_queue().len(), 1);

            roll_to_round_begin(3);
            assert_ok!(Lottery::process_matured_withdrawals(RawOrigin::Root.into()));
            assert_eq!(Lottery::sum_of_deposits(), 0);
            assert_eq!(
                Balances::free_balance(ALICE),
                alice_balance_after_request + balance
            );
            assert_eq!(Balances::free_balance(ALICE), alice_starting_balance);
            assert_eq!(Lottery::withdrawal_request_queue().len(), 0);
            assert_eq!(Lottery::unlocked_unstaking_funds(), 0);
        });
}

#[test]
fn multiple_request_withdraw_processing_withdrawing_leaves_correct_balance_with_user() {
    let balance = 500_000_000 * UNIT;
    let one = 100_000_000 * UNIT;
    ExtBuilder::default()
        .with_balances(vec![(ALICE, HIGH_BALANCE), (BOB, HIGH_BALANCE)])
        .with_candidates(vec![(BOB, balance)])
        .with_funded_lottery_account(HIGH_BALANCE)
        .build()
        .execute_with(|| {
            assert!(HIGH_BALANCE > balance);
            let alice_starting_balance = Balances::free_balance(ALICE);
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), balance));
            assert_eq!(
                Balances::free_balance(ALICE),
                alice_starting_balance - balance
            );
            assert_eq!(Lottery::sum_of_deposits(), balance);
            assert_eq!(Lottery::total_pot(), balance);

            // request withdraw 5 times
            for i in 1..6 {
                assert_ok!(Lottery::request_withdraw(Origin::signed(ALICE), one));
                assert_eq!(Lottery::total_pot(), balance - i * one);
            }

            assert_eq!(Lottery::sum_of_deposits(), balance);
            assert_eq!(Lottery::total_pot(), 0);
            let alice_balance_after_request = Balances::free_balance(ALICE);
            assert_eq!(
                alice_balance_after_request,
                alice_starting_balance - balance
            );
            assert_eq!(Lottery::withdrawal_request_queue().len(), 5);

            roll_to_round_begin(3);
            assert_ok!(Lottery::process_matured_withdrawals(RawOrigin::Root.into()));
            assert_eq!(Lottery::sum_of_deposits(), 0);
            assert_eq!(
                Balances::free_balance(ALICE),
                alice_balance_after_request + balance
            );
            assert_eq!(Balances::free_balance(ALICE), alice_starting_balance);
            assert_eq!(Lottery::withdrawal_request_queue().len(), 0);
            assert_eq!(Lottery::unlocked_unstaking_funds(), 0);
        });
}

#[test]
fn double_processing_withdrawals_does_not_double_pay() {
    let balance = 500_000_000 * UNIT;
    ExtBuilder::default()
        .with_balances(vec![(ALICE, HIGH_BALANCE), (BOB, HIGH_BALANCE)])
        .with_candidates(vec![(BOB, balance)])
        .with_funded_lottery_account(HIGH_BALANCE)
        .build()
        .execute_with(|| {
            assert!(HIGH_BALANCE > balance);
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), balance));
            assert_ok!(Lottery::request_withdraw(Origin::signed(ALICE), balance));
            let alice_balance_after_request = Balances::free_balance(ALICE);
            roll_to_round_begin(3);
            assert_ok!(Lottery::process_matured_withdrawals(RawOrigin::Root.into()));
            assert_eq!(
                Balances::free_balance(ALICE),
                alice_balance_after_request + balance
            );
            assert_ok!(Lottery::process_matured_withdrawals(RawOrigin::Root.into()));
            assert_eq!(
                Balances::free_balance(ALICE),
                alice_balance_after_request + balance
            );
        });
}

#[test]
fn deposit_staking_to_one_underallocated_collator_works() {
    let balance4 = 40_000_000 * UNIT;
    let balance5 = 50_000_000 * UNIT;
    let balance6 = 60_000_000 * UNIT;
    ExtBuilder::default()
        .with_balances(vec![
            (ALICE, HIGH_BALANCE),
            (BOB, HIGH_BALANCE),
            (CHARLIE, HIGH_BALANCE),
        ])
        .with_candidates(vec![
            (ALICE, balance4),
            (BOB, balance5),
            (CHARLIE, balance6),
        ])
        .build()
        .execute_with(|| {
            assert!(HIGH_BALANCE > balance6 + balance5 + balance4);
            assert_eq!(
                ParachainStaking::candidate_info(ALICE)
                    .unwrap()
                    .total_counted,
                balance4
            );
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), balance6));
            // Median = 50k, ALICE is the only underallocated collator, gets all token
            assert_eq!(
                ParachainStaking::candidate_info(ALICE)
                    .unwrap()
                    .total_counted,
                balance4 + balance6
            );

            assert_ok!(Lottery::deposit(Origin::signed(ALICE), balance5));
            //  Median = 60k, BOB is the only underallocated, gets all token
            assert_eq!(
                ParachainStaking::candidate_info(BOB).unwrap().total_counted,
                balance5 + balance5
            );

            assert_ok!(Lottery::deposit(Origin::signed(ALICE), balance4));
            // Median = 100k CHARLIE is the only underallocated, gets all token
            assert_eq!(
                ParachainStaking::candidate_info(CHARLIE)
                    .unwrap()
                    .total_counted,
                balance6 + balance4
            );

            // Now all 3 tie at 100k, there is no underallocation, deposit is given randomly
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), balance6));
            assert!(
                (ParachainStaking::candidate_info(ALICE)
                    .unwrap()
                    .total_counted
                    == balance4 + balance6 + balance6)
                    || (ParachainStaking::candidate_info(BOB).unwrap().total_counted
                        == balance5 + balance5 + balance6)
                    || (ParachainStaking::candidate_info(CHARLIE)
                        .unwrap()
                        .total_counted
                        == balance6 + balance4 + balance6),
            );
        });
}

#[test]
fn unstaking_works_with_zero_collators_left() {
    let balance = 50_000_000 * UNIT;
    ExtBuilder::default()
        .with_balances(vec![(ALICE, HIGH_BALANCE), (BOB, HIGH_BALANCE)])
        .with_candidates(vec![(ALICE, balance), (BOB, balance)])
        .with_funded_lottery_account(HIGH_BALANCE)
        .build()
        .execute_with(|| {
            assert!(HIGH_BALANCE > balance);
            assert_eq!(
                ParachainStaking::candidate_info(ALICE)
                    .unwrap()
                    .total_counted,
                balance
            );
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 0);
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), balance));
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), balance));
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 2);
            assert_eq!(Balances::free_balance(ALICE), HIGH_BALANCE - 2 * balance);
            assert_eq!(
                Balances::free_balance(crate::Pallet::<Test>::account_id()),
                HIGH_BALANCE + 2 * balance
            );
            assert_eq!(
                ParachainStaking::candidate_info(ALICE)
                    .unwrap()
                    .total_counted,
                balance * 2
            );
            assert_eq!(
                ParachainStaking::candidate_info(BOB).unwrap().total_counted,
                balance * 2
            );

            assert_ok!(Lottery::request_withdraw(
                Origin::signed(ALICE),
                balance * 2
            ));
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 2);
            assert_eq!(crate::UnstakingCollators::<Test>::get().len(), 2);
            assert_eq!(Lottery::withdrawal_request_queue().len(), 1);

            assert_ok!(Lottery::start_lottery(RawOrigin::Root.into()));
            roll_to_round_begin(3);
            // by now the withdrawal should have happened by way of lottery drawing
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 0);
            assert_eq!(crate::UnstakingCollators::<Test>::get().len(), 0);
            assert_eq!(Lottery::withdrawal_request_queue().len(), 0);
            assert_eq!(Lottery::surplus_unstaking_balance(), 0);
            assert_eq!(Lottery::unlocked_unstaking_funds(), 0);
            assert_eq!(Balances::free_balance(ALICE), HIGH_BALANCE);
            assert_eq!(
                Balances::free_balance(crate::Pallet::<Test>::account_id()),
                HIGH_BALANCE
            );
        });
}

#[test]
fn winner_distribution_should_be_equality_with_equal_deposits() {
    let balance = 500_000_000 * UNIT;
    ExtBuilder::default()
        .with_balances(vec![
            (ALICE, HIGH_BALANCE),
            (BOB, HIGH_BALANCE),
        ])
        .with_candidates(vec![(BOB, balance)])
        .build()
        .execute_with(|| {
            assert!(HIGH_BALANCE > balance);
            <Test as pallet_parachain_staking::Config>::Currency::make_free_balance_be(&Lottery::account_id(), Lottery::gas_reserve());
            // Deposit 50 users with equal deposits
            const WINNING_AMT: u32 = 1;
            const NUMBER_OF_DRAWINGS: u32 = 10_000;
            const NUMBER_OF_USERS: u32 = 50;
            const USER_SEED: u32 = 696_969;
            let min_delegator_bond =
                <<Test as pallet_parachain_staking::Config>::MinDelegatorStk as frame_support::traits::Get<pallet_parachain_staking::BalanceOf<Test>>>::get();
            let deposit_amount: pallet_parachain_staking::BalanceOf<Test> = min_delegator_bond * 10_000u128;
            for user in 0..NUMBER_OF_USERS {
                System::set_block_number(user.into());
                let (depositor, _) =
                    crate::mock::from_bench::create_funded_user::<Test>("depositor", USER_SEED - 1 - user, deposit_amount);
                assert_ok!(Lottery::deposit(
                    RawOrigin::Signed(depositor).into(),
                    deposit_amount
                ));
            }
            // loop 10000 times, starting from block 5000 to hopefully be outside of drawing freezeout
            for x in 5_000..5_000+NUMBER_OF_DRAWINGS {
                // advance block number to reseed RNG
                System::set_block_number((NUMBER_OF_USERS + x).into());
                // simulate accrued staking rewards
                assert_ok!(Balances::deposit_into_existing(
                    &Lottery::account_id(),
                    WINNING_AMT.into(),
                ));
                // draw lottery
                assert_ok!(Lottery::draw_lottery(RawOrigin::Root.into()));
            }
            assert_eq!(
                Lottery::total_unclaimed_winnings(),
                NUMBER_OF_DRAWINGS as u128 * WINNING_AMT as u128
            );

            // ensure every user has won > 190 times ( 200 optimum = NUMBER_OF_DRAWINGS/NUMBER_OF_USERS )
            let mut winners = vec![];
            for (user, user_winnings) in crate::UnclaimedWinningsByAccount::<Test>::iter() {
                winners.push((user,user_winnings));
                assert!(user_winnings >= (WINNING_AMT as f32 * NUMBER_OF_DRAWINGS as f32 / NUMBER_OF_USERS as f32 * 0.80) as u128);
            }
            log::error!("{:?}",winners);
            assert_eq!(
                winners.len() as u32,
                NUMBER_OF_USERS
            );
        });
}

#[test]
fn depsiting_to_one_collator_multiple_times_in_one_block_should_work() {
    let balance = 50_000_000 * UNIT;
    ExtBuilder::default()
        .with_balances(vec![(ALICE, HIGH_BALANCE), (BOB, HIGH_BALANCE)])
        .with_candidates(vec![(BOB, balance)])
        .with_funded_lottery_account(HIGH_BALANCE)
        .build()
        .execute_with(|| {
            assert!(HIGH_BALANCE > balance);
            assert_eq!(Lottery::staked_collators(BOB), 0);
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), balance));
            assert_eq!(Lottery::staked_collators(BOB), balance);
            assert_last_event!(crate::mock::RuntimeEvent::Lottery(
                crate::Event::Deposited {
                    account: ALICE,
                    amount: balance
                }
            ));
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), balance));
            assert_eq!(Lottery::staked_collators(BOB), 2 * balance);
            assert_last_event!(crate::mock::RuntimeEvent::Lottery(
                crate::Event::Deposited {
                    account: ALICE,
                    amount: balance
                }
            ));
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), balance));
            assert_eq!(Lottery::staked_collators(BOB), 3 * balance);
            assert_last_event!(crate::mock::RuntimeEvent::Lottery(
                crate::Event::Deposited {
                    account: ALICE,
                    amount: balance
                }
            ));
            assert_eq!(Lottery::sum_of_deposits(), 3 * balance);
        });
}

#[test]
fn depsiting_to_two_collator_multiple_times_in_one_block_should_work() {
    let balance = 50_000_000 * UNIT;
    let balance1 = 20_000_000 * UNIT;
    ExtBuilder::default()
        .with_balances(vec![(ALICE, HIGH_BALANCE), (BOB, HIGH_BALANCE)])
        .with_candidates(vec![(ALICE, balance1), (BOB, balance)])
        .with_funded_lottery_account(HIGH_BALANCE)
        .build()
        .execute_with(|| {
            assert!(HIGH_BALANCE > balance);
            assert_eq!(Lottery::staked_collators(BOB), 0);
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), balance1));
            assert_eq!(Lottery::staked_collators(ALICE), balance1);
            assert_last_event!(crate::mock::RuntimeEvent::Lottery(
                crate::Event::Deposited {
                    account: ALICE,
                    amount: balance1
                }
            ));
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), balance1));
            assert_eq!(Lottery::staked_collators(ALICE), 2 * balance1);
            assert_last_event!(crate::mock::RuntimeEvent::Lottery(
                crate::Event::Deposited {
                    account: ALICE,
                    amount: balance1
                }
            ));
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), balance1));
            assert_eq!(Lottery::staked_collators(BOB), balance1);
            assert_last_event!(crate::mock::RuntimeEvent::Lottery(
                crate::Event::Deposited {
                    account: ALICE,
                    amount: balance1
                }
            ));
            assert_eq!(Lottery::sum_of_deposits(), 3 * balance1);
        });
}

#[test]
fn deposit_withdraw_deposit_to_new_joined_collator_works() {
    let balance = 50_000_000 * UNIT;
    ExtBuilder::default()
        .with_balances(vec![(ALICE, HIGH_BALANCE), (BOB, HIGH_BALANCE)])
        .with_candidates(vec![(BOB, balance)])
        .with_funded_lottery_account(HIGH_BALANCE)
        .build()
        .execute_with(|| {
            assert!(HIGH_BALANCE > balance);
            assert_eq!(Lottery::staked_collators(BOB), 0);
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), balance));
            assert_eq!(Lottery::staked_collators(BOB), balance);

            assert_ok!(Lottery::request_withdraw(Origin::signed(ALICE), balance));
            assert_eq!(Lottery::staked_collators(BOB), balance);
            assert_eq!(Lottery::withdrawal_request_queue().len(), 1);
            assert_eq!(ParachainStaking::selected_candidates().len(), 1);

            // join a new collator because BOB is now ineligible to receive deposits
            assert_noop!(
                Lottery::deposit(Origin::signed(ALICE), balance),
                Error::<Test>::NoCollatorForDeposit
            );
            let (new_collator, _) = crate::mock::from_bench::create_funded_user::<Test>(
                "collator",
                0xDEADBEEF,
                HIGH_BALANCE,
            );
            assert_ok!(ParachainStaking::join_candidates(
                Origin::signed(new_collator),
                balance,
                10
            ));
            assert_eq!(ParachainStaking::candidate_pool().len(), 2);

            roll_to_round_begin(2);
            assert_eq!(ParachainStaking::selected_candidates().len(), 2);
            assert_eq!(ParachainStaking::selected_candidates()[1], new_collator);
            // pretend the collator got some rewards
            pallet_parachain_staking::AwardedPts::<Test>::insert(1, new_collator, 20);
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), balance));
            assert_eq!(Lottery::staked_collators(new_collator), balance);
        });
}

#[test]
fn deposit_withdraw_partial_draw_lottery_works() {
    let balance = 500_000_000 * UNIT;
    let half_balance = 250_000_000 * UNIT;
    let quarter_balance = 125_000_000 * UNIT;
    ExtBuilder::default()
        .with_balances(vec![(ALICE, HIGH_BALANCE), (BOB, HIGH_BALANCE)])
        .with_candidates(vec![(BOB, balance)])
        .with_funded_lottery_account(balance)
        .build()
        .execute_with(|| {
            assert!(HIGH_BALANCE > balance);
            assert_eq!(0, Lottery::staked_collators(BOB));
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), balance));
            let alice_post_deposit_balance = Balances::free_balance(ALICE);
            assert_eq!(balance, Lottery::staked_collators(BOB));
            assert_eq!(balance, Lottery::total_pot());
            assert_eq!(balance, Lottery::sum_of_deposits());

            assert_ok!(Lottery::request_withdraw(
                Origin::signed(ALICE),
                half_balance
            ));
            // surplus = balance - half_balance = half_balance
            assert_eq!(half_balance, Lottery::surplus_unstaking_balance());

            roll_one_block();
            assert_ok!(Lottery::request_withdraw(
                Origin::signed(ALICE),
                quarter_balance
            ));
            assert_eq!(balance, Lottery::staked_collators(BOB));
            // surplus = half_balance - quarter_balance = quarter_balance
            assert_eq!(quarter_balance, Lottery::surplus_unstaking_balance());

            pallet_parachain_staking::AwardedPts::<Test>::insert(2, BOB, 20);
            roll_to_round_begin(3);
            // funds should be unlocked now and BOB is finished unstaking, so it's eligible for redepositing
            assert_ok!(Lottery::draw_lottery(RawOrigin::Root.into()));
            assert_eq!(
                alice_post_deposit_balance + half_balance + quarter_balance,
                Balances::free_balance(ALICE)
            );
            assert_eq!(0, Lottery::surplus_unstaking_balance());
            assert_eq!(0, Lottery::unlocked_unstaking_funds());
            assert!(Lottery::withdrawal_request_queue().is_empty());
            assert!(crate::UnstakingCollators::<Test>::get().is_empty());
            // draw lottery rebalance will restake surplus funds to collators.
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 1);
            assert_eq!(quarter_balance, Lottery::staked_collators(BOB));
            assert_eq!(quarter_balance, Lottery::total_pot());
            assert_eq!(quarter_balance, Lottery::sum_of_deposits());
        });
}

#[test]
fn multiround_withdraw_partial_deposit_works() {
    let balance = 500_000_000 * UNIT;
    let half_balance = 250_000_000 * UNIT;
    let quarter_balance = 125_000_000 * UNIT;
    ExtBuilder::default()
        .with_balances(vec![(ALICE, HIGH_BALANCE), (BOB, HIGH_BALANCE)])
        .with_candidates(vec![(BOB, balance)])
        .with_funded_lottery_account(balance)
        .build()
        .execute_with(|| {
            assert!(HIGH_BALANCE > balance);
            // one round to unstake
            assert!(
                <Test as pallet_parachain_staking::Config>::LeaveCandidatesDelay::get() == 1u32
            );
            assert_eq!(0, Lottery::staked_collators(BOB));
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), balance));
            let alice_post_deposit_balance = Balances::free_balance(ALICE);
            assert_eq!(balance, Lottery::staked_collators(BOB));
            assert_eq!(balance, Lottery::total_pot());
            assert_eq!(balance, Lottery::sum_of_deposits());

            roll_one_block(); // ensure this unlocks *after* round 3 start
            assert_ok!(Lottery::request_withdraw(
                Origin::signed(ALICE),
                half_balance
            ));
            assert_eq!(balance, Lottery::staked_collators(BOB));
            assert_eq!(1, Lottery::withdrawal_request_queue().len());
            assert_eq!(half_balance, Lottery::surplus_unstaking_balance());

            // withdrawing funds are still locked
            roll_to_round_begin(2);
            roll_one_block(); // ensure this unlocks *after* round 3 start
            pallet_parachain_staking::AwardedPts::<Test>::insert(1, BOB, 20);
            assert_ok!(Lottery::request_withdraw(
                Origin::signed(ALICE),
                quarter_balance
            ));
            assert_eq!(balance, Lottery::staked_collators(BOB));
            assert_eq!(2, Lottery::withdrawal_request_queue().len());
            assert_eq!(
                half_balance - quarter_balance,
                Lottery::surplus_unstaking_balance()
            );
            assert_ok!(Lottery::draw_lottery(RawOrigin::Root.into()));
            assert_eq!(balance, Lottery::staked_collators(BOB));
            assert_eq!(
                half_balance - quarter_balance,
                Lottery::surplus_unstaking_balance()
            );
            assert_eq!(0, Lottery::unlocked_unstaking_funds());
            assert_eq!(2, Lottery::withdrawal_request_queue().len());

            // collator becomes unstaked on draw_lottery, must keep quarter for withdrawal, can restake other quarter
            roll_to_round_begin(3);
            pallet_parachain_staking::AwardedPts::<Test>::insert(2, BOB, 20);
            assert_ok!(Lottery::draw_lottery(RawOrigin::Root.into()));
            // balance - half - quarter
            assert_eq!(quarter_balance, Lottery::staked_collators(BOB));
            assert_eq!(quarter_balance, Lottery::unlocked_unstaking_funds());
            assert_eq!(0, Lottery::surplus_unstaking_balance());
            assert_eq!(
                alice_post_deposit_balance + half_balance,
                Balances::free_balance(ALICE)
            );
            assert_eq!(1, Lottery::withdrawal_request_queue().len());
            assert!(crate::UnstakingCollators::<Test>::get().is_empty());

            roll_to_round_begin(4);
            pallet_parachain_staking::AwardedPts::<Test>::insert(3, BOB, 20);
            // second withdrawal can be paid out at new round
            assert_ok!(Lottery::draw_lottery(RawOrigin::Root.into()));
            assert_eq!(
                alice_post_deposit_balance + half_balance + quarter_balance,
                Balances::free_balance(ALICE)
            );
            assert_eq!(quarter_balance, Lottery::staked_collators(BOB));
            assert_eq!(0, Lottery::surplus_unstaking_balance());
            assert_eq!(0, Lottery::unlocked_unstaking_funds());
            assert!(Lottery::withdrawal_request_queue().is_empty());
        });
}

#[test]
fn multiround_withdraw_partial_with_two_collators_works() {
    let reserve = 10_000 * UNIT;
    let balance = 500_000_000 * UNIT;
    let quarter_balance = 125_000_000 * UNIT;
    ExtBuilder::default()
        .with_balances(vec![
            (ALICE, HIGH_BALANCE),
            (BOB, HIGH_BALANCE),
            (CHARLIE, HIGH_BALANCE),
        ])
        .with_candidates(vec![(BOB, balance), (CHARLIE, balance)])
        .with_funded_lottery_account(reserve) // minimally fund lottery
        .build()
        .execute_with(|| {
            assert_eq!(reserve, Lottery::gas_reserve()); // XXX: Cant use getter in the ExtBuilder
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), balance));
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 1);
            assert_ok!(Lottery::request_withdraw(
                Origin::signed(ALICE),
                quarter_balance
            ));

            roll_to_round_begin(2);
            assert_ok!(Lottery::draw_lottery(RawOrigin::Root.into()));
            roll_one_block();
            assert_ok!(Lottery::request_withdraw(
                Origin::signed(ALICE),
                quarter_balance
            ));
            assert_ok!(Lottery::request_withdraw(
                Origin::signed(ALICE),
                quarter_balance
            ));
            assert_eq!(3, Lottery::withdrawal_request_queue().len());
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 1);
            assert_eq!(crate::UnstakingCollators::<Test>::get().len(), 1);

            roll_to_round_begin(3);
            pallet_parachain_staking::AwardedPts::<Test>::insert(3, BOB, 20);
            pallet_parachain_staking::AwardedPts::<Test>::insert(3, CHARLIE, 20);
            assert_ok!(Lottery::draw_lottery(RawOrigin::Root.into()));
            assert_eq!(2, Lottery::withdrawal_request_queue().len());
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 0);
            assert_eq!(crate::UnstakingCollators::<Test>::get().len(), 0);

            roll_to_round_begin(4);
            pallet_parachain_staking::AwardedPts::<Test>::insert(4, BOB, 20);
            pallet_parachain_staking::AwardedPts::<Test>::insert(4, CHARLIE, 20);
            assert_ok!(Lottery::draw_lottery(RawOrigin::Root.into()));
            assert_eq!(0, Lottery::unlocked_unstaking_funds());
            assert_eq!(0, Lottery::surplus_unstaking_balance());
            assert!(Lottery::withdrawal_request_queue().is_empty());
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 1);
            assert_eq!(crate::UnstakingCollators::<Test>::get().len(), 0);
        });
}

#[test]
fn delegator_less_than_bottom_cannot_deposit() {
    let reserve = 10_000 * UNIT;
    let balance = 500_000_000 * UNIT;
    let delegate_amt = 100_000_000 * UNIT;
    let delegate_amt_1 = 10_000_000 * UNIT;
    ExtBuilder::default()
        .with_balances(vec![
            (ALICE, HIGH_BALANCE),
            (BOB, HIGH_BALANCE),
            (CHARLIE, HIGH_BALANCE),
            (DAVE, HIGH_BALANCE),
            (DELEGATOR1, HIGH_BALANCE),
            (DELEGATOR2, HIGH_BALANCE),
            (DELEGATOR3, HIGH_BALANCE),
            (DELEGATOR4, HIGH_BALANCE),
            (DELEGATOR5, HIGH_BALANCE),
            (DELEGATOR6, HIGH_BALANCE),
            (DELEGATOR7, HIGH_BALANCE),
            (DELEGATOR8, HIGH_BALANCE),
        ])
        .with_candidates(vec![(BOB, balance), (CHARLIE, balance)])
        .with_delegations(vec![
            (DELEGATOR1, BOB, delegate_amt),
            (DELEGATOR2, BOB, delegate_amt),
            (DELEGATOR3, BOB, delegate_amt),
            (DELEGATOR4, BOB, delegate_amt),
            (DELEGATOR5, BOB, delegate_amt),
            (DELEGATOR6, BOB, delegate_amt),
            (DELEGATOR7, BOB, delegate_amt),
            (DELEGATOR8, BOB, delegate_amt),
            (DELEGATOR1, CHARLIE, delegate_amt),
            (DELEGATOR2, CHARLIE, delegate_amt),
            (DELEGATOR3, CHARLIE, delegate_amt),
            (DELEGATOR4, CHARLIE, delegate_amt),
            (DELEGATOR5, CHARLIE, delegate_amt),
            (DELEGATOR6, CHARLIE, delegate_amt),
            (DELEGATOR7, CHARLIE, delegate_amt),
            (DELEGATOR8, CHARLIE, delegate_amt),
        ])
        .with_funded_lottery_account(reserve)
        .build()
        .execute_with(|| {
            assert_noop!(
                Lottery::deposit(Origin::signed(ALICE), delegate_amt_1),
                pallet_parachain_staking::Error::<Test>::CannotDelegateLessThanOrEqualToLowestBottomWhenFull
            );
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 0);
        });
}

#[test]
fn delegator_more_than_bottom_can_deposit_twocollator_secondfailed() {
    let reserve = 10_000 * UNIT;
    let balance = 500_000_000 * UNIT;
    let quarter_balance = 125_000_000 * UNIT;
    let delegate_amt = 100_000_000 * UNIT;
    ExtBuilder::default()
        .with_balances(vec![
            (ALICE, HIGH_BALANCE),
            (BOB, HIGH_BALANCE),
            (CHARLIE, HIGH_BALANCE),
            (DAVE, HIGH_BALANCE),
            (DELEGATOR1, HIGH_BALANCE),
            (DELEGATOR2, HIGH_BALANCE),
            (DELEGATOR3, HIGH_BALANCE),
            (DELEGATOR4, HIGH_BALANCE),
            (DELEGATOR5, HIGH_BALANCE),
            (DELEGATOR6, HIGH_BALANCE),
            (DELEGATOR7, HIGH_BALANCE),
            (DELEGATOR8, HIGH_BALANCE),
        ])
        .with_candidates(vec![(BOB, balance), (CHARLIE, balance)])
        .with_delegations(vec![
            (DELEGATOR1, BOB, delegate_amt),
            (DELEGATOR2, BOB, delegate_amt),
            (DELEGATOR3, BOB, delegate_amt),
            (DELEGATOR4, BOB, delegate_amt),
            (DELEGATOR5, BOB, delegate_amt),
            (DELEGATOR6, BOB, delegate_amt),
            (DELEGATOR7, BOB, delegate_amt),
            (DELEGATOR8, BOB, delegate_amt),
            (DELEGATOR1, CHARLIE, delegate_amt),
            (DELEGATOR2, CHARLIE, delegate_amt),
            (DELEGATOR3, CHARLIE, delegate_amt),
            (DELEGATOR4, CHARLIE, delegate_amt),
            (DELEGATOR5, CHARLIE, delegate_amt),
            (DELEGATOR6, CHARLIE, delegate_amt),
            (DELEGATOR7, CHARLIE, delegate_amt),
            (DELEGATOR8, CHARLIE, delegate_amt),
        ])
        .with_funded_lottery_account(reserve)
        .build()
        .execute_with(|| {
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), quarter_balance));
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 1);
            // After deposit, DELEGATOR8 is kicked cause he is the last one on bottom.

            // staked:125, lowestTop=100
            // If first deposit choose Bob as collator, then second will chose Charlie as collator.
            // deposit failed cause his deposit balance is less than smallest bottom
            assert_noop!(
                Lottery::deposit(Origin::signed(DELEGATOR8), delegate_amt),
                pallet_parachain_staking::Error::<Test>::CannotDelegateLessThanOrEqualToLowestBottomWhenFull
            );
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 1);

            assert_ok!(Lottery::deposit(Origin::signed(DELEGATOR8), delegate_amt+1));
        });
}

#[test]
fn delegator_more_than_bottom_can_deposit_onecollator_secondok() {
    let reserve = 10_000 * UNIT;
    let balance = 500_000_000 * UNIT;
    let quarter_balance = 125_000_000 * UNIT;
    let delegate_amt = 100_000_000 * UNIT;
    let delegation_min = 5_000 * UNIT;
    ExtBuilder::default()
        .with_balances(vec![
            (ALICE, HIGH_BALANCE),
            (BOB, HIGH_BALANCE),
            (CHARLIE, HIGH_BALANCE),
            (DAVE, HIGH_BALANCE),
            (DELEGATOR1, HIGH_BALANCE),
            (DELEGATOR2, HIGH_BALANCE),
            (DELEGATOR3, HIGH_BALANCE),
            (DELEGATOR4, HIGH_BALANCE),
            (DELEGATOR5, HIGH_BALANCE),
            (DELEGATOR6, HIGH_BALANCE),
            (DELEGATOR7, HIGH_BALANCE),
            (DELEGATOR8, HIGH_BALANCE),
        ])
        .with_candidates(vec![(BOB, balance)])
        .with_delegations(vec![
            (DELEGATOR1, BOB, delegate_amt),
            (DELEGATOR2, BOB, delegate_amt),
            (DELEGATOR3, BOB, delegate_amt),
            (DELEGATOR4, BOB, delegate_amt),
            (DELEGATOR5, BOB, delegate_amt),
            (DELEGATOR6, BOB, delegate_amt),
            (DELEGATOR7, BOB, delegate_amt),
            (DELEGATOR8, BOB, delegate_amt),
        ])
        .with_funded_lottery_account(reserve)
        .build()
        .execute_with(|| {
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), quarter_balance));
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 1);

            assert_ok!(Lottery::deposit(Origin::signed(ALICE), delegate_amt));
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 1);

            assert_ok!(Lottery::deposit(Origin::signed(ALICE), delegation_min));
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 1);
        });
}

#[test]
fn delegator_kicked_not_in_state_cannot_deposit() {
    let reserve = 10_000 * UNIT;
    let balance = 500_000_000 * UNIT;
    ExtBuilder::default()
        .with_balances(vec![
            (ALICE, HIGH_BALANCE),
            (BOB, HIGH_BALANCE),
            (CHARLIE, HIGH_BALANCE),
            (DAVE, HIGH_BALANCE),
            (DELEGATOR1, HIGH_BALANCE),
            (DELEGATOR2, HIGH_BALANCE),
            (DELEGATOR3, HIGH_BALANCE),
            (DELEGATOR4, HIGH_BALANCE),
            (DELEGATOR5, HIGH_BALANCE),
            (DELEGATOR6, HIGH_BALANCE),
            (DELEGATOR7, HIGH_BALANCE),
            (DELEGATOR8, HIGH_BALANCE),
        ])
        .with_candidates(vec![(BOB, balance)])
        .with_delegations(vec![
            (DELEGATOR1, BOB, 800_000_000 * UNIT),
            (DELEGATOR2, BOB, 700_000_000 * UNIT),
            (DELEGATOR3, BOB, 600_000_000 * UNIT),
            (DELEGATOR4, BOB, 500_000_000 * UNIT),
            (DELEGATOR5, BOB, 400_000_000 * UNIT),
            (DELEGATOR6, BOB, 300_000_000 * UNIT),
            (DELEGATOR7, BOB, 200_000_000 * UNIT),
            (DELEGATOR8, BOB, 100_000_000 * UNIT)
        ])
        .with_funded_lottery_account(reserve)
        .build()
        .execute_with(|| {
            // Large than lowest bottom, kicked DELEGATOR8
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), 150_000_000 * UNIT));
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 1);

            // Check PotAccount state, exist!
            let pot_state1 = ParachainStaking::delegator_state(crate::Pallet::<Test>::account_id()).expect("just delegated => exists");
            assert_eq!(pot_state1.delegations.0[0].owner, BOB);
            assert_eq!(pot_state1.delegations.0[0].amount, 150_000_000 * UNIT);

            // If delegator less than lowest bottom: 150, failed
            assert_noop!(ParachainStaking::delegate(
                Origin::signed(CHARLIE),
                BOB,
                100_000_000 * UNIT,
                8,
                0
            ),pallet_parachain_staking::Error::<Test>::CannotDelegateLessThanOrEqualToLowestBottomWhenFull);

            // If delegator large than lowest bottom: 150, ok 
            assert_ok!(ParachainStaking::delegate(
                Origin::signed(CHARLIE),
                BOB,
                160_000_000 * UNIT,
                8,
                0
            ));
            // Check Charlie state, exist!
            let charlie_state = ParachainStaking::delegator_state(CHARLIE).expect("just delegated => exists");
            assert_eq!(charlie_state.delegations.0[0].owner, BOB);
            assert_eq!(charlie_state.delegations.0[0].amount, 160_000_000 * UNIT);

            // Check PotAccount state, not exist!
            let _pot_state2 = ParachainStaking::delegator_state(crate::Pallet::<Test>::account_id());
            assert!(_pot_state2.is_none());

            // Delegator: PotAccount is not exist on DelegatorState.
            // Now PotAccount is kicked, try to deposit, failed!
            // In this case, we should delegate instead bond more.
            assert_noop!(
                Lottery::deposit(Origin::signed(ALICE), 150_000_000 * UNIT),
                pallet_parachain_staking::Error::<Test>::DelegatorDNE
            );
        });
}

#[test]
fn delegator_state_eq_lowest_top_choose_diff_collator() {
    let reserve = 10_000 * UNIT;
    let balance = 500_000_000 * UNIT;
    ExtBuilder::default()
        .with_balances(vec![
            (ALICE, HIGH_BALANCE),
            (BOB, HIGH_BALANCE),
            (CHARLIE, HIGH_BALANCE),
            (DAVE, HIGH_BALANCE),
            (DELEGATOR1, HIGH_BALANCE),
            (DELEGATOR2, HIGH_BALANCE),
            (DELEGATOR3, HIGH_BALANCE),
            (DELEGATOR4, HIGH_BALANCE),
            (DELEGATOR5, HIGH_BALANCE),
            (DELEGATOR6, HIGH_BALANCE),
            (DELEGATOR7, HIGH_BALANCE),
            (DELEGATOR8, HIGH_BALANCE),
        ])
        .with_candidates(vec![(BOB, balance), (CHARLIE, balance)])
        .with_delegations(vec![
            (DELEGATOR1, BOB, 80_000_000 * UNIT),
            (DELEGATOR2, BOB, 70_000_000 * UNIT),
            (DELEGATOR3, BOB, 60_000_000 * UNIT),
            (DELEGATOR4, BOB, 50_000_000 * UNIT),
            (DELEGATOR5, BOB, 40_000_000 * UNIT),
            (DELEGATOR6, BOB, 30_000_000 * UNIT),
            (DELEGATOR7, BOB, 20_000_000 * UNIT),
            (DELEGATOR1, CHARLIE, 80_000_000 * UNIT),
            (DELEGATOR2, CHARLIE, 70_000_000 * UNIT),
            (DELEGATOR3, CHARLIE, 60_000_000 * UNIT),
            (DELEGATOR4, CHARLIE, 50_000_000 * UNIT),
            (DELEGATOR5, CHARLIE, 40_000_000 * UNIT),
            (DELEGATOR6, CHARLIE, 30_000_000 * UNIT),
            (DELEGATOR7, CHARLIE, 20_000_000 * UNIT),
        ])
        .with_funded_lottery_account(reserve)
        .build()
        .execute_with(|| {
            // 51 large than top lowest 50, 50 will be top bottom now
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), 51_000_000 * UNIT));
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 1);

            // staked: 51 eq to lowest top: 51, choose another collator
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), 51_000_000 * UNIT));
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 2);

            // third deposit, both collator's staked and lowest top is 51, choose random one.
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), 51_000_000 * UNIT));
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 2);
        });
}

#[test]
fn delegator_kicked_when_reactivate_bottom_should_ignored() {
    let reserve = 10_000 * UNIT;
    let balance = 500_000_000 * UNIT;
    ExtBuilder::default()
        .with_balances(vec![
            (ALICE, HIGH_BALANCE),
            (BOB, HIGH_BALANCE),
            (CHARLIE, HIGH_BALANCE),
            (DAVE, HIGH_BALANCE),
            (DELEGATOR1, HIGH_BALANCE),
            (DELEGATOR2, HIGH_BALANCE),
            (DELEGATOR3, HIGH_BALANCE),
            (DELEGATOR4, HIGH_BALANCE),
            (DELEGATOR5, HIGH_BALANCE),
            (DELEGATOR6, HIGH_BALANCE),
            (DELEGATOR7, HIGH_BALANCE),
            (DELEGATOR8, HIGH_BALANCE),
        ])
        .with_candidates(vec![(BOB, balance), (CHARLIE, balance)])
        .with_delegations(vec![
            (DELEGATOR1, BOB, 80_000_000 * UNIT),
            (DELEGATOR2, BOB, 70_000_000 * UNIT),
            (DELEGATOR3, BOB, 60_000_000 * UNIT),
            (DELEGATOR4, BOB, 50_000_000 * UNIT),
            (DELEGATOR1, CHARLIE, 80_000_000 * UNIT),
            (DELEGATOR2, CHARLIE, 70_000_000 * UNIT),
            (DELEGATOR3, CHARLIE, 60_000_000 * UNIT),
            (DELEGATOR4, CHARLIE, 50_000_000 * UNIT),
        ])
        .with_funded_lottery_account(reserve)
        .build()
        .execute_with(|| {
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), 51_000_000 * UNIT));
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 1);

            assert_ok!(Lottery::deposit(Origin::signed(ALICE), 51_000_000 * UNIT));
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 2);

            // PotAccount delegate to two collators.
            let pot_state1 = ParachainStaking::delegator_state(crate::Pallet::<Test>::account_id())
                .expect("just delegated => exists");
            assert_eq!(pot_state1.delegations.len(), 2);
            assert_eq!(pot_state1.delegations.0[0].amount, 51_000_000 * UNIT);
            assert_eq!(pot_state1.delegations.0[1].amount, 51_000_000 * UNIT);

            assert_ok!(ParachainStaking::delegate(
                Origin::signed(DELEGATOR5),
                BOB,
                52_000_000 * UNIT,
                5,
                0
            ));
            assert_ok!(ParachainStaking::delegate(
                Origin::signed(DELEGATOR6),
                BOB,
                52_000_000 * UNIT,
                6,
                0
            ));
            assert_ok!(ParachainStaking::delegate(
                Origin::signed(DELEGATOR7),
                BOB,
                52_000_000 * UNIT,
                7,
                0
            ));
            assert_ok!(ParachainStaking::delegate(
                Origin::signed(DELEGATOR8),
                BOB,
                52_000_000 * UNIT,
                8,
                0
            ));
            assert_ok!(ParachainStaking::delegate(
                Origin::signed(DAVE),
                BOB,
                52_000_000 * UNIT,
                9,
                0
            ));
            // PotAccount now is kicked from collator BOB's delegator list.
            // But PotAccount is still on the delegator list of CHARLIE.
            let pot_state2 = ParachainStaking::delegator_state(crate::Pallet::<Test>::account_id())
                .expect("just delegated => exists");
            assert_eq!(pot_state2.delegations.len(), 1);
            assert_eq!(pot_state2.delegations.0[0].owner, CHARLIE);
            assert_eq!(pot_state2.delegations.0[0].amount, 51_000_000 * UNIT);

            // StakedCollators didn't remove BOB.
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 2);
            assert_eq!(crate::StakedCollators::<Test>::get(BOB), 51_000_000 * UNIT);
            assert_eq!(
                crate::StakedCollators::<Test>::get(CHARLIE),
                51_000_000 * UNIT
            );

            let collator1_state = ParachainStaking::candidate_info(BOB).unwrap();
            let collator2_state = ParachainStaking::candidate_info(CHARLIE).unwrap();
            assert_eq!(
                collator1_state.lowest_bottom_delegation_amount,
                52_000_000 * UNIT
            );
            assert_eq!(
                collator2_state.lowest_bottom_delegation_amount,
                50_000_000 * UNIT
            );
            assert_eq!(
                collator1_state.lowest_top_delegation_amount,
                52_000_000 * UNIT
            );
            assert_eq!(
                collator2_state.lowest_top_delegation_amount,
                51_000_000 * UNIT
            );

            let _pot_state2 =
                ParachainStaking::delegator_state(crate::Pallet::<Test>::account_id())
                    .expect("just delegated => exists");
            assert_eq!(_pot_state2.delegations.len(), 1);
            assert_eq!(_pot_state2.delegations.0[0].owner, CHARLIE);
            assert_eq!(_pot_state2.delegations.0[0].amount, 51_000_000 * UNIT);

            // Deposit should only delegate to CHRALIE.
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), 53_000_000 * UNIT));
            assert_eq!(crate::StakedCollators::<Test>::get(BOB), 51_000_000 * UNIT);
            assert_eq!(
                crate::StakedCollators::<Test>::get(CHARLIE),
                104_000_000 * UNIT
            );

            let _pot_state3 =
                ParachainStaking::delegator_state(crate::Pallet::<Test>::account_id())
                    .expect("just delegated => exists");
            assert_eq!(_pot_state3.delegations.len(), 1);
            assert_eq!(_pot_state3.delegations.0[0].owner, CHARLIE);
            assert_eq!(_pot_state3.delegations.0[0].amount, 104_000_000 * UNIT);
        });
}

#[test]
fn delegator_unstaking_then_kicked_should_ignored() {
    let balance = 500_000_000 * UNIT;
    ExtBuilder::default()
        .with_balances(vec![
            (ALICE, HIGH_BALANCE),
            (BOB, HIGH_BALANCE),
            (CHARLIE, HIGH_BALANCE),
            (DAVE, HIGH_BALANCE),
            (DELEGATOR1, HIGH_BALANCE),
            (DELEGATOR2, HIGH_BALANCE),
            (DELEGATOR3, HIGH_BALANCE),
            (DELEGATOR4, HIGH_BALANCE),
            (DELEGATOR5, HIGH_BALANCE),
            (DELEGATOR6, HIGH_BALANCE),
            (DELEGATOR7, HIGH_BALANCE),
            (DELEGATOR8, HIGH_BALANCE),
        ])
        .with_candidates(vec![(BOB, balance), (CHARLIE, balance)])
        .with_delegations(vec![
            (DELEGATOR1, BOB, 80_000_000 * UNIT),
            (DELEGATOR2, BOB, 70_000_000 * UNIT),
            (DELEGATOR3, BOB, 60_000_000 * UNIT),
            (DELEGATOR4, BOB, 50_000_000 * UNIT),
            (DELEGATOR1, CHARLIE, 80_000_000 * UNIT),
            (DELEGATOR2, CHARLIE, 70_000_000 * UNIT),
            (DELEGATOR3, CHARLIE, 60_000_000 * UNIT),
            (DELEGATOR4, CHARLIE, 50_000_000 * UNIT),
        ])
        .with_funded_lottery_account(balance)
        .build()
        .execute_with(|| {
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), 51_000_000 * UNIT));
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 1);

            assert_ok!(Lottery::deposit(Origin::signed(ALICE), 51_000_000 * UNIT));
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 2);

            // unstaking from CHARLIE
            assert_ok!(Lottery::request_withdraw(
                Origin::signed(ALICE),
                51_000_000 * UNIT
            ));
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 2);
            assert_eq!(crate::UnstakingCollators::<Test>::get().len(), 1);

            // PotAccount delegate to two collators.
            let pot_state1 = ParachainStaking::delegator_state(crate::Pallet::<Test>::account_id())
                .expect("just delegated => exists");
            assert_eq!(pot_state1.delegations.len(), 2);
            assert_eq!(pot_state1.delegations.0[0].amount, 51_000_000 * UNIT);
            assert_eq!(pot_state1.delegations.0[1].amount, 51_000_000 * UNIT);

            assert_ok!(ParachainStaking::delegate(
                Origin::signed(DELEGATOR5),
                BOB,
                53_000_000 * UNIT,
                5,
                0
            ));
            assert_ok!(ParachainStaking::delegate(
                Origin::signed(DELEGATOR6),
                BOB,
                53_000_000 * UNIT,
                6,
                0
            ));
            assert_ok!(ParachainStaking::delegate(
                Origin::signed(DELEGATOR7),
                BOB,
                53_000_000 * UNIT,
                7,
                0
            ));
            assert_ok!(ParachainStaking::delegate(
                Origin::signed(DELEGATOR8),
                BOB,
                53_000_000 * UNIT,
                8,
                0
            ));
            assert_ok!(ParachainStaking::delegate(
                Origin::signed(DAVE),
                BOB,
                53_000_000 * UNIT,
                9,
                0
            ));
            let pot_state2 = ParachainStaking::delegator_state(crate::Pallet::<Test>::account_id())
                .expect("just delegated => exists");
            assert_eq!(pot_state2.delegations.len(), 1);
            assert_eq!(pot_state2.delegations.0[0].owner, CHARLIE);
            assert_eq!(pot_state2.delegations.0[0].amount, 51_000_000 * UNIT);

            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 2);
            assert_eq!(crate::UnstakingCollators::<Test>::get().len(), 1);

            let collator1_state = ParachainStaking::candidate_info(BOB).unwrap();
            let collator2_state = ParachainStaking::candidate_info(CHARLIE).unwrap();
            assert_eq!(
                collator1_state.lowest_top_delegation_amount,
                53_000_000 * UNIT
            );
            assert_eq!(
                collator1_state.lowest_bottom_delegation_amount,
                53_000_000 * UNIT
            );
            assert_eq!(
                collator2_state.lowest_top_delegation_amount,
                51_000_000 * UNIT
            );
            assert_eq!(
                collator2_state.lowest_bottom_delegation_amount,
                50_000_000 * UNIT
            );

            // Check PotAccount state, exist!
            let _pot_state2 =
                ParachainStaking::delegator_state(crate::Pallet::<Test>::account_id())
                    .expect("just delegated => exists");
            assert_eq!(_pot_state2.delegations.len(), 1);
            assert_eq!(_pot_state2.delegations.0[0].owner, CHARLIE);
            assert_eq!(_pot_state2.delegations.0[0].amount, 51_000_000 * UNIT);

            // Staked to BOB was already kicked PotAccount.
            // And staked to CHARLIE is unstaking, now there're no collators to deposit.
            // If we choose random one which is BOB, then delegator_bond_more has error.
            assert_noop!(
                Lottery::deposit(Origin::signed(ALICE), 53_000_000 * UNIT),
                pallet_parachain_staking::Error::<Test>::DelegationDNE
            );

            // Check PotAccount state, exist!
            let _pot_state3 =
                ParachainStaking::delegator_state(crate::Pallet::<Test>::account_id())
                    .expect("just delegated => exists");
            assert_eq!(_pot_state3.delegations.len(), 1);
            assert_eq!(_pot_state3.delegations.0[0].owner, CHARLIE);
            assert_eq!(_pot_state3.delegations.0[0].amount, 51_000_000 * UNIT);

            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 2);
            assert_eq!(crate::UnstakingCollators::<Test>::get().len(), 1);

            // All collator not eligible, withdraw failed.
            assert_noop!(
                Lottery::request_withdraw(Origin::signed(ALICE), 51_000_000 * UNIT),
                sp_runtime::DispatchError::Other(
                    "FATAL: Didn't unstake the full requested balance (or more)"
                )
            );

            // draw lottery to make unstaking collator removed.
            pallet_parachain_staking::AwardedPts::<Test>::insert(2, BOB, 20);
            pallet_parachain_staking::AwardedPts::<Test>::insert(2, CHARLIE, 20);
            roll_to_round_begin(3);
            assert_ok!(Lottery::process_matured_withdrawals(RawOrigin::Root.into()));

            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 1);
            assert_eq!(crate::UnstakingCollators::<Test>::get().len(), 0);
            assert_eq!(crate::StakedCollators::<Test>::get(BOB), 51_000_000 * UNIT);

            // DelegatorState of PotAccount is empty
            let _pot_state4 =
                ParachainStaking::delegator_state(crate::Pallet::<Test>::account_id());
            assert!(_pot_state4.is_none());

            // The only one staked collator BOB is not exist in parachain staking DelegatorState.
            assert_noop!(
                Lottery::request_withdraw(Origin::signed(ALICE), 51_000_000 * UNIT),
                sp_runtime::DispatchError::Other(
                    "FATAL: Didn't unstake the full requested balance (or more)"
                )
            );

            assert_noop!(
                Lottery::deposit(Origin::signed(ALICE), 53_000_000 * UNIT),
                pallet_parachain_staking::Error::<Test>::DelegatorDNE
            );
        });
}

#[test]
fn delegator_unstaking_kicked_same_collator_should_ignored() {
    let reserve = 10_000 * UNIT;
    let balance = 500_000_000 * UNIT;
    ExtBuilder::default()
        .with_balances(vec![
            (ALICE, HIGH_BALANCE),
            (BOB, HIGH_BALANCE),
            (CHARLIE, HIGH_BALANCE),
            (DAVE, HIGH_BALANCE),
            (DELEGATOR1, HIGH_BALANCE),
            (DELEGATOR2, HIGH_BALANCE),
            (DELEGATOR3, HIGH_BALANCE),
            (DELEGATOR4, HIGH_BALANCE),
            (DELEGATOR5, HIGH_BALANCE),
            (DELEGATOR6, HIGH_BALANCE),
            (DELEGATOR7, HIGH_BALANCE),
            (DELEGATOR8, HIGH_BALANCE),
        ])
        .with_candidates(vec![(BOB, balance), (CHARLIE, balance)])
        .with_delegations(vec![
            (DELEGATOR1, BOB, 80_000_000 * UNIT),
            (DELEGATOR2, BOB, 70_000_000 * UNIT),
            (DELEGATOR3, BOB, 60_000_000 * UNIT),
            (DELEGATOR4, BOB, 50_000_000 * UNIT),
            (DELEGATOR1, CHARLIE, 80_000_000 * UNIT),
            (DELEGATOR2, CHARLIE, 70_000_000 * UNIT),
            (DELEGATOR3, CHARLIE, 60_000_000 * UNIT),
            (DELEGATOR4, CHARLIE, 50_000_000 * UNIT),
        ])
        .with_funded_lottery_account(reserve)
        .build()
        .execute_with(|| {
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), 52_000_000 * UNIT));
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 1);

            assert_ok!(Lottery::deposit(Origin::signed(ALICE), 51_000_000 * UNIT));
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 2);

            // unstaking from BOB
            assert_ok!(Lottery::request_withdraw(
                Origin::signed(ALICE),
                52_000_000 * UNIT
            ));
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 2);
            assert_eq!(crate::UnstakingCollators::<Test>::get().len(), 1);

            // PotAccount delegate to two collators.
            let pot_state1 = ParachainStaking::delegator_state(crate::Pallet::<Test>::account_id())
                .expect("just delegated => exists");
            assert_eq!(pot_state1.delegations.len(), 2);

            let first_deposit_staked_to_bob =
                pot_state1.delegations.0[0].amount == 52_000_000 * UNIT;

            // Can't ensure which collator accept which deposit.
            // Either Bob get 52 and Charlie get 51, or Bob get 51 and Charlie get 52.
            if first_deposit_staked_to_bob {
                assert_eq!(pot_state1.delegations.0[1].amount, 51_000_000 * UNIT);
            } else {
                assert_eq!(pot_state1.delegations.0[1].amount, 52_000_000 * UNIT);
            }

            assert_ok!(ParachainStaking::delegate(
                Origin::signed(DELEGATOR5),
                BOB,
                53_000_000 * UNIT,
                5,
                0
            ));
            assert_ok!(ParachainStaking::delegate(
                Origin::signed(DELEGATOR6),
                BOB,
                53_000_000 * UNIT,
                6,
                0
            ));
            assert_ok!(ParachainStaking::delegate(
                Origin::signed(DELEGATOR7),
                BOB,
                53_000_000 * UNIT,
                7,
                0
            ));
            assert_ok!(ParachainStaking::delegate(
                Origin::signed(DELEGATOR8),
                BOB,
                53_000_000 * UNIT,
                8,
                0
            ));
            assert_ok!(ParachainStaking::delegate(
                Origin::signed(DAVE),
                BOB,
                53_000_000 * UNIT,
                9,
                0
            ));
            let pot_state2 = ParachainStaking::delegator_state(crate::Pallet::<Test>::account_id())
                .expect("just delegated => exists");
            assert_eq!(pot_state2.delegations.len(), 1);
            // We can ensure that PotAccount now only staked to Charlie, but can't ensure the staked amount.
            assert_eq!(pot_state2.delegations.0[0].owner, CHARLIE);
            if first_deposit_staked_to_bob {
                assert_eq!(pot_state2.delegations.0[0].amount, 51_000_000 * UNIT);
            } else {
                assert_eq!(pot_state2.delegations.0[0].amount, 52_000_000 * UNIT);
            }

            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 2);

            let collator1_state = ParachainStaking::candidate_info(BOB).unwrap();
            let collator2_state = ParachainStaking::candidate_info(CHARLIE).unwrap();
            if first_deposit_staked_to_bob {
                assert_eq!(
                    collator1_state.lowest_top_delegation_amount,
                    53_000_000 * UNIT
                );
                assert_eq!(
                    collator1_state.lowest_bottom_delegation_amount,
                    53_000_000 * UNIT
                );

                assert_eq!(
                    collator2_state.lowest_top_delegation_amount,
                    51_000_000 * UNIT
                );
                assert_eq!(
                    collator2_state.lowest_bottom_delegation_amount,
                    50_000_000 * UNIT
                );
            } else {
                assert_eq!(
                    collator1_state.lowest_top_delegation_amount,
                    53_000_000 * UNIT
                );
                assert_eq!(
                    collator1_state.lowest_bottom_delegation_amount,
                    53_000_000 * UNIT
                );

                assert_eq!(
                    collator2_state.lowest_top_delegation_amount,
                    52_000_000 * UNIT
                );
                assert_eq!(
                    collator2_state.lowest_bottom_delegation_amount,
                    50_000_000 * UNIT
                );
            }

            // Check PotAccount state, exist!
            let _pot_state2 =
                ParachainStaking::delegator_state(crate::Pallet::<Test>::account_id())
                    .expect("just delegated => exists");
            assert_eq!(_pot_state2.delegations.len(), 1);
            assert_eq!(_pot_state2.delegations.0[0].owner, CHARLIE);
            if first_deposit_staked_to_bob {
                assert_eq!(_pot_state2.delegations.0[0].amount, 51_000_000 * UNIT);
            } else {
                assert_eq!(_pot_state2.delegations.0[0].amount, 52_000_000 * UNIT);
            }

            if first_deposit_staked_to_bob {
                assert_ok!(Lottery::deposit(Origin::signed(ALICE), 53_000_000 * UNIT));
            } else {
                assert_noop!(
                    Lottery::deposit(Origin::signed(ALICE), 53_000_000 * UNIT),
                    pallet_parachain_staking::Error::<Test>::DelegationDNE
                );
            }

            // Check PotAccount state, exist!
            let _pot_state3 =
                ParachainStaking::delegator_state(crate::Pallet::<Test>::account_id())
                    .expect("just delegated => exists");
            assert_eq!(_pot_state3.delegations.len(), 1);
            assert_eq!(_pot_state3.delegations.0[0].owner, CHARLIE);

            if first_deposit_staked_to_bob {
                assert_eq!(_pot_state3.delegations.0[0].amount, 104_000_000 * UNIT);
            } else {
                assert_eq!(_pot_state3.delegations.0[0].amount, 52_000_000 * UNIT);
            }
        });
}

#[test]
#[ignore = "Will fix it in next release"]
fn many_deposit_withdrawals_work() {
    let balance = 50_000_000 * UNIT;
    let mut round_count = 2;
    ExtBuilder::default()
        .with_balances(vec![
            (ALICE, HIGH_BALANCE),
            (BOB, HIGH_BALANCE),
            (CHARLIE, HIGH_BALANCE),
            (DAVE, HIGH_BALANCE),
            (EVE, HIGH_BALANCE),
        ])
        .with_candidates(vec![
            (ALICE, HIGH_BALANCE),
            (BOB, HIGH_BALANCE),
            (CHARLIE, HIGH_BALANCE),
            (DAVE, HIGH_BALANCE),
            (EVE, HIGH_BALANCE),
        ])
        .with_funded_lottery_account(HIGH_BALANCE)
        .build()
        .execute_with(|| {
            assert!(HIGH_BALANCE > balance);
            let all_collators = &[ALICE, BOB, CHARLIE, DAVE, EVE];
            reward_collators_for_round(round_count - 1, all_collators);
            roll_to_round_end(1);
            assert_ok!(Lottery::start_lottery(RawOrigin::Root.into()));
            const USER_SEED: u32 = 696_969;
            for user in 0..500 {
                let (depositor, _) = crate::mock::from_bench::create_funded_user::<Test>(
                    "depositor",
                    USER_SEED - user,
                    HIGH_BALANCE,
                );
                assert_ok!(Lottery::deposit(Origin::signed(depositor), balance));
                assert_ok!(Lottery::deposit(Origin::signed(depositor), balance));
                assert_ok!(Lottery::request_withdraw(
                    Origin::signed(depositor),
                    balance
                ));
                assert_ok!(Lottery::request_withdraw(
                    Origin::signed(depositor),
                    balance
                ));
                assert_ok!(Lottery::deposit(Origin::signed(depositor), balance));
                assert_ok!(Lottery::deposit(Origin::signed(depositor), balance));
                assert_ok!(Lottery::deposit(Origin::signed(depositor), balance));
                assert_ok!(Lottery::deposit(Origin::signed(depositor), balance));
                assert_ok!(Lottery::deposit(Origin::signed(depositor), balance));
                assert_ok!(Lottery::deposit(Origin::signed(depositor), balance));
                assert_ok!(Lottery::request_withdraw(
                    Origin::signed(depositor),
                    balance
                ));
                assert_ok!(Lottery::request_withdraw(
                    Origin::signed(depositor),
                    balance
                ));
                assert_ok!(Lottery::deposit(Origin::signed(depositor), balance));
                assert_ok!(Lottery::deposit(Origin::signed(depositor), balance));

                assert_eq!(Lottery::active_balance_per_user(depositor), 6 * balance);
                // we only have 5 collators available
                // if all 5 are unstaking, further deposits fail
                // forward the chain until they are unstaked
                reward_collators_for_round(round_count, all_collators);
                round_count += 1;
                roll_to_round_begin((round_count * 2) - 1);
                reward_collators_for_round(round_count * 2 - 2, all_collators);
                roll_to_round_begin(round_count * 2);
                reward_collators_for_round(round_count * 2 - 1, all_collators);
                // drawing happens (twice), all unstaking collators have finished unstaking
                // ensure lottery doesnt run out of gas (it's not getting staking rewards in test)
                assert_ok!(
                    <Test as pallet_parachain_staking::Config>::Currency::deposit_into_existing(
                        &crate::Pallet::<Test>::account_id(),
                        crate::Pallet::<Test>::gas_reserve(),
                    )
                );
            }
        });
}

fn reward_collators_for_round(round: u32, collators: &[AccountId]) {
    for c in collators {
        pallet_parachain_staking::AwardedPts::<Test>::insert(round, c, 20);
    }
}

#[test]
fn enable_farming_works() {
    ExtBuilder::default().build().execute_with(|| {
        let farming_params = FarmingParameters::<Test>::get();
        assert!(!farming_params.mint_farming_token);
        assert!(!farming_params.destroy_farming_token);
        assert_eq!(farming_params.currency_id, 0);
        assert_eq!(farming_params.pool_id, 0);

        assert_noop!(
            Lottery::set_farming_params(Origin::signed(BOB), true, true, 1, 1),
            sp_runtime::DispatchError::BadOrigin
        );

        let farming_params = FarmingParameters::<Test>::get();
        assert!(!farming_params.mint_farming_token);
        assert!(!farming_params.destroy_farming_token);
        assert_eq!(farming_params.currency_id, 0);
        assert_eq!(farming_params.pool_id, 0);

        assert_ok!(Lottery::set_farming_params(
            Origin::root(),
            true,
            true,
            1,
            1
        ));

        let farming_params = FarmingParameters::<Test>::get();
        assert!(farming_params.mint_farming_token);
        assert!(farming_params.destroy_farming_token);
        assert_eq!(farming_params.currency_id, 1);
        assert_eq!(farming_params.pool_id, 1);
    });
}

#[test]
fn farming_deposit_withdraw() {
    let balance = 500_000_000 * UNIT;
    let half_balance = 250_000_000 * UNIT;
    let quarter_balance = 125_000_000 * UNIT;
    ExtBuilder::default()
        .with_balances(vec![(ALICE, HIGH_BALANCE), (BOB, HIGH_BALANCE)])
        .with_candidates(vec![(BOB, balance)])
        .with_funded_lottery_account(balance)
        .with_farming()
        .build()
        .execute_with(|| {
            assert_eq!(INIT_JUMBO_AMOUNT, Assets::total_supply(JUMBO_ID));
            assert_eq!(INIT_V_MANTA_AMOUNT, Assets::total_supply(V_MANTA_ID));

            assert!(HIGH_BALANCE > balance);
            assert_eq!(0, Lottery::staked_collators(BOB));
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), balance));
            let alice_post_deposit_balance = Balances::free_balance(ALICE);
            assert_eq!(balance, Lottery::staked_collators(BOB));
            assert_eq!(balance, Lottery::total_pot());
            assert_eq!(balance, Lottery::sum_of_deposits());

            // asset accounting is correct
            assert_eq!(INIT_JUMBO_AMOUNT, Assets::total_supply(JUMBO_ID));
            assert_eq!(
                balance + INIT_V_MANTA_AMOUNT,
                Assets::total_supply(V_MANTA_ID)
            );
            assert_eq!(0, Assets::balance(V_MANTA_ID, ALICE));

            assert_ok!(Lottery::request_withdraw(
                Origin::signed(ALICE),
                half_balance
            ));
            // surplus = balance - half_balance = half_balance
            assert_eq!(half_balance, Lottery::surplus_unstaking_balance());

            assert_eq!(
                half_balance + INIT_V_MANTA_AMOUNT,
                Assets::total_supply(V_MANTA_ID)
            );
            // no rewards when no time has past
            assert_eq!(0, Assets::balance(JUMBO_ID, ALICE));
            assert_eq!(0, Assets::balance(JUMBO_ID, ALICE));

            roll_one_block();
            assert_ok!(Lottery::request_withdraw(
                Origin::signed(ALICE),
                quarter_balance
            ));
            assert_eq!(balance, Lottery::staked_collators(BOB));
            // surplus = half_balance - quarter_balance = quarter_balance
            assert_eq!(quarter_balance, Lottery::surplus_unstaking_balance());

            assert_eq!(
                quarter_balance + INIT_V_MANTA_AMOUNT,
                Assets::total_supply(V_MANTA_ID)
            );
            // no rewards when no time has past
            assert_eq!(0, Assets::balance(V_MANTA_ID, ALICE));
            assert_eq!(0, Assets::balance(JUMBO_ID, ALICE));

            pallet_parachain_staking::AwardedPts::<Test>::insert(2, BOB, 20);
            roll_to_round_begin(3);
            // funds should be unlocked now and BOB is finished unstaking, so it's eligible for redepositing
            assert_ok!(Lottery::draw_lottery(RawOrigin::Root.into()));
            assert_eq!(
                alice_post_deposit_balance + half_balance + quarter_balance,
                Balances::free_balance(ALICE)
            );
            assert_eq!(0, Lottery::surplus_unstaking_balance());
            assert_eq!(0, Lottery::unlocked_unstaking_funds());
            assert!(Lottery::withdrawal_request_queue().is_empty());
            assert!(crate::UnstakingCollators::<Test>::get().is_empty());
            // draw lottery rebalance will restake surplus funds to collators.
            assert_eq!(crate::StakedCollators::<Test>::iter().count(), 1);
            assert_eq!(quarter_balance, Lottery::staked_collators(BOB));
            assert_eq!(quarter_balance, Lottery::total_pot());
            assert_eq!(quarter_balance, Lottery::sum_of_deposits());

            assert_eq!(
                quarter_balance + INIT_V_MANTA_AMOUNT,
                Assets::total_supply(V_MANTA_ID)
            );
            // no rewards when no time has past
            assert_eq!(0, Assets::balance(V_MANTA_ID, ALICE));
            assert_eq!(0, Assets::balance(JUMBO_ID, ALICE));
        });
}

#[test]
fn fails_withdrawing_more_than_vmanta() {
    let balance = 500_000_000 * UNIT;
    let half_balance = 250_000_000 * UNIT;
    ExtBuilder::default()
        .with_balances(vec![
            (ALICE, HIGH_BALANCE),
            (BOB, HIGH_BALANCE),
            (CHARLIE, HIGH_BALANCE),
        ])
        .with_candidates(vec![(BOB, balance)])
        .with_funded_lottery_account(balance)
        .with_farming()
        .build()
        .execute_with(|| {
            assert_ok!(Lottery::deposit(Origin::signed(CHARLIE), balance));
            assert_ok!(Lottery::deposit(Origin::signed(ALICE), balance));

            assert_eq!(
                (balance * 2) + INIT_V_MANTA_AMOUNT,
                Assets::total_supply(V_MANTA_ID)
            );
            assert_eq!(0, Assets::balance(V_MANTA_ID, CHARLIE));
            assert_ok!(Farming::withdraw(
                Origin::signed(CHARLIE),
                POOL_ID,
                Some(half_balance)
            ));
            assert_ok!(Farming::withdraw_claim(Origin::signed(CHARLIE), POOL_ID));
            assert_eq!(half_balance, Assets::balance(V_MANTA_ID, CHARLIE));
            assert_eq!(
                (balance * 2) + INIT_V_MANTA_AMOUNT,
                Assets::total_supply(V_MANTA_ID)
            );

            assert_ok!(Assets::transfer(
                Origin::signed(CHARLIE),
                V_MANTA_ID,
                ALICE,
                half_balance
            ));
            assert_noop!(
                Lottery::request_withdraw(Origin::signed(CHARLIE), balance),
                TokenError::FundsUnavailable
            );
            assert_eq!(0, Assets::balance(V_MANTA_ID, CHARLIE));

            assert_ok!(Assets::transfer(
                Origin::signed(ALICE),
                V_MANTA_ID,
                CHARLIE,
                half_balance
            ));
            // will work if V_MANTA is not in farming pool but is in user account
            assert_ok!(Lottery::request_withdraw(Origin::signed(CHARLIE), balance));
            // no leftover V_MANTA
            assert_eq!(0, Assets::balance(V_MANTA_ID, CHARLIE));
        });
}
