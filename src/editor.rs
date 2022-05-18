use druid::{
    widget::{RawLabel, Scroll, TextBox},
    BoxConstraints, Point, Size, Widget, WidgetExt, WidgetPod,
};

use crate::{data::AppState, view::RichTextRebuilder};

pub struct Editor {
    source_text: WidgetPod<AppState, Box<dyn Widget<AppState>>>,
    rendered_text: WidgetPod<AppState, Box<dyn Widget<AppState>>>,
}

impl Editor {
    pub fn new(_data: &AppState) -> Self {
        let source_text = WidgetPod::new(
            TextBox::multiline()
                .expand()
                .lens(AppState::source)
                .controller(RichTextRebuilder)
                .boxed(),
        );

        let rendered_text = WidgetPod::new(
            Scroll::new(RawLabel::new().lens(AppState::rendered).expand_width())
                .vertical()
                .expand()
                .boxed(),
        );

        Self {
            source_text,
            rendered_text,
        }
    }
}

impl Widget<AppState> for Editor {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut AppState,
        env: &druid::Env,
    ) {
        self.source_text.event(ctx, event, data, env);
        self.rendered_text.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &AppState,
        env: &druid::Env,
    ) {
        self.source_text.lifecycle(ctx, event, data, env);
        self.rendered_text.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        _old_data: &AppState,
        data: &AppState,
        env: &druid::Env,
    ) {
        self.source_text.update(ctx, data, env);
        self.rendered_text.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &AppState,
        env: &druid::Env,
    ) -> druid::Size {
        let size = bc.max();
        let half_width = size.width / 2.0;
        let height = size.height;

        let half = BoxConstraints::tight(Size::new(half_width, height));

        self.source_text.layout(ctx, &half, data, env);
        self.source_text.set_origin(ctx, data, env, Point::ZERO);

        self.rendered_text.layout(ctx, &half, data, env);
        self.rendered_text
            .set_origin(ctx, data, env, Point::new(half_width, 0.0));

        size
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &AppState, env: &druid::Env) {
        self.source_text.paint(ctx, data, env);
        self.rendered_text.paint(ctx, data, env);
    }
}
