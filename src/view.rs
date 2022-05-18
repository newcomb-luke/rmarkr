use druid::widget::Controller;
use druid::{Data, Env, EventCtx, LocalizedString, MenuItem, Point, SysMods, WidgetId, WindowId};
use druid::{Menu, Widget};

use crate::data::AppState;
use crate::render::rebuild_rendered_text;
use crate::widgets::MenuData;

/// A controller that rebuilds the preview when edits occur
pub struct RichTextRebuilder;

impl<W: Widget<AppState>> Controller<AppState, W> for RichTextRebuilder {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &druid::Event,
        data: &mut AppState,
        env: &Env,
    ) {
        let pre_data = data.source.to_owned();
        child.event(ctx, event, data, env);
        if !data.source.same(&pre_data) {
            data.rendered = rebuild_rendered_text(&data.source);
        }
    }
}

pub fn build_menu_data() -> MenuData {
    MenuData {
        active: 0,
        widget_id: WidgetId::next(),
        origin: Point::new(0.0, 0.0),
        items: vec![].into(),
        shown: false,
    }
}

fn preferences() -> MenuItem<AppState> {
    MenuItem::new(LocalizedString::new("macos-menu-preferences"))
        .command(druid::commands::SHOW_PREFERENCES)
        .hotkey(SysMods::Cmd, ",")
}

pub fn build_window_menu(
    _window_id: Option<WindowId>,
    _app_state: &AppState,
    _env: &Env,
) -> Menu<AppState> {
    let mut base = Menu::empty();

    #[cfg(target_os = "macos")]
    {
        base = base.entry(
            druid::Menu::new(LocalizedString::new("common-menu-file-menu"))
                .entry(druid::platform_menus::mac::file::new_file())
                .entry(druid::platform_menus::mac::file::open_file())
                .separator()
                .entry(druid::platform_menus::mac::file::save())
                .entry(druid::platform_menus::mac::file::save_as())
                .separator()
                .entry(druid::platform_menus::mac::file::close()),
        );

        base = base.entry(druid::platform_menus::mac::application::default());
    }
    base = base.entry(
        druid::Menu::new(LocalizedString::new("common-menu-file-menu"))
            .entry(druid::platform_menus::win::file::new())
            .entry(druid::platform_menus::win::file::open())
            .separator()
            .entry(druid::platform_menus::win::file::save())
            .entry(druid::platform_menus::win::file::save_as())
            .separator()
            .entry(druid::platform_menus::win::file::close()),
    );

    base.entry(
        druid::Menu::new(LocalizedString::new("common-menu-edit-menu"))
            .entry(druid::platform_menus::common::undo())
            .entry(druid::platform_menus::common::redo())
            .separator()
            .entry(druid::platform_menus::common::cut())
            .entry(druid::platform_menus::common::copy())
            .entry(druid::platform_menus::common::paste())
            .separator()
            .entry(preferences()),
    )
}
