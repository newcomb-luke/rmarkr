// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

mod data;
mod render;
mod view;

use std::sync::Arc;

use data::AppState;
use druid::text::RichText;
use druid::{
    theme, AppDelegate, AppLauncher, Command, Data, DelegateCtx, Env, Handled, Selector, Target,
    WindowDesc,
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
        .window_size((1200.0, 600.0));

    let source = Arc::new(String::new());

    let data = AppState {
        rendered: RichText::new("".into()),
        source,
    };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .configure_env(|env, _state| {
            env.set(theme::TEXTBOX_BORDER_WIDTH, 0.0);
            env.set(theme::TEXTBOX_BORDER_RADIUS, 0.0);
        })
        .delegate(CommandDelegate)
        .launch(data)
        .expect("Failed to launch application");
}
