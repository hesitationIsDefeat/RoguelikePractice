use std::cmp::{min, max};
use std::fmt::Display;
use rltk::{RGB, Rltk, BLACK};
use serde::{Deserialize, Serialize};
use specs::{Join, World, WorldExt};
use super::{Impassable, Position, Rect};

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 43;
pub const MAP_TILES: i32 = MAP_WIDTH * MAP_HEIGHT;

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum TileType {
    Wall,
    Floor,
    RequiresKey,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
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
    pub fn adjust_tiles(&mut self, ecs: &mut World) {
        let impassables = ecs.read_storage::<Impassable>();
        let positions = ecs.read_storage::<Position>();
        for (_imp, pos) in (&impassables, &positions).join() {
            self.tiles[Map::xy_to_tile(pos.x, pos.y)] = TileType::RequiresKey;
        }
    }

    /// Takes a room, in the form of a rect, and alters the map accordingly to project the room
    fn apply_room_to_map(&mut self, room: &Rect) {
        for x in room.x1 + 1..=room.x2 {
            for y in room.y1 + 1..=room.y2 {
                let index = Map::xy_to_tile(x, y);
                if index > 0 && index < self.width as usize * self.height as usize {
                    self.tiles[index] = TileType::Floor;
                }
            }
        }
    }

    /// Creates a horizontal corridor connecting the provided x values on the provided y coordinate
    fn apply_horizontal_corridor(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let index = Map::xy_to_tile(x, y);
            if index > 0 && index < MAP_TILES as usize {
                self.tiles[index] = TileType::Floor;
            }
        }
    }

    /// Creates a vertical corridor connecting the provided y values on the provided x coordinate
    fn apply_vertical_corridor(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let index = Map::xy_to_tile(x, y);
            if index > 0 && index < MAP_TILES as usize {
                self.tiles[index] = TileType::Floor;
            }
        }
    }

    pub fn new_map_rooms_and_corridors(place: Place) -> (Map, (i32, i32)) {
        let mut map = Map {
            tiles: vec![TileType::Wall; MAP_TILES as usize],
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
            place,
        };

        let school = Rect::new(4, 4, 12, 10);
        let library = Rect::new(4, 30, 12, 10);
        let home = Rect::new(29, 16, 20, 16);
        let gym = Rect::new(62, 4, 12, 12);

        map.apply_room_to_map(&school);
        map.apply_room_to_map(&library);
        map.apply_room_to_map(&home);
        map.apply_room_to_map(&gym);

        map.apply_vertical_corridor(school.center.1, library.center.1, school.center.0);
        map.apply_vertical_corridor(home.center.1, gym.center.1, gym.center.0);
        map.apply_horizontal_corridor(school.center.0, gym.center.0, home.center.1);

        (map, home.center)
    }
}

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for tile in map.tiles.iter() {
        match tile {
            TileType::Wall => {
                ctx.set(x, y, RGB::from_u8(0, 127, 0), RGB::named(BLACK), rltk::to_cp437('#'));
            }
            TileType::Floor => {
                ctx.set(x, y, RGB::from_u8(0, 63, 0), RGB::named(BLACK), rltk::to_cp437('.'));
            }
            _ => {}
        }
        x += 1;
        if x >= MAP_WIDTH {
            x = 0;
            y += 1;
        }
    }
}

