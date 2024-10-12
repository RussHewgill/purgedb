use tracing::{debug, error, info, trace, warn};

use super::App;

impl App {
    pub fn show_options_tab(&mut self, ui: &mut egui::Ui) {
        egui::widgets::global_theme_preference_buttons(ui);
    }
}
