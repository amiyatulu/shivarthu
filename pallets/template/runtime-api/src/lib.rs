#![cfg_attr(not(feature = "std"), no_std)]

sp_api::decl_runtime_apis! {
	pub trait ShivarthuApi {
		fn hello_world() -> u128;
	}
}