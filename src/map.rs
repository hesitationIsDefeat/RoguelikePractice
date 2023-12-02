use rltk::Rltk;
use serde::{Deserialize, Serialize};
use specs::{Join, World, WorldExt};
use crate::constants::{BACKGROUND_COLOR, CURRENT_DATE, MAP_HEIGHT, MAP_TILES, MAP_WIDTH, PAST_DATE, PLACE_CLASS_NAME, PLACE_HOME_NAME, PLACE_LIB_NAME, PLACE_OTTOMAN_BOTTOM_NAME, PLACE_OTTOMAN_LEFT_NAME, PLACE_OTTOMAN_MAIN_NAME, PLACE_OTTOMAN_RIGHT_NAME, PLACE_OTTOMAN_TOP_NAME, PLACE_SCHOOL_NAME, SPACE_COLOR, TILE_COLOR, WALL_COLOR};
use super::{BelongsTo, Npc, Portal, Position, Rect, RequiresItem};

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum TileType {
    Space,
    Wall,
    Floor,
    RequiresKey,
    Portal,
    NPC,
}

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum Place {
    Home,
    School,
    Class,
    Library,
    Ottoman_Main,
    Ottoman_Left,
    Ottoman_Right,
    Ottoman_Top,
    Ottoman_Bottom,
}

impl Place {
    pub fn get_name(&self) -> String {
        String::from(match self {
            Place::Home => PLACE_HOME_NAME,
            Place::School => PLACE_SCHOOL_NAME,
            Place::Class => PLACE_CLASS_NAME,
            Place::Library => PLACE_LIB_NAME,
            Place::Ottoman_Main => PLACE_OTTOMAN_MAIN_NAME,
            Place::Ottoman_Left => PLACE_OTTOMAN_LEFT_NAME,
            Place::Ottoman_Right => PLACE_OTTOMAN_RIGHT_NAME,
            Place::Ottoman_Top => PLACE_OTTOMAN_TOP_NAME,
            Place::Ottoman_Bottom => PLACE_OTTOMAN_BOTTOM_NAME
        })
    }
    pub fn get_year(&self) -> String {
        String::from(match self {
            Place::School | Place::Home | Place::Class | Place::Library => CURRENT_DATE,
            Place::Ottoman_Main | Place::Ottoman_Left | Place::Ottoman_Right | Place::Ottoman_Top | Place::Ottoman_Bottom => PAST_DATE,
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
        let npcs = ecs.read_storage::<Npc>();
        let entities = ecs.entities();
        for (_portal, pos, bel, ent) in (&portals, &positions, &belongs, &entities).join() {
            if bel.domain == *current_place {
                self.tiles[Map::xy_to_tile(pos.x, pos.y)] = match impassables.contains(ent) {
                    true => TileType::RequiresKey,
                    false => TileType::Portal
                }
            }
        }
        for (_npc, pos, bel) in (&npcs, &positions, &belongs).join() {
            if bel.domain == *current_place {
                self.tiles[Map::xy_to_tile(pos.x, pos.y)] = TileType::NPC;
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
        let created_place: Rect = match place {
            Place::Home => Rect::new(20, 15, 10, 10),
            Place::School => Rect::new(9, 9, 30, 20),
            Place::Class => Rect::new(20, 10, 15, 20),
            Place::Library => Rect::new(15, 15, 25, 13),
            Place::Ottoman_Main => Rect::new(10, 10, 30, 30),
            Place::Ottoman_Left => Rect::new(15, 15, 20, 20),
            Place::Ottoman_Right => Rect::new(15, 15, 20, 20),
            Place::Ottoman_Top => Rect::new(15, 15, 20, 20),
            Place::Ottoman_Bottom => Rect::new(15, 15, 20, 20),
        };
        map.apply_room_to_map(&created_place);

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
        let glyph;
        let fg;
        match tile {
            TileType::Wall => {
                glyph = wall_glyph(&*map, x, y);
                fg = WALL_COLOR;
                ctx.set(x, y, fg, BACKGROUND_COLOR, glyph);
            }
            TileType::Floor => {
                glyph = rltk::to_cp437('.');
                fg = TILE_COLOR;
                ctx.set(x, y, fg, BACKGROUND_COLOR, glyph);
            }
            TileType::Space => {
                ctx.set_bg(x, y, SPACE_COLOR);
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

