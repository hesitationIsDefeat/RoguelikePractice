use rltk::{RGB, Rltk, Point, WHITE, BLACK, VirtualKeyCode, RED, GREY0, GREY3, GREY, YELLOW};
use specs::prelude::*;
use crate::{Map, Name, Place, Portal, Position, RequiresItem, RunState, save_load_system, State, Stored};
use crate::constants::{BACKGROUND_COLOR, CONSOLE_BACKGROUND_COLOR, CONSOLE_BORDER_COLOR, CURSOR_COLOR, INVENTORY_BACKGROUND_COLOR, INVENTORY_BANNER, INVENTORY_BANNER_X, INVENTORY_BORDER_COLOR, INVENTORY_HEIGHT, INVENTORY_ITEMS_X, INVENTORY_WIDTH, INVENTORY_X, INVENTORY_Y, LOAD_GAME_STR, LOAD_GAME_Y, MAP_HEIGHT, MENU_SELECTED_COLOR, MENU_UNSELECTED_COLOR, NEW_GAME_STR, NEW_GAME_Y, QUIT_GAME_STR, QUIT_GAME_Y, SCREEN_HEIGHT, SCREEN_WIDTH, TITLE_STR, TITLE_Y};
use crate::gamelog::GameLog;

#[derive(PartialEq, Copy, Clone)]
pub enum ItemMenuResult { Cancel, NoResponse, Selected }

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuSelection {
    NewGame,
    LoadGame,
    QuitGame,
}

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuResult {
    NoSelection { selected: MainMenuSelection },
    Selected { selected: MainMenuSelection },
}


pub fn main_menu(gs: &mut State, ctx: &mut Rltk) -> MainMenuResult {
    let save_exists = save_load_system::save_exists();
    let state = gs.ecs.fetch::<RunState>();
    ctx.print_color_centered(TITLE_Y, RGB::named(RED), RGB::named(BLACK), TITLE_STR);

    if let RunState::Menu { menu_selection: selected } = *state {
        if selected == MainMenuSelection::NewGame {
            ctx.print_color_centered(NEW_GAME_Y, MENU_SELECTED_COLOR, BACKGROUND_COLOR, NEW_GAME_STR);
        } else {
            ctx.print_color_centered(NEW_GAME_Y, MENU_UNSELECTED_COLOR, BACKGROUND_COLOR, NEW_GAME_STR);
        }
        if save_exists {
            if selected == MainMenuSelection::LoadGame {
                ctx.print_color_centered(LOAD_GAME_Y, MENU_SELECTED_COLOR, BACKGROUND_COLOR, LOAD_GAME_STR);
            } else {
                ctx.print_color_centered(LOAD_GAME_Y, MENU_UNSELECTED_COLOR, BACKGROUND_COLOR, LOAD_GAME_STR);
            }
        }
        if selected == MainMenuSelection::QuitGame {
            ctx.print_color_centered(QUIT_GAME_Y, MENU_SELECTED_COLOR, BACKGROUND_COLOR, QUIT_GAME_STR);
        } else {
            ctx.print_color_centered(QUIT_GAME_Y, MENU_UNSELECTED_COLOR, BACKGROUND_COLOR, QUIT_GAME_STR);
        }

        return match ctx.key {
            None => MainMenuResult::NoSelection { selected },
            Some(key) => {
                match key {
                    VirtualKeyCode::Up => {
                        let new_selection;
                        match selected {
                            MainMenuSelection::NewGame => new_selection = MainMenuSelection::QuitGame,
                            MainMenuSelection::LoadGame => new_selection = MainMenuSelection::NewGame,
                            MainMenuSelection::QuitGame => new_selection = match save_exists {
                                true => MainMenuSelection::LoadGame,
                                false => MainMenuSelection::NewGame
                            }
                        }
                        MainMenuResult::NoSelection { selected: new_selection }
                    }
                    VirtualKeyCode::Down => {
                        let new_selection;
                        match selected {
                            MainMenuSelection::NewGame => new_selection = match save_exists {
                                true => MainMenuSelection::LoadGame,
                                false => MainMenuSelection::QuitGame
                            },
                            MainMenuSelection::LoadGame => new_selection = MainMenuSelection::QuitGame,
                            MainMenuSelection::QuitGame => new_selection = MainMenuSelection::NewGame,
                        }
                        MainMenuResult::NoSelection { selected: new_selection }
                    }
                    VirtualKeyCode::Return => MainMenuResult::Selected { selected },
                    _ => MainMenuResult::NoSelection { selected }
                }
            }
        };
    }
    MainMenuResult::NoSelection { selected: MainMenuSelection::NewGame }
}

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    // CONSOLE
    ctx.draw_box(0, MAP_HEIGHT, SCREEN_WIDTH - 1, SCREEN_HEIGHT - MAP_HEIGHT - 1,
                 CONSOLE_BORDER_COLOR, CONSOLE_BACKGROUND_COLOR);
    let log = ecs.fetch::<GameLog>();
    let mut y = MAP_HEIGHT + 1;
    for s in log.entries.iter().rev() {
        if y < SCREEN_HEIGHT - 1 {
            ctx.print(2, y, s);
        }
        y += 1;
    }
    // MOUSE
    let cursor_pos = ctx.mouse_pos();
    ctx.set_bg(cursor_pos.0, cursor_pos.1, CURSOR_COLOR);
    // INVENTORY
    ctx.draw_box(INVENTORY_X, INVENTORY_Y, INVENTORY_WIDTH,
                 INVENTORY_HEIGHT, INVENTORY_BORDER_COLOR, INVENTORY_BACKGROUND_COLOR);
    //
    let current_place = ecs.fetch::<Place>();
    let place_name_year_str = format!("{}, {}", current_place.get_name(), current_place.get_year());
    ctx.print_color(2, MAP_HEIGHT - 2, RGB::named(RED), RGB::named(BLACK), place_name_year_str);
    draw_tooltips(ecs, ctx);
}

fn draw_tooltips(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();
    let names = ecs.read_storage::<Name>();
    let positions = ecs.read_storage::<Position>();
    let entities = ecs.entities();

    let mouse_pos = ctx.mouse_pos();
    if mouse_pos.0 >= map.width || mouse_pos.1 >= map.height { return; }
    let mut tooltip: Vec<String> = Vec::new();
    for (name, position, ent) in (&names, &positions, &entities).join() {
        if position.x == mouse_pos.0 && position.y == mouse_pos.1 {
            let mut name = name.name.to_string();
            if ecs.read_storage::<Portal>().get(ent).is_some() {
                name += match ecs.read_storage::<RequiresItem>().get(ent).is_some() {
                    true => " (Kapali)",
                    false => " (Acik)"
                };
            }
            tooltip.push(name);
        }
    }

    if !tooltip.is_empty() {
        let mut width: i32 = 0;
        for s in tooltip.iter() {
            if width < s.len() as i32 { width = s.len() as i32; }
        }
        width += 3;

        if mouse_pos.0 > 40 {
            let arrow_pos = Point::new(mouse_pos.0 - 2, mouse_pos.1);
            let left_x = mouse_pos.0 - width;
            let mut y = mouse_pos.1;
            for s in tooltip.iter() {
                ctx.print_color(left_x, y, RGB::named(RED), RGB::named(GREY), s);
                let padding = (width - s.len() as i32) - 1;
                for i in 0..padding {
                    ctx.print_color(arrow_pos.x - i, y, RGB::named(RED), RGB::named(GREY), &" ".to_string());
                }
                y += 1;
            }
            ctx.print_color(arrow_pos.x, arrow_pos.y, RGB::named(RED), RGB::named(GREY), &"->".to_string());
        } else {
            let arrow_pos = Point::new(mouse_pos.0 + 1, mouse_pos.1);
            let left_x = mouse_pos.0 + 3;
            let mut y = mouse_pos.1;
            for s in tooltip.iter() {
                ctx.print_color(left_x + 1, y, RGB::named(RED), RGB::named(GREY), s);
                let padding = (width - s.len() as i32) - 1;
                for i in 0..padding {
                    ctx.print_color(arrow_pos.x + 1 + i, y, RGB::named(RED), RGB::named(GREY), &" ".to_string());
                }
                y += 1;
            }
            ctx.print_color(arrow_pos.x, arrow_pos.y, RGB::named(RED), RGB::named(GREY), &"<-".to_string());
        }
    }
}

pub fn draw_inventory(ecs: &World, ctx: &mut Rltk) {
    let names = ecs.read_storage::<Name>();
    let backpack = ecs.read_storage::<Stored>();

    let mut y = 25;
    ctx.print_color(INVENTORY_BANNER_X, y - 2, RGB::named(YELLOW), BACKGROUND_COLOR, INVENTORY_BANNER);

    for (_pack, name) in (&backpack, &names).join() {
        ctx.print(INVENTORY_ITEMS_X, y, &name.name.to_string());
        y += 2;
    }
}

pub fn use_item(gs: &mut State, ctx: &mut Rltk) -> (ItemMenuResult, Option<Entity>) {
    let names = gs.ecs.read_storage::<Name>();
    let backpack = gs.ecs.read_storage::<Stored>();
    let entities = gs.ecs.entities();

    let inventory = (&backpack, &names).join();
    let count = inventory.count();

    let mut y = (25 - (count / 2)) as i32;
    ctx.draw_box(15, y - 2, 31, (count + 3) as i32, RGB::named(WHITE), RGB::named(BLACK));
    ctx.print_color(18, y - 2, RGB::named(YELLOW), RGB::named(BLACK), "Esyalar");
    ctx.print_color(18, y + count as i32 + 1, RGB::named(YELLOW), RGB::named(BLACK), "Cikmak icin: I");

    let mut j = 0;
    let mut usable: Vec<Entity> = Vec::new();
    for (item_ent, _pack, name) in (&entities, &backpack, &names).join() {
        ctx.set(17, y, RGB::named(WHITE), RGB::named(BLACK), rltk::to_cp437('('));
        ctx.set(18, y, RGB::named(YELLOW), RGB::named(BLACK), 97 + j as rltk::FontCharType);
        ctx.set(19, y, RGB::named(WHITE), RGB::named(BLACK), rltk::to_cp437(')'));

        ctx.print(21, y, &name.name.to_string());
        usable.push(item_ent);
        y += 1;
        j += 1;
    }

    if let Some(key) = ctx.key {
        let key_num = rltk::letter_to_option(key);
        return if key_num > -1 && key_num < count as i32 {
            (ItemMenuResult::Selected, Some(usable[key_num as usize]))
        } else {
            (ItemMenuResult::Cancel, None)
        };
    }
    (ItemMenuResult::NoResponse, None)
}



