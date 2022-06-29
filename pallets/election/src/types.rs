use frame_support::{pallet_prelude::*};
use frame_support::sp_std::{vec::Vec};
use scale_info::TypeInfo;
use frame_support::traits::ConstU32;


#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct DepartmentDetails {
    pub name: Vec<u8>,
    pub locationid: u128,
    pub details: Vec<u8>,
    pub departmentid: u128,
}