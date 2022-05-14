use std::sync::Arc;

use druid::{text::RichText, Data, Lens};

#[derive(Debug, Data, Clone, Lens)]
pub struct AppState {
    pub rendered: RichText,
    pub source: Arc<String>,
}
