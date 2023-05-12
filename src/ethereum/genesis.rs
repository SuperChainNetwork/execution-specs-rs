///
/// Genesis Configuration
/// ^^^^^^^^^^^^^^^^^^^^^
///
/// .. contents:: Table of Contents
///     :backlinks: none
///     :local:
///
/// Introduction
/// ------------
///
/// Functionalities and entities to obtain the genesis configurations for
/// different chains.
///
// NOTE: Import::Import unsupported
// NOTE: Import::Import unsupported

// use ::dataclasses::{dataclass};
// use ::typing::{Any, Dict, cast};
// use ::ethereum::{rlp};
// use ::ethereum::base_types::{U64, U256, Bytes, Bytes8, Bytes20, Uint, slotted_freezable};
// use ::ethereum::utils::hexadecimal::{hex_to_bytes, hex_to_bytes8, hex_to_bytes20, hex_to_u256, hex_to_uint};

use std::collections::HashMap;

use num_bigint::BigUint;
use num_traits::Num;

use super::{base_types::{Bytes20, U64, Uint, U256, Bytes8, Bytes}, exceptions::EthereumException};


type Address = Bytes20;

///
///     Configuration for the first block of an Ethereum chain.
///
///     Specifies the allocation of ether set out in the pre-sale, and some of
///     the fields of the genesis block.
///
#[derive(Default)]
pub struct GenesisConfiguration {
    pub chain_id: U64,
    pub difficulty: Uint,
    pub extra_data: Bytes,
    pub gas_limit: Uint,
    pub nonce: Bytes8,
    pub timestamp: U256,
    pub initial_balances: HashMap<Address, U256>,
}

// TODO: unhack
fn uint_from_hex(hex: &str) -> Option<BigUint> {
    if hex.starts_with("0x") {
        Some(BigUint::from_str_radix(&hex[2..], 16).unwrap())
    } else {
        None
    }
}

// TODO: unhack
fn bytes_from_hex(hex: &str) -> Option<Bytes> {
    if hex.starts_with("0x") {
        let mut res = vec![];
        for d in hex[2..].as_bytes().chunks(2) {
            let d0 = if d[0] <= b'9' { d[0] } else { d[0].wrapping_sub(7) } & 0xf;
            let d1 = if d[1] <= b'9' { d[1] } else { d[1].wrapping_sub(7) } & 0xf;
            res.push(d0*16 + d1);
        }
        Some(Bytes::from(res))
    } else {
        None
    }
}

///
///     Obtain the genesis configuration from the given genesis json file.
///
///     The genesis file should be present in the `assets` directory.
///
///     Parameters
///     ----------
///     genesis_file :
///         The json file which contains the parameters for the genesis block
///         and the pre-sale allocation data.
///
///     Returns
///     -------
///     configuration : `GenesisConfiguration`
///         The genesis configuration obtained from the json genesis file.
///
pub fn get_genesis_configuration(genesis_file: &str) -> Result<GenesisConfiguration, EthereumException> {
    let path = format!("execution-specs/src/ethereum/assets/{genesis_file}");
    let file = std::fs::read_to_string(&path)
        .map_err(|_| EthereumException::FileNotFound(path))?;

    let value : serde_json::Value = serde_json::from_str(&file)
        .map_err(|e| EthereumException::JsonDecodeError(e.to_string()))?;


    let mut res = GenesisConfiguration::default();
    // pub chain_id: U64,
    // pub difficulty: Uint,
    // pub extra_data: Bytes,
    // pub gas_limit: Uint,
    // pub nonce: Bytes8,
    // pub timestamp: U256,

    let nonce = bytes_from_hex(value["nonce"].as_str().unwrap()).unwrap();
    res.nonce[8-nonce.len()..].copy_from_slice(&nonce);
    res.timestamp = uint_from_hex(value["timestamp"].as_str().unwrap()).unwrap();
    res.extra_data = bytes_from_hex(value["extraData"].as_str().unwrap()).unwrap();
    res.gas_limit = uint_from_hex(value["gasLimit"].as_str().unwrap()).unwrap();
    res.difficulty = uint_from_hex(value["difficulty"].as_str().unwrap()).unwrap();

    // TODO:

    // for v in value["alloc"].as_array().unwrap() {
    //     let v = v
    // }

    Ok(res)
}


///
///     Adds the genesis block to an empty blockchain.
///
///     The genesis block is an entirely sui generis block (unique) that is not
///     governed by the general rules applying to all other Ethereum blocks.
///     Instead, the only consensus requirement is that it must be identical to
///     the block added by this function.
///
///     The mainnet genesis configuration was originally created using the
///     `mk_genesis_block.py` script. It is long since defunct, but is still
///     available at https://github.com/ethereum/genesis_block_generator.
///
///     The initial state is populated with balances based on the Ethereum presale
///     that happened on the Bitcoin blockchain. Additional Ether worth 1.98% of
///     the presale was given to the foundation.
///
///     The `state_root` is set to the root of the initial state. The `gas_limit`
///     and `difficulty` are set to suitable starting values. In particular the
///     low gas limit made sending transactions impossible in the early stages of
///     Frontier.
///
///     The `nonce` field is `0x42` referencing Douglas Adams' "HitchHiker's Guide
///     to the Galaxy".
///
///     The `extra_data` field contains the hash of block `1028201` on
///     the pre-launch Olympus testnet. The creation of block `1028201` on Olympus
///     marked the "starting gun" for Ethereum block creation. Including its hash
///     in the genesis block ensured a fair launch of the Ethereum mining process.
///
///     The remaining fields are set to appropriate default values.
///
///     On testnets the genesis configuration usually allocates 1 wei to addresses
///     `0x00` to `0xFF` to avoid edgecases around precompiles being created or
///     cleared (by EIP 161).
///
///     Parameters
///     ----------
///     hardfork:
///         The module containing the initial hardfork
///     chain :
///         An empty `Blockchain` object.
///     genesis :
///         The genesis configuration to use.
///
pub fn add_genesis_block() {

}

// pub fn add_genesis_block<H : HardFork, C: BlockChain>(hardfork: H, chain: C, genesis: GenesisConfiguration) -> Result<(), Error> {
//     for (account, balance) in genesis.initial_balances.items()? {
//         hardfork.state.create_ether(chain.state, account, balance)?;
//     }
//     genesis_header = hardfork.eth_types.Header(parent_hash = hardfork.eth_types.Hash32([0] * 32)?, ommers_hash = rlp.rlp_hash(())?, coinbase = Address([0] * 20)?, state_root = hardfork.state.state_root(chain.state)?, transactions_root = hardfork.trie.root(hardfork.trie.Trie(false, ())?)?, receipt_root = hardfork.trie.root(hardfork.trie.Trie(false, ())?)?, bloom = hardfork.eth_types.Bloom([0] * 256)?, difficulty = genesis.difficulty, number = Uint(0)?, gas_limit = genesis.gas_limit, gas_used = Uint(0)?, timestamp = genesis.timestamp, extra_data = genesis.extra_data, mix_digest = hardfork.eth_types.Hash32([0] * 32)?, nonce = genesis.nonce)?;
//     genesis_block = hardfork.eth_types.Block(header = genesis_header, transactions = (), ommers = ())?;
//     chain.blocks.append(genesis_block)?;
//     chain.chain_id = genesis.chain_id;
// }