use specs::prelude::*;
use specs_derive::*;
use rltk::{RGB};

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component, Debug)]
pub struct Name {
    pub name: String,
}

#[derive(Component)]
pub struct Item {}

#[derive(Component)]
pub struct Stored {}

// PERSONAL WORK
#[derive(Component)]
pub struct Impassable {}

#[derive(Component)]
pub struct RequiresItem {
    pub key: Entity,
}

