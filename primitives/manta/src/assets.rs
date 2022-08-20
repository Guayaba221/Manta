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

//! Asset Utilities

use alloc::vec::Vec;
use codec::{Codec, Decode, Encode};
use core::{borrow::Borrow, marker::PhantomData};
use frame_support::{
    pallet_prelude::Get,
    traits::tokens::{
        currency::Currency,
        fungible::Inspect as FungibleInspect,
        fungibles::{Inspect as FungiblesInspect, Mutate, Transfer},
        DepositConsequence, ExistenceRequirement, WithdrawReasons,
    },
    Parameter,
};
use frame_system::Config;
use scale_info::TypeInfo;
use sp_core::H160;
use sp_runtime::{traits::Member, DispatchError, DispatchResult};
use xcm::{
    v1::{Junctions, MultiLocation},
    VersionedMultiLocation,
};
use xcm_executor::traits::Convert;

/// Asset Id
pub trait AssetIdType {
    /// Asset Id Type
    type AssetId;
}

/// Asset Id Type
pub type AssetId<T> = <T as AssetIdType>::AssetId;

/// Balance
pub trait BalanceType {
    /// Balance Type
    type Balance;
}

/// Balance Type
pub type Balance<T> = <T as BalanceType>::Balance;

/// Location
pub trait LocationType {
    /// Location Type
    type Location;
}

/// Location Type
pub type Location<T> = <T as LocationType>::Location;

/// Asset Metadata
pub trait AssetMetadata: BalanceType {
    /// Returns the minimum balance to hold this asset.
    fn min_balance(&self) -> &Self::Balance;

    /// Returns a boolean value indicating whether this asset needs an existential deposit.
    fn is_sufficient(&self) -> bool;
}

/// Asset Configuration
///
pub trait AssetConfig<C>: 'static + Clone + Eq + AssetIdType + BalanceType + LocationType
where
    C: Config,
{
    /// The AssetId that the non-native asset starts from.
    ///
    /// A typical configuration is 8, so that asset 0 - 7 is reserved.
    type StartNonNativeAssetId: Get<Self::AssetId>;

    /// The Native Asset Id, a typical configuration is 1.
    type NativeAssetId: Get<Self::AssetId>;

    /// Native Asset Location
    type NativeAssetLocation: Get<Self::Location>;

    /// Native Asset Metadata
    type NativeAssetMetadata: Get<Self::AssetRegistryMetadata>;

    ///
    ///
    /// The trait we use to register Assets and mint assets
    type AssetRegistry: AssetRegistry<C, Self>;

    /// Metadata type that required in token storage: e.g. AssetMetadata in Pallet-Assets.
    type StorageMetadata: Default + Member + Parameter + From<Self::AssetRegistryMetadata>;

    /// The Asset Metadata type stored in this pallet.
    type AssetRegistryMetadata: AssetMetadata + Codec + Default + Member + Parameter;

    /*
    /// The AssetLocation type: could be just a thin wrapper of MultiLocation
    type AssetLocation: Default
        + Member
        + Parameter
        + TypeInfo
        + From<MultiLocation>
        + Into<Option<MultiLocation>>;
    */

    /// Fungible Ledger
    type FungibleLedger: FungibleLedger<
        AccountId = C::AccountId,
        AssetId = Self::AssetId,
        Balance = Self::Balance,
    >;
}

/// Asset Registry
///
/// The registrar trait: defines the interface of creating an asset in the asset implementation
/// layer. We may revisit this interface design (e.g. add change asset interface). However, change
/// in StorageMetadata should be rare.
pub trait AssetRegistry<C, T>
where
    C: Config,
    T: AssetConfig<C>,
{
    /// Creates an new asset.
    ///
    /// * `asset_id`: the asset id to be created
    /// * `metadata`: the metadata that the implementation layer stores
    /// * `min_balance`: the minimum balance to hold this asset
    /// * `is_sufficient`: whether this asset can be used as reserve asset,
    ///     to the first approximation. More specifically, Whether a non-zero balance of this asset
    ///     is deposit of sufficient value to account for the state bloat associated with its
    ///     balance storage. If set to `true`, then non-zero balances may be stored without a
    ///     `consumer` reference (and thus an ED in the Balances pallet or whatever else is used to
    ///     control user-account state growth).
    fn create_asset(
        asset_id: T::AssetId,
        metadata: T::StorageMetadata,
        min_balance: T::Balance,
        is_sufficient: bool,
    ) -> DispatchResult;

    /// Update asset metadata by `AssetId`.
    ///
    /// * `asset_id`: the asset id to be created.
    /// * `metadata`: the metadata that the implementation layer stores.
    fn update_asset_metadata(asset_id: T::AssetId, metadata: T::StorageMetadata) -> DispatchResult;
}

/// Asset Storage Metadata
#[derive(Clone, Debug, Decode, Encode, Eq, Hash, Ord, PartialEq, PartialOrd, TypeInfo)]
pub struct AssetStorageMetadata {
    /// Asset Name
    pub name: Vec<u8>,

    /// Asset Symbol
    pub symbol: Vec<u8>,

    /// Number of Decimals
    pub decimals: u8,

    /// Frozen Flag
    ///
    /// Whether or not transfers of the asset are allowed.
    pub is_frozen: bool,
}

impl<B> From<AssetRegistryMetadata<B>> for AssetStorageMetadata {
    #[inline]
    fn from(source: AssetRegistryMetadata<B>) -> Self {
        source.metadata
    }
}

/// Asset Registry Metadata
#[derive(Clone, Debug, Decode, Encode, Eq, Hash, Ord, PartialEq, PartialOrd, TypeInfo)]
pub struct AssetRegistryMetadata<B> {
    /// Asset Storage Metadata
    pub metadata: AssetStorageMetadata,

    /// Optional EVM Address for the Asset
    pub evm_address: Option<H160>,

    /// Minimum Balance
    pub min_balance: B,

    /// Sufficiency Flag
    ///
    /// This flag should be set to `true` whenever a non-zero balance of this asset is deposit of
    /// sufficient value to account for the state bloat associated with its balance storage. If set
    /// to `true`, then non-zero balances may be stored without a `consumer` reference (and thus an
    /// existential deposit in the balances pallet or whatever else is used to control user-account
    /// state growth).
    ///
    /// If this flag is set to `false`, a fresh account cannot receive XCM tokens.
    pub is_sufficient: bool,
}

/*
impl Default for AssetRegistrarMetadata {
    fn default() -> Self {
        Self {
            name: b"Dolphin".to_vec(),
            symbol: b"DOL".to_vec(),
            decimals: 12,
            evm_address: None,
            is_frozen: false,
            min_balance: TEST_DEFAULT_ASSET_ED,
            is_sufficient: true,
        }
    }
}
*/

impl<B> BalanceType for AssetRegistryMetadata<B> {
    type Balance = B;
}

impl<B> AssetMetadata for AssetRegistryMetadata<B> {
    #[inline]
    fn min_balance(&self) -> &B {
        &self.min_balance
    }

    #[inline]
    fn is_sufficient(&self) -> bool {
        self.is_sufficient
    }
}

/// Asset Location
#[derive(Clone, Debug, Decode, Encode, Eq, PartialEq, TypeInfo)]
pub struct AssetLocation(pub VersionedMultiLocation);

impl Default for AssetLocation {
    #[inline]
    fn default() -> Self {
        Self(VersionedMultiLocation::V1(MultiLocation {
            parents: 0,
            interior: Junctions::Here,
        }))
    }
}

impl From<MultiLocation> for AssetLocation {
    /// Converts a [`MultiLocation`] into an [`AssetLocation`].
    ///
    /// # Safety
    ///
    /// This method does not guarantee that the output [`AssetLocation`] is registered, i.e. has a
    /// valid [`AssetId`].
    #[inline]
    fn from(location: MultiLocation) -> Self {
        AssetLocation(VersionedMultiLocation::V1(location))
    }
}

impl From<AssetLocation> for Option<MultiLocation> {
    /// Converts an [`AssetLocation`] into an optional [`MultiLocation`], returning `None` if it
    /// represents a native asset.
    #[inline]
    fn from(location: AssetLocation) -> Self {
        match location {
            AssetLocation(VersionedMultiLocation::V1(location)) => Some(location),
            _ => None,
        }
    }
}

///
pub trait AssetIdLocationMap: AssetIdType + LocationType {
    /// Returns the [`Location`](LocationType::Location) of `asset_id`.
    fn location(asset_id: &Self::AssetId) -> Option<Self::Location>;

    /// Returns the [`AssetId`](AssetIdType::AssetId) located at `location`.
    fn asset_id(location: &Self::Location) -> Option<Self::AssetId>;
}

/// Defines the units per second charged given an `AssetId`.
pub trait UnitsPerSecond: AssetIdType {
    /// Returns the units per second for `asset_id`.
    fn units_per_second(asset_id: &Self::AssetId) -> Option<u128>;
}

///
pub struct AssetIdLocationConvert<M>(PhantomData<M>);

impl<M> Convert<MultiLocation, M::AssetId> for AssetIdLocationConvert<M>
where
    M: AssetIdLocationMap,
    M::AssetId: Clone,
    M::Location: Clone + From<MultiLocation> + Into<Option<MultiLocation>>,
{
    #[inline]
    fn convert_ref(location: impl Borrow<MultiLocation>) -> Result<M::AssetId, ()> {
        M::asset_id(&location.borrow().clone().into()).ok_or(())
    }

    #[inline]
    fn reverse_ref(asset_id: impl Borrow<M::AssetId>) -> Result<MultiLocation, ()> {
        M::location(asset_id.borrow())
            .and_then(Into::into)
            .ok_or(())
    }
}

/// Fungible Ledger Error
#[derive(Clone, Copy, Debug, Decode, Encode, Eq, PartialEq, TypeInfo)]
pub enum FungibleLedgerError<I, B> {
    /// Invalid Asset Id
    InvalidAssetId(I),

    /// Deposit couldn't happen due to the amount being too low. This is usually because the account
    /// doesn't yet exist and the deposit wouldn't bring it to at least the minimum needed for
    /// existence.
    BelowMinimum,

    /// Deposit cannot happen since the account cannot be created (usually because it's a consumer
    /// and there exists no provider reference).
    CannotCreate,

    /// The asset is unknown. Usually because an `AssetId` has been presented which doesn't exist
    /// on the system.
    UnknownAsset,

    /// An overflow would occur. This is practically unexpected, but could happen in test systems
    /// with extremely small balance types or balances that approach the max value of the balance
    /// type.
    Overflow,

    /// Cannot withdraw more than the specified amount
    CannotWithdrawMoreThan(B),

    /// Unable to Mint an Asset
    InvalidMint(DispatchError),

    /// Unable to Burn an Asset
    InvalidBurn(DispatchError),

    /// Unable to Transfer an Asset
    InvalidTransfer(DispatchError),
}

impl<I, B> FungibleLedgerError<I, B> {
    /// Converts a deposit `consequence` into a [`FungibleLedgerError`] or into `Ok(())` when the
    /// value of `consequence` is [`Success`](DepositConsequence::Success).
    #[inline]
    pub fn from_deposit(consequence: DepositConsequence) -> Result<(), Self> {
        Err(match consequence {
            DepositConsequence::BelowMinimum => Self::BelowMinimum,
            DepositConsequence::CannotCreate => Self::CannotCreate,
            DepositConsequence::Overflow => Self::Overflow,
            DepositConsequence::UnknownAsset => Self::UnknownAsset,
            DepositConsequence::Success => return Ok(()),
        })
    }
}

impl<I, B> AssetIdType for FungibleLedgerError<I, B> {
    type AssetId = I;
}

impl<I, B> BalanceType for FungibleLedgerError<I, B> {
    type Balance = B;
}

/// Unified Interface for Fungible Assets
///
/// This trait unifies the interface for the [`fungible`] and [`fungibles`] modules.
///
/// [`fungible`]: frame_support::traits::tokens::fungible
/// [`fungibles`]: frame_support::traits::tokens::fungibles
///
/// It is assumed that the supply of native asset cannot be changed,
/// while the supply of non-native assets can increase or decrease.
pub trait FungibleLedger: AssetIdType + BalanceType {
    /// Account Id Type
    type AccountId;

    /// Checks if an asset id is valid and returning and [`Error`](FungibleLedgerError) otherwise.
    fn ensure_valid(
        asset_id: &Self::AssetId,
    ) -> Result<(), FungibleLedgerError<Self::AssetId, Self::Balance>>;

    /// Check whether `account` can increase its balance by `amount` in the given `asset_id`.
    /// Non-native assets will use the `can_increase_total_supply` check, while native assets will not.
    fn can_deposit(
        asset_id: &Self::AssetId,
        account: &Self::AccountId,
        amount: &Self::Balance,
        can_increase_total_supply: bool,
    ) -> Result<(), FungibleLedgerError<Self::AssetId, Self::Balance>>;

    /// Check whether `account` can decrease its balance by `amount` in the given `asset_id`.
    fn can_reduce_by_amount(
        asset_id: &Self::AssetId,
        account: &Self::AccountId,
        amount: &Self::Balance,
        existence_requirement: ExistenceRequirement,
    ) -> Result<(), FungibleLedgerError<Self::AssetId, Self::Balance>>;

    /// Deposit `amount` of an asset with the given `asset_id` to `beneficiary`.
    /// Will mint and increase the total supply of non-native assets.
    fn deposit_can_mint(
        asset_id: &Self::AssetId,
        beneficiary: &Self::AccountId,
        amount: &Self::Balance,
    ) -> Result<(), FungibleLedgerError<Self::AssetId, Self::Balance>>;

    /// Performs a transfer from `source` to `destination` of
    fn transfer(
        asset_id: &Self::AssetId,
        source: &Self::AccountId,
        destination: &Self::AccountId,
        amount: &Self::Balance,
        existence_requirement: ExistenceRequirement,
    ) -> Result<(), FungibleLedgerError<Self::AssetId, Self::Balance>>;

    /// Performs a withdraw from `who` for `amount` of `asset_id`
    /// Will burn and decrease total supply of non-native assets
    fn withdraw_can_burn(
        asset_id: &Self::AssetId,
        who: &Self::AccountId,
        amount: &Self::Balance,
        existence_requirement: ExistenceRequirement,
    ) -> Result<(), FungibleLedgerError<Self::AssetId, Self::Balance>>;
}

///
pub type FungiblesAssetId<C, F> = <F as FungiblesInspect<<C as Config>::AccountId>>::AssetId;

///
pub type FungibleBalance<C, F> = <F as FungibleInspect<<C as Config>::AccountId>>::Balance;

///
pub type FungiblesBalance<C, F> = <F as FungiblesInspect<<C as Config>::AccountId>>::Balance;

/*

/// Concrete Fungible Ledger Implementation
pub struct ConcreteFungibleLedger<C, A, Native, NonNative> {
    ///  Type Parameter Marker
    __: PhantomData<(C, A, Native, NonNative)>,
}

impl<C, A, Native, NonNative> FungibleLedger for ConcreteFungibleLedger<C, A, Native, NonNative>
where
    C: Config,
    A: AssetConfig<C>,
    Native: FungibleInspect<C::AccountId, Balance = Self::Balance>
        + Currency<C::AccountId, Balance = Self::Balance>,
    NonNative: FungiblesInspect<C::AccountId> + Mutate<C::AccountId> + Transfer<C::AccountId>,
{
    type AccountId = C::AccountId;
    type AssetId = <NonNative as FungiblesInspect<C::AccountId>>::AssetId;
    type AssetValue = <NonNative as FungiblesInspect<C::AccountId>>::Balance;

    #[inline]
    fn ensure_valid(asset_id: &Self::AssetId) -> Result<(), FungibleLedgerError> {
        if *asset_id >= A::StartNonNativeAssetId::get() || *asset_id == A::NativeAssetId::get() {
            Ok(())
        } else {
            Err(FungibleLedgerError::InvalidAssetId)
        }
    }

    #[inline]
    fn can_deposit(
        asset_id: &Self::AssetId,
        account: &C::AccountId,
        amount: &Self::AssetValue,
        can_increase_total_supply: bool,
    ) -> Result<(), FungibleLedgerError> {
        Self::ensure_valid(asset_id)?;
        FungibleLedgerError::from_deposit(if *asset_id == A::NativeAssetId::get() {
            <Native as FungibleInspect<C::AccountId>>::can_deposit(account, *amount, false)
        } else {
            <NonNative as FungiblesInspect<C::AccountId>>::can_deposit(
                *asset_id,
                account,
                *amount,
                can_increase_total_supply,
            )
        })
    }

    #[inline]
    fn can_reduce_by_amount(
        asset_id: &Self::AssetId,
        account: &C::AccountId,
        amount: &Self::AssetValue,
        existence_requirement: ExistenceRequirement,
    ) -> Result<(), FungibleLedgerError> {
        Self::ensure_valid(asset_id)?;
        let keep_alive = match existence_requirement {
            ExistenceRequirement::KeepAlive => true,
            ExistenceRequirement::AllowDeath => false,
        };
        let reducible_amount = if *asset_id == A::NativeAssetId::get() {
            <Native as FungibleInspect<C::AccountId>>::reducible_balance(account, keep_alive)
        } else {
            <NonNative as FungiblesInspect<C::AccountId>>::reducible_balance(
                *asset_id, account, keep_alive,
            )
        };
        if reducible_amount >= *amount {
            return Ok(());
        }
        Err(FungibleLedgerError::CannotWithdrawMoreThan(
            reducible_amount,
        ))
    }

    #[inline]
    fn deposit_can_mint(
        asset_id: &Self::AssetId,
        beneficiary: &C::AccountId,
        amount: &Self::AssetValue,
    ) -> Result<(), FungibleLedgerError> {
        Self::ensure_valid(asset_id)?;
        if *asset_id == A::NativeAssetId::get() {
            <Native as Currency<C::AccountId>>::deposit_creating(beneficiary, amount);
        } else {
            <NonNative as Mutate<C::AccountId>>::mint_into(asset_id, beneficiary, amount)
                .map_err(FungibleLedgerError::InvalidMint)?;
        }
        Ok(())
    }

    #[inline]
    fn transfer(
        asset_id: &Self::AssetId,
        source: &C::AccountId,
        destination: &C::AccountId,
        amount: &Self::AssetValue,
        existence_requirement: ExistenceRequirement,
    ) -> Result<(), FungibleLedgerError> {
        Self::ensure_valid(asset_id)?;
        if asset_id == A::NativeAssetId::get() {
            <Native as Currency<C::AccountId>>::transfer(
                source,
                destination,
                amount,
                existence_requirement,
            )
        } else {
            let keep_alive = match existence_requirement {
                ExistenceRequirement::KeepAlive => true,
                ExistenceRequirement::AllowDeath => false,
            };
            <NonNative as Transfer<C::AccountId>>::transfer(
                asset_id,
                source,
                destination,
                amount,
                keep_alive,
            )
            .map(|_| ())
        }
        .map_err(FungibleLedgerError::InvalidTransfer)
    }

    #[inline]
    fn withdraw_can_burn(
        asset_id: &Self::AssetId,
        who: &C::AccountId,
        amount: &Self::AssetValue,
        existence_requirement: ExistenceRequirement,
    ) -> Result<(), FungibleLedgerError> {
        Self::ensure_valid(asset_id)?;
        Self::can_reduce_by_amount(asset_id, who, amount, existence_requirement)?;
        if asset_id == A::NativeAssetId::get() {
            <Native as Currency<C::AccountId>>::withdraw(
                who,
                amount,
                WithdrawReasons::TRANSFER,
                existence_requirement,
            )
            .map_err(FungibleLedgerError::InvalidBurn)?;
        } else {
            // `existence_requirement` is used in the `can_reduce_by_amount` checks,
            // so it doesn't matter that `burn_from` uses `allow_death` by default in our chosen implementation
            <NonNative as Mutate<C::AccountId>>::burn_from(asset_id, who, amount)
                .map_err(FungibleLedgerError::InvalidBurn)?;
        }
        Ok(())
    }
}

*/
