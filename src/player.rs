use rltk::{VirtualKeyCode, Rltk, Point};
use specs::prelude::*;
use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use super::{Position, Player, TileType, Map, State, RunState, TargetedPosition, Place, Portal, BelongsTo};

pub fn try_to_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) -> RunState {
    let mut player_point = ecs.write_resource::<Point>();
    let positions = ecs.write_storage::<Position>();
    let map = ecs.write_resource::<Map>();

    let new_x = (player_point.x + delta_x).max(0).min(MAP_WIDTH - 1);
    let new_y = (player_point.y + delta_y).max(0).min(MAP_HEIGHT - 1);
    match map.tiles[Map::xy_to_tile(new_x, new_y)] {
        TileType::Floor => {
            player_point.x = new_x;
            player_point.y = new_y;
            println!("x: {}, y: {}", new_x, new_y);
        }
        TileType::RequiresKey => {
            let mut targeted_pos = ecs.write_resource::<TargetedPosition>();
            targeted_pos.x = new_x;
            targeted_pos.y = new_y;
            return RunState::UseInventory;
        }
        TileType::Portal => {
            let mut current_place = ecs.write_resource::<Place>();
            let portals = ecs.read_storage::<Portal>();
            let mut belongs = ecs.write_storage::<BelongsTo>();
            let mut changed_domain = None;
            for (bel, pos, portal) in (&belongs, &positions, &portals).join() {
                if bel.domain == *current_place && pos.x == new_x && pos.y == new_y {
                    *current_place = portal.target;
                    *player_point = Point::new(portal.warp_place.0, portal.warp_place.1);
                    changed_domain = Some(portal.target);
                }
            }
            let mut player = ecs.write_storage::<Player>();
            if let Some(domain) = changed_domain {
                for (bel, _) in (&mut belongs, &mut player).join() {
                    bel.domain = domain;
                }
            }
        }
        TileType::NPC => {
            let mut targeted_pos = ecs.write_resource::<TargetedPosition>();
            targeted_pos.x = new_x;
            targeted_pos.y = new_y;
            return RunState::InteractNpc { index: 0 };
        }
        _ => {}
    }
    RunState::Game
}


pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Up => return try_to_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => return try_to_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::Left => return try_to_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => return try_to_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Escape => return RunState::SaveGame,
            _ => {}
        },
    }
    RunState::Game
}
