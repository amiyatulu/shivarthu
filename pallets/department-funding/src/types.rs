use super::*;
use codec::{Decode, Encode, EncodeLike, MaxEncodedLen};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;

pub const DEPARTMENT_REQUIRED_FUND_ID: DepartmentRequiredFundId = 1;

pub const TIME_FOR_STAKING_FUNDING_STATUS_FAILED: u64 = (3 * 30 * 24 * 60 * 60) / 6; // 3 months time

pub const TIME_FOR_STAKING_FUNDING_STATUS_PASSED: u64 = (6 * 30 * 24 * 60 * 60) / 6; // 6 months time


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
#[scale_info(skip_type_params(T))]
pub struct DepartmentRequiredFund<T: Config> {
	pub created: WhoAndWhenOf<T>,
    pub department_required_fund_id: DepartmentRequiredFundId,
	pub department_id: DepartmentId,
	pub tipping_name: TippingName,
	pub funding_needed: BalanceOf<T>,
	pub creator: T::AccountId,
}


#[derive(Encode, Decode, Clone, Copy, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub enum FundingStatus {
	Processing,
	Success,
	Failed,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct DepartmentFundingStatus<BlockNumber, FundingStatus> {
	pub block_number: BlockNumber,
	pub status: FundingStatus,
}




