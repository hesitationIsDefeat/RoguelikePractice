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

    spawner::build_door(&mut gs, String::from("Gizli Kapi"), Place::School, (24, 8), Place::OttomanMain, (25, 24), ItemName::SecretGateKey);

    spawner::build_door(&mut gs, String::from("Kapi 1"), Place::OttomanMain, (9, 24), Place::OttomanLeft, (34, 24), ItemName::OttomanKey1);
    spawner::build_portal(&mut gs, String::from("Meydan Kapisi"), Place::OttomanLeft, (35, 24), Place::OttomanMain, (10, 24));

    spawner::build_door(&mut gs, String::from("Kapi 2"), Place::OttomanMain, (24, 9), Place::OttomanTop, (24, 34), ItemName::OttomanKey2);
    spawner::build_portal(&mut gs, String::from("Meydan Kapisi"), Place::OttomanTop, (24, 35), Place::OttomanMain, (24, 10));

    spawner::build_door(&mut gs, String::from("Kapi 3"), Place::OttomanMain, (40, 24), Place::OttomanRight, (15, 24), ItemName::OttomanKey3);
    spawner::build_portal(&mut gs, String::from("Meydan Kapisi"), Place::OttomanRight, (14, 24), Place::OttomanMain, (39, 24));

    spawner::build_door(&mut gs, String::from("Kapi 4"), Place::OttomanMain, (24, 40), Place::OttomanBottom, (24, 15), ItemName::OttomanKey4);
    spawner::build_portal(&mut gs, String::from("Meydan Kapisi"), Place::OttomanBottom, (24, 14), Place::OttomanMain, (24, 39));

    spawner::build_door(&mut gs, String::from("Zaman Kapisi"), Place::OttomanMain, (26, 24), Place::School, (20, 20), ItemName::OttomanKeyMain);


    spawner::build_active_item(&mut gs, ItemName::Book, Place::Library, (19, 19), true);
    spawner::build_active_item(&mut gs, ItemName::Book, Place::Library, (20, 20), true);
    spawner::build_dormant_item(&mut gs, ItemName::SecretGateKey);

    spawner::build_dormant_item(&mut gs, ItemName::OttomanKey1);
    spawner::build_dormant_item(&mut gs, ItemName::OttomanReward1);

    spawner::build_dormant_item(&mut gs, ItemName::OttomanKey2);
    spawner::build_dormant_item(&mut gs, ItemName::OttomanReward2);

    spawner::build_dormant_item(&mut gs, ItemName::OttomanKey3);
    spawner::build_dormant_item(&mut gs, ItemName::OttomanReward3);

    spawner::build_dormant_item(&mut gs, ItemName::OttomanKey4);
    spawner::build_dormant_item(&mut gs, ItemName::OttomanReward4);

    spawner::build_dormant_item(&mut gs, ItemName::OttomanKeyMain);

    spawner::build_npc(&mut gs, String::from("Taylan Hoca"), Place::Class, (31, 12),
                       vec!(vec!("Merhaba Onat", "Bugun derste gösterecegim kitaplari kutuphanede unutmusum", "Rica etsem getirebilir misiniz?"),
                            vec!("Super, bir tane daha olmali"),
                            vec!("Cok tesekkurler", "Sana bu anahtari hediye ediyorum"),
                            vec!("Iyi gunler")),
                       Some(vec!(ItemName::Book, ItemName::Book)),
                       Some(vec!(ItemName::SecretGateKey)),
                       vec!(0, 1),
                       vec!(2),
                       vec!(0, 2));

    spawner::build_npc(&mut gs, String::from("Gizemli Karakter"), Place::OttomanMain, (28, 24),
                       vec!(vec!("Merhabalar gelecekten gelen", "Yuzundeki ifadeden anladigim kadar,yla oldukca sasirmis durumdasın ", "O yuzden aciklamama izin ver:"),
                            vec!("Taylan Hoca, tarihi ögrenmek icin bir caba icerisinde olmayanlara iyi bir ders vermek icin onlari gecmise yollar", "Bu sefer de o sanli kisi sensin belli ki"),
                            vec!("Eger kendi zamanina donmek istiyorsan dersini burada, yasayarak ogrenmek zorundasin", "Bu seneki konu Osmanli'da sanat", "Gordugun kapilarin arkasinda, icra ettikleri sanatlari sana anlatacak dort zanaatkar bulunuyor", "Her birini iyice dinle ve isin bitince bana geri don"),
                            vec!("Iyi dersler"),
                            vec!("Demek ilk dersi dinledin", "Afferin", "Şimdi ikinci ders"),
                            vec!("Demek ikinci dersi dinledin", "Afferin", "Şimdi ucuncu ders"),
                            vec!("Demek ucuncu dersi dinledin", "Afferin", "Şimdi dorduncu ders"),
                            vec!("Demek dorduncu dersi dinledin", "Afferin", "Artik kendi zamanina donebilirsin"),
                            vec!("Kendine iyi bak")),
                       Some(vec!(ItemName::OttomanReward1, ItemName::OttomanReward2, ItemName::OttomanReward3, ItemName::OttomanReward4)),
                       Some(vec!(ItemName::OttomanKey1, ItemName::OttomanKey2, ItemName::OttomanKey3, ItemName::OttomanKey4, ItemName::OttomanKeyMain)),
                       vec!(3, 4, 5, 6),
                       vec!(3, 4, 5, 6, 7),
                       vec!(3, 4, 5, 6, 7));

    spawner::build_npc(&mut gs, String::from("Karakter 1"), Place::OttomanLeft, (22, 20),
                       vec!(vec!("Al bakalim dostum"),
                            vec!("İyi gunler")
                       ),
                       None,
                       Some(vec!(ItemName::OttomanReward1)),
                       vec!(),
                       vec!(0),
                       vec!(0));

    spawner::build_npc(&mut gs, String::from("Karakter 2"), Place::OttomanTop, (22, 20),
                       vec!(vec!("Al bakalim dostum"),
                            vec!("İyi gunler")
                       ),
                       None,
                       Some(vec!(ItemName::OttomanReward2)),
                       vec!(),
                       vec!(0),
                       vec!(0));

    spawner::build_npc(&mut gs, String::from("Karakter 3"), Place::OttomanRight, (22, 20),
                       vec!(vec!("Al bakalim dostum"),
                            vec!("İyi gunler")
                       ),
                       None,
                       Some(vec!(ItemName::OttomanReward3)),
                       vec!(),
                       vec!(0),
                       vec!(0));

    spawner::build_npc(&mut gs, String::from("Karakter 4"), Place::OttomanBottom, (22, 20),
                       vec!(vec!("Al bakalim dostum"),
                            vec!("İyi gunler")
                       ),
                       None,
                       Some(vec!(ItemName::OttomanReward4)),
                       vec!(),
                       vec!(0),
                       vec!(0));

    let map = Map::new_map_rooms_and_corridors(&mut gs.ecs, Place::Home);
    gs.ecs.insert(map);
    gs.ecs.insert(log);
    gs.ecs.insert(player_entity);
    gs.ecs.insert(Point::new(player_coord.0, player_coord.1));
    gs.ecs.insert(TargetedPosition { x: -1, y: -1 });
    gs.ecs.insert(RunState::Game);
    gs.ecs.insert(RunState::Menu { menu_selection: MainMenuSelection::NewGame });
    gs.ecs.insert(Objective {
        objectives: vec!("Sınıfa git ve Taylan Hoca ile konus".to_string(), "Taylan Hoca'ya kitapları ulastir".to_string(), "Gizli gecidi bul ve arastir".to_string(),
                         "Ilk dersi dinle".to_string(), "Hediye 1'i gotur".to_string(),
                         "Ikinci dersi dinle".to_string(), "Hediye 2'i gotur".to_string(),
                         "Ucuncu dersi dinle".to_string(), "Hediye 3'i gotur".to_string(),
                         "Dorduncu dersi dinle".to_string(), "Hediye 4'i gotur".to_string(),
                         "Kendi zamanina don".to_string()),
        index: 0,
    });

    rltk::main_loop(context, gs)
}