use super::App;


#[derive(Debug, Default)]
pub struct NewFilament {
  pub name:           String,
  pub manufacturer:   String,
  pub color:          String,
  pub material:       String,
}

impl NewFilament {
  pub fn new(name: &str, manufacturer: &str, color: &str, material: &str) -> Self { 
    Self { 
      name: name.to_string(),
      manufacturer: manufacturer.to_string(), 
      color: color.to_string(),
      material: color.to_string(),
    }
  }

  pub fn not_empty(&self) -> bool {
    self.name != "" 
      && self.manufacturer != "" 
      && self.color != "" 
      && self.material != "" 
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
        let response_name = ui.add(egui::TextEdit::singleline(&mut self.new_filament.name));
        ui.end_row();

        ui.label("Manufacturer: ");
        let response_name = ui.add(egui::TextEdit::singleline(&mut self.new_filament.manufacturer));
        ui.end_row();

        ui.label("Color: ");
        let response_name = ui.add(egui::TextEdit::singleline(&mut self.new_filament.color));
        ui.end_row();

        ui.label("Material: ");
        let response_name = ui.add(egui::TextEdit::singleline(&mut self.new_filament.material));
        ui.end_row();
      });

      if ui.add(egui::Button::new("Add New Filament")).clicked() {
        if self.new_filament.not_empty() {
          eprintln!("adding filament TODO: {:?}", &self.new_filament);
          self.db.add_filament(&self.new_filament).unwrap();
        } else {
          eprintln!("missing fields");
        }
      }

    });
  }
}
