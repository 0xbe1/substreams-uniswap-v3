use substreams::{log, Hex};
use substreams_ethereum::pb::eth as ethpb;
use crate::{eth, UniswapToken};

pub fn create_uniswap_token(token_address: &String) -> UniswapToken {
    let rpc_calls = create_rpc_calls(&hex::decode(token_address).unwrap());

    let rpc_responses_unmarshalled: ethpb::rpc::RpcResponses =
        substreams_ethereum::rpc::eth_call(&rpc_calls);
    let responses = rpc_responses_unmarshalled.responses;

    if responses[0].failed || responses[1].failed || responses[2].failed {
        let decimals_error = String::from_utf8_lossy(responses[0].raw.as_ref());
        let name_error = String::from_utf8_lossy(responses[1].raw.as_ref());
        let symbol_error = String::from_utf8_lossy(responses[2].raw.as_ref());

        log::debug!(
            "{} is not a an ERC20 token contract because of 'eth_call' failures [decimals: {}, name: {}, symbol: {}]",
            Hex(&token_address),
            decimals_error,
            name_error,
            symbol_error,
        );
        panic!("pool contains tokens which do not exist on chain addr {}", token_address)
    };

    let decoded_decimals = eth::read_uint32(responses[0].raw.as_ref());
    if decoded_decimals.is_err() {
        log::debug!(
            "{} is not a an ERC20 token contract decimal `eth_call` failed: {}",
            &token_address,
            decoded_decimals.err().unwrap(),
        );
        panic!("pool contains tokens which do not exist on chain")
    }
    log::debug!("decoded_decimals ok");

    let decoded_name = eth::read_string(responses[1].raw.as_ref());
    if decoded_name.is_err() {
        log::debug!(
            "{} is not a an ERC20 token contract name `eth_call` failed: {}",
            &token_address,
            decoded_name.err().unwrap(),
        );
        panic!("pool contains tokens which do not exist on chain")
    }
    log::debug!("decoded_name ok");

    let decoded_symbol = eth::read_string(responses[2].raw.as_ref());
    if decoded_symbol.is_err() {
        log::debug!(
            "{} is not a an ERC20 token contract symbol `eth_call` failed: {}",
            &token_address,
            decoded_symbol.err().unwrap(),
        );
        panic!("pool contains tokens which do not exist on chain")
    }
    log::debug!("decoded_symbol ok");

    let decimals = decoded_decimals.unwrap() as u64;
    let symbol = decoded_symbol.unwrap();
    let name = decoded_name.unwrap();

    return UniswapToken{
        address: String::from(token_address),
        name,
        symbol,
        decimals,
        whitelist_pools: vec![]
    };
}

fn create_rpc_calls(addr: &Vec<u8>) -> ethpb::rpc::RpcCalls {
    let decimals = hex::decode("313ce567").unwrap();
    let name = hex::decode("06fdde03").unwrap();
    let symbol = hex::decode("95d89b41").unwrap();

    return ethpb::rpc::RpcCalls {
        calls: vec![
            ethpb::rpc::RpcCall {
                to_addr: Vec::from(addr.clone()),
                method_signature: decimals,
            },
            ethpb::rpc::RpcCall {
                to_addr: Vec::from(addr.clone()),
                method_signature: name,
            },
            ethpb::rpc::RpcCall {
                to_addr: Vec::from(addr.clone()),
                method_signature: symbol,
            },
        ],
    };
}

