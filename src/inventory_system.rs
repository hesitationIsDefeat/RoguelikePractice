use rltk::Point;
use specs::{Entities, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};
use crate::gamelog::GameLog;
use crate::{Item, Name, Position, Stored};

pub struct ItemCollectionSystem {}

impl<'a> System<'a> for ItemCollectionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Point>,
        ReadStorage<'a, Item>,
        ReadStorage<'a, Name>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Stored>,
        Entities<'a>);

    fn run(&mut self, data: Self::SystemData) {
        let (
            player_pos,
            items,
            names,
            mut log,
            mut positions,
            mut stored,
            entities) = data;
        let mut items_to_store = Vec::new();
        for (item_ent, item_pos, _item) in (&entities, &positions, &items).join() {
            if item_pos.x == player_pos.x && item_pos.y == player_pos.y {
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
