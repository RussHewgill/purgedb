use super::App;


#[derive(Debug, Default)]
pub struct NewFilament {
  pub name:           String,
  pub manufacturer:   String,
  // pub material:       String,
  pub color1:         [u8; 3],
  // pub color2:         [u8; 3],
  // pub color3:         [u8; 3],
}

impl NewFilament {
  pub fn new(
    name: &str, 
    manufacturer: &str, 
    // material: &str, 
    color1: [u8; 3], 
    // color2: [u8; 3], 
    // color3: [u8; 3],
  ) -> Self { 
    Self { 
      name: name.to_string(),
      manufacturer: manufacturer.to_string(), 
      // material: material.to_string(),
      color1,
      // color2,
      // color3,
    }
  }

  pub fn not_empty(&self) -> bool {
    self.name != ""
      && self.manufacturer != ""
      // && self.material != ""
      // && self.color1 != ""
  }
}

impl App {
  pub fn show_new_filament(&mut self, ui: &mut egui::Ui) {
    egui::Frame::none()
    .show(ui, |ui| {
      ui.label("new filament");

      egui::Grid::new("my_grid")
      .num_columns(2)
      .spacing([40.0, 4.0])
      .striped(false)
      .show(ui, |ui| {
        ui.label("Name: ");
        ui.add(egui::TextEdit::singleline(&mut self.new_filament.name));
        ui.end_row();

        ui.label("Manufacturer: ");
        ui.add(egui::TextEdit::singleline(&mut self.new_filament.manufacturer));
        ui.end_row();

        ui.label("Color 1: ");
        // let response_name = ui.add(egui::TextEdit::singleline(&mut self.new_filament.color1));
        ui.color_edit_button_srgb(&mut self.new_filament.color1);
        ui.end_row();

        // ui.label("Color 2: ");
        // let response_name = ui.add(egui::TextEdit::singleline(&mut self.new_filament.color2));
        // ui.end_row();

        // ui.label("Color 3: ");
        // let response_name = ui.add(egui::TextEdit::singleline(&mut self.new_filament.color3));
        // ui.end_row();

        // ui.label("Material: ");
        // let response_name = ui.add(egui::TextEdit::singleline(&mut self.new_filament.material));
        // ui.end_row();
      });

      if ui.add(egui::Button::new("Add New Filament")).clicked() {
        if self.new_filament.not_empty() {
          // eprintln!("adding filament TODO: {:?}", &self.new_filament);
          self.db.add_filament(&self.new_filament).unwrap();
        } else {
          eprintln!("missing fields");
        }
      }

    });
  }
}
