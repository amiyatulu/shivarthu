#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod types;

#[frame_support::pallet]
pub mod pallet {
	// use rand::Rng;
	use crate::types::{
		ChallengerFundInfo, CitizenDetails, CommitVote, DepartmentDetails,
		DrawJurorsForProfileLimit, Period, ProfileFundInfo, SchellingType, SortitionSumTree,
		StakeDetails, StakingTime, SumTreeName, VoteStatus,
	};
	use frame_support::sp_runtime::traits::AccountIdConversion;
	use frame_support::sp_runtime::traits::{CheckedAdd, CheckedSub};
	use frame_support::sp_runtime::SaturatedConversion;
	use frame_support::sp_std::{collections::btree_map::BTreeMap, vec::Vec};
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_support::{sp_runtime::app_crypto::sp_core::H256, traits::Randomness};
	use frame_support::{
		traits::{Currency, ExistenceRequirement, Get, ReservableCurrency, WithdrawReasons},
		PalletId,
	};
	use frame_system::pallet_prelude::*;
	use sp_io;
	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
	type ProfileFundInfoOf<T> =
		ProfileFundInfo<BalanceOf<T>, <T as frame_system::Config>::BlockNumber>;
	type CitizenDetailsOf<T> = CitizenDetails<AccountIdOf<T>>;
	type ChallengerFundInfoOf<T> =
		ChallengerFundInfo<BalanceOf<T>, <T as frame_system::Config>::BlockNumber, AccountIdOf<T>>;
	type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;

	type FundIndex = u32;

	const PALLET_ID: PalletId = PalletId(*b"ex/cfund");

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: ReservableCurrency<Self::AccountId>;

		type RandomnessSource: Randomness<Self::Hash, Self::BlockNumber>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	pub type Nonce<T> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn department_count)]
	pub type DepartmentCount<T> = StorageValue<_, u128, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn admin)]
	pub type Admin<T: Config> = StorageValue<_, T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn citizen_count)]
	pub type CitizenCount<T> = StorageValue<_, u128, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn citizen_id)]
	pub type CitizenId<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u128>;

	#[pallet::storage]
	#[pallet::getter(fn citizen_profile)]
	pub type CitizenProfile<T> = StorageMap<_, Blake2_128Concat, u128, CitizenDetailsOf<T>>; // Peer account id => Peer Profile Hash

	// Registration Fees

	#[pallet::type_value]
	pub fn DefaultRegistrationFee<T: Config>() -> BalanceOf<T> {
		1000u128.saturated_into::<BalanceOf<T>>()
	}
	// Registration challege fees
	#[pallet::type_value]
	pub fn DefaultRegistrationChallengeFee<T: Config>() -> BalanceOf<T> {
		100u128.saturated_into::<BalanceOf<T>>()
	}

	#[pallet::storage]
	#[pallet::getter(fn profile_registration_fees)]
	pub type RegistrationFee<T> =
		StorageValue<_, BalanceOf<T>, ValueQuery, DefaultRegistrationFee<T>>;

	#[pallet::storage]
	#[pallet::getter(fn profile_registration_challege_fees)]
	pub type RegistrationChallengeFee<T> =
		StorageValue<_, BalanceOf<T>, ValueQuery, DefaultRegistrationChallengeFee<T>>;

	#[pallet::storage]
	#[pallet::getter(fn profile_fund)]
	pub type ProfileFundDetails<T> = StorageMap<_, Blake2_128Concat, u128, ProfileFundInfoOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn challenger_fund)]
	pub type ChallengerFundDetails<T> =
		StorageMap<_, Blake2_128Concat, u128, ChallengerFundInfoOf<T>>;

	// #[pallet::storage]
	// #[pallet::getter(fn citizen_profile_status)]

	#[pallet::storage]
	#[pallet::getter(fn department_profile)]
	pub type Department<T> = StorageMap<_, Blake2_128Concat, u128, DepartmentDetails>; // Deparment id => (Name, Location, Details hash)

	#[pallet::storage]
	#[pallet::getter(fn outergroup)]
	pub type OuterGroup<T> = StorageMap<_, Blake2_128Concat, u128, Vec<u128>>; // Department id => Candidate account address set

	#[pallet::storage]
	#[pallet::getter(fn innergroup)]
	pub type InnerGroup<T> = StorageMap<_, Blake2_128Concat, u128, Vec<u128>>; // Department id => Candidate account address set

	#[pallet::storage]
	#[pallet::getter(fn fullgroup)]
	pub type FullGroup<T> = StorageMap<_, Blake2_128Concat, u128, Vec<u128>>; // Department id => Candidate account address set

	#[pallet::storage]
	#[pallet::getter(fn citizen_departments)]
	pub type CitizenDepartments<T> = StorageMap<_, Blake2_128Concat, u128, Vec<u128>>; // Peer account address => Department id set

	#[pallet::storage]
	#[pallet::getter(fn governor_group)]
	pub type GovernorGroup<T> = StorageMap<_, Blake2_128Concat, u128, Vec<u128>>; // Department id => Candidate account address set

	#[pallet::storage]
	#[pallet::getter(fn candidate_nominee)]
	pub type CandidateNominees<T> = StorageMap<_, Blake2_128Concat, (u128, u128), Vec<u128>>; // Department id, Voting cycle => Candidate account address set

	#[pallet::storage]
	#[pallet::getter(fn candidate_approval_votes)]
	pub type CandidateApprovalVotes<T> = StorageMap<_, Blake2_128Concat, (u128, u128, u128), u128>; // Candidate account address, Department id, voting cycle=> Positive Votes

	// Schelling Game Storage

	#[pallet::storage]
	#[pallet::getter(fn schelling_stake)]
	pub type SchellingStake<T> = StorageDoubleMap<
		_,
		Twox64Concat,
		u128,
		Twox64Concat,
		SchellingType,
		StakeDetails<BalanceOf<T>>,
	>; // (citizen id, schelling type => stake)

	#[pallet::storage]
	#[pallet::getter(fn sortition_sum_trees)]
	pub type SortitionSumTrees<T> =
		StorageMap<_, Blake2_128Concat, SumTreeName, SortitionSumTree<AccountIdOf<T>>>;

	#[pallet::storage]
	#[pallet::getter(fn get_period)]
	pub type PeriodName<T> = StorageMap<_, Blake2_128Concat, SumTreeName, Period>;

	#[pallet::type_value]
	pub fn DefaultMinBlockTime<T: Config>() -> StakingTime<BlockNumberOf<T>> {
		let staking_time = StakingTime {
			min_challenge_time: 43200u128.saturated_into::<BlockNumberOf<T>>(),
			min_block_length: 144000u128.saturated_into::<BlockNumberOf<T>>(),
		};
		staking_time
		// 3 days, 10 days
	}

	#[pallet::storage]
	#[pallet::getter(fn min_block_time)]
	pub type MinBlockTime<T> =
		StorageValue<_, StakingTime<BlockNumberOf<T>>, ValueQuery, DefaultMinBlockTime<T>>;
	#[pallet::type_value]
	pub fn DefaultMinStake<T: Config>() -> BalanceOf<T> {
		100u128.saturated_into::<BalanceOf<T>>()
	}
	#[pallet::storage]
	#[pallet::getter(fn min_juror_stake)]
	pub type MinJurorStake<T> = StorageValue<_, BalanceOf<T>, ValueQuery, DefaultMinStake<T>>;

	#[pallet::storage]
	#[pallet::getter(fn draws_in_round)]
	pub type DrawsInRound<T> = StorageMap<_, Blake2_128Concat, SumTreeName, u128, ValueQuery>; // A counter of draws made in the current round.

	#[pallet::storage]
	#[pallet::getter(fn staking_start_time)]
	pub type StakingStartTime<T> =
		StorageMap<_, Blake2_128Concat, SumTreeName, BlockNumberOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn commit_start_time)]
	pub type CommitStartTime<T> =
		StorageMap<_, Blake2_128Concat, SumTreeName, BlockNumberOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn vote_start_time)]
	pub type VoteStartTime<T> =
		StorageMap<_, Blake2_128Concat, SumTreeName, BlockNumberOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn  drawn_jurors)]
	pub type DrawnJurors<T: Config> =
		StorageMap<_, Blake2_128Concat, SumTreeName, Vec<T::AccountId>, ValueQuery>;
	#[pallet::storage]
	#[pallet::getter(fn unstaked_jurors)]
	pub type UnstakedJurors<T: Config> =
		StorageMap<_, Blake2_128Concat, SumTreeName, Vec<T::AccountId>, ValueQuery>;

	#[pallet::type_value]
	pub fn DefaultDrawJurorsForProfileLimit<T: Config>() -> DrawJurorsForProfileLimit {
		let draw_juror_limit = DrawJurorsForProfileLimit { max_draws: 5, max_draws_appeal: 60 };
		// change max draws more than 30 in production
		draw_juror_limit
	}

	#[pallet::storage]
	#[pallet::getter(fn draw_jurors_for_profile_limit)]
	pub type DrawJurorsForProfileLimitData<T> =
		StorageValue<_, DrawJurorsForProfileLimit, ValueQuery, DefaultDrawJurorsForProfileLimit<T>>;
	#[pallet::storage]
	#[pallet::getter(fn vote_commits)]
	pub type VoteCommits<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		SumTreeName,
		Blake2_128Concat,
		T::AccountId,
		CommitVote,
	>;

	#[pallet::storage]
	#[pallet::getter(fn decision_count)]
	pub type DecisionCount<T> =
		StorageMap<_, Blake2_128Concat, SumTreeName, (u64, u64), ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
		RandomNumber(u64, T::AccountId),
		CreateDepartment(u128, T::AccountId),
		CitizenDepartment(u128, T::AccountId),
		CreateCitizen(T::AccountId, Vec<u8>),
		VoteCast(u128, u128, u128), // Departement id, cycle, department vote count
		NomineeDeparment(u128, u128, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		InvalidIndex,
		FailedUnwrap,
		DepartmentExists,
		DepartmentDoNotExists,
		DepartmentNotAssociated,
		ProfileExists,
		ProfileFundExists,
		ChallegerFundInfoExists,
		ProfileFundNotExists,
		NomineeExists,
		CitizenDoNotExists,
		ProfileIsAlreadyValidated,
		AlreadyCommitUsed,
		VoteAlreadyRevealed,
		VoteCommitNotPresent,
		CommitVoteMismatch,
		ProfileNotFunded,
		ProfileValidationOver,
		AlreadyStaked,
		ApplyJurorTimeNotEnded,
		KMustGreaterThanOne,
		TreeAlreadyExists,
		TreeDoesnotExist,
		PeriodExists,
		PeriodDoesNotExists,
		ChallengerFundDoesNotExists,
		PeriodDontMatch,
		StakeLessThanMin,
		MaxDrawExceeded,
		StakingPeriodNotOver,
		EvidencePeriodNotOver,
		MaxJurorNotDrawn,
		JurorDoesNotExists,
		CommitDoesNotExists,
		CommitDoesNotMatch,
		CommitPeriodNotOver,
		VotePeriodNotOver,
		VoteStatusNotCommited,
		NotValidChoice,
		StakeDoesNotExists,
		AlreadyUnstaked,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// Adds profile details in ipfs hash `profile_hash`
		// Set citizen id from count
		// Set citizen profile from citizen id and citizen details that contains profile hash
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2,2))]
		pub fn add_citizen(origin: OriginFor<T>, profile_hash: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let count = <CitizenCount<T>>::get();
			match <CitizenId<T>>::get(&who) {
				Some(_citizen_id) => Err(Error::<T>::ProfileExists)?,
				None => {
					<CitizenId<T>>::insert(&who, count);
					let citizen_details = CitizenDetails {
						profile_hash: profile_hash.clone(),
						citizenid: count,
						accountid: who.clone(),
					};
					<CitizenProfile<T>>::insert(&count, citizen_details);
					let newcount = count.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					<CitizenCount<T>>::put(newcount);
					Self::deposit_event(Event::CreateCitizen(who, profile_hash));
					Ok(())
				}
			}
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2,2))]
		pub fn add_profile_fund(origin: OriginFor<T>, profile_citizenid: u128) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _citizen_account_id = Self::get_citizen_accountid(profile_citizenid)?;
			let deposit = <RegistrationFee<T>>::get();
			let now = <frame_system::Pallet<T>>::block_number();

			let imb = T::Currency::withdraw(
				&who,
				deposit,
				WithdrawReasons::TRANSFER,
				ExistenceRequirement::AllowDeath,
			)?;

			T::Currency::resolve_creating(&Self::fund_profile_account(), imb);

			match <ProfileFundDetails<T>>::get(&profile_citizenid) {
				// 📝 To write update stake for reapply when disapproved
				Some(_profilefundinfo) => Err(Error::<T>::ProfileExists)?,
				None => {
					let profile_fund_info =
						ProfileFundInfo { deposit, start: now, validated: false, reapply: false };
					<ProfileFundDetails<T>>::insert(&profile_citizenid, profile_fund_info);
				}
			}

			let key = SumTreeName::UniqueIdenfier1 {
				citizen_id: profile_citizenid,
				name: "challengeprofile".as_bytes().to_vec(),
			};

			match <PeriodName<T>>::get(&key) {
				Some(_period) => Err(Error::<T>::PeriodExists)?,
				None => {
					let period = Period::Evidence;
					<PeriodName<T>>::insert(&key, period);
				}
			}

			Ok(())
		}

		// Does citizen exists
		// Has the citizen added profile fund
		// Create tree
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2,2))]
		pub fn challenge_profile(origin: OriginFor<T>, profile_citizenid: u128) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let now = <frame_system::Pallet<T>>::block_number();
			let _citizen_account_id = Self::get_citizen_accountid(profile_citizenid)?;
			match <ProfileFundDetails<T>>::get(&profile_citizenid) {
				Some(profilefundinfo) => {
					if profilefundinfo.validated == true {
						Err(Error::<T>::ProfileIsAlreadyValidated)?;
					}
				}
				None => {
					Err(Error::<T>::ProfileFundNotExists)?;
				}
			}
			let deposit = <RegistrationChallengeFee<T>>::get();
			let imb = T::Currency::withdraw(
				&who,
				deposit,
				WithdrawReasons::TRANSFER,
				ExistenceRequirement::AllowDeath,
			)?;

			T::Currency::resolve_creating(&Self::fund_profile_account(), imb);

			match <ChallengerFundDetails<T>>::get(&profile_citizenid) {
				// 📝 To write update stake for reapply
				Some(_challengerfundinfo) => Err(Error::<T>::ChallegerFundInfoExists)?,
				None => {
					let challenger_fund_info = ChallengerFundInfo {
						challengerid: who,
						deposit,
						start: now,
						challenge_completed: false,
					};
					<ChallengerFundDetails<T>>::insert(&profile_citizenid, challenger_fund_info);
				}
			}

			let key = SumTreeName::UniqueIdenfier1 {
				citizen_id: profile_citizenid,
				name: "challengeprofile".as_bytes().to_vec(),
			};

			let result = Self::create_tree(key.clone(), 3);
			result
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2,2))]
		pub fn pass_period(origin: OriginFor<T>, profile_citizenid: u128) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			let key = SumTreeName::UniqueIdenfier1 {
				citizen_id: profile_citizenid,
				name: "challengeprofile".as_bytes().to_vec(),
			};

			let now = <frame_system::Pallet<T>>::block_number();

			match <PeriodName<T>>::get(&key) {
				Some(period) => {
					if period == Period::Evidence {
						match <ChallengerFundDetails<T>>::get(&profile_citizenid) {
							Some(challenger_fund_info) => {
								let block_number = challenger_fund_info.start;
								let time = now.checked_sub(&block_number).expect("Overflow");
								let block_time = <MinBlockTime<T>>::get();
								if time >= block_time.min_challenge_time {
									let new_period = Period::Staking;
									<PeriodName<T>>::insert(&key, new_period);
									<StakingStartTime<T>>::insert(&key, now);
								} else {
									Err(Error::<T>::EvidencePeriodNotOver)?
								}
							}
							None => Err(Error::<T>::ChallengerFundDoesNotExists)?,
						}
					}
					if period == Period::Staking {
						match <ChallengerFundDetails<T>>::get(&profile_citizenid) {
							Some(_challenger_fund_info) => {
								let staking_start_time = <StakingStartTime<T>>::get(&key);
								let block_time = <MinBlockTime<T>>::get();
								if now >= block_time.min_block_length + staking_start_time {
									let new_period = Period::Drawing;
									<PeriodName<T>>::insert(&key, new_period);
								} else {
									Err(Error::<T>::StakingPeriodNotOver)?
								}
							}
							None => Err(Error::<T>::ChallengerFundDoesNotExists)?,
						}
					}
					if period == Period::Drawing {
						let draw_limit = <DrawJurorsForProfileLimitData<T>>::get();
						let draws_in_round = <DrawsInRound<T>>::get(&key);
						if draws_in_round >= draw_limit.max_draws.into() {
							<CommitStartTime<T>>::insert(&key, now);
							let new_period = Period::Commit;
							<PeriodName<T>>::insert(&key, new_period);
						} else {
							Err(Error::<T>::MaxJurorNotDrawn)?
						}
					}

					if period == Period::Commit {
						let commit_start_time = <CommitStartTime<T>>::get(&key);
						let block_time = <MinBlockTime<T>>::get();
						if now >= block_time.min_block_length + commit_start_time {
							<VoteStartTime<T>>::insert(&key, now);
							let new_period = Period::Vote;
							<PeriodName<T>>::insert(&key, new_period);
						} else {
							Err(Error::<T>::CommitPeriodNotOver)?
						}
					}

					if period == Period::Vote {
						let vote_start_time = <VoteStartTime<T>>::get(&key);
						let block_time = <MinBlockTime<T>>::get();
						if now >= block_time.min_block_length + vote_start_time {
							let new_period = Period::Execution;
							<PeriodName<T>>::insert(&key, new_period);
						} else {
							Err(Error::<T>::VotePeriodNotOver)?
						}
					}
				}
				None => Err(Error::<T>::PeriodDoesNotExists)?,
			}

			Ok(())
		}

		// To Do
		// Apply jurors ✔️
		// Draw jurors ✔️
		// Unstaking non selected jurors ✔️
		// Commit vote ✔️
		// Reveal vote ✔️
		// Get winning decision
		// Incentive distribution

		// Generic Schelling game
		// 1. Check for minimum stake ✔️
		// 2. Block time, apply jurors time is available ✔️
		// 3. Number of people staked
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2,2))]
		pub fn apply_jurors(
			origin: OriginFor<T>,
			profile_citizenid: u128,
			stake: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let key = SumTreeName::UniqueIdenfier1 {
				citizen_id: profile_citizenid,
				name: "challengeprofile".as_bytes().to_vec(),
			};
			match <PeriodName<T>>::get(&key) {
				Some(period) => {
					ensure!(period == Period::Staking, Error::<T>::PeriodDontMatch);
				}
				None => Err(Error::<T>::PeriodDoesNotExists)?,
			}

			let min_stake = <MinJurorStake<T>>::get();

			ensure!(stake >= min_stake, Error::<T>::StakeLessThanMin);

			let imb = T::Currency::withdraw(
				&who,
				stake,
				WithdrawReasons::TRANSFER,
				ExistenceRequirement::AllowDeath,
			)?;

			T::Currency::resolve_creating(&Self::juror_stake_account(), imb);

			// let stake_of = Self::stake_of(key.clone(), profile_citizenid)?;

			let stake_u64 = Self::balance_to_u64_saturated(stake);

			let result = Self::set(key, stake_u64, who);

			result
		}

		// Draw jurors
		// Check whether juror application time is over, if not throw error
		// Check mininum number of juror staked
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2,2))]
		pub fn draw_jurors(
			origin: OriginFor<T>,
			profile_citizenid: u128,
			interations: u128,
		) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			let key = SumTreeName::UniqueIdenfier1 {
				citizen_id: profile_citizenid,
				name: "challengeprofile".as_bytes().to_vec(),
			};

			match <PeriodName<T>>::get(&key) {
				Some(period) => {
					ensure!(period == Period::Drawing, Error::<T>::PeriodDontMatch);
				}
				None => Err(Error::<T>::PeriodDoesNotExists)?,
			}
			let draw_limit = <DrawJurorsForProfileLimitData<T>>::get();
			let draws_in_round = <DrawsInRound<T>>::get(&key);
			ensure!(draws_in_round < draw_limit.max_draws.into(), Error::<T>::MaxDrawExceeded);
			let mut end_index = draws_in_round + interations;
			if draws_in_round + interations >= draw_limit.max_draws as u128 {
				end_index = draw_limit.max_draws as u128;
			}
			let mut draw_increment = draws_in_round.clone();

			for _ in draws_in_round..end_index {
				let nonce = Self::get_and_increment_nonce();
				let random_seed = T::RandomnessSource::random(&nonce).encode();
				let random_number = u64::decode(&mut random_seed.as_ref())
					.expect("secure hashes should always be bigger than u64; qed");
				// let mut rng = rand::thread_rng();
				// let random_number: u64 = rng.gen();

				let data = Self::draw(key.clone(), random_number)?;
				let mut drawn_juror = <DrawnJurors<T>>::get(&key);
				match drawn_juror.binary_search(&data) {
					Ok(_) => {}
					Err(index) => {
						drawn_juror.insert(index, data);
						<DrawnJurors<T>>::insert(&key, drawn_juror);
						draw_increment = draw_increment + 1;
						// println!("draw_increment, {:?}", draw_increment);
					}
				}
				<DrawsInRound<T>>::insert(&key, draw_increment);
			}
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2,2))]
		pub fn unstaking(origin: OriginFor<T>, profile_citizenid: u128) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let key = SumTreeName::UniqueIdenfier1 {
				citizen_id: profile_citizenid,
				name: "challengeprofile".as_bytes().to_vec(),
			};
			match <PeriodName<T>>::get(&key) {
				Some(period) => {
					ensure!(
						period != Period::Evidence
							&& period != Period::Staking && period != Period::Drawing,
						Error::<T>::PeriodDontMatch
					);
				}
				None => Err(Error::<T>::PeriodDoesNotExists)?,
			}

			let stake_of = Self::stake_of(key.clone(), who.clone())?;

			match stake_of {
				Some(stake) => {
					let balance = Self::u64_to_balance_saturated(stake);
					let mut unstaked_jurors = <UnstakedJurors<T>>::get(&key);
					match unstaked_jurors.binary_search(&who) {
						Ok(_) => Err(Error::<T>::AlreadyUnstaked)?,
						Err(index) => {
							unstaked_jurors.insert(index, who.clone());
							<UnstakedJurors<T>>::insert(&key, unstaked_jurors);
							let _ = T::Currency::resolve_into_existing(
								&who,
								T::Currency::withdraw(
									&Self::juror_stake_account(),
									balance,
									WithdrawReasons::TRANSFER,
									ExistenceRequirement::AllowDeath,
								)?,
							);
						}
					}
				}
				None => Err(Error::<T>::StakeDoesNotExists)?,
			}

			// println!("stakeof {:?}", stake_of);

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2,2))]
		pub fn commit_vote(
			origin: OriginFor<T>,
			profile_citizenid: u128,
			vote_commit: [u8; 32],
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let key = SumTreeName::UniqueIdenfier1 {
				citizen_id: profile_citizenid,
				name: "challengeprofile".as_bytes().to_vec(),
			};
			match <PeriodName<T>>::get(&key) {
				Some(period) => {
					ensure!(period == Period::Commit, Error::<T>::PeriodDontMatch);
				}
				None => Err(Error::<T>::PeriodDoesNotExists)?,
			}
			let drawn_jurors = <DrawnJurors<T>>::get(&key);
			match drawn_jurors.binary_search(&who) {
				Ok(_) => {
					let vote_commit_struct =
						CommitVote { commit: vote_commit, votestatus: VoteStatus::Commited };
					<VoteCommits<T>>::insert(&key, &who, vote_commit_struct);
				}
				Err(_) => Err(Error::<T>::JurorDoesNotExists)?,
			}
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2,2))]
		pub fn reveal_vote(
			origin: OriginFor<T>,
			profile_citizenid: u128,
			choice: Vec<u8>,
			salt: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let key = SumTreeName::UniqueIdenfier1 {
				citizen_id: profile_citizenid,
				name: "challengeprofile".as_bytes().to_vec(),
			};
			match <PeriodName<T>>::get(&key) {
				Some(period) => {
					ensure!(period == Period::Vote, Error::<T>::PeriodDontMatch);
				}
				None => Err(Error::<T>::PeriodDoesNotExists)?,
			}
			let who_commit_vote = <VoteCommits<T>>::get(&key, &who);
			match who_commit_vote {
				Some(mut commit_struct) => {
					ensure!(
						commit_struct.votestatus == VoteStatus::Commited,
						Error::<T>::VoteStatusNotCommited
					);
					let mut vote = choice.clone();
					let mut salt_a = salt.clone();
					vote.append(&mut salt_a);
					let vote_bytes: &[u8] = &vote;
					let hash = sp_io::hashing::keccak_256(vote_bytes);
					let commit: &[u8] = &commit_struct.commit;
					if hash == commit {
						let mut decision_tuple = <DecisionCount<T>>::get(&key);
						if choice == "1".as_bytes().to_vec() {
							decision_tuple.1 = decision_tuple.1 + 1;
							<DecisionCount<T>>::insert(&key, decision_tuple);
						} else if choice == "0".as_bytes().to_vec() {
							decision_tuple.0 = decision_tuple.0 + 1;
							<DecisionCount<T>>::insert(&key, decision_tuple);
						} else {
							Err(Error::<T>::NotValidChoice)?
						}
						commit_struct.votestatus = VoteStatus::Revealed;
						<VoteCommits<T>>::insert(&key, &who, commit_struct);
					} else {
						Err(Error::<T>::CommitDoesNotMatch)?
					}
				}
				None => Err(Error::<T>::CommitDoesNotExists)?,
			}

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2,2))]
		pub fn get_random_number(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let nonce = Self::get_and_increment_nonce();
			let random_seed = T::RandomnessSource::random(&nonce).encode();

			let random_number = u64::decode(&mut random_seed.as_ref())
				.expect("secure hashes should always be bigger than u64; qed");

			Self::deposit_event(Event::RandomNumber(random_number, who));
			Ok(())
		}

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				}
			}
		}
	}

	impl<T: Config> Pallet<T> {
		fn get_citizen_accountid(citizenid: u128) -> Result<T::AccountId, DispatchError> {
			let profile = Self::citizen_profile(citizenid).ok_or(Error::<T>::CitizenDoNotExists)?;
			Ok(profile.accountid)
		}

		fn get_citizen_id(accountid: T::AccountId) -> Result<u128, DispatchError> {
			match Self::citizen_id(accountid) {
				Some(citizen_id) => Ok(citizen_id),
				None => Err(Error::<T>::ProfileNotFunded)?,
			}
		}

		fn profile_fund_added(citizenid: u128) -> DispatchResult {
			match <ProfileFundDetails<T>>::get(&citizenid) {
				Some(profilefundinfo) => {
					let validated = profilefundinfo.validated;
					let reapply = profilefundinfo.reapply;
					if validated == false && reapply == false {
						Ok(())
					} else {
						Err(Error::<T>::ProfileValidationOver)?
					}
				}
				None => Err(Error::<T>::ProfileNotFunded)?,
			}
		}

		fn get_profile_fund_info(citizenid: u128) -> Result<ProfileFundInfoOf<T>, DispatchError> {
			match <ProfileFundDetails<T>>::get(&citizenid) {
				Some(profilefundinfo) => {
					let validated = profilefundinfo.validated;
					let reapply = profilefundinfo.reapply;
					if validated == false && reapply == false {
						Ok(profilefundinfo)
					} else {
						Err(Error::<T>::ProfileValidationOver)?
					}
				}
				None => Err(Error::<T>::ProfileNotFunded)?,
			}
		}

		fn balance_to_u64_saturated(input: BalanceOf<T>) -> u64 {
			input.saturated_into::<u64>()
		}

		fn u64_to_balance_saturated(input: u64) -> BalanceOf<T> {
			input.saturated_into::<BalanceOf<T>>()
		}

		fn fund_profile_account() -> T::AccountId {
			PALLET_ID.into_sub_account(1)
		}

		fn juror_stake_account() -> T::AccountId {
			PALLET_ID.into_sub_account(2)
		}

		fn draw_juror_for_citizen_profile_function(
			citizen_id: u128,
			length: usize,
		) -> DispatchResult {
			let nonce = Self::get_and_increment_nonce();

			let random_seed = T::RandomnessSource::random(&nonce).encode();
			let random_number = u64::decode(&mut random_seed.as_ref())
				.expect("secure hashes should always be bigger than u64; qed");
			Ok(())
		}

		fn get_and_increment_nonce() -> Vec<u8> {
			let nonce = <Nonce<T>>::get();
			<Nonce<T>>::put(nonce.wrapping_add(1));
			let n = nonce * 1000 + 1000; // remove and uncomment in production
			n.encode()

			// nonce.encode()
		}

		// SortitionSumTree
		pub fn create_tree(key: SumTreeName, k: u64) -> DispatchResult {
			if k < 1 {
				Err(Error::<T>::KMustGreaterThanOne)?
			}
			let tree_option = <SortitionSumTrees<T>>::get(&key);
			match tree_option {
				Some(_tree) => Err(Error::<T>::TreeAlreadyExists)?,
				None => {
					let mut sum_tree = SortitionSumTree {
						k,
						stack: Vec::new(),
						nodes: Vec::new(),
						ids_to_node_indexes: BTreeMap::new(),
						node_indexes_to_ids: BTreeMap::new(),
					};

					sum_tree.nodes.push(0);

					<SortitionSumTrees<T>>::insert(&key, &sum_tree);
				}
			}
			Ok(())
		}

		pub fn set(key: SumTreeName, value: u64, citizen_id: AccountIdOf<T>) -> DispatchResult {
			let tree_option = <SortitionSumTrees<T>>::get(&key);

			match tree_option {
				None => Err(Error::<T>::TreeDoesnotExist)?,
				Some(mut tree) => match tree.ids_to_node_indexes.get(&citizen_id) {
					Some(tree_index_data) => {
						let tree_index = *tree_index_data;
						if tree_index == 0 {
							Self::if_tree_index_zero(value, citizen_id, tree, tree_index, key);
						} else {
							// Existing node
							if value == 0 {
								let value = tree.nodes[tree_index as usize];
								tree.nodes[tree_index as usize] = 0;
								tree.stack.push(tree_index);
								tree.ids_to_node_indexes.remove(&citizen_id);
								tree.node_indexes_to_ids.remove(&tree_index);

								// UpdateParents 🟥
								Self::update_parents(tree, tree_index, false, value, key);
							} else if value != tree.nodes[tree_index as usize] {
								let plus_or_minus = tree.nodes[tree_index as usize] <= value;
								let plus_or_minus_value = if plus_or_minus {
									value
										.checked_sub(tree.nodes[tree_index as usize])
										.ok_or("StorageOverflow")?
								} else {
									(tree.nodes[tree_index as usize])
										.checked_sub(value)
										.ok_or("StorageOverflow")?
								};
								tree.nodes[tree_index as usize] = value;

								// update parents 🟥
								Self::update_parents(
									tree,
									tree_index,
									plus_or_minus,
									plus_or_minus_value,
									key,
								);
							}
						}
					}

					None => {
						Self::if_tree_index_zero(value, citizen_id, tree, 0, key);
					}
				},
			}

			Ok(())
		}

		fn update_parents(
			mut tree: SortitionSumTree<AccountIdOf<T>>,
			tree_index: u64,
			plus_or_minus: bool,
			value: u64,
			key: SumTreeName,
		) {
			let mut parent_index = tree_index;
			while parent_index != 0 {
				parent_index = (parent_index - 1) / tree.k;
				tree.nodes[parent_index as usize] = if plus_or_minus {
					(tree.nodes[parent_index as usize]).checked_add(value).expect("StorageOverflow")
				} else {
					(tree.nodes[parent_index as usize]).checked_sub(value).expect("StorageOverflow")
				};

				<SortitionSumTrees<T>>::insert(&key, &tree);
			}
		}
		fn if_tree_index_zero(
			value: u64,
			citizen_id: AccountIdOf<T>,
			mut tree: SortitionSumTree<AccountIdOf<T>>,
			mut tree_index: u64,
			key: SumTreeName,
		) {
			// No existing node.
			if value != 0 {
				// Non zero value.
				// Append.
				// Add node.
				if tree.stack.len() == 0 {
					// No vacant spots.
					// Get the index and append the value.
					tree_index = tree.nodes.len() as u64;
					tree.nodes.push(value);

					// println!("{}", tree_index);

					// Potentially append a new node and make the parent a sum node.
					if tree_index != 1 && (tree_index - 1) % tree.k == 0 {
						// Is first child.
						let parent_index = tree_index / tree.k;
						let parent_id =
							tree.node_indexes_to_ids.get(&parent_index).unwrap().clone();
						let new_index = tree_index + 1;
						tree.nodes.push(*tree.nodes.get(parent_index as usize).unwrap());
						tree.node_indexes_to_ids.remove(&parent_index);
						tree.ids_to_node_indexes.insert(parent_id.clone(), new_index);
						tree.node_indexes_to_ids.insert(new_index, parent_id);
					}
				} else {
					let tree_index = tree.stack.get(tree.stack.len() - 1);
					tree.nodes[*tree_index.unwrap() as usize] = value;
					tree.stack.pop();
				}

				tree.ids_to_node_indexes.insert(citizen_id.clone(), tree_index);
				tree.node_indexes_to_ids.insert(tree_index, citizen_id);

				// update_parents 🟥

				Self::update_parents(tree, tree_index, true, value, key);
			}
		}

		pub fn stake_of(
			key: SumTreeName,
			citizen_id: AccountIdOf<T>,
		) -> Result<Option<u64>, DispatchError> {
			let tree_option = <SortitionSumTrees<T>>::get(&key);
			match tree_option {
				None => Err(Error::<T>::TreeDoesnotExist)?,
				Some(tree) => {
					let tree_index_data;
					match tree.ids_to_node_indexes.get(&citizen_id) {
						Some(v) => tree_index_data = v,
						None => return Ok(None),
					}

					let value: u64;
					let tree_index = *tree_index_data;
					if tree_index == 0 {
						value = 0;
					} else {
						value = tree.nodes[tree_index as usize];
					}
					Ok(Some(value))
				}
			}
		}

		pub fn draw(key: SumTreeName, draw_number: u64) -> Result<AccountIdOf<T>, DispatchError> {
			let tree_option = <SortitionSumTrees<T>>::get(&key);

			match tree_option {
				None => Err(Error::<T>::TreeDoesnotExist)?,
				Some(tree) => {
					let mut tree_index = 0;
					let mut current_draw_number = draw_number % tree.nodes[0];

					while (tree.k * tree_index) + 1 < (tree.nodes.len() as u64) {
						for i in 1..tree.k + 1 {
							let node_index = (tree.k * tree_index) + i;
							let node_value = tree.nodes[node_index as usize];

							if current_draw_number >= node_value {
								current_draw_number -= node_value;
							} else {
								tree_index = node_index;
								break;
							}
						}
					}
					let account_id = tree.node_indexes_to_ids.get(&tree_index).unwrap().clone();
					Ok(account_id)
				}
			}
		}

		/**
		 *  @dev Query the leaves of a tree. Note that if `startIndex == 0`, the tree is empty and the root node will be returned.
		 *  @param key The key of the tree to get the leaves from.
		 *  @param cursor The pagination cursor.
		 *  @param count The number of items to return.
		 *  @return The index at which leaves start, the values of the returned leaves, and whether there are more for pagination.
		 *  `O(n)` where
		 *  `n` is the maximum number of nodes ever appended.
		 */
		pub fn query_leafs(
			key: SumTreeName,
			cursor: u64,
			count: u64,
		) -> Result<(u64, Vec<u64>, bool), DispatchError> {
			let tree_option = <SortitionSumTrees<T>>::get(&key);

			match tree_option {
				None => Err(Error::<T>::TreeDoesnotExist)?,
				Some(tree) => {
					let mut start_index = 0;
					for i in 0..tree.nodes.len() {
						if (tree.k * i as u64) + 1 >= tree.nodes.len() as u64 {
							start_index = i as u64;
							break;
						}
					}
					let loop_start_index = start_index + cursor;

					// let value = if loop_start_index + count > tree.nodes.len() as u64 {
					// 	tree.nodes.len() as u64 - loop_start_index
					// } else {
					// 	count
					// };

					let mut values = Vec::new();
					let mut values_index = 0;
					let mut has_more = false;
					for j in loop_start_index..tree.nodes.len() as u64 {
						if values_index < count {
							values.push(tree.nodes[j as usize]);
							values_index = values_index + 1;
						} else {
							has_more = true;
							break;
						}
					}

					Ok((start_index, values, has_more))
				}
			}
		}
	}
}
