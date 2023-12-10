use serde::{Deserialize, Serialize};

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum NpcState {
    HasDialogue,
    WantsItem,
    WillGiveItem,
    Done,
}