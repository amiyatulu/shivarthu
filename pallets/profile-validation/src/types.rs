use frame_support::{pallet_prelude::*};
use scale_info::TypeInfo;
// use frame_support::sp_std::{vec::Vec};

use super::*;

pub const FIRST_CITIZEN_ID: CitizenId  = 1;
pub const FIRST_CHALLENGE_POST_ID: ChallengePostId = 1;


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
pub struct ProfileFundInfo<Balance, AccountId> {
    pub funder_account_id: AccountId,
    pub validation_account_id: AccountId,
    pub deposit: Balance,
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

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ChallengeEvidencePost<T: Config> {
    pub created: WhoAndWhenOf<T>,
    pub owner: T::AccountId,
    pub kyc_profile_id: T::AccountId,
    pub content:  Content,
    pub post_id_if_comment: Option<ChallengePostId>,
    pub is_comment: bool,
}