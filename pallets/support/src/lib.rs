#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use scale_info::TypeInfo;

use frame_support::pallet_prelude::*;
// use frame_support::sp_std::{vec::Vec};
use sp_std::{collections::btree_set::BTreeSet, vec, vec::Vec};


pub type SpaceId = u64;
pub type PostId = u64;
pub type PositiveExternalityPostId = u64;

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

impl From<Content> for Vec<u8> {
    fn from(content: Content) -> Vec<u8> {
        match content {
            Content::None => vec![],
            Content::Other(vec_u8) => vec_u8,
            Content::IPFS(vec_u8) => vec_u8,
        }
    }
}

impl Default for Content {
    fn default() -> Self {
        Self::None
    }
}

impl Content {
    pub fn is_none(&self) -> bool {
        self == &Self::None
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    pub fn is_ipfs(&self) -> bool {
        matches!(self, Self::IPFS(_))
    }
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

/// Ensure that a given content is not `None`.
pub fn ensure_content_is_some(content: &Content) -> DispatchResult {
    ensure!(content.is_some(), ContentError::ContentIsEmpty);
    Ok(())
}

pub fn remove_from_vec<F: PartialEq>(vector: &mut Vec<F>, element: F) {
    if let Some(index) = vector.iter().position(|x| *x == element) {
        vector.swap_remove(index);
    }
}

pub fn remove_from_bounded_vec<F: PartialEq, S>(vector: &mut BoundedVec<F, S>, element: F) {
    if let Some(index) = vector.iter().position(|x| *x == element) {
        vector.swap_remove(index);
    }
}

pub fn bool_to_option(value: bool) -> Option<bool> {
    if value {
        Some(value)
    } else {
        None
    }
}
