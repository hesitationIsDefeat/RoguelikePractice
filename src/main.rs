use rltk::{GameState, Rltk, Point, RGB, BLACK};
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
use crate::constants::{HOME_FROM_SCHOOL_COORD, HOME_TO_SCHOOL_PORTAL_COORD, SCHOOL_SOUTH_FROM_HOME_COORD, SCHOOL_SOUTH_FROM_CLASS_COORD, SCHOOL_TO_HOME_PORTAL_COORD, SCHOOL_SOUTH_TO_CLASS_PORTAL_COORD, CLASS_TO_SCHOOL_PORTAL_COORD, CLASS_FROM_SCHOOL_COORD, SCHOOL_SOUTH_TO_SCHOOL_NORTH_PORTAL_COORD, SCHOOL_SOUTH_FROM_SCHOOL_NORTH_COORD, SCHOOL_SOUTH_TO_OTTOMAN_PORTAL_COORD, OTTOMAN_FROM_SCHOOL_COORD, SCHOOL_SOUTH_FROM_OTTOMAN_COORD, OTTOMAN_TO_LEFT_PORTAL_COORD, OTTOMAN_FROM_LEFT_COORD, OTTOMAN_LEFT_FROM_MAIN_COORD, OTTOMAN_LEFT_TO_MAIN_PORTAL_COORD, OTTOMAN_TO_TOP_PORTAL_COORD, OTTOMAN_FROM_TOP_COORD, OTTOMAN_TOP_FROM_MAIN_COORD, OTTOMAN_TOP_TO_MAIN_PORTAL_COORD, OTTOMAN_TO_SCHOOL_PORTAL_COORD, OTTOMAN_TO_RIGHT_PORTAL_COORD, OTTOMAN_FROM_RIGHT_COORD, OTTOMAN_RIGHT_FROM_MAIN_COORD, OTTOMAN_RIGHT_TO_MAIN_PORTAL_COORD, OTTOMAN_TO_BOTTOM_PORTAL_COORD, OTTOMAN_BOTTOM_FROM_MAIN_COORD, OTTOMAN_BOTTOM_TO_MAIN_PORTAL_COORD, OTTOMAN_FROM_BOTTOM_COORD, CLASS_WIDTH, CLASS_X, CLASS_Y, CLASS_HEIGHT, OTTOMAN_MAIN_X, OTTOMAN_MAIN_WIDTH, OTTOMAN_MAIN_Y, OTTOMAN_MAIN_HEIGHT, OTTOMAN_LEFT_WIDTH, OTTOMAN_LEFT_X, OTTOMAN_LEFT_HEIGHT, OTTOMAN_LEFT_Y, OTTOMAN_TOP_X, OTTOMAN_TOP_WIDTH, OTTOMAN_TOP_Y, OTTOMAN_TOP_HEIGHT, OTTOMAN_RIGHT_X, OTTOMAN_RIGHT_WIDTH, OTTOMAN_RIGHT_Y, OTTOMAN_RIGHT_HEIGHT, OTTOMAN_BOTTOM_X, OTTOMAN_BOTTOM_WIDTH, OTTOMAN_BOTTOM_Y, OTTOMAN_BOTTOM_HEIGHT, SCHOOL_NORTH_FROM_SCHOOL_SOUTH_COORD, SCHOOL_NORTH_TO_SCHOOL_SOUTH_PORTAL_COORD, SCHOOL_NORTH_TO_LIBRARY_PORTAL_COORD, LIBRARY_FROM_SCHOOL_NORTH_COORD, LIBRARY_TO_SCHOOL_NORTH_PORTAL_COORD, SCHOOL_NORTH_FROM_LIBRARY_COORD};
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

        let mut item_combination_system = systems::ItemCombinationSystem {};
        item_combination_system.run_now(&self.ecs);

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
                if *current_place == Place::SchoolSouth && item.name == ItemName::OttomanKeyMain {
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
        .with_title("Tarih Oyunu")
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

    spawner::build_portal(&mut gs, String::from("Guney Kapisi"), Place::Home, HOME_TO_SCHOOL_PORTAL_COORD, Place::SchoolSouth, SCHOOL_SOUTH_FROM_HOME_COORD);
    spawner::build_portal(&mut gs, String::from("Ev Kapisi"), Place::SchoolSouth, SCHOOL_TO_HOME_PORTAL_COORD, Place::Home, HOME_FROM_SCHOOL_COORD);

    spawner::build_portal(&mut gs, String::from("M2152 Kapisi"), Place::SchoolSouth, SCHOOL_SOUTH_TO_CLASS_PORTAL_COORD, Place::Class, CLASS_FROM_SCHOOL_COORD);
    spawner::build_portal(&mut gs, String::from("Guney Kapisi"), Place::Class, CLASS_TO_SCHOOL_PORTAL_COORD, Place::SchoolSouth, SCHOOL_SOUTH_FROM_CLASS_COORD);

    spawner::build_portal(&mut gs, String::from("Kuzey Kapisi"), Place::SchoolSouth, SCHOOL_SOUTH_TO_SCHOOL_NORTH_PORTAL_COORD, Place::SchoolNorth, SCHOOL_NORTH_FROM_SCHOOL_SOUTH_COORD);
    spawner::build_portal(&mut gs, String::from("Guney Kapisi"), Place::SchoolNorth, SCHOOL_NORTH_TO_SCHOOL_SOUTH_PORTAL_COORD, Place::SchoolSouth, SCHOOL_SOUTH_FROM_SCHOOL_NORTH_COORD);

    spawner::build_portal(&mut gs, String::from("Kutuphane Kapisi"), Place::SchoolNorth, SCHOOL_NORTH_TO_LIBRARY_PORTAL_COORD, Place::Library, LIBRARY_FROM_SCHOOL_NORTH_COORD);
    spawner::build_portal(&mut gs, String::from("Kuzey Kapisi"), Place::Library, LIBRARY_TO_SCHOOL_NORTH_PORTAL_COORD, Place::SchoolNorth, SCHOOL_NORTH_FROM_LIBRARY_COORD);

    spawner::build_dormant_door(&mut gs, String::from("Gizli Kapi"), Place::SchoolSouth, SCHOOL_SOUTH_TO_OTTOMAN_PORTAL_COORD, Place::OttomanMain, OTTOMAN_FROM_SCHOOL_COORD, ItemName::SecretGateKey,
                                (SCHOOL_SOUTH_TO_OTTOMAN_PORTAL_COORD.0 - 2, SCHOOL_SOUTH_TO_OTTOMAN_PORTAL_COORD.0 + 2), (SCHOOL_SOUTH_TO_OTTOMAN_PORTAL_COORD.1, SCHOOL_SOUTH_TO_OTTOMAN_PORTAL_COORD.1 + 2),
                                ItemName::SecretGateKey, TileType::Wall);

    spawner::build_door(&mut gs, String::from("Bati Cikisi"), Place::OttomanMain, OTTOMAN_TO_LEFT_PORTAL_COORD, Place::OttomanLeft, OTTOMAN_LEFT_FROM_MAIN_COORD, ItemName::OttomanKey1);
    spawner::build_portal(&mut gs, String::from("Meydan Kapisi"), Place::OttomanLeft, OTTOMAN_LEFT_TO_MAIN_PORTAL_COORD, Place::OttomanMain, OTTOMAN_FROM_LEFT_COORD);

    spawner::build_door(&mut gs, String::from("Kuzey Cikisi"), Place::OttomanMain, OTTOMAN_TO_TOP_PORTAL_COORD, Place::OttomanTop, OTTOMAN_TOP_FROM_MAIN_COORD, ItemName::OttomanKey2);
    spawner::build_portal(&mut gs, String::from("Meydan Kapisi"), Place::OttomanTop, OTTOMAN_TOP_TO_MAIN_PORTAL_COORD, Place::OttomanMain, OTTOMAN_FROM_TOP_COORD);

    spawner::build_door(&mut gs, String::from("Dogu Cikisi"), Place::OttomanMain, OTTOMAN_TO_RIGHT_PORTAL_COORD, Place::OttomanRight, OTTOMAN_RIGHT_FROM_MAIN_COORD, ItemName::OttomanKey3);
    spawner::build_portal(&mut gs, String::from("Meydan Kapisi"), Place::OttomanRight, OTTOMAN_RIGHT_TO_MAIN_PORTAL_COORD, Place::OttomanMain, OTTOMAN_FROM_RIGHT_COORD);

    spawner::build_dormant_door(&mut gs, String::from("Zaman Kapisi"), Place::OttomanMain, OTTOMAN_TO_SCHOOL_PORTAL_COORD, Place::SchoolSouth, SCHOOL_SOUTH_FROM_OTTOMAN_COORD, ItemName::OttomanKeyMain,
                                (OTTOMAN_MAIN_X, OTTOMAN_MAIN_X + OTTOMAN_MAIN_WIDTH), (OTTOMAN_MAIN_Y, OTTOMAN_MAIN_Y + OTTOMAN_MAIN_HEIGHT), ItemName::OttomanKeyMain, TileType::Floor);


    spawner::build_active_item(&mut gs, ItemName::Book, Place::Library, (19, 19), true);
    spawner::build_active_item(&mut gs, ItemName::Book, Place::Library, (20, 20), true);
    spawner::build_dormant_item(&mut gs, ItemName::SecretGateKey);

    spawner::build_dormant_item(&mut gs, ItemName::OttomanKey1);
    spawner::build_dormant_item(&mut gs, ItemName::OttomanRewardPoem);
    spawner::build_dormant_item(&mut gs, ItemName::OttomanRewardBookCover);
    spawner::build_dormant_item(&mut gs, ItemName::OttomanRewardGlue);
    spawner::build_dormant_item(&mut gs, ItemName::OttomanCombinedRewardPoemBook);


    spawner::build_dormant_item(&mut gs, ItemName::OttomanKey2);
    spawner::build_dormant_item(&mut gs, ItemName::OttomanRewardMosquePart1);
    spawner::build_dormant_item(&mut gs, ItemName::OttomanRewardMosquePart2);
    spawner::build_dormant_item(&mut gs, ItemName::OttomanCombinedRewardMosqueModel);

    spawner::build_dormant_item(&mut gs, ItemName::OttomanKey3);
    spawner::build_dormant_item(&mut gs, ItemName::OttomanRewardNotePaper);
    spawner::build_dormant_item(&mut gs, ItemName::OttomanRewardCanvas);
    spawner::build_dormant_item(&mut gs, ItemName::OttomanRewardClay);
    spawner::build_dormant_item(&mut gs, ItemName::OttomanCombinedRewardWeirdCollage);

    spawner::build_dormant_item(&mut gs, ItemName::OttomanKeyMain);

    spawner::build_npc_human(&mut gs, "Taylan Hoca", Place::Class, (CLASS_X + CLASS_WIDTH - 2, CLASS_Y + 2),
                             vec!(vec!("Merhabalar.", "Bugun derste gosterecegim kitaplari kutuphanede unutmusum."),
                                  vec!("Rica etsem kitaplari getirebilir misiniz?"),
                                  vec!("Super, iki tane daha olmali."),
                                  vec!("Cok tesekkurler!", "Sonuncuyu da alabilir miyim?"),
                                  vec!("Demek sizde degil...", "O zaman size bu anahtari veriyorum.", "Guney Kampus'te biraz gezerseniz bu anahtarin kullnailacagi bir kapi bulacaksiniz ve kitabim de o kapinin ardinda."),
                                  vec!("Iyi gunler, kitabi bulmayi unutmayin.")),
                             Some(vec!(ItemName::Book, ItemName::Book)),
                             Some(vec!(ItemName::SecretGateKey)),
                             vec!(1, 2),
                             vec!(4),
                             vec!(1, 4));

    spawner::build_npc_human(&mut gs, "Gizemli Karakter", Place::OttomanMain, (OTTOMAN_MAIN_X + OTTOMAN_MAIN_WIDTH / 2 + 2, OTTOMAN_MAIN_Y + OTTOMAN_MAIN_HEIGHT / 2),
                             vec!(vec!("Merhabalar gelecekten gelen!", "Yuzundeki ifadeden anladigim kadariyla oldukca sasirmis durumdasin.", "O yuzden aciklamama izin ver:"),
                                  vec!("Taylan Hoca, tarihi ogrenmek icin bir caba icerisinde olmayanlara iyi bir ders vermek icin onlari gecmise yollar.", "Bu sefer de o sansli kisi sensin belli ki."),
                                  vec!("Eger kendi zamanina donmek istiyorsan dersini burada, yasayarak ogrenmek zorundasin!", "Bu seneki konu Osmanli'nin son dönemlerinde sanat alininda yasadigi degisimler.", "Gordugun kapilarin arkasinda, konu hakkinda seni ilgilendirecek insanlar bulunuyor.", "Her birini iyice dinle ve isin bitince bana geri don."),
                                  vec!("Hadi bakalim!", "Ilk ders ile basla."),
                                  vec!("Demek ilk dersi dinledin.", "Afferin!"),
                                  vec!("Simdi sirada ikinci ders var."),
                                  vec!("Dersi dinle ve bana geri don!"),
                                  vec!("Demek ikinci dersi dinledin.", "Afferin!"),
                                  vec!("Simdi ucuncu ders var."),
                                  vec!("Dersi dinle ve bana geri don!"),
                                  vec!("Demek ucuncu dersi dinledin.", "Afferin!", "Artik kendi zamanina donebilirsin!"),
                                  vec!("Kendine iyi bak.")),
                             Some(vec!(ItemName::OttomanCombinedRewardPoemBook, ItemName::OttomanCombinedRewardMosqueModel, ItemName::OttomanCombinedRewardWeirdCollage)),
                             Some(vec!(ItemName::OttomanKey1, ItemName::OttomanKey2, ItemName::OttomanKey3, ItemName::OttomanKeyMain)),
                             vec!(3, 6, 9),
                             vec!(2, 5, 8, 10),
                             vec!(2, 5, 8, 10));


    spawner::build_npc_human(&mut gs, "Osman Bey", Place::OttomanLeft, (OTTOMAN_LEFT_X + OTTOMAN_LEFT_WIDTH / 2, OTTOMAN_LEFT_Y + OTTOMAN_LEFT_HEIGHT / 2 - 1),
                             vec!(vec!("Medeniyetler, birbirinden ayri dusunulemezler. Bu sebepten mutevellit, medeniyetler arasindaki etkilesimin bir sekilde saglanmasi gerekir. Bu etkilesim de tercume sayesinde paylasilir. Tercumeye gerekli degeri ilk verenler Tanzimat dusunurleriydi. Sinasi’nin 1869’da Fransiz siirlerini cevirmesinden sonra 1880’e kadar sadece 25 siir tercume edildi. Neyse ki 1880’den sonra bati ulkelerine ait siirlerin tercume edilip dilimize aktarilmasi bir hayli hiz kazandi."),
                                  vec!("1859-1901 yillari arasinda Bati edebiyatindan en az 802 siir dilimize tercume edildi ve bu sayede bizim siirlerimizde de farkli formlar ve konular gorulmeye baslandi. Bu farkli konulardan belki de en onemlisi, tekrardan bireylerin gunluk problemlerinin siirlere aktarilmasiydi. O donemin onemli sair-mutercimlerine Muallim Naci, Ahmed Rasim, Recâizâde Mahmud Ekrem, Şinasi ve Nâbizâde Nâzim gibi isimleri; daha cok mutercimlikleri ile bilinen isimlere Halil Edib, Ahmed Refik ve Ali Riza Seyfi gibi isimleri ornek verebilirim."),
                                  vec!("Seninle ilginc bir bilgi paylasmak istiyorum: Su yenililesme karsiti diye anilan Muallim Naci var ya; belirttigim yillar arasinda 62 manzume ile en fazla tercume yapan sairimiz odur.  Yapilan siir cevirilerinin buyuk bir cogunlugu 650 siir ile Fransiz edebiyatina aitti ve devaminda Ingiliz ve Alman edebiyati geliyordu. Yapilan tercumelerin gazetelerde yayinlanmasi ve gunluk hayatin bir parcasi haline gelmesi, Bati siirine olan ilgiyi daha da arttirmistir. Bu ilgiden kaynakli olarak daha once de belirttigim gibi yeni siir formlari ortaya cikmaya basladi."),
                                  vec!("Bu durum hem bir zenginlige ve duzensizlige kapi acmis oldu. Neden oldugunu anlayamasam da fabller Bati siir dunyasi icerisinde onemli bir yer kaplamakta idi ve tercumeler sayesinde bizim edebiyatimizda da benimsenmis oldu. Yapilan tercumelerin hangi eserlere ait olduklarina bakarken, mutercimlerin kisisel tercihlerinin de bu secimlerde onemli roller oynadigini unutmamak gerekir. Mesela Recaizade Ekrem daha cok La Fontaine’nin fabblerini tercume etmeyi tercih etmistir. Naci ise tercihini Florian’in eserlerinden yana kullanmistir."),
                                  vec!("Umarim anlattiklarim faydali olmustur.")
                             ),
                             None,
                             Some(vec!(ItemName::OttomanRewardPoem)),
                             vec!(),
                             vec!(3),
                             vec!(3));
    spawner::build_npc_human(&mut gs, "Zeliha Hanim", Place::OttomanLeft, (OTTOMAN_LEFT_X + OTTOMAN_LEFT_WIDTH / 2 - 1, OTTOMAN_LEFT_Y + OTTOMAN_LEFT_HEIGHT / 2 + 1),
                             vec!(vec!("Tanzimar sonrasi edebiyatinin demirbaslari sayilabilecek kisilerinden Sinasi, Namik Kemal, Recaizade Mahmut Ekrem gibi isimler duzenli bir egitimden gecmediler. Bunun yerine ozel dersler almislardir. Hepsi Fransiz lisanina hakimdiler. Tanzimat’tan sonra baslayan okullasmanin urunu olan okullarda Arapca ogretilmemekteydi. Arapca ve Farsca eserler yerine bati edebiyati eserleri daha cazip gorulmekte, bu eserler okutulmakta, dolayisiyla edebiyati anlayisi da buna gore sekillenmekteydi."),
                                  vec!("Namik Kemal, yayimladigi makalesinde (Lisân-i Osmanînin Edebiyati Hakkinda Bazi Mulâhazâti Şamildir) bati rhe- torique’inin prensiplerinden olan hakikat ve tabiata uygunlugu ozellikle vurgulamistir. Namik Kemal’in Avrupa’ya gitmeden Batili eserlerin etkisinde kaldigi buradan anlasilabilir. Bunun devaminda Suleyman Pasa “Mebani’l Insa” adinda, Ali Cemaleddin “Arûz-t Turkî” adinda, Mihalicli Mustafa Efendi ise “Zubdetu’l-Beyan” adinda eserler yayimlarlar ancak hicbiri yeni edebiyat icin bir oncu gorevi goremez."),
                                  vec!("Derken 1879’da Recaizade Mahmut Ekrem “Talim-i Edebiyat”’i tas baski halinde cikarmistir. Eserinde edebiyatin degistiginden, bu yeni edebiyatin aciklanmasi gerektiginden bahsetmektedir ve bu ihtiyaci karsilamaya calismistir. Namik Kemal, yazilanlarin konusma dilinde olmasi gerektigini hem sozlerin hem de hayallerin milli olmasi gerektigini, dolayisiyla edebiyati yaklasmakta oldugu Batili fikirlerden de korumak gerektigini vurgulamistir. Milliyet ve vatan meselelerini on plana cikararak yeni bir anlayis ortaya koymustur."),
                                  vec!("Dinledigin icin tesekkur ederim.")
                             ),
                             None,
                             Some(vec!(ItemName::OttomanRewardBookCover)),
                             vec!(),
                             vec!(2),
                             vec!(2));

    spawner::build_npc_human(&mut gs, "Zeyneb Hanim", Place::OttomanLeft, (OTTOMAN_LEFT_X + OTTOMAN_LEFT_WIDTH / 2 - 1, OTTOMAN_LEFT_Y + OTTOMAN_LEFT_HEIGHT / 2 + 3),
                             vec!(vec!("Devletimizin Bati karsisinda surekli guc kaybettigi donemlerde, aydinlarimiz edebiyati halki yasadigi bunalimdan biraz da olsa kurtarabilmek adina kullanmislardir. Bu sebepten dolayi son donem edebiyatimizda sosyal, siyasal, kulturel problemlere atiflara oldukca rastlanir. Son donem yazarlarimizdan Munif Paşa, Ahmet Mithat Efendi, Sadullah Paşa gibi isimler donemin fikir hareketlerinden etkilenerek bu fikirleri eserlerine yansitmislardir. Batidan bizim topraklarimiza ulasan rasyonalist akim da bu fikir akimlarindan biridir."),
                                  vec!("Tercume-i Telemak cevirisi, direkt olarak bati dusuncesine dayanan bir kitaptir mesela.Namik Kemal icin cok degerli olarak gorulmesi ve Sinasi tarafindan ikinci baskisinin cevrilmesi, son donem edebiyatcilarimizin rasyonalizm ve realizmin etkisi altinda kaldigini gosterir. Baska bir ornek ise, Mumif Pasa’nin Mecmua-i Funun’da arka arkaya yayinladigi makaleler icerisinde batiya ait calismalardan faydalanilmasi ve bazi calismalarin dogrudan cevirilmesidir."),
                                  vec!("Belirtmek istedigim baska bir durum ise, Fransiz aydinlanmasinda buyuk rol oynayan Montesqieu, Voltaire, Jean Jack Rousseau ve Diderot’un akla dayali ansiklopedik bilgiyi edebiyatlari ile birlestirme endiselerini; 1839-1896 yillari arasinda eser ureten edebiyatcilarimizin cogunda da gorebilmekteyiz."),
                                  vec!("Iyi gunler dilerim.")
                             ),
                             None,
                             Some(vec!(ItemName::OttomanRewardGlue)),
                             vec!(),
                             vec!(2),
                             vec!(2));


    spawner::build_npc_human(&mut gs, "Ali Bey", Place::OttomanTop, (OTTOMAN_TOP_X + OTTOMAN_TOP_WIDTH / 2 + 1, OTTOMAN_TOP_Y + OTTOMAN_TOP_HEIGHT / 2),
                             vec!(vec!("18. yuzyilin baslarindan itibaren Osmanli’nin Bati’nin gerisinde kalmasi, Bati’ya olan ilgiyi arttirmis ve Bati’yi ornek alma istegini kacinilmaz kilmisti. Istanbul’da bulunan Nuruosmaniye Camisi, Bati etkisinin mimari alanda gorulmeye baslandigi ilk orneklerdendir. Ne gibi bir farki vardi diyecek olursan ; avlusu klasik forma sahip diger camilere nazaran oval bir bicimdeydi. Anadolu camilerine deginecek olursak ise; batidan esinlenilen yuvarlak kemerler bulundurmalari, agirlik kulelerine her zaman olmasa da yeni bicimler verilmesi Bati’dan etkilendigimizin baska bir gostergesidir."),
                                  vec!("Camilere yerlestirilen kapilarin uzerlerindeki motifler icin de zaman icinde batili tarzda motifler kullanildigini goruyoruz. Baska bir ornek ise 1895 tarihinde tamamlanan Soke Haci Ziya Bey Camii’sinin cephesi itibariyle adete Avrupai bir kosku andirmasidir. Turbeler acisindan bakacak olursak eger, 18. ve 19. Yuzyillar arasinda Anadolu turbe mimarisi onemini giderek yitirmis, sanat degerini kaybetmis ve cagin modasina uyum saglayamamistir. Medreseler, hanlar ve cesmeler, gerek Bati gerek baska fikir akimlari tarafindan etkilenmeyerek, ozlerinden pek kopmamislardir."),
                                  vec!("Saraylar, biraz once bahsetmis oldugumuz yapilara nazaran, kendi kulturumuze ait unsurlari bulundurmanin yaninda, daha fazla Avrupai fikirler ile yapilmis unsurlari bunyesinde bulundurmaktadir. Gunluk hayata uyum saglamayi basaran han, carsi gibi yapilar varliklarini surdurmeye devam etmislerdir. Diger geleneksel yapilar ise toplumdaki ve kulturdeki degisimlere uyum saglayamadiklarindan mutevellit onemlerini yitirerek sayica azalmaya baslamislardir."),
                                  vec!("Bunun onemli sebeplerinden biri Bati’ya duyulan ozenti sonucunda bakis acilarinin degismesi ve yeni ihtiyaclarin ortaya cikmasidir. Onemini yitirmeyen en onemli yapilar camiilerdir ancak gorunus acisindan daha once de bahsettigim gibi degisimler gecirmisler, batili bicimde gorunum kazanmislardir. Bu degisimlerin sebebi olan Batili gelisme donemleri ise belirli bir sira ile degil, daha cok karma olarak uygulanmistir. Buna ek olarak, camiilerde batili tarzda motiflere, nakislara ve kabartmalara yer verilmistir ve bu Turk resim sanati da degistiren bir degisim olmustur."),
                                  vec!("Yabanci sanatcilar Istanbul’a gelerek eser vermisler, kendi bildiklerini yaymaya calismislardir. Bircoguna gore bu sanat alanindaki batililasma icin oncu niteliginde bir olaydir. Bana soracak olursan eger zaten Turk Sanati o donem Avrupa’dan cok etkilenmis bir noktadaydi, onlar ise oncu degil de degisimi hizlandiran etkenler olmuslardir."),
                                  vec!("Haydi selametle.")
                             ),
                             None,
                             Some(vec!(ItemName::OttomanRewardMosquePart1)),
                             vec!(),
                             vec!(4),
                             vec!(4));

    spawner::build_npc_human(&mut gs, "Emine Hanim", Place::OttomanTop, (OTTOMAN_TOP_X + OTTOMAN_TOP_WIDTH / 2 - 1, OTTOMAN_TOP_Y + OTTOMAN_TOP_HEIGHT / 2),
                             vec!(vec!("17. yuzyilin sonlarinda Lale Devri adinda bir uslup dogmustur. Kullanilan sade desenler, yerini daha karmasik desenlere birakmistir ve Barok uslubu kullanilmaya baslanmistir. Bunun devaminda Rokoko uslubuna gecilmis, daha hafif desenler kullanilmaya devam edilmistir. Barok ve Rokoko uslubunun devaminda ise Ampir uslubuna gecilmistir. Ampir uslubun en onemli orneklerinden biri ise Nusretiye Camii’dir ve 1826’da yapilmistir. Osmanli’nin son donemlerinde Bati ile, ozellikle de Fransa ile, etkilesimlerin artmasi sanati bircok alanda etkilemistir ve mimari de kuskusuz bu alanlardan biridir."),
                                  vec!("18. Yuzyilda baslayan ve 19. Yuzyil boyunca devam eden bu etkilenme donemine “Batililasma Hareketi” denmistir. Yurt disindan gelen yabanci mimarlarin Osmanli’da yapi faaliyetlerinin bir parcasi olmalari, batililasmayi hizlandirmistir. Avrupa’da ulusculuk dusuncesinin onem kazanmasi sonucunda gecmise duyulan hayranlik ve ozlem artmis, sonucunda Neo-Klasik uslup dogmustur. Bu uslup cercevesinde cephe duzenlemelerinde gecmise dayanan teknikler kullanilmistir."),
                                  vec!("Biraz once bahsetmis oldugum sebeplerden mutevellit, Osmanli mimarisinde ozellikle cephe yapilarinda da benzer degisimler gorulmustur. Yabanci mimarlar ozellikle anitsal yapilar uzerine calismislardir. 1890 yilinda tamamlanan Istanbul Sirkeci Gari, Osmanli ve Alman Mimari’lerinin kaynastigi bir yapidir ve gecis doneminin bir parcasi olarak kabul edilir. 19. Yuzyilin sonu ve 20. Yuzyilin baslarinda gelisen Turkculuk anlayisi isiginda mimarlik sanati ulusal olma yolunda ilerlemistir."),
                                  vec!("Bunun sebebi olarak batili yontemlerden uzaklasilmistir. Turkculuk anlayisi, 2. Mesrutiyet’in ilani ile daha da guclenmis ve 1930’lu yillara kadar varligini surdurmustur. Bu anlayis sayesinde batili yontemlerden farkli, milli ve ulusal bir mimari uslup icerisinde eserler verilmistir. Donemlerin degismesi ile birlikte toplumun yeni ihtiyaclari ortaya cikmistir ve bu ihtiyaclari karsilamak adina yeni yapilar yapilmaya baslanmistir. Bu yapilara bankalar, hastaneler, muzeler ornek gosterilebilir."),
                                  vec!("Cephe duzenlemeleri Ronesans \
                                  yapilarina benzeyecek sekilde on cephenin gosterisli olacak sekilde, diger cepheler daha sade kalacak sekilde ayarlanmistir. Dis gorunus konusunda degisimler yasanmasina karsin ic duzenlemeler konusunda pek bir degisim olmamistir. Bunun sebebi donemin getirmis oldugu bir bakis acisi olan, fonksiyonel bir amac gutmeden, mimariyi daha cok bir sanat olarak gormektir."),
                                  vec!("Kalin saglicagla.")
                             ),
                             None,
                             Some(vec!(ItemName::OttomanRewardMosquePart2)),
                             vec!(),
                             vec!(4),
                             vec!(4));


    spawner::build_npc_human(&mut gs, "Ahmet Bey", Place::OttomanRight, (OTTOMAN_RIGHT_X + OTTOMAN_RIGHT_WIDTH / 2 + 2, OTTOMAN_RIGHT_Y + OTTOMAN_RIGHT_HEIGHT / 2),
                             vec!(vec!("Klasik bati muzigine olan hayranligin artmasi sonucu, Osmanli muzigi bir baskalasim gecirmistir. Bunun arkasindaki en onemli etkenlerden biri de yabanci muzik ustatlarinin Osmanli’da verdigi konserlerdir. Buna ornek olarak Franz Lizst’in Istanbul’da verdigi konserler verilebilir. Biraz enteresan gelebilir ancak o donemlerde Avrupa’da da Turk muziginden alintilar gormek mumkundur. Mozart ve Beethoven’in kullanmis olduklari ritimler ve melodiler mehter muziginin ozelliklerini bunyesinde barindirmaktadirlar."),
                                  vec!("Padisahlara eserler hediye edilmesi de sikca gozlenen bir durumdur ve Avrupa muzigi ile kurulan bu yakinlik, bati muzigine olan ilgiyi arttirmis ve iki muzik kulturune de katkilarda bulunmustur."),
                                  vec!("Hayirli gunler.")
                             ),
                             None,
                             Some(vec!(ItemName::OttomanRewardNotePaper)),
                             vec!(),
                             vec!(1),
                             vec!(1));
    spawner::build_npc_human(&mut gs, "Nefise Hanim", Place::OttomanRight, (OTTOMAN_RIGHT_X + OTTOMAN_RIGHT_WIDTH / 2, OTTOMAN_RIGHT_Y + OTTOMAN_RIGHT_HEIGHT / 2 - 1),
                             vec!(vec!("19. yuzyil sonlarina dogru Osmanli’da batili sayilabilecek ilk tuval resimleri ortaya cikmistir. Bilindigi uzere Osmanli padisahlari siyasi cokusun onune gecebilmek adina batili teknolojiyi benimsemislerdir. Dogal olarak bu benimseyis bircok alanda degisimlere yol acmistir. Simdi enteresan ve arasinda baglanti olmasi beklenmeyen iki durumdan bahsedecegim. Askeri gucu arttirabilmek adina batili egitimi oncu kabul eden askeri okullar acildi. Acilan bu askeri okullarda da resim dersi mevcuttu."),
                                  vec!("Bu ders ilk baslarda teknik bir ders olarak verilmekteydi. Bunun sonucunda da batiyi ornek alan Turk resimleri ortaya cikmaya basladi. Zaman icinde teknik olan bu ders sanatsal bir deger de kazanmaya basladi. Batidaki tekniklerin ogrenimi daha ileri bir seviyeye tasimak amaciyla, bu teknikleri yerinde ogrenmeleri icin Avrupa’ya ogrenci gruplari gonderilmistir. Ilk grup 1829’da gonderilmis, devami 1834, 1835, 1846 seklinde devam etmistir. Gidilen Avrupa ulkeleri ise Ingiltere, Fransa ve Avusturya olmustur."),
                                  vec!("Ileriki senelerde, sanat egitimini Avrupa’da almalari icin de ogrenci gonderimi devam etmistir. Bu sekilde egitim goren ve Turk resmine buyuk katkilari bulunan ressamlara Halil Pasa, Sami Yetik, Ibrahim Calli verilebilir. 1883 yilinda Sanayi-i Nefise Mekteb-i Alisi adinda bir guzel sanatlar akademisi kurulmus ve resim egitimini akademik bir disiplin ile yurutecek bir birim olusturulmustur. Asker ressamlar arasinda ogretmen-ogrenci iliskileri kurulmustur. Her kusak kendilerinden once gelenlerin biraktiklari mirasa eklemeler yaparak eser vermeye devam etmislerdir."),
                                  vec!("Iyi gunler dilerim.")
                             ),
                             None,
                             Some(vec!(ItemName::OttomanRewardCanvas)),
                             vec!(),
                             vec!(2),
                             vec!(2));
    spawner::build_npc_human(&mut gs, "Almila Hanim", Place::OttomanRight, (OTTOMAN_RIGHT_X + OTTOMAN_RIGHT_WIDTH / 2 + 1, OTTOMAN_RIGHT_Y + OTTOMAN_RIGHT_HEIGHT / 2 + 1),
                             vec!(vec!("Heykel uretimi Osmanli topraklarina 19.yuzyilin sonlarina dogru varmistir. Bunun en buyuk sebebi, toplumun geleneksel degerlerinden uzaklasmak istememesidir. 1871 yilinda, Sultan Abdulaziz’in Avrupa kentlerine yapmis oldugu gezi donusunde kendi heykelini yaptirmistir ve bu olay heykel sanatinin kesin olarak Osmanli sanatinin bir parcasi haline gelmesini saglamistir. 1883 yilinda Sanayi-i Nefise Mekteb-i Alisi’nin acilmasinin ardindan Osmanli’da heykeltiras yetistirecek bir okul ilk kez acilmis olur."),
                                  vec!("Simdi izninle Yervant Osgan Efendi’yi yad etmek istiyorum. Kendisi Avrupa’da heykel uzerine ogrenim goren ilk Osmanli genci olmasi ile bilinir. Sanayi-i Nefise’de 32 yil boyunca ogretim uyesi olarak calisir, bircok sanatcinin yetismesinde rol oynar. Ayni zamanda kendisi de gercekci yorumlarini kattigi bir suru heykel uretmistir."),
                                  vec!("Gorusmek uzere.")
                             ),
                             None,
                             Some(vec!(ItemName::OttomanRewardClay)),
                             vec!(),
                             vec!(1),
                             vec!(1));


    spawner::build_npc_human_one_liner(&mut gs, "Efe", Place::SchoolSouth, (17, 14), "Merhaba.");
    spawner::build_npc_human_one_liner(&mut gs, "Aysila", Place::SchoolSouth, (32, 23), "Selamlar!");

    spawner::build_npc_dog(&mut gs, "Karbeyaz", Place::SchoolNorth, (31, 22), RGB::from_u8(10, 10, 10), "HAV HAV");
    spawner::build_npc_dog(&mut gs, "Naci", Place::SchoolNorth, (17, 15), RGB::from_u8(230, 230, 132), "Hav");
    spawner::build_npc_dog(&mut gs, "Pasa", Place::SchoolNorth, (16, 22), RGB::from_u8(30, 30, 30), "Hav Hav");
    spawner::build_npc_cat(&mut gs, "Adolf", Place::SchoolNorth, (31, 25), RGB::from_u8(229, 229, 201), "Mrrnav");
    spawner::build_npc_cat(&mut gs, "Deli", Place::SchoolNorth, (34, 24), RGB::from_u8(228, 228, 49), "Miav");


    let map = Map::new_map_rooms_and_corridors(&mut gs.ecs, Place::Home);
    gs.ecs.insert(map);
    gs.ecs.insert(log);
    gs.ecs.insert(player_entity);
    gs.ecs.insert(Point::new(player_coord.0, player_coord.1));
    gs.ecs.insert(TargetedPosition { x: -1, y: -1 });
    gs.ecs.insert(RunState::Menu { menu_selection: MainMenuSelection::NewGame });
    gs.ecs.insert(Objective {
        objectives: vec!("Sinifa git ve Taylan Hoca ile konus".to_string(), "Taylan Hoca'nin kitaplarini bul ve derse getir".to_string(), "Gizli gecidi bul ve arastir".to_string(),
                         "Ilk dersi dinle".to_string(), "Ikınci dersi dinle".to_string(), "Ucuncu dersi dinle".to_string(), "Gizemli karakter ile tekrardan konus".to_string(),
                         "Dorduncu dersi dinle".to_string(), "Besinci dersi dinle".to_string(), "Gizemli karakter ile tekrardan konus".to_string(),
                         "Altinci dersi dinle".to_string(), "Yedinci dersi dinle".to_string(), "Sekizinci dersi dinle".to_string(), "Gizemli karakter ile tekrardan konus".to_string(),
                         "Kendi zaman dilimine don".to_string()),
        index: 0,
    });

    rltk::main_loop(context, gs)
}