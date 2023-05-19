use frame_support::{pallet_prelude::*};
use scale_info::TypeInfo;
use frame_support::sp_std::{vec::Vec};

use super::*;

pub const FIRST_CITIZEN_ID: CitizenId  = 1;


#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct CitizenDetailsPost<T: Config> {
    pub created: WhoAndWhenOf<T>,
    pub content: Content,
    pub citizen_id: CitizenId,
    pub owner: T::AccountId,
    pub edited: bool,
    pub hidden: bool,
    pub upvotes_count: u32,
    pub downvotes_count: u32,
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ProfileFundInfo<Balance, BlockNumber, AccountId> {
    pub funder_account_id: AccountId,
    pub deposit: Balance,
    pub start: BlockNumber,
    pub validated: bool,
    pub reapply: bool,
    pub deposit_returned:bool,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ChallengerFundInfo<Balance, BlockNumber, AccountId> {
    pub challengerid: AccountId,
    pub deposit: Balance,
    pub start: BlockNumber,
    pub challenge_completed: bool,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ChallengeEvidencePost<AccountId> {
    pub author_account_id: AccountId,
    pub post_hash: Vec<u8>,
    pub is_comment: bool,
}