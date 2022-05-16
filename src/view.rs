use druid::widget::{Controller, Flex, Label, RawLabel, Scroll};
use druid::{theme, Data, Env, EventCtx, WidgetExt};
use druid::{widget::TextBox, Widget};

use crate::data::AppState;
use crate::render::rebuild_rendered_text;

/// A controller that rebuilds the preview when edits occur
struct RichTextRebuilder;

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

pub fn build_root_widget() -> impl Widget<AppState> {
    let menu = Label::new("Placeholder");

    let source_text = TextBox::multiline()
        .expand()
        .lens(AppState::source)
        .controller(RichTextRebuilder);

    let rendered_text = Scroll::new(RawLabel::new().lens(AppState::rendered).expand_width())
        .vertical()
        .expand();

    Flex::column().with_child(menu).with_flex_child(
        Flex::row()
            .with_flex_child(Flex::column().with_flex_child(source_text, 1.0), 1.0)
            .with_flex_child(Flex::column().with_flex_child(rendered_text, 1.0), 1.0)
            .background(theme::BACKGROUND_DARK),
        1.0,
    )
}
