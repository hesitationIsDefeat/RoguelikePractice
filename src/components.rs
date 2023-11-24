use specs::prelude::*;
use specs::saveload::{Marker, ConvertSaveload};
use specs::error::NoError;
use specs_derive::*;
use rltk::{RGB};
use serde::{Deserialize, Serialize};


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

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Item {}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Stored {}

// PERSONAL WORK
#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Impassable {}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Door {}

#[derive(Component, ConvertSaveload, Clone)]
pub struct RequiresItem {
    pub key: Entity,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct PermanentItem {}

pub struct SerializeMe;

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct SerializationHelper {
    pub map: super::map::Map,
}

