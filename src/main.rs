use rltk::{GameState, Rltk, Point};
use specs::prelude::*;

mod player;
mod components;
mod map;
mod rect;
mod gui;
mod gamelog;
mod spawner;
mod save_load_system;
mod constants;
mod items;
mod systems;

use player::*;
pub use components::*;
pub use map::*;
use rect::*;
use crate::gamelog::GameLog;
use crate::gui::{ItemMenuResult, MainMenuResult, MainMenuSelection, NpcInteractionResult};
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};
use crate::constants::ITEM_LIB_KEY_NAME;
use crate::items::ItemName;

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
        let mut item_collection_system = systems::ItemCollectionSystem {};
        item_collection_system.run_now(&self.ecs);

        let mut item_adjustment_system = systems::ItemAdjustmentSystem {};
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
                        let items = self.ecs.read_storage::<Item>();
                        let mut requires_item = self.ecs.write_storage::<RequiresItem>();
                        let mut log = self.ecs.write_resource::<GameLog>();
                        let mut map = self.ecs.write_resource::<Map>();
                        let entities = self.ecs.entities();

                        for (pos, req, ent) in (&positions, &requires_item, &entities).join() {
                            if pos.x == target_pos.x && pos.y == target_pos.y {
                                if req.item == item {
                                    log.entries.push(format!("Esya kullanildi: {}", item.to_string()));
                                    if self.ecs.read_storage::<PermanentItem>().get(ent).is_none() {
                                        self.ecs.write_storage::<Stored>().remove(ent);
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
        .with_tile_dimensions(12, 12)
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
    gs.ecs.register::<RequiresItems>();
    gs.ecs.register::<ContainsItems>();
    gs.ecs.register::<PermanentItem>();
    gs.ecs.register::<SimpleMarker<SerializeMe>>();
    gs.ecs.register::<SerializationHelper>();
    gs.ecs.register::<Portal>();
    gs.ecs.register::<BelongsTo>();
    gs.ecs.register::<Npc>();
    gs.ecs.register::<Objective>();
    gs.ecs.register::<Interaction>();

    gs.ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());
    gs.ecs.insert(Place::Home);

    let player_coord = (25, 20);
    let log = GameLog::new(vec!["Oyuna hosgeldin!".to_string()]);
    let player_entity = spawner::build_player(&mut gs, String::from("Onat"), player_coord);

    spawner::build_portal(&mut gs, String::from("Okul Kapisi"), Place::Home, (19, 20), Place::School, (38, 20));
    spawner::build_portal(&mut gs, String::from("Ev Kapisi"), Place::School, (39, 20), Place::Home, (20, 20));
    spawner::build_portal(&mut gs, String::from("M2152 Kapisi"), Place::School, (24, 29), Place::Class, (20, 13));
    spawner::build_portal(&mut gs, String::from("Okul Kapisi"), Place::Class, (19, 13), Place::School, (24, 28));
    spawner::build_portal(&mut gs, String::from("Kutuphane Kapisi"), Place::School, (8, 20), Place::Library, (39, 20));
    spawner::build_portal(&mut gs, String::from("Okul Kapisi"), Place::Library, (40, 20), Place::School, (9, 20));
    spawner::build_portal(&mut gs, String::from("Gizli Kapi"), Place::School, (24, 8), Place::Ottoman_Main, (20, 20));


    spawner::build_active_item(&mut gs, ItemName::OldKey1, Place::Library, (19, 19), true);
    spawner::build_active_item(&mut gs, ItemName::OldKey2, Place::Library, (20, 20), true);
    spawner::build_dormant_item(&mut gs, ItemName::Sword);

    spawner::build_npc(&mut gs, String::from("Taylan Hoca"), Place::Class, (31, 12),
                       vec!(vec!("Merhaba Onat", "Bana eski anahtar 1 getir"), vec!("Harika", "Simdi bana eski anahtar 2 getir"), vec!("Anahtar için tesekkurler", "Sana bu kilici hediye ediyorum"), vec!("Iyi gunler")),
                       Some(vec!(ItemName::OldKey1, ItemName::OldKey2)), Some(vec!(ItemName::Sword)), vec!(0, 1), vec!(2), vec!(0, 1));

    let map = Map::new_map_rooms_and_corridors(&mut gs.ecs, Place::Home);
    gs.ecs.insert(map);
    gs.ecs.insert(log);
    gs.ecs.insert(player_entity);
    gs.ecs.insert(Point::new(player_coord.0, player_coord.1));
    gs.ecs.insert(TargetedPosition { x: -1, y: -1 });
    gs.ecs.insert(RunState::Game);
    gs.ecs.insert(RunState::Menu { menu_selection: MainMenuSelection::NewGame });
    gs.ecs.insert(Objective {
        objectives: vec!("Taylan Hoca ile konus".to_string(), "Eski anahtar 1 bul ve Taylan Hoca'ya getir".to_string(),
                         "Eski anahtar 2 bul ve Taylan Hoca'ya getir".to_string()),
        index: 0,
    });

    rltk::main_loop(context, gs)
}