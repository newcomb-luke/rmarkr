// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use std::sync::Arc;

use druid::text::{AttributesAdder, RichText, RichTextBuilder};
use druid::widget::{prelude::*, RawLabel, Scroll};
use druid::widget::{Controller, Flex, TextBox};
use druid::{
    theme, AppDelegate, AppLauncher, Command, Data, DelegateCtx, Env, FontFamily, FontStyle,
    FontWeight, Handled, Lens, LocalizedString, Menu, Selector, Target, Widget, WidgetExt,
    WindowDesc, WindowId,
};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag};

const OPEN_LINK: Selector<String> = Selector::new("druid-example.open-link");

#[derive(Debug, Data, Clone, Lens)]
struct AppState {
    rendered: RichText,
    source: Arc<String>,
}

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

fn build_root_widget() -> impl Widget<AppState> {
    let source_text = TextBox::multiline()
        .expand()
        .lens(AppState::source)
        .controller(RichTextRebuilder);

    let rendered_text = Scroll::new(RawLabel::new().lens(AppState::rendered).expand_width())
        .vertical()
        .expand();

    Flex::row()
        .with_flex_child(Flex::column().with_flex_child(source_text, 1.0), 1.0)
        .with_flex_child(Flex::column().with_flex_child(rendered_text, 1.0), 1.0)
        .background(theme::BACKGROUND_DARK)
}

/// Parse a markdown string and generate a `RichText` object with
/// the appropriate attributes.
fn rebuild_rendered_text(text: &str) -> RichText {
    let mut current_pos = 0;
    let mut builder = RichTextBuilder::new();
    let mut tag_stack = Vec::new();

    let parser = Parser::new_ext(text, Options::ENABLE_STRIKETHROUGH);
    for event in parser {
        match event {
            Event::Start(tag) => {
                tag_stack.push((current_pos, tag));
            }
            Event::Text(txt) => {
                builder.push(&txt);
                current_pos += txt.len();
            }
            Event::End(end_tag) => {
                let (start_off, tag) = tag_stack
                    .pop()
                    .expect("parser does not return unbalanced tags");
                assert_eq!(end_tag, tag, "mismatched tags?");
                add_attribute_for_tag(
                    &tag,
                    builder.add_attributes_for_range(start_off..current_pos),
                );
                if add_newline_after_tag(&tag) {
                    builder.push("\n\n");
                    current_pos += 2;
                }
            }
            Event::Code(txt) => {
                builder.push(&txt).font_family(FontFamily::MONOSPACE);
                current_pos += txt.len();
            }
            Event::Html(txt) => {
                builder.push(&txt).font_family(FontFamily::MONOSPACE);
                current_pos += txt.len();
            }
            Event::HardBreak => {
                builder.push("\n\n");
                current_pos += 2;
            }
            _ => (),
        }
    }
    builder.build()
}

fn add_newline_after_tag(tag: &Tag) -> bool {
    !matches!(
        tag,
        Tag::Emphasis | Tag::Strong | Tag::Strikethrough | Tag::Link(..)
    )
}

fn add_attribute_for_tag(tag: &Tag, mut attrs: AttributesAdder) {
    match tag {
        Tag::Heading(lvl, _fragment, _classes) => {
            let font_size = match lvl {
                HeadingLevel::H1 => 38.,
                HeadingLevel::H2 => 32.0,
                HeadingLevel::H3 => 26.0,
                HeadingLevel::H4 => 20.0,
                HeadingLevel::H5 => 16.0,
                _ => 12.0,
            };
            attrs.size(font_size).weight(FontWeight::BOLD);
        }
        Tag::BlockQuote => {
            attrs.style(FontStyle::Italic);
        }
        Tag::CodeBlock(_) => {
            attrs.font_family(FontFamily::MONOSPACE);
        }
        Tag::Emphasis => {
            attrs.style(FontStyle::Italic);
        }
        Tag::Strong => {
            attrs.weight(FontWeight::BOLD);
        }
        Tag::Strikethrough => {
            attrs.strikethrough(true);
        }
        Tag::Link(_link_ty, target, _title) => {
            attrs
                .underline(true)
                .link(OPEN_LINK.with(target.to_string()));
        }
        // ignore other tags for now
        _ => (),
    }
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
