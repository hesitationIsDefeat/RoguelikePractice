use rltk::{RGB, Rltk, Point, WHITE, BLACK, VirtualKeyCode, RED, GREY, YELLOW};
use specs::prelude::*;
use crate::{BelongsTo, ContainsItems, Interaction, Item, Map, Name, Npc, Objective, Place, PlayerName, Portal, Position, Renderable, RequiresItem, RequiresItems, RunState, save_load_system, State, Stored, TargetedPosition};
use crate::constants::{BACKGROUND_COLOR, CONSOLE_BACKGROUND_COLOR, CONSOLE_BORDER_COLOR, CREDITS_STR, CURSOR_COLOR, MENU_DELTA_Y, INVENTORY_BACKGROUND_COLOR, INVENTORY_BANNER, INVENTORY_BANNER_COLOR, INVENTORY_BANNER_X, INVENTORY_BORDER_COLOR, INVENTORY_DELTA_Y, INVENTORY_HEIGHT, INVENTORY_ITEMS_X, INVENTORY_STRING_COLOR, INVENTORY_WIDTH, INVENTORY_X, INVENTORY_Y, LOAD_GAME_STR, MAP_HEIGHT, MENU_ITEM_1_Y, MENU_SELECTED_COLOR, MENU_UNSELECTED_COLOR, NEW_GAME_STR, NPC_INTERACTION_DIALOGUE_DELTA, NPC_INTERACTION_DIALOGUE_HEADING_X, NPC_INTERACTION_DIALOGUE_HEADING_Y, NPC_INTERACTION_DIALOGUE_X, NPC_INTERACTION_DIALOGUE_Y, NPC_INTERACTION_GLYPH_X, NPC_INTERACTION_SCREEN_BG, NPC_INTERACTION_SCREEN_FG, NPC_INTERACTION_SCREEN_GAP_WIDTH, NPC_INTERACTION_SCREEN_HEIGHT, NPC_INTERACTION_SCREEN_WIDTH, NPC_INTERACTION_SCREEN_X, NPC_INTERACTION_SCREEN_Y, QUIT_GAME_STR, SCREEN_HEIGHT, SCREEN_WIDTH, TITLE_STR, TITLE_Y, CREDITS_1_COLOR, CREDIT_1_STR, CREDITS_THANKS_Y, CREDIT_3_Y, CREDIT_2_Y, CREDIT_1_Y, CREDITS_3_COLOR, CREDITS_2_COLOR, CREDITS_THANKS_COLOR, CREDIT_2_STR, CREDIT_3_STR, CREDITS_THANKS_STR, PLACE_DATE_BOX_X, PLACE_DATE_BOX_Y, PLACE_DATE_BOX_WIDTH, PLACE_DATE_BOX_HEIGHT, PLACE_DATE_BOX_FG, PLACE_DATE_BOX_BG, PLACE_DATE_X, PLACE_DATE_Y, PLACE_DATE_COLOR, CONSOLE_LOG_COLOR, OBJECTIVE_BOX_GAP, OBJECTIVE_X, OBJECTIVE_Y, OBJECTIVE_DELTA_Y, OBJECTIVE_BOX_X, OBJECTIVE_BOX_Y, OBJECTIVE_BOX_WIDTH, OBJECTIVE_BOX_HEIGHT, OBJECTIVE_BOX_FG, OBJECTIVE_BOX_BG, PLACE_DATE_BOX_GAP, PLACE_DATE_DELTA_Y, PLACE_DATE_BANNER_X, PLACE_DATE_BANNER, PLACE_DATE_BANNER_COLOR, OBJECTIVE_BANNER_X, OBJECTIVE_BANNER, OBJECTIVE_BANNER_COLOR, INVENTORY_ITEMS_Y, CONSOLE_ITEM_ACQUIRED, CONSOLE_ITEM_USED, CONSOLE_ITEM_NOT_EXIST};
use crate::gamelog::GameLog;
use crate::items::ItemName;
use crate::npcs::NpcState;

#[derive(PartialEq, Copy, Clone)]
pub enum ItemMenuResult { Cancel, NoResponse, Selected }

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuSelection {
    NewGame,
    LoadGame,
    QuitGame,
    Credits,
}

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuResult {
    NoSelection { selected: MainMenuSelection },
    Selected { selected: MainMenuSelection },
}

#[derive(PartialEq, Copy, Clone)]
pub enum NpcInteractionResult { NoResponse, Done, NextDialogue { index: usize } }

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

pub fn draw_main_menu(gs: &mut State, ctx: &mut Rltk) -> MainMenuResult {
    let save_exists = save_load_system::save_exists();
    let state = gs.ecs.fetch::<RunState>();

    ctx.print_color_centered(TITLE_Y, RGB::named(RED), RGB::named(BLACK), TITLE_STR);

    let mut y = MENU_ITEM_1_Y;

    if let RunState::Menu { menu_selection: selected } = *state {
        ctx.print_color_centered(y, match selected == MainMenuSelection::NewGame {
            true => MENU_SELECTED_COLOR,
            false => MENU_UNSELECTED_COLOR
        }, BACKGROUND_COLOR, NEW_GAME_STR);

        if save_exists {
            y += MENU_DELTA_Y;
            ctx.print_color_centered(y, match selected == MainMenuSelection::LoadGame {
                true => MENU_SELECTED_COLOR,
                false => MENU_UNSELECTED_COLOR
            }, BACKGROUND_COLOR, LOAD_GAME_STR);
        }

        y += MENU_DELTA_Y;
        ctx.print_color_centered(y, match selected == MainMenuSelection::QuitGame {
            true => MENU_SELECTED_COLOR,
            false => MENU_UNSELECTED_COLOR
        }, BACKGROUND_COLOR, QUIT_GAME_STR);

        y += MENU_DELTA_Y;
        ctx.print_color_centered(y, match selected == MainMenuSelection::Credits {
            true => MENU_SELECTED_COLOR,
            false => MENU_UNSELECTED_COLOR
        }, BACKGROUND_COLOR, CREDITS_STR);


        return match ctx.key {
            None => MainMenuResult::NoSelection { selected },
            Some(key) => {
                match key {
                    VirtualKeyCode::Up => {
                        let new_selection = match selected {
                            MainMenuSelection::NewGame => MainMenuSelection::Credits,
                            MainMenuSelection::LoadGame => MainMenuSelection::NewGame,
                            MainMenuSelection::QuitGame => match save_exists {
                                true => MainMenuSelection::LoadGame,
                                false => MainMenuSelection::NewGame
                            }
                            MainMenuSelection::Credits => MainMenuSelection::QuitGame
                        };
                        MainMenuResult::NoSelection { selected: new_selection }
                    }
                    VirtualKeyCode::Down => {
                        let new_selection = match selected {
                            MainMenuSelection::NewGame => match save_exists {
                                true => MainMenuSelection::LoadGame,
                                false => MainMenuSelection::QuitGame
                            },
                            MainMenuSelection::LoadGame => MainMenuSelection::QuitGame,
                            MainMenuSelection::QuitGame => MainMenuSelection::Credits,
                            MainMenuSelection::Credits => MainMenuSelection::NewGame
                        };
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
    draw_objective(ecs, ctx);
}

fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    // CONSOLE
    ctx.draw_box(0, MAP_HEIGHT, SCREEN_WIDTH - 1, SCREEN_HEIGHT - MAP_HEIGHT - 1,
                 CONSOLE_BORDER_COLOR, CONSOLE_BACKGROUND_COLOR);
    let log = ecs.fetch::<GameLog>();
    let mut y = MAP_HEIGHT + 1;
    for s in log.entries.iter().rev() {
        if y < SCREEN_HEIGHT - 1 {
            ctx.print_color(2, y, CONSOLE_LOG_COLOR, CONSOLE_BACKGROUND_COLOR, s);
        }
        y += 1;
    }
    // MOUSE
    let cursor_pos = ctx.mouse_pos();
    ctx.set_bg(cursor_pos.0, cursor_pos.1, CURSOR_COLOR);
}

fn draw_objective(ecs: &World, ctx: &mut Rltk) {
    let objective = ecs.fetch::<Objective>();
    ctx.draw_box(OBJECTIVE_BOX_X, OBJECTIVE_BOX_Y, OBJECTIVE_BOX_WIDTH,
                 OBJECTIVE_BOX_HEIGHT, OBJECTIVE_BOX_FG, OBJECTIVE_BOX_BG);
    ctx.print_color(OBJECTIVE_BANNER_X, OBJECTIVE_BOX_Y, OBJECTIVE_BANNER_COLOR, BACKGROUND_COLOR, OBJECTIVE_BANNER);
    print_as_paragraph(ctx, &objective.objectives[objective.index], OBJECTIVE_BOX_GAP as usize, OBJECTIVE_X, OBJECTIVE_Y, OBJECTIVE_DELTA_Y);
}

fn draw_time_and_date(ecs: &World, ctx: &mut Rltk) {
    ctx.draw_box(PLACE_DATE_BOX_X, PLACE_DATE_BOX_Y, PLACE_DATE_BOX_WIDTH, PLACE_DATE_BOX_HEIGHT, PLACE_DATE_BOX_FG, PLACE_DATE_BOX_BG);
    ctx.print_color(PLACE_DATE_BANNER_X, PLACE_DATE_BOX_Y, PLACE_DATE_BANNER_COLOR, BACKGROUND_COLOR, PLACE_DATE_BANNER);
    let current_place = ecs.fetch::<Place>();
    let place_name_year_str = format!("{}, {}", current_place.get_name(), current_place.get_year());
    print_as_paragraph(ctx, place_name_year_str.as_str(), PLACE_DATE_BOX_GAP as usize, PLACE_DATE_X, PLACE_DATE_Y, PLACE_DATE_DELTA_Y);
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

    let mut y = INVENTORY_ITEMS_Y;
    ctx.print_color(INVENTORY_BANNER_X, y - 2, RGB::named(YELLOW), BACKGROUND_COLOR, INVENTORY_BANNER);

    for (_pack, name) in (&backpack, &names).join() {
        ctx.print_color(INVENTORY_ITEMS_X, y, INVENTORY_STRING_COLOR, BACKGROUND_COLOR, &name.name.to_string());
        y += INVENTORY_DELTA_Y;
    }
}

pub fn draw_use_item(gs: &mut State, ctx: &mut Rltk) -> (ItemMenuResult, Option<ItemName>) {
    let items = gs.ecs.read_storage::<Item>();
    let backpack = gs.ecs.read_storage::<Stored>();

    let inventory = (&backpack, &items).join();
    let count = inventory.count();

    let mut y = INVENTORY_ITEMS_Y;

    let mut j = 0;
    let mut usable: Vec<ItemName> = Vec::new();
    for (_pack, item) in (&backpack, &items).join() {
        ctx.set(INVENTORY_ITEMS_X - 3, y, RGB::named(WHITE), RGB::named(BLACK), rltk::to_cp437('('));
        ctx.set(INVENTORY_ITEMS_X - 2, y, RGB::named(YELLOW), RGB::named(BLACK), 97 + j as rltk::FontCharType);
        ctx.set(INVENTORY_ITEMS_X - 1, y, RGB::named(WHITE), RGB::named(BLACK), rltk::to_cp437(')'));

        usable.push(item.name);
        y += INVENTORY_DELTA_Y;
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

pub fn draw_npc_interaction(ecs: &mut World, ctx: &mut Rltk, dialogue_index: usize) -> NpcInteractionResult {
    ctx.draw_box(NPC_INTERACTION_SCREEN_X, NPC_INTERACTION_SCREEN_Y,
                 NPC_INTERACTION_SCREEN_WIDTH, NPC_INTERACTION_SCREEN_HEIGHT,
                 NPC_INTERACTION_SCREEN_FG, NPC_INTERACTION_SCREEN_BG);
    let positions = ecs.read_storage::<Position>();
    let names = ecs.read_storage::<Name>();
    let renderables = ecs.read_storage::<Renderable>();
    let items = ecs.read_storage::<Item>();
    let belongs = ecs.read_storage::<BelongsTo>();
    let current_place = ecs.fetch::<Place>();
    let mut npcs = ecs.write_storage::<Npc>();
    let mut contains_items = ecs.write_storage::<ContainsItems>();
    let mut log = ecs.write_resource::<GameLog>();
    let mut has_interaction = ecs.write_storage::<Interaction>();
    let mut target = ecs.fetch_mut::<TargetedPosition>();
    let mut stored_items = ecs.write_storage::<Stored>();
    let mut requires_items = ecs.write_storage::<RequiresItems>();
    let entities = ecs.entities();
    for (npc, interaction, pos, name, rend, cont, req, bel) in (&mut npcs, &mut has_interaction, &positions, &names, &renderables, &mut contains_items, &mut requires_items, &belongs).join() {
        if bel.domain == *current_place && pos.x == target.x && pos.y == target.y {
            let mut dialogue_index_returned = dialogue_index + 1;
            match npc.state {
                NpcState::HasDialogue => {
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
                    if interaction.dialogue_index >= interaction.dialogues.len() - 1 {
                        npc.state = NpcState::Done;
                    }
                    if let Some(key) = ctx.key {
                        if key == VirtualKeyCode::Return {
                            if dialogue_index >= interaction.dialogues[interaction.dialogue_index].len() - 1 {
                                dialogue_index_returned = 0;
                                if interaction.give_item_indices.contains(&interaction.dialogue_index) {
                                    npc.state = NpcState::WillGiveItem;
                                } else if interaction.get_item_indices.contains(&interaction.dialogue_index) {
                                    npc.state = NpcState::WantsItem;
                                } else {
                                    interaction.dialogue_index += 1;
                                }
                            }
                            return NpcInteractionResult::NextDialogue { index: dialogue_index_returned };
                        }
                    }
                }
                NpcState::WantsItem => {
                    let mut dont_have_the_item = true;
                    for (item, ent) in (&items, &entities).join() {
                        if req.items.get(0).unwrap() == &item.name && stored_items.contains(ent) {
                            let required_item = req.items.remove(0);
                            let required_item_name = required_item.to_string();
                            interaction.get_item_indices.remove(0);
                            stored_items.remove(ent);
                            log.entries.push(format!("{} {}", CONSOLE_ITEM_USED, required_item_name));
                            dont_have_the_item = false;
                            npc.state = NpcState::HasDialogue;
                            interaction.dialogue_index += 1;
                            interaction.repeat = false;
                            break;
                        }
                    }
                    if dont_have_the_item {
                        if interaction.print_no_item {
                            log.entries.push(format!("{}", CONSOLE_ITEM_NOT_EXIST));
                            interaction.print_no_item = false;
                        }
                        if interaction.repeat {
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
                                    target.x = -1;
                                    target.y = -1;
                                    interaction.print_no_item = true;
                                    return NpcInteractionResult::Done;
                                }
                            }
                        } else {
                            target.x = -1;
                            target.y = -1;
                            interaction.repeat = true;
                            interaction.print_no_item = true;
                            return NpcInteractionResult::Done;
                        }
                    }
                }
                NpcState::WillGiveItem => {
                    let removed_item_name = cont.items.remove(0);
                    interaction.give_item_indices.remove(0);
                    let item_name = removed_item_name.to_string();
                    for (item, ent) in (&items, &entities).join() {
                        if item.name == removed_item_name {
                            stored_items.insert(ent, Stored {}).expect("Error during inserting into stored items");
                            break;
                        }
                    }
                    log.entries.push(format!("{} {}", CONSOLE_ITEM_ACQUIRED, item_name));
                    npc.state = NpcState::HasDialogue;
                    interaction.dialogue_index += 1;
                }
                NpcState::Done => {
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
                            return NpcInteractionResult::Done;
                        }
                    }
                }
            }
            if interaction.change_objective_indices.contains(&interaction.dialogue_index) {
                let mut objective = ecs.fetch_mut::<Objective>();
                objective.index += 1;
                interaction.change_objective_indices.remove(0);
            }
            break;
        }
    }
    NpcInteractionResult::NoResponse
}

pub fn draw_credits(ctx: &mut Rltk) {
    ctx.print_color_centered(CREDIT_1_Y, CREDITS_1_COLOR, BACKGROUND_COLOR, CREDIT_1_STR);
    ctx.print_color_centered(CREDIT_2_Y, CREDITS_2_COLOR, BACKGROUND_COLOR, CREDIT_2_STR);
    ctx.print_color_centered(CREDIT_3_Y, CREDITS_3_COLOR, BACKGROUND_COLOR, CREDIT_3_STR);
    ctx.print_color_centered(CREDITS_THANKS_Y, CREDITS_THANKS_COLOR, BACKGROUND_COLOR, CREDITS_THANKS_STR);
}

fn keycode_to_char(key: Option<VirtualKeyCode>) -> Option<char> {
    if let Some(keycode) = key {
        match keycode {
            VirtualKeyCode::A
            | VirtualKeyCode::B
            | VirtualKeyCode::C
            | VirtualKeyCode::D
            | VirtualKeyCode::E
            | VirtualKeyCode::F
            | VirtualKeyCode::G
            | VirtualKeyCode::H
            | VirtualKeyCode::I
            | VirtualKeyCode::J
            | VirtualKeyCode::K
            | VirtualKeyCode::L
            | VirtualKeyCode::M
            | VirtualKeyCode::N
            | VirtualKeyCode::O
            | VirtualKeyCode::P
            | VirtualKeyCode::Q
            | VirtualKeyCode::R
            | VirtualKeyCode::S
            | VirtualKeyCode::T
            | VirtualKeyCode::U
            | VirtualKeyCode::V
            | VirtualKeyCode::W
            | VirtualKeyCode::X
            | VirtualKeyCode::Y
            | VirtualKeyCode::Z => {
                let key_char = (keycode as u8 - VirtualKeyCode::A as u8 + b'A') as char;
                return Some(key_char);
            }
            _ => {}
        }
    }
    None
}

pub fn draw_enter_name(ecs: &mut World, ctx: &mut Rltk) -> bool {
    ctx.print_centered(SCREEN_HEIGHT / 2 - 5, "ISMINIZI GIRIN:");
    let mut player_name = ecs.fetch_mut::<PlayerName>();
    match ctx.key {
        Some(VirtualKeyCode::A)
        | Some(VirtualKeyCode::B)
        | Some(VirtualKeyCode::C)
        | Some(VirtualKeyCode::D)
        | Some(VirtualKeyCode::E)
        | Some(VirtualKeyCode::F)
        | Some(VirtualKeyCode::G)
        | Some(VirtualKeyCode::H)
        | Some(VirtualKeyCode::I)
        | Some(VirtualKeyCode::J)
        | Some(VirtualKeyCode::K)
        | Some(VirtualKeyCode::L)
        | Some(VirtualKeyCode::M)
        | Some(VirtualKeyCode::N)
        | Some(VirtualKeyCode::O)
        | Some(VirtualKeyCode::P)
        | Some(VirtualKeyCode::Q)
        | Some(VirtualKeyCode::R)
        | Some(VirtualKeyCode::S)
        | Some(VirtualKeyCode::T)
        | Some(VirtualKeyCode::U)
        | Some(VirtualKeyCode::V)
        | Some(VirtualKeyCode::W)
        | Some(VirtualKeyCode::X)
        | Some(VirtualKeyCode::Y)
        | Some(VirtualKeyCode::Z) => {
            if let Some(char) = keycode_to_char(ctx.key) {
                player_name.name.push(char);
            }
        }
        Some(VirtualKeyCode::Return) => {
            return true;
        }
        Some(VirtualKeyCode::Delete) => {
            player_name.name.pop();
        }
        _ => {}
    }
    ctx.print_color_centered_at(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, MENU_SELECTED_COLOR, BACKGROUND_COLOR, &player_name.name);
    false
}

pub fn draw_game_over(ctx: &mut Rltk) {
    ctx.print_color_centered(SCREEN_HEIGHT / 2, MENU_SELECTED_COLOR, BACKGROUND_COLOR, "OYUNU KAZANDIN");
}



