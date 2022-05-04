#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, style::Widgets};

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.transparent = true;
    options.vsync = true;

    eframe::run_native(
        "Hello World",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    );
}

struct MyApp {
    text: String,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        MyApp::style(&cc.egui_ctx);

        Self {
            text: String::from("Hello world!"),
        }
    }

    fn style(ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        let mut visuals = style.visuals;
        let dark = Widgets::dark();

        // Set our app to the default dark mode
        visuals.dark_mode = true;
        visuals.widgets = dark;

        style.visuals = visuals;

        ctx.set_style(style);
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(&self.text);
            });
        });
    }
}
