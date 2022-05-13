use eframe::epaint::{Color32, Stroke};

#[derive(Debug, Clone, Copy)]
pub enum Theme {
    Dark,
}

impl Theme {
    pub fn colors(&self) -> ThemeColors {
        match self {
            Self::Dark => ThemeColors {
                is_dark: true,
                source_bg_color: Color32::from_rgb(45, 45, 45),
                source_text_color: Color32::from_rgb(200, 200, 200),
                render_bg_color: Color32::from_rgb(53, 53, 53),
                menu_color: Color32::from_rgb(40, 40, 40),
                button_colors: ButtonColors {
                    inactive_color: Color32::from_rgb(60, 60, 60),
                    inactive_stroke: Stroke::new(1.0, Color32::from_rgba_premultiplied(0, 0, 0, 0)),
                    hovered_color: Color32::from_rgb(40, 40, 40),
                    hovered_stroke: Stroke::new(1.0, Color32::from_rgb(150, 150, 150)),
                    active_color: Color32::from_rgb(40, 40, 40),
                    active_stroke: Stroke::new(1.0, Color32::from_rgb(255, 255, 255)),
                    text_color: Color32::from_rgb(230, 230, 230),
                },
                scroll_colors: ScrollColors {
                    inactive_color: Color32::from_rgb(140, 140, 139),
                    hovered_color: Color32::from_rgb(170, 170, 168),
                    active_color: Color32::from_rgb(31, 94, 171),
                },
                divider_color: Color32::from_rgb(7, 7, 7),
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ButtonColors {
    pub inactive_color: Color32,
    pub inactive_stroke: Stroke,
    pub hovered_color: Color32,
    pub hovered_stroke: Stroke,
    pub active_color: Color32,
    pub active_stroke: Stroke,
    pub text_color: Color32,
}

#[derive(Debug, Clone, Copy)]
pub struct ScrollColors {
    pub inactive_color: Color32,
    pub hovered_color: Color32,
    pub active_color: Color32,
}

#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
    pub is_dark: bool,
    pub source_bg_color: Color32,
    pub source_text_color: Color32,
    pub render_bg_color: Color32,
    pub menu_color: Color32,
    pub button_colors: ButtonColors,
    pub scroll_colors: ScrollColors,
    pub divider_color: Color32,
}
