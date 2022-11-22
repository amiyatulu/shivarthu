use frame_support::{pallet_prelude::*};
use frame_support::sp_std::{vec::Vec};
use sp_std::{prelude::*};
use scale_info::TypeInfo;


pub const FIRST_SPACE_ID: u64 = 1;
pub const RESERVED_SPACE_COUNT: u64 = 1000;

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