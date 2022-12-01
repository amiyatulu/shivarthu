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

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub enum Content {
    /// No content.
    None,
    /// A raw vector of bytes.
    Other(Vec<u8>),
    /// IPFS CID v0 of content.
    IPFS(Vec<u8>),
}


#[derive(Encode, Decode, RuntimeDebug, strum::IntoStaticStr)]
pub enum ContentError {
    /// IPFS CID is invalid.
    InvalidIpfsCid,
    /// `Other` content type is not yet supported.
    OtherContentTypeNotSupported,
    /// Content type is `None`.
    ContentIsEmpty,
}

impl From<ContentError> for DispatchError {
    fn from(err: ContentError) -> DispatchError {
        Self::Other(err.into())
    }
}



pub fn ensure_content_is_valid(content: Content) -> DispatchResult {
    match content {
        Content::None => Ok(()),
        Content::Other(_) => Err(ContentError::OtherContentTypeNotSupported.into()),
        Content::IPFS(ipfs_cid) => {
            let len = ipfs_cid.len();
            // IPFS CID v0 is 46 bytes.
            // IPFS CID v1 is 59 bytes.
            ensure!(len == 46 || len == 59, ContentError::InvalidIpfsCid);
            Ok(())
        },
    }
}