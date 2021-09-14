use frame_support::{pallet_prelude::*};
use frame_support::sp_std::{vec::Vec};


#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, RuntimeDebug)]
pub struct DepartmentDetails {
    pub name: Vec<u8>,
    pub location: Vec<u8>,
    pub details: Vec<u8>,
    pub departmentid: u128,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, RuntimeDebug)]
pub struct CitizenDetails<AccountId> {
    pub profile_hash: Vec<u8>,
    pub citizenid: u128,
    pub accountid: AccountId,
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ProfileFundInfo<Balance, BlockNumber> {
    pub deposit: Balance,
    pub start: BlockNumber,
}



