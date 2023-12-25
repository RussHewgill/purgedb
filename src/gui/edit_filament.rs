use super::{filament_picker::FilamentPicker, new_filament::NewFilament, App};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct EditFilament {
  picker: FilamentPicker,
  filament: NewFilament,
}

impl App {
  pub fn show_edit_filament(&mut self, ui: &mut egui::Ui) {
    egui::Frame::none().show(ui, |ui| {
      let filaments = self.db.get_all_filaments().unwrap();
      if self
        .edit_filament
        .picker
        .filament_picker(&filaments, ui)
        .changed()
      {
        if let Some(f) = &self.edit_filament.picker.selected {
          //
        }
      }
    });
  }
}
