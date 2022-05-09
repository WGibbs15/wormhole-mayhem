//#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::{env, ext_contract, near_bindgen, AccountId, Gas, Promise, PromiseResult};

use near_sdk::serde_json::Value;

use hex;

pub mod byte_utils;
pub mod state;

use crate::byte_utils::ByteUtils;

const CHAIN_ID_NEAR: u16 = 15;
const CHAIN_ID_SOL: u16 = 1;

#[ext_contract(ext_core_bridge)]
pub trait CoreBridge {
    fn verify_vaa(&self, vaa: String) -> (String, i32);
    fn publish_message(&self, data: Vec<u8>) -> u64;
}

#[ext_contract(ext_self)]
pub trait TokenBridgeCallback {
    fn submit_vaa_callback(&mut self);
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct TokenBridge {
    dups: UnorderedSet<Vec<u8>>,
    booted: bool,
    core: AccountId,
}

impl Default for TokenBridge {
    fn default() -> Self {
        Self {
            dups: UnorderedSet::new(b"d".to_vec()),
            booted: false,
            core: AccountId::new_unchecked("".to_string()),
        }
    }
}

fn hdl_governance(storage: &mut TokenBridge, vaa: state::ParsedVAA) {
    env::panic_str("governance not implemented");
}

#[near_bindgen]
impl TokenBridge {
    pub fn submit_vaa(&mut self, vaa: String) -> Promise {
        ext_core_bridge::verify_vaa(
            vaa,
            self.core.clone(),        // contract account id
            0,                        // yocto NEAR to attach
            Gas(100_000_000_000_000), // gas to attach
        )
        .then(ext_self::submit_vaa_callback(
            env::current_account_id(), // me
            0,                         // yocto NEAR to attach to the callback
            Gas(100_000_000_000_000),  // gas to attach
        ))
    }

    #[private] // So, all of wormhole security rests in this one statement?
    pub fn submit_vaa_callback(&mut self) {
        // well, and this one...
        if (env::promise_results_count() != 1)
            || (env::predecessor_account_id() != env::current_account_id())
        {
            env::panic_str("BadPredecessorAccount");
        }

        let data: String;
        match env::promise_result(0) {
            PromiseResult::Successful(result) => {
                data = String::from_utf8(result).unwrap();
            }
            _ => env::panic_str("vaaVerifyFail"),
        }

        let v: Value = near_sdk::serde_json::from_str(&data).unwrap();

        // Please, what is the correct way of just getting a fricken string?!
        let _vaa = v[0].to_string();
        let vaa = &_vaa[1.._vaa.len() - 1];

        let gov_idx = v[1].as_i64().unwrap() as u32;

        let h = hex::decode(vaa).expect("invalidVaa");

        let vaa = state::ParsedVAA::parse(&h);

        if vaa.version != 1 {
            env::panic_str("InvalidVersion");
        }

        // Check if VAA with this hash was already accepted
        if self.dups.contains(&vaa.hash) {
            env::panic_str("alreadyExecuted");
        }
        self.dups.insert(&vaa.hash);

        let data: &[u8] = &vaa.payload;

        if data[0..32]
            == hex::decode("000000000000000000000000000000000000000000546f6b656e427269646765")
                .unwrap()
        {
            if gov_idx != vaa.guardian_set_index {
                env::panic_str("InvalidGovernanceSet");
            }

            if (CHAIN_ID_SOL != vaa.emitter_chain)
                || (hex::decode("0000000000000000000000000000000000000000000000000000000000000004")
                    .unwrap()
                    != vaa.emitter_address)
            {
                env::panic_str("InvalidGovernanceEmitter");
            }

            hdl_governance(self, vaa);
            return;
        }

        env::log_str("looking good");
    }

    pub fn boot_portal(&mut self, core: String) {
        if self.booted {
            env::panic_str("no donut");
        }
        self.booted = true;
        self.core = AccountId::try_from(core.clone()).unwrap();
    }
}
