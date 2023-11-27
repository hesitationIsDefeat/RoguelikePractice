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
pub const DELTA_Y: i32 = 2;
pub const NEW_GAME_Y: i32 = SCREEN_HEIGHT / 2;
pub const LOAD_GAME_Y: i32 = NEW_GAME_Y + DELTA_Y;
pub const QUIT_GAME_Y: i32 = LOAD_GAME_Y + DELTA_Y;
pub const TITLE_STR: &str = "OYUNA HOS GELDIN";
pub const NEW_GAME_STR: &str = "YENI OYUN";
pub const LOAD_GAME_STR: &str = "OYUN YUKLE";
pub const QUIT_GAME_STR: &str = "OYUNDAN CIK";
// CHARS
pub const PLAYER_CHAR: char = '@';
pub const KEY_CHAR: char = 'k';
pub const PORTAL_CHAR: char = 'p';
pub const NPC_CHAR: char = '☺';

// INVENTORY
pub const INVENTORY_X: i32 = MAP_WIDTH;
pub const INVENTORY_Y: i32 = MAP_HEIGHT - INVENTORY_HEIGHT - 1;
pub const INVENTORY_WIDTH: i32 = SCREEN_WIDTH - MAP_WIDTH - 1;
pub const INVENTORY_HEIGHT: i32 = 19;
pub const INVENTORY_BANNER: &str = "Esyalar";
pub const INVENTORY_BANNER_X: i32 = INVENTORY_X + 8;
pub const INVENTORY_ITEMS_X: i32 = INVENTORY_X + 5;
// COLORS
pub const BACKGROUND_COLOR: RGB = RGB { r: 0., g: 0., b: 0. };
// COLORS MENU
pub const MENU_SELECTED_COLOR: RGB = RGB { r: 128f32 / 255.0, g: 0., b: 0. };
pub const MENU_UNSELECTED_COLOR: RGB = RGB { r: 1.0, g: 1.0, b: 1.0 };
// COLORS ITEMS
pub const ITEM_KEY_COLOR: RGB = RGB { r: 240f32 / 255.0, g: 250f32 / 255.0, b: 30f32 / 255.0 };
pub const ITEM_DOOR_COLOR: RGB = RGB { r: 52f32 / 255.0, g: 27f32 / 255.0, b: 212f32 / 255.0 };
pub const ITEM_PORTAL_COLOR: RGB = RGB { r: 21f32 / 255.0, g: 246f32 / 255.0, b: 111f32 / 255.0 };
// COLORS TILES
pub const TILE_COLOR: RGB = RGB { r: 188f32 / 255.0, g: 188f32 / 255.0, b: 188f32 / 255.0 };
pub const WALL_COLOR: RGB = RGB { r: 130f32 / 255.0, g: 130f32 / 255.0, b: 130f32 / 255.0 };
// COLORS CURSOR
pub const CURSOR_COLOR: RGB = RGB { r: 242f32 / 255.0, g: 47f32 / 255.0, b: 196f32 / 255.0 };
// COLORS CONSOLE
pub const CONSOLE_BORDER_COLOR: RGB = RGB { r: 1.0, g: 1.0, b: 1.0 };
pub const CONSOLE_BACKGROUND_COLOR: RGB = RGB { r: 0., g: 0., b: 0. };
pub const CONSOLE_LOG_COLOR: RGB = RGB { r: 1.0, g: 1.0, b: 1.0 };
// COLORS INVENTORY
pub const INVENTORY_BORDER_COLOR: RGB = RGB { r: 1.0, g: 1.0, b: 1.0 };
pub const INVENTORY_BACKGROUND_COLOR: RGB = RGB { r: 0., g: 0., b: 0. };
pub const INVENTORY_BANNER_COLOR: RGB = RGB { r: 238f32 / 255.0, g: 253f32 / 255.0, b: 28f32 / 255.0 };
pub const INVENTORY_STRING_COLOR: RGB = RGB { r: 1.0, g: 1.0, b: 1.0 };
// COLORS CHARACTERS
pub const PLAYER_COLOR: RGB = RGB { r: 1.0, g: 50f32 / 255.0, b: 0. };
pub const NPC_COLOR: RGB = RGB { r: 1.0, g: 111f32 / 255.0, b: 0. };




