use std::sync::Arc;

use druid::{text::RichText, Data, Lens};

use crate::widgets::MenuData;

#[derive(Data, Clone, Lens)]
pub struct AppState {
    pub rendered: RichText,
    pub menu: Arc<MenuData>,
    pub source: Arc<String>,
}
