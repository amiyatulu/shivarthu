#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use scale_info::TypeInfo;

use frame_support::pallet_prelude::*;
// use sp_std::{collections::btree_set::BTreeSet, vec, vec::Vec};


pub type SpaceId = u64;
pub type PostId = u64;

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct WhoAndWhen<AccountId, BlockNumber, Moment> {
    pub account: AccountId,
    pub block: BlockNumber,
    pub time: Moment,
}

pub type WhoAndWhenOf<T> = WhoAndWhen<
    <T as frame_system::Config>::AccountId,
    <T as frame_system::Config>::BlockNumber,
    <T as pallet_timestamp::Config>::Moment,
>;

pub fn new_who_and_when<T>(
    account: T::AccountId,
) -> WhoAndWhen<T::AccountId, T::BlockNumber, T::Moment>
where
    T: frame_system::Config + pallet_timestamp::Config,
{
    WhoAndWhen {
        account,
        block: frame_system::Pallet::<T>::block_number(),
        time: pallet_timestamp::Pallet::<T>::now(),
    }
}