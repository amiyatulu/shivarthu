use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use shivarthu_runtime_api::ShivarthuApi as ShivarthuRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;

#[rpc]
pub trait ShivarthuApi<BlockHash> {
	#[rpc(name = "shivarthu_helloWorld")]
	fn hello_world(&self, at: Option<BlockHash>) -> Result<u128>;
	#[rpc(name = "shivarthu_challengerevidence")]
	fn get_challengers_evidence(
		&self,
		profile_citizenid: u128,
		offset: u64,
		limit: u16,
		at: Option<BlockHash>,
	) -> Result<Vec<u128>>;
	#[rpc(name = "shivarthu_evidenceperiodendblock")]
	fn get_evidence_period_end_block(
		&self,
		profile_citizenid: u128,
		at: Option<BlockHash>,
	) -> Result<Option<u32>>;
}

/// A struct that implements the `SumStorageApi`.
pub struct Shivarthu<C, M> {
	// If you have more generics, no need to SumStorage<C, M, N, P, ...>
	// just use a tuple like SumStorage<C, (M, N, P, ...)>
	client: Arc<C>,
	_marker: std::marker::PhantomData<M>,
}

impl<C, M> Shivarthu<C, M> {
	/// Create new `SumStorage` instance with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

impl<C, Block> ShivarthuApi<<Block as BlockT>::Hash> for Shivarthu<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block>,
	C::Api: ShivarthuRuntimeApi<Block>,
{
	fn hello_world(&self, at: Option<<Block as BlockT>::Hash>) -> Result<u128> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let runtime_api_result = api.hello_world(&at);
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
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
}
