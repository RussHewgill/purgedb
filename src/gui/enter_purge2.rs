use super::{App, filament_grid::FilamentGridData};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct EnterPurge {
    data: FilamentGridData,
}

impl App {
    pub fn show_enter_purge2(&mut self, ui: &mut egui::Ui) {
        let filaments = self.db.get_all_filaments().unwrap();
        self.update_filtered_filaments(&filaments.0);

        egui::Frame::new()
            .outer_margin(5.)
            .inner_margin(5.)
            .show(ui, |ui| {});
    }
}
