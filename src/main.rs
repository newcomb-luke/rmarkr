#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use app::RMarkrApp;
use theme::Theme;

mod app;
mod theme;

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.transparent = true;
    options.vsync = true;
    options.maximized = true;

    let theme = Theme::Dark;

    eframe::run_native(
        "rmarkr",
        options,
        Box::new(move |cc| Box::new(RMarkrApp::new(cc, theme))),
    );
}

pub const TEST_SRC: &'static str = r#"impl Style {
    pub fn ui(&mut self, ui: &mut crate::Ui) {
        let Self {
            override_font_id,
            override_text_style,
            text_styles,
            wrap: _,
            spacing,
            interaction,
            visuals,
            animation_time,
            debug,
            explanation_tooltips,
        } = self;

        visuals.light_dark_radio_buttons(ui);

        crate::Grid::new("_options").show(ui, |ui| {
            ui.label("Override font id:");
            ui.horizontal(|ui| {
                ui.radio_value(override_font_id, None, "None");
                if ui.radio(override_font_id.is_some(), "override").clicked() {
                    *override_font_id = Some(FontId::default());
                }
                if let Some(override_font_id) = override_font_id {
                    crate::introspection::font_id_ui(ui, override_font_id);
                }
            });
            ui.end_row();

            ui.label("Override text style:");
            crate::ComboBox::from_id_source("Override text style")
                .selected_text(match override_text_style {
                    None => "None".to_owned(),
                    Some(override_text_style) => override_text_style.to_string(),
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(override_text_style, None, "None");
                    let all_text_styles = ui.style().text_styles();
                    for style in all_text_styles {
                        let text =
                            crate::RichText::new(style.to_string()).text_style(style.clone());
                        ui.selectable_value(override_text_style, Some(style), text);
                    }
                });
            ui.end_row();

            ui.label("Animation duration:");
            ui.add(
                Slider::new(animation_time, 0.0..=1.0)
                    .clamp_to_range(true)
                    .suffix(" s"),
            );
            ui.end_row();
        });

        ui.collapsing("🔠 Text Styles", |ui| text_styles_ui(ui, text_styles));
        ui.collapsing("📏 Spacing", |ui| spacing.ui(ui));
        ui.collapsing("☝ Interaction", |ui| interaction.ui(ui));
        ui.collapsing("🎨 Visuals", |ui| visuals.ui(ui));
        ui.collapsing("🐛 Debug", |ui| debug.ui(ui));

        ui.checkbox(explanation_tooltips, "Explanation tooltips")
            .on_hover_text(
                "Show explanatory text when hovering DragValue:s and other egui widgets",
            );

        ui.vertical_centered(|ui| reset_button(ui, self));
    }
}

fn text_styles_ui(ui: &mut Ui, text_styles: &mut BTreeMap<TextStyle, FontId>) -> Response {
    ui.vertical(|ui| {
        crate::Grid::new("text_styles").show(ui, |ui| {
            for (text_style, font_id) in text_styles.iter_mut() {
                ui.label(RichText::new(text_style.to_string()).font(font_id.clone()));
                crate::introspection::font_id_ui(ui, font_id);
                ui.end_row();
            }
        });
        crate::reset_button_with(ui, text_styles, default_text_styles());
    })
    .response
}"#;
