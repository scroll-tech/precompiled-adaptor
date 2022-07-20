use fp_evm::{
    Context, ExitError, ExitReason, ExitSucceed, LinearCostPrecompile, Precompile,
    PrecompileFailure, PrecompileHandle, PrecompileOutput, PrecompileResult, Transfer,
};
use pallet_evm_precompile_blake2::Blake2F;
use pallet_evm_precompile_bn128::{Bn128Add, Bn128Mul, Bn128Pairing};
use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_simple::{ECRecover, Identity, Ripemd160, Sha256};

#[cfg(not(feature = "legacy"))]
type Address = primitive_types::H160;
#[cfg(feature = "legacy")]
type Address = primitive_types_10::H160;

pub fn execute_precompiled(address: &Address, input: &[u8]) -> Vec<u8> {
    match address.as_bytes()[19] {
        0x01 => extract_linear_result(<ECRecover as LinearCostPrecompile>::execute(input, 0)),
        0x02 => extract_linear_result(<Sha256 as LinearCostPrecompile>::execute(input, 0)),
        0x03 => extract_linear_result(<Ripemd160 as LinearCostPrecompile>::execute(input, 0)),
        0x04 => extract_linear_result(<Identity as LinearCostPrecompile>::execute(input, 0)),
        0x05 => extract_result(Modexp::execute(&mut DummyHandler { input })),
        0x06 => extract_result(Bn128Add::execute(&mut DummyHandler { input })),
        0x07 => extract_result(Bn128Mul::execute(&mut DummyHandler { input })),
        0x08 => extract_result(Bn128Pairing::execute(&mut DummyHandler { input })),
        0x09 => extract_result(Blake2F::execute(&mut DummyHandler { input })),
        _ => panic!("calling non-exist precompiled contract address"),
    }
}

fn extract_linear_result(result: Result<(ExitSucceed, Vec<u8>), PrecompileFailure>) -> Vec<u8> {
    match result {
        Ok((exit_status, return_data)) => {
            assert_eq!(exit_status, ExitSucceed::Returned);
            return_data
        }
        Err(failure) => {
            unreachable!("{:?} should not happen in precompiled contract", failure)
        }
    }
}

fn extract_result(result: PrecompileResult) -> Vec<u8> {
    match result {
        Ok(PrecompileOutput {
            exit_status,
            output,
        }) => {
            assert_eq!(exit_status, ExitSucceed::Returned);
            output
        }
        Err(failure) => {
            unreachable!("{:?} should not happen in precompiled contract", failure)
        }
    }
}

struct DummyHandler<'a> {
    input: &'a [u8],
}

impl<'a> PrecompileHandle for DummyHandler<'a> {
    fn call(
        &mut self,
        _to: primitive_types::H160,
        _transfer: Option<Transfer>,
        _input: Vec<u8>,
        _gas_limit: Option<u64>,
        _is_static: bool,
        _context: &Context,
    ) -> (ExitReason, Vec<u8>) {
        unreachable!("we don't use this")
    }

    fn record_cost(&mut self, _: u64) -> Result<(), ExitError> {
        Ok(())
    }

    fn remaining_gas(&self) -> u64 {
        u64::MAX
    }

    fn log(
        &mut self,
        _: primitive_types::H160,
        _: Vec<primitive_types::H256>,
        _: Vec<u8>,
    ) -> Result<(), ExitError> {
        Ok(())
    }

    fn code_address(&self) -> primitive_types::H160 {
        unreachable!("we don't use this")
    }

    fn input(&self) -> &[u8] {
        self.input
    }

    fn context(&self) -> &Context {
        unreachable!("we don't use this")
    }

    fn is_static(&self) -> bool {
        unreachable!("we don't use this")
    }

    fn gas_limit(&self) -> Option<u64> {
        None
    }
}
