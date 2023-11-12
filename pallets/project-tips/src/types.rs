use super::*;
use codec::{Decode, Encode, EncodeLike, MaxEncodedLen};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;

pub const PROJECT_ID: ProjectId = 1;




#[derive(Encode, Decode, Clone, Copy, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub enum TippingName {
	SmallTipper,
	BigTipper,
	SmallSpender,
	MediumSpender,
	BigSpender,
}

#[derive(Encode, Decode, Clone, Copy, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct TippingValue<Balance> {
	pub max_tipping_value: Balance,
	pub stake_required: Balance,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct Project<T: Config> {
	pub created: WhoAndWhenOf<T>,
	pub project_id: ProjectId,
	pub department_id: DepartmentId,
	pub tipping_name: TippingName,
	pub funding_needed: BalanceOf<T>,
	pub project_leader: T::AccountId,
}



