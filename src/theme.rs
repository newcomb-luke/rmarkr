use druid::{Color, Env, Key};

use crate::data::AppState;

pub const MENU_BACKGROUND_COLOR: Key<Color> = Key::new("rmarkr.theme.menu.menu-background-color");
pub const MENU_BUTTON_TEXT_COLOR: Key<Color> = Key::new("rmarkr.theme.menu.menu-button.text-color");
pub const MENU_BUTTON_CLICKED_COLOR: Key<Color> =
    Key::new("rmarkr.theme.menu.menu-button.clicked-color");

pub(crate) fn add_to_env(env: &mut Env, _state: &AppState) {
    env.set(MENU_BACKGROUND_COLOR, Color::rgb8(0x48, 0x48, 0x48));
    env.set(MENU_BUTTON_TEXT_COLOR, Color::WHITE);
    env.set(MENU_BUTTON_CLICKED_COLOR, Color::rgb8(0x29, 0x29, 0x29));
}
