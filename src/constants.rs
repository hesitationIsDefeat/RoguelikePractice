use rltk::RGB;

// SCREEN
pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;
// MAP
pub const MAP_WIDTH: i32 = 55;
pub const MAP_HEIGHT: i32 = 43;
pub const MAP_TILES: i32 = MAP_WIDTH * MAP_HEIGHT;
// MENU
pub const TITLE_Y: i32 = SCREEN_HEIGHT / 3;
pub const MENU_DELTA_Y: i32 = 2;
pub const MENU_ITEM_1_Y: i32 = SCREEN_HEIGHT / 2;
pub const TITLE_STR: &str = "OYUNA HOS GELDIN";
pub const NEW_GAME_STR: &str = "YENI OYUN";
pub const LOAD_GAME_STR: &str = "OYUN YUKLE";
pub const QUIT_GAME_STR: &str = "OYUNDAN CIK";
pub const CREDITS_STR: &str = "KATKIDA BULUNANLAR";
// CREDITS
pub const CREDIT_1_Y: i32 = SCREEN_HEIGHT / 3;
pub const CREDIT_2_Y: i32 = CREDIT_1_Y + 4;
pub const CREDIT_3_Y: i32 = CREDIT_2_Y + 4;
pub const CREDITS_THANKS_Y: i32 = CREDIT_3_Y + 10;
pub const CREDIT_1_STR: &str = "Aysila Cengiz: Fikri ve destegi icin";
pub const CREDIT_2_STR: &str = "Herbert Wolverson: Rust ile oyun gelistirmeyi ogrettigi icin";
pub const CREDIT_3_STR: &str = "Oramakoma Buramako: Tarih uzerine arastirmalari icin";
pub const CREDITS_THANKS_STR: &str = "TESEKKURLER";
// CHARS
pub const PLAYER_CHAR: char = '☻';
pub const KEY_CHAR: char = '♪';
pub const PORTAL_CHAR: char = 'Ω';
pub const NPC_CHAR: char = '☺';
// OBJECTIVE
pub const OBJECTIVE_BOX_X: i32 = INVENTORY_X;
pub const OBJECTIVE_BOX_Y: i32 = PLACE_DATE_BOX_Y - OBJECTIVE_BOX_HEIGHT - 1;
pub const OBJECTIVE_BOX_WIDTH: i32 = INVENTORY_WIDTH;
pub const OBJECTIVE_BOX_GAP: i32 = OBJECTIVE_BOX_WIDTH - 2 * OBJECTIVE_DELTA_Y;
pub const OBJECTIVE_BOX_HEIGHT: i32 = 10;
pub const OBJECTIVE_X: i32 = OBJECTIVE_BOX_X + 2;
pub const OBJECTIVE_Y: i32 = OBJECTIVE_BOX_Y + 2;
pub const OBJECTIVE_DELTA_Y: i32 = 2;
// PLACE DATE
pub const PLACE_HOME_NAME: &str = "Ev";
pub const PLACE_SCHOOL_NAME: &str = "Bogazici Guney Kampus";
pub const PLACE_CLASS_NAME: &str = "M 2152";
pub const PLACE_LIB_NAME: &str = "Bogazici Olmayan Kutuphane";
pub const PLACE_OTTOMAN_MAIN_NAME: &str = "Osmanli Meydan";
pub const PLACE_OTTOMAN_LEFT_NAME: &str = "Osmanli Sol";
pub const PLACE_OTTOMAN_RIGHT_NAME: &str = "Osmanli Sag";
pub const PLACE_OTTOMAN_TOP_NAME: &str = "Osmanli Yukari";
pub const PLACE_OTTOMAN_BOTTOM_NAME: &str = "Osmanli Asagi";
pub const CURRENT_DATE: &str = "2023";
pub const PAST_DATE: &str = "1900";
pub const PLACE_DATE_BOX_X: i32 = INVENTORY_X;
pub const PLACE_DATE_BOX_Y: i32 = INVENTORY_Y - 3;
pub const PLACE_DATE_BOX_WIDTH: i32 = INVENTORY_WIDTH;
pub const PLACE_DATE_BOX_HEIGHT: i32 = 2;
pub const PLACE_DATE_X: i32 = PLACE_DATE_BOX_X + 2;
pub const PLACE_DATE_Y: i32 = PLACE_DATE_BOX_Y + 1;
// INVENTORY
pub const INVENTORY_X: i32 = MAP_WIDTH;
pub const INVENTORY_Y: i32 = MAP_HEIGHT - INVENTORY_HEIGHT - 1;
pub const INVENTORY_DELTA_Y: i32 = 2;
pub const INVENTORY_WIDTH: i32 = SCREEN_WIDTH - MAP_WIDTH - 1;
pub const INVENTORY_HEIGHT: i32 = 19;
pub const INVENTORY_BANNER: &str = "Esyalar";
pub const INVENTORY_BANNER_X: i32 = INVENTORY_X + 8;
pub const INVENTORY_ITEMS_X: i32 = INVENTORY_X + 5;
// ITEMS
pub const ITEM_LIB_KEY_NAME: &str = "Kutuphane Anahtari";
pub const ITEM_OLD_KEY_1_NAME: &str = "Eski Anahtar 1";
pub const ITEM_OLD_KEY_2_NAME: &str = "Eski Anahtar 2";
pub const ITEM_SWORD_NAME: &str = "Kilic";
// NPC INTERACTION
pub const NPC_INTERACTION_SCREEN_WIDTH: i32 = 40;
pub const NPC_INTERACTION_SCREEN_HEIGHT: i32 = 40;
pub const NPC_INTERACTION_SCREEN_GAP_WIDTH: i32 = NPC_INTERACTION_SCREEN_WIDTH - 3 * NPC_INTERACTION_DIALOGUE_DELTA;
pub const NPC_INTERACTION_SCREEN_X: i32 = (MAP_WIDTH - NPC_INTERACTION_SCREEN_WIDTH) / 2;
pub const NPC_INTERACTION_SCREEN_Y: i32 = (MAP_HEIGHT - NPC_INTERACTION_SCREEN_HEIGHT) / 2;
pub const NPC_INTERACTION_DIALOGUE_DELTA: i32 = 2;
pub const NPC_INTERACTION_DIALOGUE_HEADING_X: i32 = NPC_INTERACTION_SCREEN_X + NPC_INTERACTION_SCREEN_WIDTH / 2;
pub const NPC_INTERACTION_DIALOGUE_HEADING_Y: i32 = NPC_INTERACTION_SCREEN_Y + NPC_INTERACTION_DIALOGUE_DELTA;
pub const NPC_INTERACTION_GLYPH_X: i32 = NPC_INTERACTION_SCREEN_X + NPC_INTERACTION_DIALOGUE_DELTA;
pub const NPC_INTERACTION_DIALOGUE_X: i32 = NPC_INTERACTION_GLYPH_X + NPC_INTERACTION_DIALOGUE_DELTA;
pub const NPC_INTERACTION_DIALOGUE_Y: i32 = NPC_INTERACTION_DIALOGUE_HEADING_Y + NPC_INTERACTION_DIALOGUE_DELTA;
// COLORS
pub const BACKGROUND_COLOR: RGB = RGB { r: 0., g: 0., b: 0. };
// COLORS MENU
pub const MENU_SELECTED_COLOR: RGB = RGB { r: 1.0, g: 0., b: 0. };
pub const MENU_UNSELECTED_COLOR: RGB = RGB { r: 1.0, g: 1.0, b: 1.0 };
// COLORS CREDITS
pub const CREDITS_1_COLOR: RGB = RGB { r: 1.0, g: 0.0, b: 0.0 };
pub const CREDITS_2_COLOR: RGB = RGB { r: 0.0, g: 1.0, b: 0.0 };
pub const CREDITS_3_COLOR: RGB = RGB { r: 0.0, g: 0.0, b: 1.0 };
pub const CREDITS_THANKS_COLOR: RGB = RGB { r: 0.2, g: 0.4, b: 0.6 };
// COLORS ITEMS
pub const ITEM_KEY_COLOR: RGB = RGB { r: 240f32 / 255.0, g: 250f32 / 255.0, b: 30f32 / 255.0 };
pub const ITEM_DOOR_COLOR: RGB = RGB { r: 70f32 / 255.0, g: 200f32 / 255.0, b: 200f32 / 255.0 };
pub const ITEM_PORTAL_COLOR: RGB = RGB { r: 21f32 / 255.0, g: 246f32 / 255.0, b: 111f32 / 255.0 };
// COLORS TILES
pub const SPACE_COLOR: RGB = RGB { r: 131f32 / 255.0, g: 131f32 / 255.0, b: 131f32 / 255.0 };
pub const TILE_COLOR: RGB = RGB { r: 188f32 / 255.0, g: 188f32 / 255.0, b: 188f32 / 255.0 };
pub const WALL_COLOR: RGB = RGB { r: 130f32 / 255.0, g: 130f32 / 255.0, b: 130f32 / 255.0 };
// COLORS CURSOR
pub const CURSOR_COLOR: RGB = RGB { r: 242f32 / 255.0, g: 47f32 / 255.0, b: 196f32 / 255.0 };
// COLORS CONSOLE
pub const CONSOLE_BORDER_COLOR: RGB = RGB { r: 1.0, g: 1.0, b: 1.0 };
pub const CONSOLE_BACKGROUND_COLOR: RGB = RGB { r: 0., g: 0., b: 0. };
pub const CONSOLE_LOG_COLOR: RGB = RGB { r: 1.0, g: 1.0, b: 1.0 };
// COLORS OBJECTIVE
pub const OBJECTIVE_BOX_FG: RGB = RGB { r: 1.0, g: 1.0, b: 1.0 };
pub const OBJECTIVE_BOX_BG: RGB = RGB { r: 0., g: 0., b: 0. };
// COLORS PLACE DATE
pub const PLACE_DATE_BOX_FG: RGB = RGB { r: 1.0, g: 1.0, b: 1.0 };
pub const PLACE_DATE_BOX_BG: RGB = RGB { r: 0., g: 0., b: 0. };
pub const PLACE_DATE_COLOR: RGB = RGB { r: 1.0, g: 1.0, b: 1.0 };
// COLORS INVENTORY
pub const INVENTORY_BORDER_COLOR: RGB = RGB { r: 1.0, g: 1.0, b: 1.0 };
pub const INVENTORY_BACKGROUND_COLOR: RGB = RGB { r: 0., g: 0., b: 0. };
pub const INVENTORY_BANNER_COLOR: RGB = RGB { r: 238f32 / 255.0, g: 253f32 / 255.0, b: 28f32 / 255.0 };
pub const INVENTORY_STRING_COLOR: RGB = RGB { r: 1.0, g: 1.0, b: 1.0 };
// COLORS NPC INTERACTION
pub const NPC_INTERACTION_SCREEN_FG: RGB = RGB { r: 1.0, g: 1.0, b: 1.0 };
pub const NPC_INTERACTION_SCREEN_BG: RGB = RGB { r: 0.0, g: 0.0, b: 0.0 };
// COLORS CHARACTERS
pub const PLAYER_COLOR: RGB = RGB { r: 1.0, g: 50f32 / 255.0, b: 0. };
pub const NPC_COLOR: RGB = RGB { r: 1.0, g: 111f32 / 255.0, b: 0. };




