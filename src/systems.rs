use rltk::Point;
use specs::{Entities, Join, Read, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};
use crate::{BelongsTo, DormantPosition, Item, Map, Name, Npc, Place, Portal, Position, Renderable, RequiresItem, RevealerInformation, RunState, Stored, TargetedPosition, TileType};
use crate::constants::{ITEM_PORTAL_COLOR, OTTOMAN_COMBINED_REWARD_MOSQUE_MODEL_NAME, OTTOMAN_COMBINED_REWARD_POEM_BOOK_NAME};
use crate::gamelog::GameLog;
use crate::items::ItemName;

pub struct ItemAdjustmentSystem {}

impl<'a> System<'a> for ItemAdjustmentSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, RequiresItem>,
        ReadStorage<'a, Portal>,
        WriteStorage<'a, Renderable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, doors, portals, mut renderables) = data;
        for (_, _, render) in (&entities, &portals, &mut renderables).join().filter(|(e, _, _)| !doors.contains(*e)) {
            render.fg = ITEM_PORTAL_COLOR;
        }
    }
}

pub struct ItemCollectionSystem {}

impl<'a> System<'a> for ItemCollectionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Point>,
        ReadExpect<'a, Place>,
        ReadStorage<'a, Item>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, BelongsTo>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Stored>,
        Entities<'a>);

    fn run(&mut self, data: Self::SystemData) {
        let (
            player_pos,
            current_place,
            items,
            names,
            belongs,
            mut log,
            mut positions,
            mut stored,
            entities) = data;
        let mut items_to_store = Vec::new();
        for (item_ent, item_pos, _item, bel) in (&entities, &positions, &items, &belongs).join() {
            if item_pos.x == player_pos.x && item_pos.y == player_pos.y && bel.domain == *current_place {
                items_to_store.push(item_ent);
            }
        }
        for item in items_to_store {
            positions.remove(item);
            stored.insert(item, Stored {}).expect("Esya alinamadi");
            log.entries.push(format!("Esyayi aldin: {}", names.get(item).unwrap().name))
        }
    }
}

pub struct DoorRevealSystem {}

impl<'a> System<'a> for DoorRevealSystem {
    type SystemData = (ReadExpect<'a, Point>,
                       ReadExpect<'a, Place>,
                       ReadStorage<'a, Stored>,
                       ReadStorage<'a, Item>,
                       ReadStorage<'a, RevealerInformation>,
                       ReadStorage<'a, DormantPosition>,
                       ReadStorage<'a, BelongsTo>,
                       WriteExpect<'a, Map>,
                       WriteStorage<'a, Position>,
                       Entities<'a>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            player_pos,
            current_place,
            stored_items,
            items,
            revealer_infos,
            dormant_positions,
            belongs_to,
            mut map,
            mut positions,
            entities
        ) = data;
        for (bel, rev_info, dorm_pos, ent) in (&belongs_to, &revealer_infos, &dormant_positions, &entities).join() {
            let x_range = rev_info.x_end_points.0..=rev_info.x_end_points.1;
            let y_range = rev_info.y_end_points.0..=rev_info.y_end_points.1;
            if bel.domain == *current_place && x_range.contains(&player_pos.x) && y_range.contains(&player_pos.y) {
                for (item, _stored) in (&items, &stored_items).join() {
                    if item.name == rev_info.revealer_item && !positions.contains(ent) {
                        println!("a");
                        positions.insert(ent, Position { x: dorm_pos.x, y: dorm_pos.y }).expect("Couldn't insert position");
                        map.tiles[Map::xy_to_tile(dorm_pos.x, dorm_pos.y)] = TileType::RequiresKey;
                    }
                }
            } else if positions.contains(ent) {
                positions.remove(ent);
                map.tiles[Map::xy_to_tile(dorm_pos.x, dorm_pos.y)] = revealer_infos.get(ent).unwrap().before_reveal;
            }
        }
    }
}

pub struct GameStateAdjustmentSystem {}

impl<'a> System<'a> for GameStateAdjustmentSystem {
    type SystemData = (ReadExpect<'a, Place>,
                       ReadExpect<'a, TargetedPosition>,
                       ReadStorage<'a, Npc>,
                       ReadStorage<'a, BelongsTo>,
                       ReadStorage<'a, Position>,
                       WriteExpect<'a, RunState>);

    fn run(&mut self, data: Self::SystemData) {
        let (
            current_place,
            target_pos,
            npcs,
            belongs_to,
            positions,
            mut state
        ) = data;
        if target_pos.x >= 0 && target_pos.y >= 0 {
            for (_npcs, belongs, pos) in (&npcs, &belongs_to, &positions).join() {
                if pos.x == target_pos.x && pos.y == target_pos.y && belongs.domain == *current_place {
                    *state = RunState::InteractNpc { index: 0 };
                }
            }
        }
    }
}

pub struct ItemCombinationSystem {}

impl<'a> System<'a> for ItemCombinationSystem {
    type SystemData = (WriteStorage<'a, Stored>,
                       ReadStorage<'a, Item>,
                       Entities<'a>,
                       WriteExpect<'a, GameLog>);

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut stored_items,
            items,
            entities,
            mut log
        ) = data;

        let mut contains_poem_piece = false;
        let mut contains_book_cover = false;
        let mut contains_glue = false;

        let mut contains_mosque_1 = false;
        let mut contains_mosque_2 = false;

        let mut contains_note_paper = false;
        let mut contains_canvas = false;
        let mut contains_clay = false;
        for (item, _stored_item) in (&items, &stored_items).join() {
            match item.name {
                ItemName::OttomanRewardPoem => contains_poem_piece = true,
                ItemName::OttomanRewardBookCover => contains_book_cover = true,
                ItemName::OttomanRewardGlue => contains_glue = true,
                ItemName::OttomanRewardMosquePart1 => contains_mosque_1 = true,
                ItemName::OttomanRewardMosquePart2 => contains_mosque_2 = true,
                ItemName::OttomanRewardNotePaper => contains_note_paper = true,
                ItemName::OttomanRewardCanvas => contains_canvas = true,
                ItemName::OttomanRewardClay => contains_clay = true,
                _ => {}
            }
        }
        if contains_poem_piece && contains_book_cover && contains_glue {
            for (item, ent) in (&items, &entities).join() {
                if item.name == ItemName::OttomanRewardPoem || item.name == ItemName::OttomanRewardBookCover || item.name == ItemName::OttomanRewardGlue {
                    stored_items.remove(ent);
                }
            }
            for (item, ent) in (&items, &entities).join() {
                if item.name == ItemName::OttomanCombinedRewardPoemBook {
                    stored_items.insert(ent, Stored {}).expect("Couldn't insert sorted");
                    log.entries.push(format!("Yeni Esya Urettin: {}", OTTOMAN_COMBINED_REWARD_POEM_BOOK_NAME))
                }
            }
            contains_book_cover = false;
            contains_poem_piece = false;
            contains_glue = false;
        }

        if contains_mosque_1 && contains_mosque_2 {
            for (item, ent) in (&items, &entities).join() {
                if item.name == ItemName::OttomanRewardMosquePart1 || item.name == ItemName::OttomanRewardMosquePart2 {
                    stored_items.remove(ent);
                }
            }
            for (item, ent) in (&items, &entities).join() {
                if item.name == ItemName::OttomanCombinedRewardMosqueModel {
                    stored_items.insert(ent, Stored {}).expect("Couldn't insert sorted");
                    log.entries.push(format!("Yeni Esya Urettin: {}", OTTOMAN_COMBINED_REWARD_MOSQUE_MODEL_NAME))
                }
            }
            contains_mosque_1 = false;
            contains_mosque_2 = false;
        }

        if contains_note_paper && contains_canvas && contains_clay {
            for (item, ent) in (&items, &entities).join() {
                if item.name == ItemName::OttomanRewardNotePaper || item.name == ItemName::OttomanRewardCanvas || item.name == ItemName::OttomanRewardClay {
                    stored_items.remove(ent);
                }
            }
            for (item, ent) in (&items, &entities).join() {
                if item.name == ItemName::OttomanCombinedRewardWeirdCollage {
                    stored_items.insert(ent, Stored {}).expect("Couldn't insert sorted");
                    log.entries.push(format!("Yeni Esya Urettin: {}", OTTOMAN_COMBINED_REWARD_MOSQUE_MODEL_NAME))
                }
            }
            contains_note_paper = false;
            contains_canvas = false;
            contains_clay = false;
        }
    }
}