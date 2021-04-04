#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
use frame_support::sp_runtime::RuntimeDebug;
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, traits::Get,
};
use frame_system::ensure_signed;
use sha3::{Digest, Keccak256};
use sp_std::vec::Vec;
// use rand::distributions::WeightedIndex;
// use rand::prelude::*;
// use rand::{rngs::StdRng, SeedableRng};

// Token
// SchellingGame (Try to make it generic)
// ApprovalVoting 🖊️
// Peer review of projects
// Escrow for projects
// Negative threhold votes to reclaim project funds

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, RuntimeDebug)]
pub struct DepartmentDetails {
	pub name: Vec<u8>,
	pub location: Vec<u8>,
	pub details: Vec<u8>,
	pub departmentid: u128,
}
// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Config> as TemplateModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		DepartmentCount get(fn deparment_count): u128;
		Admin get(fn admin) : T::AccountId;
		Citizen get(fn candidate_name): map hasher(blake2_128_concat) T::AccountId => Option<Vec<u8>>; // Peer account address => Peer Profile Hash
		Department get(fn department_name): map hasher(blake2_128_concat) u128 => DepartmentDetails;// Deparment id => (Name, Location, Details hash)
		OuterGroup get(fn outergroup): map hasher(blake2_128_concat) u128 => Vec<T::AccountId>; // Department id => Candidate account address set
		InnerGroup get(fn innergroup): map hasher(blake2_128_concat) u128 => Vec<T::AccountId>; // Department id => Candidate account address set
		FullGroup get(fn fullgroup): map hasher(blake2_128_concat) u128 => Vec<T::AccountId>; // Department id => Candidate account address set
		PeerDepartments get(fn peer_deparments): map hasher(blake2_128_concat) T::AccountId => Vec<u128>; // Peer account address => Department id set
		GovernorGroup get(fn governor_group): map hasher(blake2_128_concat) u128 => Vec<T::AccountId>; // Department id => Candidate account address set
		CandidatesNominees get(fn candidate_nominee): map hasher(blake2_128_concat) (u128, u128) => Vec<T::AccountId>; // Department id, Voting cycle => Candidate account address set
		CandidateApprovalVotes get(fn candidate_approval_votes): map hasher(blake2_128_concat) (T::AccountId, u128) => Option<u128>; // Candidate account address, Department id => Positive Votes
		CommitPhaseEndBlockCount get(fn commitphase_endblockcount): map hasher(blake2_128_concat) u128 => Option<u32>; // Department id => Number of blocks after commit phase ends
		VotingCycleTime get(fn voting_cycle_time): map hasher(blake2_128_concat) u128 => Option<u32>; // Department id => Voting cycle block count
		DefaultCommitPhaseEndBlockCount get(fn default_commit_phase_blockcount): Option<u32> = Some(216000); // Default commit phase end block count for all departments, 15 days with block time of 6 secs
		DefaultVotingCycleTime get(fn default_voting_cycle_time): Option<u32> = Some(1296000); // Default voting cycle time for all departments, 3 month with block time of 6 secs
		VotingCycleCount get(fn voting_cycle_count): map hasher(blake2_128_concat) u128 => Option<u128>; // Department id => Voting Cycle
		NumberOfVoteCast get(fn number_of_vote_cast): map hasher(blake2_128_concat) (u128, u128) => u128; // (Department id, Voting Cycle) => Number of votes
		VoteCommits get(fn vote_commits): map hasher(blake2_128_concat) (u128, u128) => Vec<Vec<u8>>; // (Department id, Voting Cycle) => Vote commit set
		VoteStatus get(fn vote_status): map hasher(blake2_128_concat) (u128, u128, Vec<u8>) => Option<bool>; // Department id, Voting Cycle, Votecommit => Status
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
	{
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, AccountId),
		CreateDepartment(u128, AccountId),
		PeerDepartment(u128, AccountId),
		CreateCitizen(AccountId, Vec<u8>),
		VoteCast(u128, u128, u128), // Departement id, cycle, department vote count
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		DepartmentExists,
		DepartmentDoNotExists,
		DepartmentNotAssociated,
		NomineeExists,
		CitizenDoNotExists,
		AlreadyVoted,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		// ⭐ Add Citizen Profile ⭐
		// Create profile ✔️
		// Update profile ✔️
		// Validate profile (staking and schelling game) during add_peers_to_deparment
		#[weight = 10_000 + T::DbWeight::get().reads_writes(2,1)]
		pub fn add_citizen(origin, profile_hash:Vec<u8>) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			Citizen::<T>::insert(&who, &profile_hash);
			Self::deposit_event(RawEvent::CreateCitizen(who, profile_hash));
			Ok(())
		}

		// ⭐ To Do ⭐
		// Who can add a department?
		// Who can edit the department?
		// Editing the department
		// Adding the department ✔️
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,2)]
		pub fn create_deparment(origin, name: Vec<u8>, location: Vec<u8>, details: Vec<u8>) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			let count = DepartmentCount::get();
			let dep_details = DepartmentDetails{
				name: name.clone(),
				location,
				details,
				departmentid: count.clone(),
			};
			Department::insert(&count, dep_details.clone());
			let newcount = count.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
			DepartmentCount::put(newcount);
			Self::deposit_event(RawEvent::CreateDepartment(count, who));
			Ok(())
		}

		// ⭐ To Do ⭐
		// Multi approval to vote a department through kyc and schelling game
		// Check profile is created  ✔️
		// Currently, direct approval ✔️
		#[weight = 10_000 + T::DbWeight::get().reads_writes(3,2)]
		pub fn add_peers_to_deparment(origin, departmentid: u128) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			Self::check_citizen_profile_exists(&who)?;
			let count = DepartmentCount::get();
			ensure!(departmentid <= count, Error::<T>::DepartmentDoNotExists);
			let mut approved_peer_dep = PeerDepartments::<T>::get(&who);

			match approved_peer_dep.binary_search(&departmentid) {
				Ok(_) => Err(Error::<T>::DepartmentExists.into()),
				Err(index) => {
					approved_peer_dep.insert(index, departmentid.clone());
					PeerDepartments::<T>::insert(&who,approved_peer_dep);
					Self::deposit_event(RawEvent::PeerDepartment(departmentid, who));
					Ok(())
				 }
			}
		 }

		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,0)]
		pub fn check_deparment_of_citizen(origin, departmentid:u128) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			Self::check_citizen_associated_department(who, departmentid)?;
			Ok(())
		}

		// ⭐ Approval Voting ⭐

		// Create or increment Voting cycle function

		// ⭐ Appoint Nominee ⭐
		// Can any one with validate evidence of expertise be nominee? If not what how to decrease the list, if nominees are in thousands
		// Check the candidate is approved for department. ✔️
		// Check its the right cycle
		// add the nominee for department cycle ✔️

		#[weight = 10_000 + T::DbWeight::get().reads_writes(2,1)]
		pub fn add_candidate_nominee(origin, departmentid:u128, voting_cycle: u128) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			Self::check_citizen_associated_department(who.clone(), departmentid)?; 
			let mut candidate_nominees = CandidatesNominees::<T>::get((departmentid, voting_cycle));
			match candidate_nominees.binary_search(&who) {
				Ok(_) => Err(Error::<T>::NomineeExists.into()),
				Err(index) => {
					candidate_nominees.insert(index, who);
					CandidatesNominees::<T>::insert((departmentid, voting_cycle), candidate_nominees);
					Ok(())
				}
			}
		}

		// ⭐ Commit Vote ⭐
		// Check blocknumber has not passed the reveal time
		// Check user is associated with department ✔️
		// Check voting status is false or None to vote ✔️
		// Add the commit to vote commit ✔️

		#[weight = 10_000 + T::DbWeight::get().reads_writes(3,3)]
		pub fn commit_vote(origin, departmentid:u128, voting_cycle:u128, vote_commit:Vec<u8>) -> dispatch::DispatchResult  {
			let who = ensure_signed(origin.clone())?;
			Self::check_citizen_associated_department(who.clone(), departmentid)?;
			let status = VoteStatus::get((departmentid, voting_cycle, vote_commit.clone()));
			match status {
				Some(value) => {
					if value == true {
						Err(Error::<T>::AlreadyVoted.into())
					} else {
					  Self::add_vote(departmentid, voting_cycle, vote_commit)?;
					  Ok(())
					}
				}
				None => {
					Self::add_vote(departmentid, voting_cycle, vote_commit)?;
					Ok(())
				}
			}

		}



	}
}

// Helper functions
impl<T: Config> Module<T> {
	fn check_citizen_profile_exists(who: &T::AccountId) -> dispatch::DispatchResult {
		let citizen = Citizen::<T>::get(&who);
		match citizen {
			Some(_) => Ok(()),
			None => Err(Error::<T>::CitizenDoNotExists.into()),
		}
	}
	fn check_citizen_associated_department(
		who: T::AccountId,
		departmentid: u128,
	) -> dispatch::DispatchResult {
		let approved_peer_dep = PeerDepartments::<T>::get(&who);

		match approved_peer_dep.binary_search(&departmentid) {
			Ok(_) => {
				Self::deposit_event(RawEvent::PeerDepartment(departmentid, who));
				Ok(())
			}
			Err(_) => Err(Error::<T>::DepartmentNotAssociated.into()),
		}
	}

	fn add_vote(
		departmentid: u128,
		voting_cycle: u128,
		vote_commit: Vec<u8>,
	) -> dispatch::DispatchResult {
		let mut vote_commit_vec = VoteCommits::get((departmentid, voting_cycle));
		vote_commit_vec.push(vote_commit.clone());
		VoteCommits::insert((departmentid, voting_cycle), vote_commit_vec);
		VoteStatus::insert((departmentid, voting_cycle, vote_commit), true);
		let count = NumberOfVoteCast::get((departmentid, voting_cycle));
		let newcount = count.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
		NumberOfVoteCast::insert((departmentid, voting_cycle), newcount);
		Self::deposit_event(RawEvent::VoteCast(departmentid, voting_cycle, newcount));
		Ok(())
	}
}
