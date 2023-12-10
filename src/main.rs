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
mod npcs;

use player::*;
pub use components::*;
pub use map::*;
use rect::*;
use crate::gamelog::GameLog;
use crate::gui::{ItemMenuResult, MainMenuResult, MainMenuSelection, NpcInteractionResult};
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};
use crate::constants::{HOME_FROM_SCHOOL_COORD, HOME_TO_SCHOOL_PORTAL_COORD, SCHOOL_FROM_HOME_COORD, SCHOOL_FROM_CLASS_COORD, SCHOOL_TO_HOME_PORTAL_COORD, SCHOOL_TO_CLASS_PORTAL_COORD, CLASS_TO_SCHOOL_PORTAL_COORD, CLASS_FROM_SCHOOL_COORD, SCHOOL_TO_LIBRARY_PORTAL_COORD, SCHOOL_FROM_LIBRARY_COORD, LIBRARY_TO_SCHOOL_PORTAL_COORD, LIBRARY_FROM_SCHOOL_COORD, SCHOOL_TO_OTTOMAN_PORTAL_COORD, OTTOMAN_FROM_SCHOOL_COORD, SCHOOL_FROM_OTTOMAN_COORD, OTTOMAN_TO_LEFT_PORTAL_COORD, OTTOMAN_FROM_LEFT_COORD, OTTOMAN_LEFT_FROM_MAIN_COORD, OTTOMAN_LEFT_TO_MAIN_PORTAL_COORD, OTTOMAN_TO_TOP_PORTAL_COORD, OTTOMAN_FROM_TOP_COORD, OTTOMAN_TOP_FROM_MAIN_COORD, OTTOMAN_TOP_TO_MAIN_PORTAL_COORD, OTTOMAN_TO_SCHOOL_PORTAL_COORD, OTTOMAN_TO_RIGHT_PORTAL_COORD, OTTOMAN_FROM_RIGHT_COORD, OTTOMAN_RIGHT_FROM_MAIN_COORD, OTTOMAN_RIGHT_TO_MAIN_PORTAL_COORD, OTTOMAN_TO_BOTTOM_PORTAL_COORD, OTTOMAN_BOTTOM_FROM_MAIN_COORD, OTTOMAN_BOTTOM_TO_MAIN_PORTAL_COORD, OTTOMAN_FROM_BOTTOM_COORD, CLASS_WIDTH, CLASS_X, CLASS_Y, CLASS_HEIGHT, OTTOMAN_MAIN_X, OTTOMAN_MAIN_WIDTH, OTTOMAN_MAIN_Y, OTTOMAN_MAIN_HEIGHT, OTTOMAN_LEFT_WIDTH, OTTOMAN_LEFT_X, OTTOMAN_LEFT_HEIGHT, OTTOMAN_LEFT_Y, OTTOMAN_TOP_X, OTTOMAN_TOP_WIDTH, OTTOMAN_TOP_Y, OTTOMAN_TOP_HEIGHT, OTTOMAN_RIGHT_X, OTTOMAN_RIGHT_WIDTH, OTTOMAN_RIGHT_Y, OTTOMAN_RIGHT_HEIGHT, OTTOMAN_BOTTOM_X, OTTOMAN_BOTTOM_WIDTH, OTTOMAN_BOTTOM_Y, OTTOMAN_BOTTOM_HEIGHT};
use crate::items::ItemName;

#[derive(PartialEq, Clone, Copy)]
pub enum RunState {
    Menu { menu_selection: MainMenuSelection },
    EnterName,
    Game,
    SaveGame,
    UseInventory,
    InteractNpc { index: usize },
    Credits,
    GameOver,
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

        let mut door_reveal_system = systems::DoorRevealSystem {};
        door_reveal_system.run_now(&self.ecs);


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

        {
            let items = self.ecs.read_storage::<Item>();
            let stored = self.ecs.read_storage::<Stored>();
            let current_place = self.ecs.read_resource::<Place>();
            for (item, _) in (&items, &stored).join() {
                if *current_place == Place::School && item.name == ItemName::OttomanKeyMain {
                    run_state = RunState::GameOver;
                }
            }
        }

        match run_state {
            RunState::Menu { .. } | RunState::Credits | RunState::EnterName | RunState::GameOver => {}
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
                            MainMenuSelection::NewGame => run_state = RunState::EnterName,
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
            RunState::EnterName => {
                let done = gui::draw_enter_name(&mut self.ecs, ctx);
                if done {
                    let mut player_name = self.ecs.fetch_mut::<PlayerName>();
                    let mut names = self.ecs.write_storage::<Name>();
                    let players = self.ecs.read_storage::<Player>();
                    for (name, _player) in (&mut names, &players).join() {
                        name.name = String::from(player_name.name.as_str());
                    }
                    run_state = RunState::Game;
                } else {
                    run_state = RunState::EnterName
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
                                if req.key == item {
                                    log.entries.push(format!("Esya kullanildi: {}", item.to_string()));
                                    if self.ecs.read_storage::<PermanentItem>().get(ent).is_none() {
                                        self.ecs.write_storage::<Stored>().remove(ent);
                                    }
                                    if self.ecs.read_storage::<Portal>().get(ent).is_some() {
                                        println!("Changed to portal");
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
                    NpcInteractionResult::NextDialogue { index } => {
                        run_state = RunState::InteractNpc { index };
                    }
                }
            }
            RunState::GameOver => {
                gui::draw_game_over(ctx);
                if let Some(_) = ctx.key {
                    std::process::exit(0);
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
    gs.ecs.register::<DormantPosition>();
    gs.ecs.register::<RevealerInformation>();
    gs.ecs.register::<PlayerName>();

    gs.ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());
    gs.ecs.insert(Place::Home);
    gs.ecs.insert(PlayerName { name: "".to_string() });

    let player_coord = (25, 20);
    let log = GameLog::new(vec!["Oyuna hosgeldin!".to_string()]);
    let player_entity = spawner::build_player(&mut gs, String::from(""), player_coord);

    spawner::build_portal(&mut gs, String::from("Okul Kapisi"), Place::Home, HOME_TO_SCHOOL_PORTAL_COORD, Place::School, SCHOOL_FROM_HOME_COORD);
    spawner::build_portal(&mut gs, String::from("Ev Kapisi"), Place::School, SCHOOL_TO_HOME_PORTAL_COORD, Place::Home, HOME_FROM_SCHOOL_COORD);

    spawner::build_portal(&mut gs, String::from("M2152 Kapisi"), Place::School, SCHOOL_TO_CLASS_PORTAL_COORD, Place::Class, CLASS_FROM_SCHOOL_COORD);
    spawner::build_portal(&mut gs, String::from("Okul Kapisi"), Place::Class, CLASS_TO_SCHOOL_PORTAL_COORD, Place::School, SCHOOL_FROM_CLASS_COORD);

    spawner::build_portal(&mut gs, String::from("Kutuphane Kapisi"), Place::School, SCHOOL_TO_LIBRARY_PORTAL_COORD, Place::Library, LIBRARY_FROM_SCHOOL_COORD);
    spawner::build_portal(&mut gs, String::from("Okul Kapisi"), Place::Library, LIBRARY_TO_SCHOOL_PORTAL_COORD, Place::School, SCHOOL_FROM_LIBRARY_COORD);

    spawner::build_dormant_door(&mut gs, String::from("Gizli Kapi"), Place::School, SCHOOL_TO_OTTOMAN_PORTAL_COORD, Place::OttomanMain, OTTOMAN_FROM_SCHOOL_COORD, ItemName::SecretGateKey,
                                (SCHOOL_TO_OTTOMAN_PORTAL_COORD.0 - 2, SCHOOL_TO_OTTOMAN_PORTAL_COORD.0 + 2), (SCHOOL_TO_OTTOMAN_PORTAL_COORD.1, SCHOOL_TO_OTTOMAN_PORTAL_COORD.1 + 2),
                                ItemName::SecretGateKey, TileType::Wall);

    spawner::build_door(&mut gs, String::from("Kapi 1"), Place::OttomanMain, OTTOMAN_TO_LEFT_PORTAL_COORD, Place::OttomanLeft, OTTOMAN_LEFT_FROM_MAIN_COORD, ItemName::OttomanKey1);
    spawner::build_portal(&mut gs, String::from("Meydan Kapisi"), Place::OttomanLeft, OTTOMAN_LEFT_TO_MAIN_PORTAL_COORD, Place::OttomanMain, OTTOMAN_FROM_LEFT_COORD);

    spawner::build_door(&mut gs, String::from("Kapi 2"), Place::OttomanMain, OTTOMAN_TO_TOP_PORTAL_COORD, Place::OttomanTop, OTTOMAN_TOP_FROM_MAIN_COORD, ItemName::OttomanKey2);
    spawner::build_portal(&mut gs, String::from("Meydan Kapisi"), Place::OttomanTop, OTTOMAN_TOP_TO_MAIN_PORTAL_COORD, Place::OttomanMain, OTTOMAN_FROM_TOP_COORD);

    spawner::build_door(&mut gs, String::from("Kapi 3"), Place::OttomanMain, OTTOMAN_TO_RIGHT_PORTAL_COORD, Place::OttomanRight, OTTOMAN_RIGHT_FROM_MAIN_COORD, ItemName::OttomanKey3);
    spawner::build_portal(&mut gs, String::from("Meydan Kapisi"), Place::OttomanRight, OTTOMAN_RIGHT_TO_MAIN_PORTAL_COORD, Place::OttomanMain, OTTOMAN_FROM_RIGHT_COORD);

    spawner::build_door(&mut gs, String::from("Kapi 4"), Place::OttomanMain, OTTOMAN_TO_BOTTOM_PORTAL_COORD, Place::OttomanBottom, OTTOMAN_BOTTOM_FROM_MAIN_COORD, ItemName::OttomanKey4);
    spawner::build_portal(&mut gs, String::from("Meydan Kapisi"), Place::OttomanBottom, OTTOMAN_BOTTOM_TO_MAIN_PORTAL_COORD, Place::OttomanMain, OTTOMAN_FROM_BOTTOM_COORD);

    spawner::build_dormant_door(&mut gs, String::from("Zaman Kapisi"), Place::OttomanMain, OTTOMAN_TO_SCHOOL_PORTAL_COORD, Place::School, SCHOOL_FROM_OTTOMAN_COORD, ItemName::OttomanKeyMain,
                                (OTTOMAN_MAIN_X, OTTOMAN_MAIN_X + OTTOMAN_MAIN_WIDTH), (OTTOMAN_MAIN_Y, OTTOMAN_MAIN_Y + OTTOMAN_MAIN_HEIGHT), ItemName::OttomanKeyMain, TileType::Floor);


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

    spawner::build_npc(&mut gs, String::from("Taylan Hoca"), Place::Class, (CLASS_X + CLASS_WIDTH - 2, CLASS_Y + 2),
                       vec!(vec!("Merhabalar", "Bugun derste gosterecegim kitaplari kutuphanede unutmusum"),
                            vec!("Rica etsem kitaplari getirebilir misiniz?"),
                            vec!("Super, bir tane daha olmali"),
                            vec!("Cok tesekkurler", "Size bu anahtari hediye ediyorum", "Guney Kampus'te biraz gezerseniz bu anahtarin kullnailacagi bir kapi bulacaksiniz"),
                            vec!("Iyi gunler", "Kapiyi bulmayi unutmayin")),
                       Some(vec!(ItemName::Book, ItemName::Book)),
                       Some(vec!(ItemName::SecretGateKey)),
                       vec!(1, 2),
                       vec!(3),
                       vec!(1, 3));

    spawner::build_npc(&mut gs, String::from("Gizemli Karakter"), Place::OttomanMain, (OTTOMAN_MAIN_X + OTTOMAN_MAIN_WIDTH / 2 + 2, OTTOMAN_MAIN_Y + OTTOMAN_MAIN_HEIGHT / 2),
                       vec!(vec!("Merhabalar gelecekten gelen", "Yuzundeki ifadeden anladigim kadariyla oldukca sasirmis durumdasin ", "O yuzden aciklamama izin ver:"),
                            vec!("Taylan Hoca, tarihi ögrenmek icin bir caba icerisinde olmayanlara iyi bir ders vermek icin onlari gecmise yollar", "Bu sefer de o sanli kisi sensin belli ki"),
                            vec!("Eger kendi zamanina donmek istiyorsan dersini burada, yasayarak ogrenmek zorundasin", "Bu seneki konu Osmanli'da sanat", "Gordugun kapilarin arkasinda, icra ettikleri sanatlari sana anlatacak dort zanaatkar bulunuyor", "Her birini iyice dinle ve isin bitince bana geri don"),
                            vec!("Hadi bakalim", "İlk ders ile basla"),
                            vec!("Demek ilk dersi dinledin", "Afferin", "Simdi ikinci ders"),
                            vec!("Demek ikinci dersi dinledin", "Afferin", "Simdi ucuncu ders"),
                            vec!("Demek ucuncu dersi dinledin", "Afferin", "Simdi dorduncu ders"),
                            vec!("Demek dorduncu dersi dinledin", "Afferin", "Artik kendi zamanina donebilirsin"),
                            vec!("Kendine iyi bak")),
                       Some(vec!(ItemName::OttomanReward1, ItemName::OttomanReward2, ItemName::OttomanReward3, ItemName::OttomanReward4)),
                       Some(vec!(ItemName::OttomanKey1, ItemName::OttomanKey2, ItemName::OttomanKey3, ItemName::OttomanKey4, ItemName::OttomanKeyMain)),
                       vec!(3, 4, 5, 6),
                       vec!(3, 4, 5, 6, 7),
                       vec!(3, 4, 5, 6, 7));

    spawner::build_npc(&mut gs, String::from("Karakter 1"), Place::OttomanLeft, (OTTOMAN_LEFT_X + OTTOMAN_LEFT_WIDTH / 2, OTTOMAN_LEFT_Y + OTTOMAN_LEFT_HEIGHT / 2),
                       vec!(vec!("Al bakalim dostum"),
                            vec!("Iyi gunler")
                       ),
                       None,
                       Some(vec!(ItemName::OttomanReward1)),
                       vec!(),
                       vec!(0),
                       vec!(0));

    spawner::build_npc(&mut gs, String::from("Karakter 2"), Place::OttomanTop, (OTTOMAN_TOP_X + OTTOMAN_TOP_WIDTH / 2, OTTOMAN_TOP_Y + OTTOMAN_TOP_HEIGHT / 2),
                       vec!(vec!("Al bakalim dostum"),
                            vec!("Iyi gunler")
                       ),
                       None,
                       Some(vec!(ItemName::OttomanReward2)),
                       vec!(),
                       vec!(0),
                       vec!(0));

    spawner::build_npc(&mut gs, String::from("Karakter 3"), Place::OttomanRight, (OTTOMAN_RIGHT_X + OTTOMAN_RIGHT_WIDTH / 2, OTTOMAN_RIGHT_Y + OTTOMAN_RIGHT_HEIGHT / 2),
                       vec!(vec!("Al bakalim dostum"),
                            vec!("Iyi gunler")
                       ),
                       None,
                       Some(vec!(ItemName::OttomanReward3)),
                       vec!(),
                       vec!(0),
                       vec!(0));

    spawner::build_npc(&mut gs, String::from("Karakter 4"), Place::OttomanBottom, (OTTOMAN_BOTTOM_X + OTTOMAN_BOTTOM_WIDTH / 2, OTTOMAN_BOTTOM_Y + OTTOMAN_BOTTOM_HEIGHT / 2),
                       vec!(vec!("Al bakalim dostum"),
                            vec!("Iyi gunler")
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
        objectives: vec!("Sinifa git ve Taylan Hoca ile konus".to_string(), "Taylan Hoca'nin kitaplarini bul ve derse getir".to_string(), "Gizli gecidi bul ve arastir".to_string(),
                         "Ilk dersi dinle".to_string(), "Gizemli karakter ile tekrardan konus".to_string(),
                         "Ikinci dersi dinle".to_string(), "Gizemli karakter ile tekrardan konus".to_string(),
                         "Ucuncu dersi dinle".to_string(), "Gizemli karakter ile tekrardan konus".to_string(),
                         "Dorduncu dersi dinle".to_string(), "Gizemli karakter ile tekrardan konus".to_string(),
                         "Kendi zaman dilimine don".to_string()),
        index: 0,
    });

    rltk::main_loop(context, gs)
}