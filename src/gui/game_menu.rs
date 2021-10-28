use macroquad::{
    experimental::collections::storage,
    prelude::*,
    ui::{hash, root_ui, widgets},
};

use crate::gui::GuiResources;

const MENU_WIDTH: f32 = 200.0;
const MENU_HEIGHT: f32 = 136.0;

const MENU_BUTTON_WIDTH: f32 = MENU_WIDTH - 48.0;
const MENU_BUTTON_HEIGHT: f32 = 42.0;

#[allow(dead_code)]
pub enum GameMenuResult {
    MainMenu,
    Quit,
    Cancel,
}

#[allow(dead_code)]
pub fn show_game_menu() -> Option<GameMenuResult> {
    let gui_resources = storage::get::<GuiResources>();

    let mut res = None;

    root_ui().push_skin(&gui_resources.skins.menu);

    widgets::Window::new(
        hash!(),
        vec2(
            (screen_width() - MENU_WIDTH) / 2.0,
            (screen_height() - MENU_HEIGHT) / 2.0,
        ),
        vec2(MENU_WIDTH, MENU_HEIGHT),
    )
    .titlebar(false)
    .ui(&mut *root_ui(), |ui| {
        let main_menu_btn = widgets::Button::new("Main Menu")
            .size(vec2(MENU_BUTTON_WIDTH, MENU_BUTTON_HEIGHT))
            .ui(ui);

        if main_menu_btn {
            res = Some(GameMenuResult::MainMenu);
        }

        let quit_btn = widgets::Button::new("Quit")
            .size(vec2(MENU_BUTTON_WIDTH, MENU_BUTTON_HEIGHT))
            .ui(ui);

        if quit_btn {
            res = Some(GameMenuResult::Quit);
        }
    });

    root_ui().pop_skin();

    res
}