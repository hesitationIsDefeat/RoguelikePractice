use specs::prelude::*;
use specs::saveload::{Marker, ConvertSaveload};
use specs::error::NoError;
use specs_derive::*;
use rltk::{RGB};
use serde::{Deserialize, Serialize};
use crate::items::ItemName;
use crate::{Place, TileType};
use crate::npcs::NpcState;


#[derive(Component, ConvertSaveload, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct TargetedPosition {
    pub x: i32,
    pub y: i32,
}


#[derive(Component, ConvertSaveload, Clone)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Player {}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Name {
    pub name: String,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct PlayerName {
    pub name: String,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Item {
    pub name: ItemName,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Stored {}

// PERSONAL WORK
#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Impassable {}


#[derive(Component, ConvertSaveload, Clone)]
pub struct RequiresItem {
    pub key: ItemName,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct ContainsItem {
    pub item: ItemName,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct RequiresItems {
    pub items: Vec<ItemName>,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct ContainsItems {
    pub items: Vec<ItemName>,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct PermanentItem {}

pub struct SerializeMe;

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct SerializationHelper {
    pub map: super::map::Map,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Portal {
    pub target: Place,
    pub warp_place: (i32, i32),
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct BelongsTo {
    pub domain: Place,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Npc {
    pub state: NpcState,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Objective {
    pub objectives: Vec<String>,
    pub index: usize,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Interaction {
    pub dialogues: Vec<Vec<String>>,
    pub dialogue_index: usize,
    pub get_item_indices: Vec<usize>,
    pub give_item_indices: Vec<usize>,
    pub change_objective_indices: Vec<usize>,
    pub repeat: bool,
    pub print_no_item: bool,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct DormantPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct RevealerInformation {
    pub x_end_points: (i32, i32),
    pub y_end_points: (i32, i32),
    pub revealer_item: ItemName,
    pub before_reveal: TileType,
}