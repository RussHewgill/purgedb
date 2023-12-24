use egui::{Response, Color32, FontId, FontFamily, TextFormat, text::LayoutJob};

use crate::types::Filament;

use super::App;

#[derive(Debug, Default)]
pub struct FilamentPicker {
  pub selected: Option<Filament>,
}

impl FilamentPicker {
  pub fn filament_picker(&mut self, filaments: &[Filament], ui: &mut egui::Ui) -> Response {
  // pub fn filament_picker(&mut self, filaments: &[(u32, String)], ui: &mut egui::Ui) {

    // let response = egui::ComboBox::from_label("Select Filament")
    let response = egui::ComboBox::from_id_source(0)
      // .width(400.)
      .width(ui.available_width())
      .selected_text(match &self.selected {
        Some(f) => &f.name,
        None => "None",
    })
      .show_ui(ui, |ui| {
        for f in filaments.iter() {



          // let w = format!("{} {}", &f.name, &f.display_color());
          ui.selectable_value(&mut self.selected, Some(f.clone()), f.colored_name());
        }
      }
    );

    response.response
  }
}
