use std::{thread, time::Duration};

use druid::{widget::Button, Point, Target, Widget, WidgetExt, WidgetPod};

use crate::{data::AppState, editor::Editor, widgets::Menu};

pub struct RmarkrWindow {
    menu: WidgetPod<AppState, Box<dyn Widget<AppState>>>,
    editor: WidgetPod<AppState, Box<dyn Widget<AppState>>>,
    button: WidgetPod<AppState, Box<dyn Widget<AppState>>>,
}

impl RmarkrWindow {
    pub fn new(data: &AppState) -> Self {
        let menu = Menu::new(&data.menu);
        let button = Button::new("Hide").on_click(onclick);
        let editor = Editor::new(data);

        Self {
            menu: WidgetPod::new(menu.boxed()),
            editor: WidgetPod::new(editor.boxed()),
            button: WidgetPod::new(Box::new(button)),
        }
    }
}

fn onclick(ctx: &mut druid::EventCtx, _data: &mut AppState, _env: &druid::Env) {
    let handle = ctx.get_external_handle();
    let window_id = ctx.window_id();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(3000));
        handle
            .submit_command(druid::commands::SHOW_WINDOW, (), Target::Window(window_id))
            .unwrap();
    });

    let handle = ctx.get_external_handle();
    let window_id = ctx.window_id();

    thread::spawn(move || {
        handle
            .submit_command(druid::commands::HIDE_WINDOW, (), Target::Window(window_id))
            .unwrap();
    });
}

impl Widget<AppState> for RmarkrWindow {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut AppState,
        env: &druid::Env,
    ) {
        self.editor.event(ctx, event, data, env);
        self.button.event(ctx, event, data, env);
        self.menu.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &AppState,
        env: &druid::Env,
    ) {
        self.editor.lifecycle(ctx, event, data, env);
        self.button.lifecycle(ctx, event, data, env);
        self.menu.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        _old_data: &AppState,
        data: &AppState,
        env: &druid::Env,
    ) {
        self.editor.update(ctx, data, env);
        self.button.update(ctx, data, env);
        self.menu.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &AppState,
        env: &druid::Env,
    ) -> druid::Size {
        let size = bc.max();

        let button_size = druid::Size::new(80.0, 30.0);
        let button_bc = druid::BoxConstraints::new(button_size, button_size);

        let button_size = self.button.layout(ctx, &button_bc, data, env);
        self.button.set_origin(ctx, data, env, Point::ZERO);

        self.editor.layout(ctx, bc, data, env);
        self.editor
            .set_origin(ctx, data, env, Point::new(0.0, button_size.height));

        self.menu.layout(ctx, bc, data, env);
        self.menu.set_origin(ctx, data, env, data.menu.origin);

        size
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &AppState, env: &druid::Env) {
        self.editor.paint(ctx, data, env);
        self.button.paint(ctx, data, env);
        self.menu.paint(ctx, data, env);
    }
}
