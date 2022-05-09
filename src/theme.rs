use eframe::epaint::Color32;

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
    pub scroll_colors: ScrollColors,
    pub divider_color: Color32,
}
