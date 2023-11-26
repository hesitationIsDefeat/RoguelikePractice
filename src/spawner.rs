use rltk::{BLACK, BLUE, RGB, YELLOW};
use specs::prelude::*;
use specs::saveload::{MarkedBuilder, SimpleMarker};
use crate::{Name, Player, Position, Renderable, State, Item, Impassable, RequiresItem, Door, PermanentItem, SerializeMe, Place, BelongsTo, Portal};

pub fn build_player(gs: &mut State, name: String, coord: (i32, i32), glyph: rltk::FontCharType, fg: RGB, bg: RGB) -> Entity {
    gs.ecs
        .create_entity()
        .with(Name { name })
        .with(BelongsTo { domain: Place::School })
        .with(Position { x: coord.0, y: coord.1 })
        .with(Renderable { glyph, fg, bg, render_order: 0 })
        .with(Player {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

pub fn build_key(gs: &mut State, name: String, domain: Place, coord: (i32, i32)) -> Entity {
    gs.ecs
        .create_entity()
        .with(Name { name })
        .with(BelongsTo { domain })
        .with(Position { x: coord.0, y: coord.1 })
        .with(Renderable { glyph: rltk::to_cp437('k'), fg: RGB::named(YELLOW), bg: RGB::named(BLACK), render_order: 1 })
        .with(Item {})
        .with(PermanentItem {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

pub fn build_door(gs: &mut State, name: String, domain: Place, coord: (i32, i32), target: Place, image: char, key: Entity) -> Entity {
    gs.ecs
        .create_entity()
        .with(Name { name })
        .with(BelongsTo { domain })
        .with(Position { x: coord.0, y: coord.1 })
        .with(Portal { target })
        .with(Renderable { glyph: rltk::to_cp437(image), fg: RGB::named(BLUE), bg: RGB::named(BLACK), render_order: 1 })
        .with(Impassable {})
        .with(Door {})
        .with(RequiresItem { key })
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}