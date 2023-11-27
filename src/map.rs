use std::fmt::Display;
use rltk::{RGB, Rltk, BLACK, RED};
use serde::{Deserialize, Serialize};
use specs::{Join, World, WorldExt};
use crate::constants::{BACKGROUND_COLOR, MAP_HEIGHT, MAP_TILES, MAP_WIDTH, TILE_COLOR, WALL_COLOR};
use super::{BelongsTo, Portal, Position, Rect, RequiresItem};

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum TileType {
    Space,
    Wall,
    Floor,
    RequiresKey,
    Portal,
}

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum Place {
    School,
    Library,
}

impl Place {
    pub fn get_name(&self) -> String {
        String::from(match self {
            Place::School => "Okul",
            Place::Library => "Kütüphane",
        })
    }
    pub fn get_year(&self) -> String {
        String::from(match self {
            Place::School => "2023",
            Place::Library => "2022",
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
    pub place: Place,
}

impl Map {
    pub fn xy_to_tile(x: i32, y: i32) -> usize {
        (y * MAP_WIDTH + x) as usize
    }

    fn adjust_tiles(&mut self, ecs: &mut World) {
        let current_place = ecs.fetch::<Place>();
        let impassables = ecs.read_storage::<RequiresItem>();
        let positions = ecs.read_storage::<Position>();
        let belongs = ecs.read_storage::<BelongsTo>();
        let portals = ecs.read_storage::<Portal>();
        for (_portal, pos, bel) in (&portals, &positions, &belongs).join() {
            if bel.domain == *current_place {
                self.tiles[Map::xy_to_tile(pos.x, pos.y)] = TileType::Portal;
            }
        }
        for (_imp, pos, bel) in (&impassables, &positions, &belongs).join() {
            if bel.domain == *current_place {
                self.tiles[Map::xy_to_tile(pos.x, pos.y)] = TileType::RequiresKey;
            }
        }
    }

    /// Takes a room, in the form of a rect, and alters the map accordingly to project the room
    fn apply_room_to_map(&mut self, room: &Rect) {
        let mut indices_to_wall: Vec<(i32, i32)> = Vec::new();
        indices_to_wall.push((room.x1 - 1, room.y1 - 1));
        indices_to_wall.push((room.x1 - 1, room.y2));
        indices_to_wall.push((room.x2, room.y1 - 1));
        indices_to_wall.push((room.x2, room.y2));
        for x in room.x1..=room.x2 - 1 {
            indices_to_wall.push((x, room.y1 - 1));
            indices_to_wall.push((x, room.y2));
            for y in room.y1..=room.y2 - 1 {
                indices_to_wall.push((room.x1 - 1, y));
                indices_to_wall.push((room.x2, y));
                self.tiles[Map::xy_to_tile(x, y)] = TileType::Floor;
            }
        }
        for (x, y) in indices_to_wall {
            self.tiles[Map::xy_to_tile(x, y)] = TileType::Wall;
        }
    }
    pub fn new_map_rooms_and_corridors(ecs: &mut World, place: Place) -> Map {
        let mut map = Map {
            tiles: vec![TileType::Space; MAP_TILES as usize],
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
            place,
        };

        match place {
            Place::School => {
                let school = Rect::new(4, 4, 30, 20);
                map.apply_room_to_map(&school);
            }
            Place::Library => {
                let library = Rect::new(10, 10, 30, 30);
                map.apply_room_to_map(&library);
            }
        }

        map.adjust_tiles(ecs);

        map
    }
}

fn is_wall(map: &Map, x: i32, y: i32) -> bool {
    map.tiles[Map::xy_to_tile(x, y)] == TileType::Wall
}

fn wall_glyph(map: &Map, x: i32, y: i32) -> rltk::FontCharType {
    if x < 1 || x > map.width - 2 || y < 1 || y > map.height - 2 { return 35; }
    let mut mask: u8 = 0;

    if is_wall(map, x, y - 1) { mask += 1; }
    if is_wall(map, x, y + 1) { mask += 2; }
    if is_wall(map, x - 1, y) { mask += 4; }
    if is_wall(map, x + 1, y) { mask += 8; }

    match mask {
        0 => { 9 } // Pillar because we can't see neighbors
        1 => { 186 } // Wall only to the north
        2 => { 186 } // Wall only to the south
        3 => { 186 } // Wall to the north and south
        4 => { 205 } // Wall only to the west
        5 => { 188 } // Wall to the north and west
        6 => { 187 } // Wall to the south and west
        7 => { 185 } // Wall to the north, south and west
        8 => { 205 } // Wall only to the east
        9 => { 200 } // Wall to the north and east
        10 => { 201 } // Wall to the south and east
        11 => { 204 } // Wall to the north, south and east
        12 => { 205 } // Wall to the east and west
        13 => { 202 } // Wall to the east, west, and south
        14 => { 203 } // Wall to the east, west, and north
        15 => { 206 }  // ╬ Wall on all sides
        _ => { 35 } // We missed one?
    }
}

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for tile in map.tiles.iter() {
        let mut glyph = rltk::to_cp437('P');
        let mut fg = RGB::named(RED);
        match tile {
            TileType::Wall => {
                glyph = wall_glyph(&*map, x, y);
                fg = WALL_COLOR;
            }
            TileType::Floor => {
                glyph = rltk::to_cp437('.');
                fg = TILE_COLOR;
            }
            _ => {}
        }
        ctx.set(x, y, fg, BACKGROUND_COLOR, glyph);
        x += 1;
        if x >= MAP_WIDTH {
            x = 0;
            y += 1;
        }
    }
}

