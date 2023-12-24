use crate::types::Filament;
use super::{filament_picker::FilamentPicker, App};

#[derive(Debug, Default)]
pub struct EnterPurge {
  // filament1: Filament,
  // filament2: Filament,
  picker1: FilamentPicker,
  picker2: FilamentPicker,
  purge1: String,
  purge2: String,
}

impl App {
  pub fn show_enter_purge(&mut self, ui: &mut egui::Ui) {
    egui::Frame::none()
    // .stroke(Stroke::new(1.0, egui::Color32::from_gray(65)))
    .outer_margin(5.)
    .inner_margin(5.)
    .show(ui, |ui| {
      let filaments = self.db.get_all_filaments().unwrap();
      self.enter_purge.picker1.filament_picker(&filaments, ui);
      self.enter_purge.picker2.filament_picker(&filaments, ui);

      ui.separator();

      match (&self.enter_purge.picker1.selected, &self.enter_purge.picker2.selected) {
        (Some(f1), Some(f2)) => {
          if f1 == f2 {
            return;
          }
          ui.horizontal(|ui| {
            ui.label("From ");
            ui.label(f1.colored_name());
            ui.label("To ");
            ui.label(f2.colored_name());
            let resp = ui.text_edit_singleline(&mut self.enter_purge.purge1);
          });

          ui.horizontal(|ui| {
            ui.label("From ");
            ui.label(f2.colored_name());
            ui.label("To ");
            ui.label(f1.colored_name());
            let resp = ui.text_edit_singleline(&mut self.enter_purge.purge2);
          });

          if ui.button("Save Vaules").clicked() {

            match (self.enter_purge.purge1.parse::<u32>(), self.enter_purge.purge2.parse::<u32>()) {
              (Ok(purge1), Ok(purge2)) => {
                self.db.set_purge_values(f1.id, f2.id, purge1).unwrap();
                self.db.set_purge_values(f2.id, f1.id, purge2).unwrap();
              }
              _ => {}
            }
          }
        },
        _ => {},
      }
    });
  }
}
