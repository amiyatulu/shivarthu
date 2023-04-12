use codec::{Decode, Encode};
use scale_info::TypeInfo;

use frame_support::pallet_prelude::*;

use super::*;

pub const FIRST_POST_ID: u64 = 1;

/// Information about a post's owner, its' related space, content, and visibility.
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct PositiveExternalityPost<T: Config> {
    pub id: PositiveExternalityPostId,

    pub created: WhoAndWhenOf<T>,

    pub edited: bool,

    pub owner: T::AccountId,

    pub content: Content,

    pub hidden: bool,

    pub upvotes_count: u32,

    pub downvotes_count: u32,
}


#[derive(Encode, Decode, Default, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct PositiveExternalityPostUpdate {
    
    pub content: Option<Content>,
    pub hidden: Option<bool>,
}

