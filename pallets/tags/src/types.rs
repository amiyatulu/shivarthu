use frame_support::{pallet_prelude::*};
use frame_support::sp_std::{vec::Vec};
use sp_std::{prelude::*};
use scale_info::TypeInfo;
use crate::DownVoteNum;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct DownVoteDetails<AccountId> {
    pub downvote: DownVoteNum,
    pub downvote_users: Vec<AccountId>,
}

impl<AccountId> Default for DownVoteDetails<AccountId> {
    fn default() -> Self {
        Self {downvote: Default::default(), downvote_users: vec![]}
    }
}