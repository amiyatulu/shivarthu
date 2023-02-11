use frame_support::dispatch::DispatchResult;
use sp_runtime::traits::Saturating;

use pallet_support::{remove_from_vec, SpaceId};

use super::*;

impl<T: Config> Post<T> {
    pub fn new(
        id: PostId,
        created_by: T::AccountId,
        space_id_opt: Option<SpaceId>,
        extension: PostExtension,
        content: Content,
    ) -> Self {
        Post {
            id,
            created: new_who_and_when::<T>(created_by.clone()),
            edited: false,
            owner: created_by,
            extension,
            space_id: space_id_opt,
            content,
            hidden: false,
            upvotes_count: 0,
            downvotes_count: 0,
        }
    }

    pub fn ensure_owner(&self, account: &T::AccountId) -> DispatchResult {
        ensure!(self.is_owner(account), Error::<T>::NotAPostOwner);
        Ok(())
    }

    pub fn is_owner(&self, account: &T::AccountId) -> bool {
        self.owner == *account
    }

    pub fn is_root_post(&self) -> bool {
        !self.is_comment()
    }

    pub fn is_regular_post(&self) -> bool {
        matches!(self.extension, PostExtension::RegularPost)
    }

    pub fn is_comment(&self) -> bool {
        matches!(self.extension, PostExtension::Comment(_))
    }

    pub fn is_shared_post(&self) -> bool {
        matches!(self.extension, PostExtension::SharedPost(_))
    }

    pub fn get_comment_ext(&self) -> Result<Comment, DispatchError> {
        match self.extension {
            PostExtension::Comment(comment_ext) => Ok(comment_ext),
            _ => Err(Error::<T>::NotComment.into()),
        }
    }

    pub fn get_original_post_id(&self) -> Result<PostId, DispatchError> {
        match self.extension {
            PostExtension::SharedPost(original_post_id) => Ok(original_post_id),
            _ => Err(Error::<T>::NotASharedPost.into()),
        }
    }

    pub fn get_root_post(&self) -> Result<Post<T>, DispatchError> {
        match self.extension {
            PostExtension::RegularPost | PostExtension::SharedPost(_) => Ok(self.clone()),
            PostExtension::Comment(comment) => Pallet::<T>::require_post(comment.root_post_id),
        }
    }

    pub fn get_space_id(&self) -> Result<SpaceId, DispatchError> {
        Self::try_get_space_id(self).ok_or_else(|| Error::<T>::PostHasNoSpaceId.into())
    }

    pub fn try_get_space_id(&self) -> Option<SpaceId> {
        if let Ok(root_post) = self.get_root_post() {
            return root_post.space_id
        }

        None
    }

    pub fn get_space(&self) -> Result<Space<T>, DispatchError> {
        let root_post = self.get_root_post()?;
        let space_id = root_post.space_id.ok_or(Error::<T>::PostHasNoSpaceId)?;
        Spaces::require_space(space_id)
    }

    pub fn try_get_space(&self) -> Option<Space<T>> {
        if let Ok(root_post) = self.get_root_post() {
            return root_post.space_id.and_then(|space_id| Spaces::require_space(space_id).ok())
        }

        None
    }

    pub fn try_get_parent_id(&self) -> Option<PostId> {
        match self.extension {
            PostExtension::Comment(comment_ext) => comment_ext.parent_id,
            _ => None,
        }
    }

    pub fn inc_upvotes(&mut self) {
        self.upvotes_count.saturating_inc();
    }

    pub fn dec_upvotes(&mut self) {
        self.upvotes_count.saturating_dec();
    }

    pub fn inc_downvotes(&mut self) {
        self.downvotes_count.saturating_inc();
    }

    pub fn dec_downvotes(&mut self) {
        self.downvotes_count.saturating_dec();
    }

    pub fn is_public(&self) -> bool {
        !self.hidden && self.content.is_some()
    }

    pub fn is_unlisted(&self) -> bool {
        !self.is_public()
    }
}

impl<T: Config> Pallet<T> {
	   /// Get `Post` by id from the storage or return `PostNotFound` error.
       pub fn require_post(post_id: SpaceId) -> Result<Post<T>, DispatchError> {
        Ok(Self::post_by_id(post_id).ok_or(Error::<T>::PostNotFound)?)
    }
}
