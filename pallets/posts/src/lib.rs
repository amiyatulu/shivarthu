#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

mod extras;
pub mod functions;
pub mod types;

pub use types::{Comment, Post, PostExtension, PostUpdate, FIRST_POST_ID};

use codec::{Decode, Encode};

use frame_support::pallet_prelude::*;
use frame_support::sp_std::prelude::*;
use frame_system::pallet_prelude::*;
use pallet_spaces::{types::Space, Pallet as Spaces};
use pallet_support::{
	ensure_content_is_valid, new_who_and_when, remove_from_vec, Content, PostId, SpaceId,
	WhoAndWhen, WhoAndWhenOf,
};

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config:
		frame_system::Config + pallet_timestamp::Config + pallet_spaces::Config
	{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::type_value]
	pub fn DefaultForNextPostId() -> PostId {
		FIRST_POST_ID
	}

	/// The next post id.
	#[pallet::storage]
	#[pallet::getter(fn next_post_id)]
	pub type NextPostId<T: Config> = StorageValue<_, PostId, ValueQuery, DefaultForNextPostId>;

	/// Get the details of a post by its' id.
	#[pallet::storage]
	#[pallet::getter(fn post_by_id)]
	pub type PostById<T: Config> = StorageMap<_, Twox64Concat, PostId, Post<T>>;

	/// Get the ids of all direct replies by their parent's post id.
	#[pallet::storage]
	#[pallet::getter(fn reply_ids_by_post_id)]
	pub type ReplyIdsByPostId<T: Config> =
		StorageMap<_, Twox64Concat, PostId, Vec<PostId>, ValueQuery>;

	/// Get the ids of all posts in a given space, by the space's id.
	#[pallet::storage]
	#[pallet::getter(fn post_ids_by_space_id)]
	pub type PostIdsBySpaceId<T: Config> =
		StorageMap<_, Twox64Concat, SpaceId, Vec<PostId>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored {
			something: u32,
			who: T::AccountId,
		},
		PostCreated {
			account: T::AccountId,
			post_id: PostId,
		},
		PostUpdated {
			account: T::AccountId,
			post_id: PostId,
		},
		PostMoved {
			account: T::AccountId,
			post_id: PostId,
			from_space: Option<SpaceId>,
			to_space: Option<SpaceId>,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		// Post related errors:
		/// Post was not found by id.
		PostNotFound,
		/// An account is not a post owner.
		NotAPostOwner,
		/// Nothing to update in this post.
		NoUpdatesForPost,
		/// Root post should have a space id.
		PostHasNoSpaceId,
		/// Not allowed to create a post/comment when a scope (space or root post) is hidden.
		CannotCreateInHiddenScope,
		/// Post has no replies.
		NoRepliesOnPost,
		/// Cannot move a post to the same space.
		CannotMoveToSameSpace,

		// Share related errors:
		/// Cannot share, because the original post was not found.
		OriginalPostNotFound,
		/// Cannot share a post that is sharing another post.
		CannotShareSharedPost,
		/// This post's extension is not a `SharedPost`.
		NotASharedPost,

		// Comment related errors:
		/// Unknown parent comment id.
		UnknownParentComment,
		/// Post by `parent_id` is not of a `Comment` extension.
		NotACommentByParentId,
		/// Cannot update space id of a comment.
		CannotUpdateSpaceIdOnComment,
		/// Max comment depth reached.
		MaxCommentDepthReached,
		/// Only comment owner can update this comment.
		NotACommentAuthor,
		/// This post's extension is not a `Comment`.
		NotComment,

		// Permissions related errors:
		/// User has no permission to create root posts in this space.
		NoPermissionToCreatePosts,
		/// User has no permission to create comments (aka replies) in this space.
		NoPermissionToCreateComments,
		/// User has no permission to share posts/comments from this space to another space.
		NoPermissionToShare,
		/// User has no permission to update any posts in this space.
		NoPermissionToUpdateAnyPost,
		/// A post owner is not allowed to update their own posts in this space.
		NoPermissionToUpdateOwnPosts,
		/// A comment owner is not allowed to update their own comments in this space.
		NoPermissionToUpdateOwnComments,

		/// `force_create_post` failed, because this post already exists.
		/// Consider removing the post with `force_remove_post` first.
		PostAlreadyExists,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create post
		///  Who can post, does kyc validation required??
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn create_post(
			origin: OriginFor<T>,
			space_id_opt: Option<SpaceId>,
			extension: PostExtension,
			content: Content,
		) -> DispatchResult {
			let creator = ensure_signed(origin)?;

			ensure_content_is_valid(content.clone())?;

			let new_post_id = Self::next_post_id();

			let new_post: Post<T> =
				Post::new(new_post_id, creator.clone(), space_id_opt, extension, content.clone());

			// Get space from either space_id_opt or Comment if a comment provided
			let space = &new_post.get_space()?;
			if new_post.is_root_post() {
				PostIdsBySpaceId::<T>::mutate(space.id, |ids| ids.push(new_post_id));
			}

			PostById::insert(new_post_id, new_post);
			NextPostId::<T>::mutate(|n| {
				*n += 1;
			});

			Self::deposit_event(Event::PostCreated { account: creator, post_id: new_post_id });

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn update_post(
			origin: OriginFor<T>,
			post_id: PostId,
			update: PostUpdate,
		) -> DispatchResult {
			let editor = ensure_signed(origin)?;

			let has_updates = update.content.is_some() || update.hidden.is_some();

			ensure!(has_updates, Error::<T>::NoUpdatesForPost);

			let mut post = Self::require_post(post_id)?;

			let space_opt = &post.try_get_space();

			let mut is_update_applied = false;

			if let Some(content) = update.content {
				if content != post.content {
					ensure_content_is_valid(content.clone())?;

					post.content = content;
					post.edited = true;
					is_update_applied = true;
				}
			}

			if let Some(hidden) = update.hidden {
				if hidden != post.hidden {
					post.hidden = hidden;
					is_update_applied = true;
				}
			}

			// Update this post only if at least one field should be updated:
			if is_update_applied {
				<PostById<T>>::insert(post.id, post);
				Self::deposit_event(Event::PostUpdated { account: editor, post_id });
			}

			Ok(())
		}
	}
}
