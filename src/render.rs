use druid::{
    text::{AttributesAdder, RichText, RichTextBuilder},
    FontFamily, FontStyle, FontWeight,
};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag};

use crate::OPEN_LINK;

/// Parse a markdown string and generate a `RichText` object with
/// the appropriate attributes.
pub fn rebuild_rendered_text(text: &str) -> RichText {
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
