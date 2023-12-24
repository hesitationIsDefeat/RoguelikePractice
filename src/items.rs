use std::fmt::{Debug, Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::constants::{ITEM_BOOK_NAME, ITEM_SECRET_GATE_KEY_NAME, OTTOMAN_KEY_1_NAME, OTTOMAN_KEY_2_NAME, OTTOMAN_KEY_3_NAME, OTTOMAN_KEY_MAIN_NAME, OTTOMAN_REWARD_POEM_NAME, OTTOMAN_REWARD_BOOK_COVER_NAME, OTTOMAN_COMBINED_REWARD_POEM_BOOK_NAME, OTTOMAN_REWARD_GLUE_NAME, OTTOMAN_REWARD_MOSQUE_PART_1_NAME, OTTOMAN_REWARD_MOSQUE_PART_2_NAME, OTTOMAN_COMBINED_REWARD_MOSQUE_MODEL_NAME, OTTOMAN_REWARD_NOTE_PAPER_NAME, OTTOMAN_REWARD_CANVAS_NAME, OTTOMAN_REWARD_CLAY_NAME, OTTOMAN_COMBINED_REWARD_WEIRD_COLLAGE_NAME};

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ItemName {
    Book,
    SecretGateKey,
    OttomanKey1,
    OttomanRewardPoem,
    OttomanRewardBookCover,
    OttomanRewardGlue,
    OttomanCombinedRewardPoemBook,
    OttomanKey2,
    OttomanRewardMosquePart1,
    OttomanRewardMosquePart2,
    OttomanCombinedRewardMosqueModel,
    OttomanKey3,
    OttomanRewardNotePaper,
    OttomanRewardCanvas,
    OttomanRewardClay,
    OttomanCombinedRewardWeirdCollage,
    OttomanKeyMain,
}

impl Display for ItemName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ItemName::Book => ITEM_BOOK_NAME,
            ItemName::SecretGateKey => ITEM_SECRET_GATE_KEY_NAME,
            ItemName::OttomanKey1 => OTTOMAN_KEY_1_NAME,
            ItemName::OttomanRewardPoem => OTTOMAN_REWARD_POEM_NAME,
            ItemName::OttomanRewardBookCover => OTTOMAN_REWARD_BOOK_COVER_NAME,
            ItemName::OttomanRewardGlue => OTTOMAN_REWARD_GLUE_NAME,
            ItemName::OttomanCombinedRewardPoemBook => OTTOMAN_COMBINED_REWARD_POEM_BOOK_NAME,
            ItemName::OttomanKey2 => OTTOMAN_KEY_2_NAME,
            ItemName::OttomanRewardMosquePart1 => OTTOMAN_REWARD_MOSQUE_PART_1_NAME,
            ItemName::OttomanRewardMosquePart2 => OTTOMAN_REWARD_MOSQUE_PART_2_NAME,
            ItemName::OttomanCombinedRewardMosqueModel => OTTOMAN_COMBINED_REWARD_MOSQUE_MODEL_NAME,
            ItemName::OttomanKey3 => OTTOMAN_KEY_3_NAME,
            ItemName::OttomanRewardNotePaper => OTTOMAN_REWARD_NOTE_PAPER_NAME,
            ItemName::OttomanRewardCanvas => OTTOMAN_REWARD_CANVAS_NAME,
            ItemName::OttomanRewardClay => OTTOMAN_REWARD_CLAY_NAME,
            ItemName::OttomanKeyMain => OTTOMAN_KEY_MAIN_NAME,
            ItemName::OttomanCombinedRewardWeirdCollage => OTTOMAN_COMBINED_REWARD_WEIRD_COLLAGE_NAME,
        }.to_string();
        write!(f, "{}", str)
    }
}