use specs::prelude::*;
use specs::saveload::{MarkedBuilder, SimpleMarker};
use crate::{Name, Player, Position, Renderable, State, Item, RequiresItem, PermanentItem, SerializeMe, Place, BelongsTo, Portal, ContainsItem, Npc, Interaction, RequiresItems, ContainsItems};
use crate::constants::{BACKGROUND_COLOR, ITEM_DOOR_COLOR, ITEM_KEY_COLOR, ITEM_PORTAL_COLOR, KEY_CHAR, NPC_CHAR, NPC_COLOR, PLAYER_CHAR, PLAYER_COLOR, PORTAL_CHAR};
use crate::items::ItemName;

pub fn build_player(gs: &mut State, name: String, coord: (i32, i32)) -> Entity {
    gs.ecs
        .create_entity()
        .with(Name { name })
        .with(BelongsTo { domain: Place::Home })
        .with(Position { x: coord.0, y: coord.1 })
        .with(Renderable { glyph: rltk::to_cp437(PLAYER_CHAR), fg: PLAYER_COLOR, bg: BACKGROUND_COLOR, render_order: 0 })
        .with(Player {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

pub fn build_active_item(gs: &mut State, name: ItemName, domain: Place, coord: (i32, i32), permanent: bool) -> Entity {
    let mut builder = gs.ecs
        .create_entity()
        .with(Name { name: name.to_string() })
        .with(BelongsTo { domain })
        .with(Position { x: coord.0, y: coord.1 })
        .with(Renderable { glyph: rltk::to_cp437(KEY_CHAR), fg: ITEM_KEY_COLOR, bg: BACKGROUND_COLOR, render_order: 1 })
        .with(Item { name })
        .marked::<SimpleMarker<SerializeMe>>();

    if permanent {
        builder = builder.with(PermanentItem {});
    }

    builder.build()
}

fn build_door_or_portal(gs: &mut State, name: String, domain: Place, coord: (i32, i32), target: Place, warp_place: (i32, i32), key: Option<ItemName>) -> Entity {
    let mut builder = gs.ecs
        .create_entity()
        .with(Name { name })
        .with(BelongsTo { domain })
        .with(Position { x: coord.0, y: coord.1 })
        .with(Portal { target, warp_place })
        .with(Renderable {
            glyph: rltk::to_cp437(PORTAL_CHAR),
            fg: match key {
                Some(_) => ITEM_DOOR_COLOR,
                None => ITEM_PORTAL_COLOR
            },
            bg: BACKGROUND_COLOR,
            render_order: 1,
        })
        .marked::<SimpleMarker<SerializeMe>>();

    if let Some(key) = key {
        builder = builder.with(RequiresItem { item: key });
    }

    builder.build()
}

pub fn build_door(gs: &mut State, name: String, domain: Place, coord: (i32, i32), target: Place, warp_place: (i32, i32), key: ItemName) -> Entity {
    build_door_or_portal(gs, name, domain, coord, target, warp_place, Some(key))
}

pub fn build_portal(gs: &mut State, name: String, domain: Place, coord: (i32, i32), target: Place, warp_place: (i32, i32)) -> Entity {
    build_door_or_portal(gs, name, domain, coord, target, warp_place, None)
}

pub fn build_npc(gs: &mut State, name: String, domain: Place, coord: (i32, i32), dialogues: Vec<Vec<&str>>,
                 requires_item: Option<Vec<ItemName>>, contains_item: Option<Vec<ItemName>>, get_item_indices: Vec<usize>, give_item_indices: Vec<usize>, change_objective_indices: Vec<usize>) -> Entity {
    gs.ecs
        .create_entity()
        .with(Name { name })
        .with(Npc {})
        .with(BelongsTo { domain })
        .with(Position { x: coord.0, y: coord.1 })
        .with(Renderable { glyph: rltk::to_cp437(NPC_CHAR), fg: NPC_COLOR, bg: BACKGROUND_COLOR, render_order: 1 })
        .with(Interaction {
            dialogues: dialogues.iter().map(|vec_s| vec_s.iter().map(|s| String::from(*s)).collect()).collect(),
            dialogue_index: 0,
            get_item_indices,
            give_item_indices,
            change_objective_indices,
        })
        .with(RequiresItems {
            items: match requires_item {
                None => { vec!() }
                Some(items) => { items }
            }
        })
        .with(ContainsItems {
            items: match contains_item {
                None => { vec!() }
                Some(items) => { items }
            }
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

pub fn build_dormant_item(gs: &mut State, name: ItemName) -> Entity {
    gs.ecs
        .create_entity()
        .with(Name { name: name.to_string() })
        .with(Item { name })
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}