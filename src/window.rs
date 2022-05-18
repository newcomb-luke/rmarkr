use druid::{Point, Widget, WidgetExt, WidgetPod};

use crate::{data::AppState, editor::Editor, widgets::Menu};

pub struct RmarkrWindow {
    menu: WidgetPod<AppState, Box<dyn Widget<AppState>>>,
    editor: WidgetPod<AppState, Box<dyn Widget<AppState>>>,
}

impl RmarkrWindow {
    pub fn new(data: &AppState) -> Self {
        let menu = Menu::new(&data.menu);
        let editor = Editor::new(data);

        Self {
            menu: WidgetPod::new(menu.boxed()),
            editor: WidgetPod::new(editor.boxed()),
        }
    }
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

        self.editor.layout(ctx, bc, data, env);
        self.editor.set_origin(ctx, data, env, Point::ZERO);

        self.menu.layout(ctx, bc, data, env);
        self.menu.set_origin(ctx, data, env, data.menu.origin);

        size
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &AppState, env: &druid::Env) {
        self.editor.paint(ctx, data, env);
        self.menu.paint(ctx, data, env);
    }
}
