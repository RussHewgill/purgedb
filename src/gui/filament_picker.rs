use egui::{text::LayoutJob, Color32, FontFamily, FontId, Response, TextFormat};

use crate::types::Filament;

use super::App;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FilamentPicker {
  id: u32,
  pub selected: Option<Filament>,
}

impl Default for FilamentPicker {
  fn default() -> Self {
    Self {
      id: rand::random::<u32>(),
      selected: None,
    }
  }
}

impl FilamentPicker {
  pub fn reset(&mut self) {
    self.selected = None;
  }

  pub fn filament_picker(&mut self, filaments: &[Filament], ui: &mut egui::Ui) -> Response {
    // pub fn filament_picker(&mut self, filaments: &[(u32, String)], ui: &mut egui::Ui) {

    // let response = egui::ComboBox::from_label("Select Filament")
    let response = egui::ComboBox::from_id_source(self.id)
      // .width(400.)
      .width(ui.available_width())
      .selected_text(match &self.selected {
        Some(f) => f.colored_name(),
        None => LayoutJob::default(),
      })
      .show_ui(ui, |ui| {
        for f in filaments.iter() {
          // let w = format!("{} {}", &f.name, &f.display_color());
          ui.selectable_value(&mut self.selected, Some(f.clone()), f.colored_name());
        }
      });

    response.response
  }
}
