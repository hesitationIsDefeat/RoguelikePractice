use rltk::Point;
use specs::{Entities, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};
use crate::{BelongsTo, Item, Name, Place, Portal, Position, Renderable, RequiresItem, Stored};
use crate::constants::ITEM_PORTAL_COLOR;
use crate::gamelog::GameLog;

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