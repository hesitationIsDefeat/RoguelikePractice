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
// INVENTORY
pub const INVENTORY_X: i32 = MAP_WIDTH;
pub const INVENTORY_Y: i32 = MAP_HEIGHT - INVENTORY_HEIGHT - 1;
pub const INVENTORY_WIDTH: i32 = SCREEN_WIDTH - MAP_WIDTH - 1;
pub const INVENTORY_HEIGHT: i32 = 19;
pub const INVENTORY_BANNER: &str = "Esyalar";
pub const INVENTORY_BANNER_X: i32 = INVENTORY_X + 8;
pub const INVENTORY_ITEMS_X: i32 = INVENTORY_X + 5;
// COLORS
pub const BACKGROUND_COLOR: RGB = RGB { r: 0.0, g: 0.0, b: 0.0 };
// COLORS MENU
pub const MENU_SELECTED_COLOR: RGB = RGB { r: 128.0, g: 0.0, b: 0.0 };
pub const MENU_UNSELECTED_COLOR: RGB = RGB { r: 255.0, g: 255.0, b: 255.0 };
// COLORS ITEMS
pub const ITEM_KEY_COLOR: RGB = RGB { r: 240.0, g: 250.0, b: 30.0 };
pub const ITEM_DOOR_COLOR: RGB = RGB { r: 52.0, g: 27.0, b: 212.0 };
pub const ITEM_PORTAL_COLOR: RGB = RGB { r: 21.0, g: 246.0, b: 111.0 };
// COLORS TILES
pub const TILE_COLOR: RGB = RGB { r: 188.0, g: 188.0, b: 188.0 };
pub const WALL_COLOR: RGB = RGB { r: 130.0, g: 130.0, b: 130.0 };
// COLORS CURSOR
pub const CURSOR_COLOR: RGB = RGB { r: 242.0, g: 47.0, b: 196.0 };
// COLORS CONSOLE
pub const CONSOLE_BORDER_COLOR: RGB = RGB { r: 255.0, g: 255.0, b: 255.0 };
pub const CONSOLE_BACKGROUND_COLOR: RGB = RGB { r: 0.0, g: 0.0, b: 0.0 };
pub const CONSOLE_LOG_COLOR: RGB = RGB { r: 255.0, g: 255.0, b: 255.0 };
// COLORS INVENTORY
pub const INVENTORY_BORDER_COLOR: RGB = RGB { r: 255.0, g: 255.0, b: 255.0 };
pub const INVENTORY_BACKGROUND_COLOR: RGB = RGB { r: 0.0, g: 0.0, b: 0.0 };
pub const INVENTORY_BANNER_COLOR: RGB = RGB { r: 238.0, g: 253.0, b: 28.0 };
pub const INVENTORY_STRING_COLOR: RGB = RGB { r: 255.0, g: 255.0, b: 255.0 };
// COLORS CHARACTERS
pub const PLAYER_COLOR: RGB = RGB { r: 255.0, g: 50.0, b: 0.0 };




