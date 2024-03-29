use super::{filament_picker::FilamentPicker, App};
use crate::types::Filament;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct EnterPurge {
  // filament1: Filament,
  // filament2: Filament,
  picker1: FilamentPicker,
  picker2: FilamentPicker,
  prev1: Option<Filament>,
  prev2: Option<Filament>,
  purge1: String,
  purge2: String,
}

impl App {
  pub fn show_enter_purge(&mut self, ui: &mut egui::Ui) {
    egui::Frame::none()
      // .stroke(Stroke::new(1.0, egui::Color32::from_gray(65)))
      // .outer_margin(5.)
      // .inner_margin(5.)
      .show(ui, |ui| {
        let filaments = self.db.get_all_filaments().unwrap();
        let resp1 = self
          .enter_purge
          .picker1
          .filament_picker(None, &filaments, ui);
        let resp2 = self
          .enter_purge
          .picker2
          .filament_picker(None, &filaments, ui);

        if self.enter_purge.picker1.selected != self.enter_purge.prev1 {
          self.enter_purge.prev1 = self.enter_purge.picker1.selected.clone();
          self.enter_purge.purge1.clear();
          self.enter_purge.purge2.clear();
        }
        if self.enter_purge.picker2.selected != self.enter_purge.prev2 {
          self.enter_purge.prev2 = self.enter_purge.picker2.selected.clone();
          self.enter_purge.purge1.clear();
          self.enter_purge.purge2.clear();
        }

        // if resp1.changed() || resp2.changed() {
        //   self.enter_purge.purge1.clear();
        //   self.enter_purge.purge2.clear();
        // }

        ui.separator();

        match (
          &self.enter_purge.picker1.selected,
          &self.enter_purge.picker2.selected,
        ) {
          (Some(f1), Some(f2)) => {
            if f1 == f2 {
              return;
            }
            ui.horizontal(|ui| {
              ui.label("From ");
              ui.label(f1.colored_name());
              ui.label("To ");
              ui.label(f2.colored_name());
              match self.db.get_purge_values(f1.id, f2.id) {
                Ok(v) => {
                  ui.visuals_mut().override_text_color = Some(egui::Color32::BLACK);
                  // ui.label(format!("Existing Value: {}", v));
                  ui.label(format!("({})", v));
                }
                _ => {}
              }
              // let edit = egui::TextEdit::singleline(&mut self.enter_purge.purge1).clip_text(true);
              // let resp = ui.add(edit);
              let resp = ui.text_edit_singleline(&mut self.enter_purge.purge1);
            });

            ui.horizontal(|ui| {
              ui.label("From ");
              ui.label(f2.colored_name());
              ui.label("To ");
              ui.label(f1.colored_name());
              match self.db.get_purge_values(f2.id, f1.id) {
                Ok(v) => {
                  ui.visuals_mut().override_text_color = Some(egui::Color32::BLACK);
                  // ui.label(format!("Existing Value: {}", v));
                  ui.label(format!("({})", v));
                }
                _ => {}
              }
              let resp = ui.text_edit_singleline(&mut self.enter_purge.purge2);
            });

            if ui.button("Save Vaules").clicked() {
              if let Ok(p) = self.enter_purge.purge1.parse::<u32>() {
                self.db.set_purge_values(f1.id, f2.id, p).unwrap();
              }
              if let Ok(p) = self.enter_purge.purge2.parse::<u32>() {
                self.db.set_purge_values(f2.id, f1.id, p).unwrap();
              }
            }
          }
          _ => {}
        }
      });
  }
}
