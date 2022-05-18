use druid::{Command, Point, RenderContext, Size, Widget, WidgetId};
use std::sync::Arc;

use crate::data::AppState;

pub struct MenuItem {
    pub desc: Option<String>,
    pub command: Command,
}

impl MenuItem {
    pub fn new(desc: Option<String>, command: Command) -> Self {
        Self { desc, command }
    }
}

pub struct MenuData {
    pub active: usize,
    pub widget_id: WidgetId,
    pub origin: Point,
    pub items: Arc<Vec<MenuItem>>,
    pub shown: bool,
}

pub struct Menu {
    widget_id: WidgetId,
    line_height: f64,
}

impl Menu {
    pub fn new(data: &MenuData) -> Self {
        Self {
            widget_id: data.widget_id,
            line_height: 20.0,
        }
    }
}

impl Widget<AppState> for Menu {
    fn id(&self) -> Option<WidgetId> {
        Some(self.widget_id)
    }

    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        _event: &druid::Event,
        _data: &mut AppState,
        _env: &druid::Env,
    ) {
        ctx.request_focus();
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        _data: &AppState,
        _env: &druid::Env,
    ) {
    }

    fn update(
        &mut self,
        _ctx: &mut druid::UpdateCtx,
        _old_data: &AppState,
        _data: &AppState,
        _env: &druid::Env,
    ) {
    }

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        _bc: &druid::BoxConstraints,
        _data: &AppState,
        _env: &druid::Env,
    ) -> druid::Size {
        let num_items = 2;
        let height = self.line_height * num_items as f64;

        Size::new(300.0, height)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &AppState, env: &druid::Env) {
        /*
        let rect = ctx.size().to_rect();

        let blur_radius = 5.0;
        let blurred_color = env.get(druid::theme::BACKGROUND_DARK);
        ctx.blurred_rect(rect, blur_radius, &blurred_color);

        ctx.fill(rect, &blurred_color);
        */
    }
}
