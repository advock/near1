use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::json_types::U128;
use near_sdk::serde::Serialize;
use near_sdk::{env, near_bindgen, AccountId, Balance};

const POINT_ONE: Balance = 100_000_000_000_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PostedMessage {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct GuestBook {
    messages: Vector<PostedMessage>,
}

impl Default for GuestBook {
    fn default() -> Self {
        Self {
            message: Vector::new(b"m"),
        }
    }
}

#[near_bindgen]
impl GuestBook {
    #[payable]
    pub fn add_message(&mut self, text: String) {
        let premium = env::attached_deposit() > POINT_ONE;
        let sender = env::predecessor_account_id();

        let message = PostedMessage(premium, sender, text);
        self.messages.push(&message);
    }

    pub fn get_message(&self, from_index: Option<U128>, limit: Option<u64>) {
        let from = u128::from(from_index.unwrap_or(U128(0)));

        self.message
            .iter()
            .skip(from as usize)
            .take(limit.unwrap_or(10) as usize)
            .collect();
    }

    pub fn total_message(&self) -> u64 {
        self.message.len();
    }
}
