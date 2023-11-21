use rltk::{GameState, Rltk, RGB, BLACK, PURPLE2, Point};
use specs::prelude::*;

mod player;
mod components;
mod map;
mod rect;
mod gui;
mod gamelog;
mod spawner;
mod inventory_system;

use player::*;
pub use components::*;
pub use map::*;
use rect::*;
use crate::gamelog::GameLog;
use crate::gui::ItemMenuResult;

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum RunState {
    Menu,
    Game,
    ShowInventory,
    UseInventory,
}

pub struct State {
    ecs: World,
}

// Rendering
fn render_entities(gs: &mut State, ctx: &mut Rltk) {
    let positions = gs.ecs.read_storage::<Position>();
    let renderables = gs.ecs.read_storage::<Renderable>();
    for (pos, rend) in (&positions, &renderables).join() {
        ctx.set(pos.x, pos.y, rend.fg, rend.bg, rend.glyph)
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut item_collection_system = inventory_system::ItemCollectionSystem {};
        item_collection_system.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        draw_map(&self.ecs, ctx);

        {
            let positions = self.ecs.read_storage::<Position>();
            let renderables = self.ecs.read_storage::<Renderable>();
            for (pos, rend) in (&positions, &renderables).join() {
                ctx.set(pos.x, pos.y, rend.fg, rend.bg, rend.glyph)
            }
        }

        gui::draw_ui(&self.ecs, ctx);

        let mut run_state;
        {
            let state_reader = self.ecs.fetch::<RunState>();
            run_state = *state_reader;
        }
        match run_state {
            RunState::Menu => {}
            RunState::Game => {
                self.run_systems();
                self.ecs.maintain();
                run_state = player_input(self, ctx);
            }
            RunState::ShowInventory => {
                if gui::show_inventory(self, ctx) == ItemMenuResult::Cancel {
                    run_state = RunState::Game;
                }
            }
            RunState::UseInventory => {
                let (result, item) = gui::use_item(self, ctx);
                match result {
                    ItemMenuResult::NoResponse => {}
                    ItemMenuResult::Cancel => {
                        run_state = RunState::Game;
                    }
                    ItemMenuResult::Selected => {
                        let item = item.unwrap();
                        let barriers_to_remove: Vec<Entity> = Vec::new();

                        let mut positions = self.ecs.write_storage::<Position>();
                        let target_pos = self.ecs.fetch::<TargetedPosition>();
                        let mut requires_item = self.ecs.write_storage::<RequiresItem>();
                        let names = self.ecs.read_storage::<Name>();
                        let mut log = self.ecs.write_resource::<GameLog>();
                        let mut map = self.ecs.write_resource::<Map>();
                        let entities = self.ecs.entities();

                        for (ent, pos, req) in (&entities, &positions, &requires_item).join() {
                            if pos.x == target_pos.x && pos.y == target_pos.y {
                                if req.key == item {
                                    log.entries.push(format!("Esya kullanildi: {}", names.get(item).unwrap().name));
                                    if self.ecs.read_storage::<PermanentItem>().get(item).is_none() {
                                        println!("Removed item");
                                        self.ecs.write_storage::<Stored>().remove(item);
                                    }
                                    if self.ecs.read_storage::<Door>().get(ent).is_some() {
                                        map.tiles[Map::xy_to_tile(pos.x, pos.y)] = TileType::Floor;
                                    }
                                    run_state = RunState::Game;
                                } else {
                                    log.entries.push(String::from("Yanlis esya"));
                                }
                            }
                        }
                        for barrier in barriers_to_remove {
                            requires_item.remove(barrier);
                        }
                    }
                }
            }
        }

        {
            let mut state_writer = self.ecs.write_resource::<RunState>();
            *state_writer = run_state;
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Position>();
    gs.ecs.register::<TargetedPosition>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<Item>();
    gs.ecs.register::<Stored>();
    gs.ecs.register::<Impassable>();
    gs.ecs.register::<Door>();
    gs.ecs.register::<RequiresItem>();
    gs.ecs.register::<PermanentItem>();

    let (mut map, player_coord) = Map::new_map_rooms_and_corridors();
    let log = gamelog::GameLog::new(vec!["Oyuna hosgeldin!".to_string()]);
    let player_entity = spawner::build_player(&mut gs,
                                              String::from("Onat"),
                                              player_coord,
                                              rltk::to_cp437('@'),
                                              RGB::named(PURPLE2),
                                              RGB::named(BLACK));
    let lib_key = spawner::build_key(&mut gs, String::from("Kütüphane Anahtari"), (41, 26));
    let home_hey = spawner::build_key(&mut gs, String::from("Ev Anahtari"), (42, 26));
    spawner::build_door(&mut gs, String::from("Kütüphane Kapisi"), (42, 27), 'D', lib_key);
    map.adjust_tiles(&mut gs.ecs);
    gs.ecs.insert(map);
    gs.ecs.insert(log);
    gs.ecs.insert(player_entity);
    gs.ecs.insert(Point::new(player_coord.0, player_coord.1));
    gs.ecs.insert(TargetedPosition { x: -1, y: -1 });
    gs.ecs.insert(RunState::Game);

    rltk::main_loop(context, gs)
}