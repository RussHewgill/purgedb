use hex_color::HexColor;

use super::App;


#[derive(Debug, serde::Serialize,serde::Deserialize)]
pub struct NewFilament {
  pub name:           String,
  pub manufacturer:   String,
  // pub material:       String,
  pub color_base:     ([u8; 3], String),
  pub colors:         Vec<([u8; 3], String)>,
}

impl Default for NewFilament {
  fn default() -> Self {
      Self {
        name: String::new(),
        manufacturer: String::new(),
        color_base: ([0, 0, 0], "000000".to_string()),
        colors: vec![],
      }
  }
}

impl NewFilament {
  pub fn new(
    name: &str, 
    manufacturer: &str, 
    // material: &str, 
    color_base: [u8; 3],
    colors: &[[u8; 3]],
    // color2: [u8; 3], 
    // color3: [u8; 3],
  ) -> Self {
    Self { 
      name: name.to_string(),
      manufacturer: manufacturer.to_string(), 
      // material: material.to_string(),
      color_base: (color_base, format!("{:02X}{:02X}{:02X}", color_base[0], color_base[1], color_base[2])),
      // colors: colors.to_vec(),
      colors: colors.iter().map(|c| (*c, format!("{:02X}{:02X}{:02X}", c[0], c[1], c[2]))).collect(),
      // color1,
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

fn color_edit_button(ui: &mut egui::Ui, c: &mut [u8; 3], s: &mut String) {
  if ui.color_edit_button_srgb(c).changed() {
    *s = format!("{:02X}{:02X}{:02X}", c[0], c[1], c[2]);
  }
  let edit = egui::TextEdit::singleline(s)
    .clip_text(false);
  if ui.add(edit).changed() {
    let s2 = format!("#{}", s);
    if let Ok(col) = HexColor::parse(&s2) {
      *c = [col.r, col.g, col.b];
      eprintln!("c = {:?}", c);
    } else {
      eprintln!("can't parse?");
    }
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

        if ui.button("+").clicked() {
          if self.new_filament.colors.len() < 2 {
            self.new_filament.colors.push(([0; 3], String::new()));
          }
        }
        if ui.button("-").clicked() {
          self.new_filament.colors.pop();
        }
        ui.end_row();

        egui::Frame::none()
          .show(ui, |ui| {
            ui.label("Color 1: ");
            // let response_name = ui.add(egui::TextEdit::singleline(&mut self.new_filament.color1));
            // ui.color_edit_button_srgb(&mut self.new_filament.color_base);
            color_edit_button(ui, &mut self.new_filament.color_base.0, &mut self.new_filament.color_base.1);
            // ui.allocate_space(ui.available_size());
          });
        ui.end_row();

        for (i, c) in self.new_filament.colors.iter_mut().enumerate() {
          egui::Frame::none()
            .show(ui, |ui| {
              ui.label(format!("Color {}: ", i + 2));
              // ui.color_edit_button_srgb(c);
              color_edit_button(ui, &mut c.0, &mut c.1);
          });
          ui.end_row();
        }

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
