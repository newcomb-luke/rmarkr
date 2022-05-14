// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

mod data;
mod render;
mod view;

use std::sync::Arc;

use data::AppState;
use druid::text::RichText;
use druid::{
    AppDelegate, AppLauncher, Command, Data, DelegateCtx, Env, Handled, LocalizedString, Menu,
    Selector, Target, WindowDesc, WindowId,
};
use view::build_root_widget;

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
    let main_window = WindowDesc::new(build_root_widget())
        .title("rmarkr")
        .transparent(true)
        .menu(make_menu)
        .window_size((400.0, 600.0));

    let source = Arc::new(String::new());

    let data = AppState {
        rendered: RichText::new("".into()),
        source,
    };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .delegate(CommandDelegate)
        .launch(data)
        .expect("Failed to launch application");
}

fn make_menu<T: Data>(_window: Option<WindowId>, _data: &AppState, _env: &Env) -> Menu<T> {
    let mut base = Menu::empty();

    base = base.entry(
        Menu::new(LocalizedString::new("common-menu-file-menu"))
            .entry(druid::platform_menus::win::file::new())
            .entry(druid::platform_menus::win::file::open())
            .entry(druid::platform_menus::win::file::save())
            .entry(druid::platform_menus::win::file::save_as())
            .separator()
            .entry(druid::platform_menus::win::file::close()),
    );

    base
}
