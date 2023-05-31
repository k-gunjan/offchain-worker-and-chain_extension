use crate::{
	sp_api_hidden_includes_construct_runtime::hidden_include::traits::Get, Encode, OffchainWorker,
};
use frame_support::{
	log::{error, trace},
	weights::{constants::RocksDbWeight, Weight},
};
use frame_system::RawOrigin;
use pallet_contracts::chain_extension::{
	ChainExtension, Environment, Ext, InitState, RetVal, SysConfig, UncheckedFrom,
};
use sp_runtime::{traits::StaticLookup, DispatchError};

#[derive(Default)]
pub struct DemoChainExtension;

impl<T> ChainExtension<T> for DemoChainExtension
where
	T: SysConfig + pallet_contracts::Config + pallet_offchain_worker::Config, /* add the pallets needed to interact with the contract */
	<T as SysConfig>::AccountId: UncheckedFrom<<T as SysConfig>::Hash> + AsRef<[u8]>,
{
	fn call<E: Ext>(&mut self, env: Environment<E, InitState>) -> Result<RetVal, DispatchError>
	where
		E: Ext<T = T>,
		<E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
	{
		let func_id = env.func_id();
		let mut env = env.buf_in_buf_out();

		// Match on function id assigned in the contract
		match func_id {
			3 | 4 => {
				match func_id {
					3 => {
						todo!();
					},

					4 => {
						trace!("contract called pallet. id {:?})", func_id);
						//fetch the price feed of pallet-offchianWorker and write into reply
						let result = OffchainWorker::prices().encode();
						env.write(&result, false, None).map_err(|_| {
							"Encountered an error when retrieving runtime storage value."
						})?;
					},
					_ => unreachable!(),
				}
			},
			_ => {
				error!("Called an unregistered `func_id`: {:}", func_id);
				return Err(DispatchError::Other("Unimplemented func_id"))
			},
		}
		// No error, return status code `0`, indicating `Ok(())`
		Ok(RetVal::Converging(0))
	}

	fn enabled() -> bool {
		true
	}
}
