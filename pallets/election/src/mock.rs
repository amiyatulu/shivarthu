use crate as pallet_template;
use frame_support::parameter_types;
use frame_support::{
	dispatch::DispatchResultWithPostInfo,
	traits::{ConstU16, ConstU64},
};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>}, // new code
		Elections: pallet_template::{Pallet, Call, Storage, Event<T>},
	}
);

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
	type AccountData = pallet_balances::AccountData<u64>; // New code
}

impl pallet_template::Config for Test {
	type Event = Event;
	type Currency = Balances; // New code
	type Slash = ();
	type Reward = ();
	type LoserCandidate = ();
	type KickedMember = ();
	type CurrencyToVote = frame_support::traits::SaturatingCurrencyToVote;
	type CandidacyBond = CandidacyBond;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = u64;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
	pub static CandidacyBond: u64 = 3;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(1, 100000),
			(2, 200000),
			(3, 300000),
			(4, 300000),
			(5, 300000),
			(6, 300000),
			(7, 300000),
			(8, 300000),
			(9, 300000),
			(10, 300000),
			(11, 300000),
			(12, 300000),
			(13, 300000),
			(14, 300000),
			(15, 300000),
			(16, 300000),
			(17, 300000),
			(18, 300000),
			(19, 300000),
			(20, 300000),
			(21, 300000),
			(22, 300000),
			(23, 300000),
			(24, 300000),
			(25, 300000),
			(26, 300000),
			(27, 300000),
			(28, 300000),
			(29, 300000),
			(30, 300000),
			(31, 300000),
			(32, 300000),
			(33, 300000),
			(34, 300000),
			(35, 300000),
		],
	} // new code
	.assimilate_storage(&mut t)
	.unwrap();
	t.into()
}

pub(super) fn candidate_ids(departmentid: u128) -> Vec<u64> {
	Elections::candidates(departmentid)
		.into_iter()
		.map(|(c, _)| c)
		.collect::<Vec<_>>()
}

pub(super) fn balances(who: &u64) -> (u64, u64) {
	(Balances::free_balance(who), Balances::reserved_balance(who))
}

pub(super) fn submit_candidacy(origin: Origin, departmentid: u128) -> DispatchResultWithPostInfo {
	Elections::submit_candidacy(
		origin,
		departmentid,
		Elections::candidates(departmentid).len() as u32,
	)
}

pub(super) fn candidate_deposit(who: &u64, departmentid: u128) -> u64 {
	Elections::candidates(departmentid)
		.into_iter()
		.find_map(|(c, d)| if c == *who { Some(d) } else { None })
		.unwrap_or_default()
}

pub(super) fn vote(
	origin: Origin,
	departmentid: u128,
	votes: Vec<u64>,
	score: u64,
) -> DispatchResultWithPostInfo {
	Elections::vote(origin, departmentid, votes, score)
}


pub(super) fn runners_up_ids(departmentid: u128) -> Vec<u64> {
	Elections::runners_up(departmentid).into_iter().map(|r| r.who).collect::<Vec<_>>()
}

pub(super) fn members_ids(departmentid: u128) -> Vec<u64> {
	Elections::members_ids(departmentid)
}
