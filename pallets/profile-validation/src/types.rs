use frame_support::{pallet_prelude::*};
use scale_info::TypeInfo;
use frame_support::sp_std::{vec::Vec};


#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct CitizenDetails<AccountId> {
    pub profile_hash: Vec<u8>,
    pub citizenid: u128,
    pub accountid: AccountId,
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