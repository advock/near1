use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::json_types::U128;
use near_sdk::serde::Serialize;
use near_sdk::{env, near_bindgen, AccountId, Balance};

const POINT_ONE: Balance = 100_000_000_000_000_000_000_000;
