use rltk::{RGB, Rltk, Point, WHITE, BLACK, VirtualKeyCode, RED, GREY, YELLOW};
use specs::prelude::*;
use crate::{BelongsTo, Interaction, Map, Name, Npc, Place, Portal, Position, Renderable, RequiresItem, RunState, save_load_system, State, Stored, TargetedPosition};
use crate::constants::{BACKGROUND_COLOR, CONSOLE_BACKGROUND_COLOR, CONSOLE_BORDER_COLOR, CURSOR_COLOR, INVENTORY_BACKGROUND_COLOR, INVENTORY_BANNER, INVENTORY_BANNER_COLOR, INVENTORY_BANNER_X, INVENTORY_BORDER_COLOR, INVENTORY_HEIGHT, INVENTORY_ITEMS_X, INVENTORY_STRING_COLOR, INVENTORY_WIDTH, INVENTORY_X, INVENTORY_Y, LOAD_GAME_STR, LOAD_GAME_Y, MAP_HEIGHT, MENU_SELECTED_COLOR, MENU_UNSELECTED_COLOR, NEW_GAME_STR, NEW_GAME_Y, NPC_INTERACTION_DIALOGUE_DELTA, NPC_INTERACTION_DIALOGUE_HEADING_X, NPC_INTERACTION_DIALOGUE_HEADING_Y, NPC_INTERACTION_DIALOGUE_X, NPC_INTERACTION_DIALOGUE_Y, NPC_INTERACTION_GLYPH_X, NPC_INTERACTION_SCREEN_BG, NPC_INTERACTION_SCREEN_FG, NPC_INTERACTION_SCREEN_GAP_WIDTH, NPC_INTERACTION_SCREEN_HEIGHT, NPC_INTERACTION_SCREEN_WIDTH, NPC_INTERACTION_SCREEN_X, NPC_INTERACTION_SCREEN_Y, PLACE_BOX_BG, PLACE_BOX_FG, PLACE_BOX_HEIGHT, PLACE_BOX_WIDTH, PLACE_BOX_X, PLACE_BOX_Y, PLACE_COLOR, PLACE_X, PLACE_Y, QUIT_GAME_STR, QUIT_GAME_Y, SCREEN_HEIGHT, SCREEN_WIDTH, TITLE_STR, TITLE_Y};
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

#[derive(PartialEq, Copy, Clone)]
pub enum NpcInteractionResult { NoResponse, Done, NextDialogue }

fn print_as_paragraph(ctx: &mut Rltk, line: &str, width: usize, x_coord: i32, y_coord: i32, delta_y: i32) -> i32 {
    let mut y = y_coord;
    let mut current_line = String::new();
    for word in line.split_whitespace() {
        if current_line.len() + word.len() <= width {
            if !current_line.is_empty() {
                current_line.push(' ');
            }
        } else {
            ctx.print(x_coord, y, &current_line);
            current_line.clear();
            y += delta_y;
        }
        current_line.push_str(word)
    }
    ctx.print(x_coord, y, &current_line);
    y
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

pub fn draw(ecs: &World, ctx: &mut Rltk) {
    draw_ui(ecs, ctx);
    draw_time_and_date(ecs, ctx);
    draw_tooltips(ecs, ctx);
    draw_inventory(ecs, ctx);
}

fn draw_ui(ecs: &World, ctx: &mut Rltk) {
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
}

fn draw_time_and_date(ecs: &World, ctx: &mut Rltk) {
    ctx.draw_box(PLACE_BOX_X, PLACE_BOX_Y, PLACE_BOX_WIDTH, PLACE_BOX_HEIGHT, PLACE_BOX_FG, PLACE_BOX_BG);
    let current_place = ecs.fetch::<Place>();
    let place_name_year_str = format!("{}, {}", current_place.get_name(), current_place.get_year());
    ctx.print_color(PLACE_X, PLACE_Y, PLACE_COLOR, BACKGROUND_COLOR, place_name_year_str);
}

fn draw_tooltips(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();
    let current_place = *ecs.fetch::<Place>();
    let names = ecs.read_storage::<Name>();
    let positions = ecs.read_storage::<Position>();
    let belongs = ecs.read_storage::<BelongsTo>();
    let entities = ecs.entities();

    let mouse_pos = ctx.mouse_pos();
    if mouse_pos.0 >= map.width || mouse_pos.1 >= map.height { return; }
    let mut tooltip: Vec<String> = Vec::new();
    for (name, position, bel, ent) in (&names, &positions, &belongs, &entities).join() {
        if bel.domain == current_place && position.x == mouse_pos.0 && position.y == mouse_pos.1 {
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

fn draw_inventory(ecs: &World, ctx: &mut Rltk) {
    ctx.draw_box(INVENTORY_X, INVENTORY_Y, INVENTORY_WIDTH,
                 INVENTORY_HEIGHT, INVENTORY_BORDER_COLOR, INVENTORY_BACKGROUND_COLOR);

    let names = ecs.read_storage::<Name>();
    let backpack = ecs.read_storage::<Stored>();

    let mut y = 25;
    ctx.print_color(INVENTORY_BANNER_X, y - 2, INVENTORY_BANNER_COLOR, BACKGROUND_COLOR, INVENTORY_BANNER);

    for (_pack, name) in (&backpack, &names).join() {
        ctx.print_color(INVENTORY_ITEMS_X, y, INVENTORY_STRING_COLOR, BACKGROUND_COLOR, &name.name.to_string());
        y += 2;
    }
}

pub fn use_item(gs: &mut State, ctx: &mut Rltk) -> (ItemMenuResult, Option<Entity>) {
    let names = gs.ecs.read_storage::<Name>();
    let backpack = gs.ecs.read_storage::<Stored>();
    let entities = gs.ecs.entities();

    let inventory = (&backpack, &names).join();
    let count = inventory.count();

    let mut y = 25;
    ctx.print_color(INVENTORY_BANNER_X, y - 2, RGB::named(YELLOW), BACKGROUND_COLOR, INVENTORY_BANNER);

    let mut j = 0;
    let mut usable: Vec<Entity> = Vec::new();
    for (item_ent, _pack, name) in (&entities, &backpack, &names).join() {
        ctx.set(INVENTORY_ITEMS_X - 3, y, RGB::named(WHITE), RGB::named(BLACK), rltk::to_cp437('('));
        ctx.set(INVENTORY_ITEMS_X - 2, y, RGB::named(YELLOW), RGB::named(BLACK), 97 + j as rltk::FontCharType);
        ctx.set(INVENTORY_ITEMS_X - 1, y, RGB::named(WHITE), RGB::named(BLACK), rltk::to_cp437(')'));

        ctx.print(INVENTORY_ITEMS_X, y, &name.name.to_string());
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

pub fn interact_with_npc(ecs: &mut World, ctx: &mut Rltk, dialogue_index: usize) -> NpcInteractionResult {
    ctx.draw_box(NPC_INTERACTION_SCREEN_X, NPC_INTERACTION_SCREEN_Y,
                 NPC_INTERACTION_SCREEN_WIDTH, NPC_INTERACTION_SCREEN_HEIGHT,
                 NPC_INTERACTION_SCREEN_FG, NPC_INTERACTION_SCREEN_BG);
    let npcs = ecs.read_storage::<Npc>();
    let has_interaction = ecs.read_storage::<Interaction>();
    let positions = ecs.read_storage::<Position>();
    let names = ecs.read_storage::<Name>();
    let renderables = ecs.read_storage::<Renderable>();
    let mut target = ecs.fetch_mut::<TargetedPosition>();
    for (_npc, interaction, pos, name, rend) in (&npcs, &has_interaction, &positions, &names, &renderables).join() {
        if pos.x == target.x && pos.y == target.y {
            if dialogue_index >= interaction.dialogues[interaction.dialogue_index].len() {
                return NpcInteractionResult::Done;
            }
            let glyph_x = NPC_INTERACTION_GLYPH_X;
            let str_x = NPC_INTERACTION_DIALOGUE_X;
            let mut y = NPC_INTERACTION_DIALOGUE_Y;
            let completed_dialogue = &interaction.dialogues[interaction.dialogue_index][0..=dialogue_index];
            ctx.print(NPC_INTERACTION_DIALOGUE_HEADING_X - name.name.len() as i32, NPC_INTERACTION_DIALOGUE_HEADING_Y, &name.name);
            for dialogue in completed_dialogue {
                ctx.set(glyph_x, y, rend.fg, rend.bg, rend.glyph);
                y = print_as_paragraph(ctx, &dialogue, NPC_INTERACTION_SCREEN_GAP_WIDTH as usize,
                                       str_x, y, NPC_INTERACTION_DIALOGUE_DELTA);
                y += NPC_INTERACTION_DIALOGUE_DELTA;
            }
            if let Some(key) = ctx.key {
                if key == VirtualKeyCode::Return {
                    return NpcInteractionResult::NextDialogue;
                }
            }
        }
    }
    NpcInteractionResult::NoResponse
}



