use super::*;
use codec::{Decode, Encode, EncodeLike, MaxEncodedLen};
use scale_info::TypeInfo;
use frame_support::pallet_prelude::*;



#[derive(Encode, Decode, Clone, Copy, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub enum TippingName {
    SmallTipper,
    BigTipper,
    SmallSpender,
    MediumSpender,
    BigSpender,
}

fn max_value_of_tipping_name(tipping: TippingName) -> u32 {
    match tipping {
        TippingName::SmallTipper => 1_000,
        TippingName::BigTipper => 10_000,
        TippingName::SmallSpender => 100_000,
        TippingName::MediumSpender => 1_000_000,
        TippingName::BigSpender => 10_000_000,
    }
}
