use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use election_runtime_api::ElectionApi as ElectionRuntimeApi;
use sp_api::codec::Codec;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;


#[rpc]
pub trait ElectionApi<BlockHash, AccountId> {
	#[rpc(name = "election_candidateids")]
	fn candidate_ids(
		&self,
		departmentid: u128,
		at: Option<BlockHash>,
	) -> Result<Vec<AccountId>>;

	#[rpc(name = "election_membersids")]
	fn members_ids(
		&self,
		departmentid: u128,
		at: Option<BlockHash>,
	) -> Result<Vec<AccountId>>;

	#[rpc(name = "election_runnersupids")]
	fn runners_up_ids(
		&self,
		departmentid: u128,
		at: Option<BlockHash>,
	) -> Result<Vec<AccountId>>;
	
}

/// A struct that implements the `SumStorageApi`.
pub struct Election<C, M> {
	// If you have more generics, no need to SumStorage<C, M, N, P, ...>
	// just use a tuple like SumStorage<C, (M, N, P, ...)>
	client: Arc<C>,
	_marker: std::marker::PhantomData<M>,
}

impl<C, M> Election<C, M> {
	/// Create new `SumStorage` instance with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

impl<C, Block, AccountId> ElectionApi<<Block as BlockT>::Hash, AccountId> for Election<C, Block>
where
	Block: BlockT,
	AccountId: Codec,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block>,
	C::Api: ElectionRuntimeApi<Block, AccountId>,
{
	fn candidate_ids(
		&self,
		departmentid: u128,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<Vec<AccountId>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let runtime_api_result = api.candidate_ids(&at, departmentid);
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}

	fn members_ids(
		&self,
		departmentid: u128,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<Vec<AccountId>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let runtime_api_result = api.members_ids(&at, departmentid);
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}

	fn runners_up_ids(
		&self,
		departmentid: u128,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<Vec<AccountId>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let runtime_api_result = api.runners_up_ids(&at, departmentid);
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
}