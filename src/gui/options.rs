use anyhow::{anyhow, bail, ensure, Context, Result};
use tracing::{debug, error, info, trace, warn};

use super::App;

impl App {
    pub fn show_options_tab(&mut self, ui: &mut egui::Ui) {
        egui::widgets::global_theme_preference_buttons(ui);

        if ui.button("Export settings").clicked() {
            if let Err(e) = self.settings_export(ui) {
                error!("Error exporting settings: {:?}", e);
            }
        }
    }
}

impl App {
    pub fn settings_export(&mut self, ui: &mut egui::Ui) -> Result<()> {
        debug!("Export settings");

        if let Some(path) = rfd::FileDialog::new()
            .set_file_name("export_purgedb.sqlite")
            .save_file()
        {
            debug!("Exporting settings to {:?}", path);

            let mut output_db = rusqlite::Connection::open(path)?;

            rusqlite::backup::Backup::new(&self.db.db, &mut output_db)
                .context("Error creating backup")?
                .run_to_completion(5, std::time::Duration::from_millis(250), None)
                .context("Error running backup")?;
            if let Err(e) = output_db.close() {
                error!("Error closing output db: {:?}", e);
            }
        }

        Ok(())
    }
}
