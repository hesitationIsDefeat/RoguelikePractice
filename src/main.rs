use rltk::{GameState, Rltk, Point};
use specs::prelude::*;

mod player;
mod components;
mod map;
mod rect;
mod gui;
mod gamelog;
mod spawner;
mod inventory_system;
mod save_load_system;
mod constants;
mod item_adjustment_system;

use player::*;
pub use components::*;
pub use map::*;
use rect::*;
use crate::gamelog::GameLog;
use crate::gui::{ItemMenuResult, MainMenuResult, MainMenuSelection, NpcInteractionResult};
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};

#[derive(PartialEq, Clone, Copy)]
pub enum RunState {
    Menu { menu_selection: MainMenuSelection },
    Game,
    SaveGame,
    UseInventory,
    InteractNpc { index: usize },
    Credits,
}

pub struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut item_collection_system = inventory_system::ItemCollectionSystem {};
        item_collection_system.run_now(&self.ecs);

        let mut item_adjustment_system = item_adjustment_system::ItemAdjustmentSystem {};
        item_adjustment_system.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        let mut run_state;
        {
            let state_reader = self.ecs.fetch::<RunState>();
            run_state = *state_reader;
        }
        ctx.cls();

        match run_state {
            RunState::Menu { .. } => {}
            RunState::Credits => {}
            _ => {
                {
                    let current_place = *self.ecs.fetch::<Place>();
                    let map_place = self.ecs.fetch::<Map>().place;
                    if map_place != current_place {
                        let new_map = Map::new_map_rooms_and_corridors(&mut self.ecs, current_place);
                        self.ecs.insert(new_map);
                    }
                }

                draw_map(&self.ecs, ctx);

                {
                    if let Some(player_pos) = self.ecs.write_storage::<Position>().get_mut(*self.ecs.fetch::<Entity>()) {
                        let player_point = *self.ecs.fetch::<Point>();
                        player_pos.x = player_point.x;
                        player_pos.y = player_point.y;
                    }
                }

                {
                    let positions = self.ecs.read_storage::<Position>();
                    let renderables = self.ecs.read_storage::<Renderable>();
                    let belongs = self.ecs.read_storage::<BelongsTo>();
                    let current_place = self.ecs.fetch::<Place>();
                    let mut data = (&positions, &renderables, &belongs).join().collect::<Vec<_>>();
                    data.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));
                    for (pos, rend, bel) in data {
                        if bel.domain == *current_place {
                            ctx.set(pos.x, pos.y, rend.fg, rend.bg, rend.glyph);
                        }
                    }
                }
                gui::draw(&self.ecs, ctx);
            }
        }

        match run_state {
            RunState::Menu { .. } => {
                let result = gui::draw_main_menu(self, ctx);
                match result {
                    MainMenuResult::NoSelection { selected } => run_state = RunState::Menu { menu_selection: selected },
                    MainMenuResult::Selected { selected } => {
                        match selected {
                            MainMenuSelection::NewGame => run_state = RunState::Game,
                            MainMenuSelection::LoadGame => {
                                save_load_system::load_game(&mut self.ecs);
                                run_state = RunState::Game;
                            }
                            MainMenuSelection::QuitGame => std::process::exit(0),
                            MainMenuSelection::Credits => {
                                run_state = RunState::Credits;
                            }
                        }
                    }
                }
            }
            RunState::Credits => {
                gui::draw_credits(ctx);
                if let Some(_) = ctx.key {
                    run_state = RunState::Menu { menu_selection: MainMenuSelection::Credits };
                }
            }
            RunState::Game => {
                self.run_systems();
                self.ecs.maintain();
                run_state = player_input(self, ctx);
            }
            RunState::SaveGame => {
                save_load_system::save_game(&mut self.ecs);
                run_state = RunState::Menu { menu_selection: MainMenuSelection::LoadGame }
            }
            RunState::UseInventory => {
                let (result, item) = gui::draw_use_item(self, ctx);
                match result {
                    ItemMenuResult::NoResponse => {}
                    ItemMenuResult::Cancel => {
                        run_state = RunState::Game;
                    }
                    ItemMenuResult::Selected => {
                        let item = item.unwrap();
                        let mut barriers_to_remove: Vec<Entity> = Vec::new();

                        let positions = self.ecs.read_storage::<Position>();
                        let target_pos = self.ecs.fetch::<TargetedPosition>();
                        let mut requires_item = self.ecs.write_storage::<RequiresItem>();
                        let names = self.ecs.read_storage::<Name>();
                        let mut log = self.ecs.write_resource::<GameLog>();
                        let mut map = self.ecs.write_resource::<Map>();
                        let entities = self.ecs.entities();

                        for (ent, pos, req) in (&entities, &positions, &requires_item).join() {
                            if pos.x == target_pos.x && pos.y == target_pos.y {
                                if req.item == item {
                                    log.entries.push(format!("Esya kullanildi: {}", names.get(item).unwrap().name));
                                    if self.ecs.read_storage::<PermanentItem>().get(item).is_none() {
                                        self.ecs.write_storage::<Stored>().remove(item);
                                    }
                                    if self.ecs.read_storage::<Portal>().get(ent).is_some() {
                                        map.tiles[Map::xy_to_tile(pos.x, pos.y)] = TileType::Portal;
                                    }
                                    barriers_to_remove.push(ent);
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
            RunState::InteractNpc { index } => {
                let result = gui::draw_npc_interaction(&mut self.ecs, ctx, index);
                match result {
                    NpcInteractionResult::NoResponse => {}
                    NpcInteractionResult::Done => {
                        run_state = RunState::Game;
                    }
                    NpcInteractionResult::NextDialogue => {
                        run_state = RunState::InteractNpc { index: index + 1 };
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
    gs.ecs.register::<RequiresItem>();
    gs.ecs.register::<ContainsItem>();
    gs.ecs.register::<PermanentItem>();
    gs.ecs.register::<SimpleMarker<SerializeMe>>();
    gs.ecs.register::<SerializationHelper>();
    gs.ecs.register::<Portal>();
    gs.ecs.register::<BelongsTo>();
    gs.ecs.register::<Npc>();
    gs.ecs.register::<Objective>();
    gs.ecs.register::<Interaction>();

    gs.ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());
    gs.ecs.insert(Place::School);

    let player_coord = (10, 10);
    let log = GameLog::new(vec!["Oyuna hosgeldin!".to_string()]);
    let player_entity = spawner::build_player(&mut gs, String::from("Onat"), player_coord);
    let lib_key = spawner::build_active_item(&mut gs, String::from("Kütüphane Anahtari"), Place::School, (11, 11), true);
    let ancient_key = spawner::build_active_item(&mut gs, String::from("Eski Anahtar"), Place::Library, (12, 11), true);
    let sword = spawner::build_dormant_item(&mut gs, String::from("Kilic"));
    spawner::build_door(&mut gs, String::from("Kütüphane Gizli Oda Kapisi"), Place::School, (12, 12), Place::Library, (20, 20), lib_key);
    spawner::build_portal(&mut gs, String::from("Kütüphane Kapisi"), Place::Library, (14, 14), Place::School, (15, 15));
    spawner::build_npc(&mut gs, String::from("Taylan Hoca"), Place::School, (20, 20),
                       vec!(vec!("Merhaba Onat", "Bana eski anahtarı getir"), vec!("Anahtar için tesekkurler", "Sana bu kilici hediye ediyorum"), vec!("Iyi gunler")),
                       Some(ancient_key), Some(sword), vec!(0), vec!(1), vec!(0));

    let map = Map::new_map_rooms_and_corridors(&mut gs.ecs, Place::School);
    gs.ecs.insert(map);
    gs.ecs.insert(log);
    gs.ecs.insert(player_entity);
    gs.ecs.insert(Point::new(player_coord.0, player_coord.1));
    gs.ecs.insert(TargetedPosition { x: -1, y: -1 });
    gs.ecs.insert(RunState::Game);
    gs.ecs.insert(RunState::Menu { menu_selection: MainMenuSelection::NewGame });
    gs.ecs.insert(Objective { objectives: vec!("Taylan Hoca ile konus".to_string(), "Eski anahtari bul".to_string()), index: 0 });

    rltk::main_loop(context, gs)
}