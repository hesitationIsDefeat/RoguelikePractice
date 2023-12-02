use std::fmt::{Debug, Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::constants::{ITEM_LIB_KEY_NAME, ITEM_OLD_KEY_1_NAME, ITEM_OLD_KEY_2_NAME, ITEM_SWORD_NAME};

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum ItemName {
    LibKey,
    OldKey1,
    OldKey2,
    Sword,
}

impl Display for ItemName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ItemName::LibKey => { ITEM_LIB_KEY_NAME }
            ItemName::OldKey1 => { ITEM_OLD_KEY_1_NAME }
            ItemName::OldKey2 => { ITEM_OLD_KEY_2_NAME }
            ItemName::Sword => { ITEM_SWORD_NAME }
        }.to_string();
        write!(f, "{}", str)
    }
}