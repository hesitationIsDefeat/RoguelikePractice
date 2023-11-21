use rltk::{VirtualKeyCode, Rltk, Point};
use specs::prelude::*;
use super::{Position, Player, TileType, Map, State, MAP_WIDTH, MAP_HEIGHT, Item, RunState};

pub fn try_to_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut player_point = ecs.write_resource::<Point>();
    let mut positions = ecs.write_storage::<Position>();
    let player = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    for (pos, _player) in (&mut positions, &player).join() {
        let new_x = (pos.x + delta_x).max(0).min(MAP_WIDTH - 1);
        let new_y = (pos.y + delta_y).max(0).min(MAP_HEIGHT - 1);
        match map.tiles[Map::xy_to_tile(new_x, new_y)] {
            TileType::Floor => {
                pos.x = new_x;
                player_point.x = new_x;
                pos.y = new_y;
                player_point.y = new_y;
            }
            _ => {}
        }
    }
}

pub fn collect_item(ecs: &mut World) {
    let player_pos = ecs.fetch::<Position>();
    let positions_read = ecs.read_storage::<Position>();
    let mut positions_write = ecs.write_storage::<Position>();
    let items = ecs.read_storage::<Item>();
    let entities = ecs.entities();

    for (pos, _item, item_entity) in (&positions_read, &items, &entities).join() {
        if player_pos.x == pos.x && player_pos.y == pos.y {
            positions_write.remove(item_entity);
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Up => try_to_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_to_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::Left => try_to_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_to_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::I => return RunState::Inventory,
            _ => {}
        },
    }
    RunState::Game
}
