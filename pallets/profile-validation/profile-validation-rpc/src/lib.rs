use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use profile_validation_runtime_api::ProfileValidationApi as ProfileValidationRuntimeApi;
use sp_api::codec::Codec;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;

#[rpc]
pub trait ProfileValidationApi<BlockHash, AccountId> {
	#[rpc(name = "profilevalidation_challengerevidence")]
	fn get_challengers_evidence(
		&self,
		profile_citizenid: u128,
		offset: u64,
		limit: u16,
		at: Option<BlockHash>,
	) -> Result<Vec<u128>>;
	#[rpc(name = "profilevalidation_evidenceperiodendblock")]
	fn get_evidence_period_end_block(
		&self,
		profile_citizenid: u128,
		at: Option<BlockHash>,
	) -> Result<Option<u32>>;
	#[rpc(name = "profilevalidation_stakingperiodendblock")]
	fn get_staking_period_end_block(
		&self,
		rofile_citizenid: u128,
		at: Option<BlockHash>,
	) -> Result<Option<u32>>;
	#[rpc(name = "profilevalidation_drawingperiodend")]
	fn get_drawing_period_end(
		&self,
		profile_citizenid: u128,
		at: Option<BlockHash>,
	) -> Result<(u64, u64, bool)>;
	#[rpc(name = "profilevalidation_commitendblock")]
	fn get_commit_period_end_block(
		&self,
		profile_citizenid: u128,
		at: Option<BlockHash>,
	) -> Result<Option<u32>>;
	#[rpc(name = "profilevalidation_voteendblock")]
	fn get_vote_period_end_block(
		&self,
		profile_citizenid: u128,
		at: Option<BlockHash>,
	) -> Result<Option<u32>>;
	#[rpc(name = "profilevalidation_selectedjuror")]
	fn selected_as_juror(
		&self,
		profile_citizenid: u128,
		who: AccountId,
		at: Option<BlockHash>,
	) -> Result<bool>;
}

/// A struct that implements the `SumStorageApi`.
pub struct ProfileValidation<C, M> {
	// If you have more generics, no need to SumStorage<C, M, N, P, ...>
	// just use a tuple like SumStorage<C, (M, N, P, ...)>
	client: Arc<C>,
	_marker: std::marker::PhantomData<M>,
}

impl<C, M> ProfileValidation<C, M> {
	/// Create new `SumStorage` instance with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

impl<C, Block, AccountId> ProfileValidationApi<<Block as BlockT>::Hash, AccountId> for ProfileValidation<C, Block>
where
	Block: BlockT,
	AccountId: Codec,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block>,
	C::Api: ProfileValidationRuntimeApi<Block, AccountId>,
{
	fn get_challengers_evidence(
		&self,
		profile_citizenid: u128,
		offset: u64,
		limit: u16,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<Vec<u128>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let runtime_api_result =
			api.get_challengers_evidence(&at, profile_citizenid, offset, limit);
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
	fn get_evidence_period_end_block(
		&self,
		profile_citizenid: u128,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<Option<u32>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let runtime_api_result = api.get_evidence_period_end_block(&at, profile_citizenid);
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
	fn get_staking_period_end_block(
		&self,
		profile_citizenid: u128,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<Option<u32>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let runtime_api_result = api.get_staking_period_end_block(&at, profile_citizenid);
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
	fn get_drawing_period_end(
		&self,
		profile_citizenid: u128,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<(u64, u64, bool)> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let runtime_api_result = api.get_drawing_period_end(&at, profile_citizenid);
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}

	fn get_commit_period_end_block(
		&self,
		profile_citizenid: u128,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<Option<u32>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let runtime_api_result = api.get_commit_period_end_block(&at, profile_citizenid);
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}

	fn get_vote_period_end_block(
		&self,
		profile_citizenid: u128,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<Option<u32>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let runtime_api_result = api.get_vote_period_end_block(&at, profile_citizenid);
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}

	fn selected_as_juror(
		&self,
		profile_citizenid: u128,
		who: AccountId,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<bool> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let runtime_api_result = api.selected_as_juror(&at, profile_citizenid, who);
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
}
