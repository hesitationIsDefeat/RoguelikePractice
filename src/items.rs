use std::fmt::{Debug, Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::constants::{ITEM_BOOK_NAME, ITEM_SECRET_GATE_KEY_NAME, OTTOMAN_KEY_1_NAME, OTTOMAN_KEY_2_NAME, OTTOMAN_KEY_3_NAME, OTTOMAN_KEY_4_NAME, OTTOMAN_KEY_MAIN_NAME, OTTOMAN_REWARD_1_NAME, OTTOMAN_REWARD_2_NAME, OTTOMAN_REWARD_3_NAME, OTTOMAN_REWARD_4_NAME};

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ItemName {
    Book,
    SecretGateKey,
    OttomanKey1,
    OttomanReward1,
    OttomanKey2,
    OttomanReward2,
    OttomanKey3,
    OttomanReward3,
    OttomanKey4,
    OttomanReward4,
    OttomanKeyMain,
}

impl Display for ItemName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ItemName::Book => ITEM_BOOK_NAME,
            ItemName::SecretGateKey => ITEM_SECRET_GATE_KEY_NAME,
            ItemName::OttomanKey1 => OTTOMAN_KEY_1_NAME,
            ItemName::OttomanReward1 => OTTOMAN_REWARD_1_NAME,
            ItemName::OttomanKey2 => OTTOMAN_KEY_2_NAME,
            ItemName::OttomanReward2 => OTTOMAN_REWARD_2_NAME,
            ItemName::OttomanKey3 => OTTOMAN_KEY_3_NAME,
            ItemName::OttomanReward3 => OTTOMAN_REWARD_3_NAME,
            ItemName::OttomanKey4 => OTTOMAN_KEY_4_NAME,
            ItemName::OttomanReward4 => OTTOMAN_REWARD_4_NAME,
            ItemName::OttomanKeyMain => OTTOMAN_KEY_MAIN_NAME,
        }.to_string();
        write!(f, "{}", str)
    }
}