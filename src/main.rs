// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

mod data;
pub mod editor;
mod locale;
mod render;
pub mod theme;
mod view;
pub mod widgets;
pub mod window;

use std::sync::Arc;

use data::AppState;
use druid::text::RichText;
use druid::{
    AppDelegate, AppLauncher, ArcStr, Command, Data, DelegateCtx, Env, Handled, Key, Selector,
    Target, Value, ValueType, WindowDesc,
};
use locale::window_localization;
use view::{build_menu_data, build_window_menu};
use window::RmarkrWindow;

pub const OPEN_LINK: Selector<String> = Selector::new("druid-example.open-link");

struct CommandDelegate;

impl<T: Data> AppDelegate<T> for CommandDelegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        _data: &mut T,
        _env: &Env,
    ) -> Handled {
        if let Some(url) = cmd.get(OPEN_LINK) {
            open::that_in_background(url);
            Handled::Yes
        } else {
            Handled::No
        }
    }
}

fn main() {
    let source = Arc::new(String::new());

    let data = AppState {
        rendered: RichText::new("".into()),
        menu: build_menu_data().into(),
        source,
    };

    let main_window = WindowDesc::new(RmarkrWindow::new(&data))
        .menu(build_window_menu)
        .title("rmarkr")
        .window_size((1200.0, 600.0));

    // let (resources, base_dir) = window_localization();

    AppLauncher::with_window(main_window)
        // .localization_resources(resources, base_dir)
        .log_to_console()
        .configure_env(|env, state| {
            env.set(druid::theme::TEXTBOX_BORDER_WIDTH, 0.0);
            env.set(druid::theme::TEXTBOX_BORDER_RADIUS, 0.0);

            theme::add_to_env(env, state);
        })
        .delegate(CommandDelegate)
        .launch(data)
        .expect("Failed to launch application");
}
