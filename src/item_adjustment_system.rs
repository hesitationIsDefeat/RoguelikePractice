use specs::{Entities, Join, ReadStorage, System, WriteStorage};
use crate::{Portal, Renderable, RequiresItem};
use crate::constants::ITEM_PORTAL_COLOR;

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