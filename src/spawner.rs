use rltk::{BLACK, BLUE, RGB, YELLOW};
use specs::prelude::*;
use crate::{Name, Player, Position, Renderable, State, Item, Impassable, RequiresItem};

pub fn build_player(gs: &mut State, name: String, coord: (i32, i32), glyph: rltk::FontCharType, fg: RGB, bg: RGB) -> Entity {
    gs.ecs
        .create_entity()
        .with(Position { x: coord.0, y: coord.1 })
        .with(Renderable { glyph, fg, bg })
        .with(Player {})
        .with(Name { name })
        .build()
}

pub fn build_key(gs: &mut State, name: String, coord: (i32, i32)) -> Entity {
    gs.ecs
        .create_entity()
        .with(Name { name })
        .with(Position { x: coord.0, y: coord.1 })
        .with(Renderable { glyph: rltk::to_cp437('k'), fg: RGB::named(YELLOW), bg: RGB::named(BLACK) })
        .with(Item {})
        .build()
}

pub fn build_barrier(gs: &mut State, name: String, coord: (i32, i32), image: char, key: Entity) -> Entity {
    gs.ecs
        .create_entity()
        .with(Name { name })
        .with(Position { x: coord.0, y: coord.1 })
        .with(Renderable { glyph: rltk::to_cp437(image), fg: RGB::named(BLUE), bg: RGB::named(BLACK) })
        .with(Impassable {})
        .with(RequiresItem { key })
        .build()
}