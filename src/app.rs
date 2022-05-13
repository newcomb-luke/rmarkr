use crate::{
    theme::{Theme, ThemeColors},
    TEST_SRC,
};

use eframe::{self, egui::style::Margin, emath::Vec2, epaint::Stroke};
use eframe::{
    egui::{self, TextStyle},
    epaint::{FontFamily, FontId},
};

pub struct RMarkrApp {
    source: String,
    theme: Theme,
    colors: ThemeColors,
}

impl RMarkrApp {
    pub fn new(cc: &eframe::CreationContext<'_>, theme: Theme) -> Self {
        let mut app = Self {
            source: String::from(TEST_SRC),
            theme,
            colors: theme.colors(),
        };

        app.apply_theme(&cc.egui_ctx, theme);
        Self::style(&cc.egui_ctx);

        app
    }

    fn style_menu(&mut self, ui: &mut egui::Ui) {
        let mut style = (**ui.style()).clone();
        let mut visuals = style.visuals;
        let mut widgets = visuals.widgets;
        let mut spacing = style.spacing;

        let text_stroke = Stroke::new(1.0, self.colors.button_colors.text_color);

        widgets.hovered.fg_stroke = text_stroke;
        widgets.inactive.fg_stroke = text_stroke;
        widgets.active.fg_stroke = text_stroke;
        widgets.open.fg_stroke = text_stroke;

        widgets.inactive.bg_fill = self.colors.button_colors.inactive_color;
        widgets.active.bg_fill = self.colors.button_colors.active_color;
        widgets.hovered.bg_fill = self.colors.button_colors.hovered_color;
        widgets.open.bg_fill = egui::Color32::WHITE;
        widgets.inactive.bg_stroke = self.colors.button_colors.inactive_stroke;
        widgets.active.bg_stroke = self.colors.button_colors.active_stroke;
        widgets.hovered.bg_stroke = self.colors.button_colors.hovered_stroke;

        spacing.button_padding = Vec2::new(16.0, 6.0);

        visuals.widgets = widgets;

        style.visuals = visuals;
        style.spacing = spacing;

        ui.set_style(style);
    }

    fn style_scroll(&mut self, ui: &mut egui::Ui) {
        let mut style = (**ui.style()).clone();
        let mut visuals = style.visuals;
        let mut widgets = visuals.widgets;

        let text_stroke = Stroke::new(1.0, self.colors.source_text_color);

        widgets.hovered.fg_stroke = text_stroke;
        widgets.inactive.fg_stroke = text_stroke;
        widgets.active.fg_stroke = text_stroke;
        widgets.open.fg_stroke = text_stroke;
        widgets.inactive.bg_fill = self.colors.scroll_colors.inactive_color;
        widgets.active.bg_fill = self.colors.scroll_colors.active_color;
        widgets.hovered.bg_fill = self.colors.scroll_colors.hovered_color;

        visuals.widgets = widgets;

        style.visuals = visuals;

        ui.set_style(style);
    }

    fn apply_theme(&mut self, ctx: &egui::Context, theme: Theme) {
        let mut style = (*ctx.style()).clone();
        let mut visuals = style.visuals;
        let mut widgets = visuals.widgets;

        self.colors = theme.colors();
        self.theme = theme;

        let text_stroke = Stroke::new(1.0, self.colors.source_text_color);
        let mut frame_stroke = widgets.noninteractive.bg_stroke;
        frame_stroke.color = self.colors.menu_color;

        widgets.hovered.fg_stroke = text_stroke;
        widgets.inactive.fg_stroke = text_stroke;
        widgets.active.fg_stroke = text_stroke;
        widgets.open.fg_stroke = text_stroke;
        widgets.noninteractive.fg_stroke = text_stroke;
        widgets.noninteractive.bg_fill = self.colors.menu_color;
        widgets.noninteractive.bg_stroke = frame_stroke;

        visuals.dark_mode = self.colors.is_dark;
        visuals.extreme_bg_color = self.colors.source_bg_color;
        visuals.selection.stroke = text_stroke;

        visuals.widgets = widgets;

        style.visuals = visuals;

        ctx.set_style(style);
    }

    fn style(ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();

        // Set various text styles. This cranks up the font size a bit
        style.text_styles = [
            (
                TextStyle::Heading,
                FontId::new(30.0, FontFamily::Proportional),
            ),
            (TextStyle::Body, FontId::new(18.0, FontFamily::Proportional)),
            (
                TextStyle::Monospace,
                FontId::new(20.0, FontFamily::Proportional),
            ),
            (
                TextStyle::Small,
                FontId::new(10.0, FontFamily::Proportional),
            ),
            (
                TextStyle::Button,
                FontId::new(18.0, FontFamily::Proportional),
            ),
        ]
        .into();

        ctx.set_style(style);
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            self.style_menu(ui);

            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    ui.add_space(20.0);

                    if ui.button("New").clicked() {
                        println!("New clicked");
                    }

                    ui.add_space(20.0);

                    if ui.button("Open").clicked() {
                        println!("Open clicked");
                    }
                });
            });
        });

        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(self.colors.menu_color) // This shouldn't ever actually show
                    .stroke(Stroke::none())
                    .inner_margin(Margin::same(0.0)),
            )
            .show(ctx, |ui| {
                self.style_scroll(ui);

                // Set 0 spacing so that our source and render panels aren't padded in
                ui.spacing_mut().item_spacing = Vec2::splat(0.0);

                ui.columns(2, |columns| {
                    self.source_ui(&mut columns[0]);
                    egui::ScrollArea::vertical()
                        .id_source("scroll_rendered")
                        .show(&mut columns[1], |ui| {
                            self.rendered_ui(ui);
                        });
                });
            });
    }

    fn rendered_ui(&mut self, ui: &mut egui::Ui) {
        egui::Frame::none()
            .fill(self.colors.render_bg_color)
            .stroke(Stroke::none())
            .inner_margin(Margin::symmetric(40.0, 40.0))
            .show(ui, |ui| {
                let mut style = (*ui.ctx().style()).clone();

                style.ui(ui);

                ui.ctx().set_style(style);

                // ui.add_sized(ui.available_size(), egui::Label::new(&self.source));
            });
    }

    fn source_ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .id_source("scroll_source")
            .always_show_scroll(true)
            .show(ui, |ui| {
                egui::Frame::none()
                    .fill(self.colors.source_bg_color)
                    .stroke(Stroke::none())
                    .show(ui, |ui| {
                        ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::multiline(&mut self.source)
                                .frame(false)
                                .text_color(self.colors.source_text_color)
                                .margin(Vec2::new(40.0, 80.0))
                                .desired_width(f32::INFINITY)
                                .font(egui::TextStyle::Monospace)
                                .id_source("source")
                                .code_editor(),
                        );
                    });
            });
    }
}

impl eframe::App for RMarkrApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ui(ctx);
    }
}
