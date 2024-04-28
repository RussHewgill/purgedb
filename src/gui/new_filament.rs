use egui::RichText;
use egui_extras::{Column, TableBuilder};
use hex_color::HexColor;

use crate::types::Material;

use super::{filament_picker::FilamentPicker, text_val::ValText, App};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct NewFilament {
    row_id: Option<u32>,
    pub name: String,
    pub manufacturer: String,
    pub color_base: ([u8; 3], String),
    pub colors: Vec<([u8; 3], String)>,
    pub material: String,
    // pub material: Option<Material>,
    pub notes: String,
    // picker: FilamentPicker,
    selected: Option<u32>,
    #[serde(skip)]
    confirm_delete: Option<u32>,
}

impl Default for NewFilament {
    fn default() -> Self {
        Self {
            row_id: None,
            name: String::new(),
            manufacturer: String::new(),
            color_base: ([0, 0, 0], "000000".to_string()),
            colors: vec![],
            material: String::new(),
            notes: String::new(),
            // picker: FilamentPicker::default(),
            selected: None,
            confirm_delete: None,
        }
    }
}

fn color_to_bytes(c: HexColor) -> ([u8; 3], String) {
    (
        [c.r, c.g, c.b],
        format!("{:02X}{:02X}{:02X}", c.r, c.g, c.b),
    )
}

impl NewFilament {
    pub fn new(
        name: &str,
        manufacturer: &str,
        // material: &str,
        color_base: [u8; 3],
        colors: &[[u8; 3]],
    ) -> Self {
        Self {
            name: name.to_string(),
            manufacturer: manufacturer.to_string(),
            // material: material.to_string(),
            color_base: (
                color_base,
                format!(
                    "{:02X}{:02X}{:02X}",
                    color_base[0], color_base[1], color_base[2]
                ),
            ),
            colors: colors
                .iter()
                .map(|c| (*c, format!("{:02X}{:02X}{:02X}", c[0], c[1], c[2])))
                .collect(),
            ..Default::default()
        }
    }

    pub fn clear(&mut self) {
        // let p = self.picker.clone();
        let p = self.selected;
        *self = Self::default();
        // self.picker = p;
        self.selected = p;
    }

    pub fn not_empty(&self) -> bool {
        self.name != "" && self.manufacturer != ""
        // && self.material != ""
        // && self.color1 != ""
    }
}

fn color_edit_button(ui: &mut egui::Ui, c: &mut [u8; 3], s: &mut String) {
    if ui.color_edit_button_srgb(c).changed() {
        *s = format!("{:02X}{:02X}{:02X}", c[0], c[1], c[2]);
    }
    let edit = egui::TextEdit::singleline(s).clip_text(false);
    if ui.add(edit).changed() {
        let s2 = format!("#{}", s);
        if let Ok(col) = HexColor::parse(&s2) {
            *c = [col.r, col.g, col.b];
            // eprintln!("c = {:?}", c);
        } else {
            eprintln!("can't parse?");
        }
    }
}

impl App {
    // pub fn show_new_filament(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
    pub fn show_new_filament(&mut self, ctx: &egui::Context) {
        let filaments = self.db.get_all_filaments().unwrap();

        egui::panel::SidePanel::right("Filament Picker Panel")
            .min_width(400.)
            .show(ctx, |ui| {
                // let mut table = TableBuilder::new(ui)
                //   .striped(true)
                //   .resizable(true)
                //   .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                //   .column(Column::auto())
                //   .column(Column::initial(100.0).range(40.0..=300.0))
                //   .column(Column::initial(100.0).at_least(40.0).clip(true))
                //   .column(Column::remainder())
                //   .min_scrolled_height(0.0);

                // table
                //   .header(20.0, |mut header| {
                //     header.col(|ui| {});
                //   })
                //   .body(|mut body| {
                //     for (f_id, f) in filaments.iter().enumerate() {
                //       body.row(18., |mut row| {
                //         row.col(|ui| {
                //           ui.label(f.colored_name());
                //         });
                //       });
                //     }
                //   });

                egui::Frame::none().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Load Filament").clicked() {
                            if let Some(id) = &self.new_filament.selected {
                                if let Ok(f) = self.db.get_filament(*id) {
                                    self.new_filament.row_id = Some(f.id);
                                    self.new_filament.name = f.name.clone();
                                    self.new_filament.manufacturer = f.manufacturer.clone();
                                    self.new_filament.material = f.material.clone();
                                    self.new_filament.notes = f.notes.clone();

                                    self.new_filament.color_base = color_to_bytes(f.color_base);

                                    self.new_filament.colors.clear();
                                    for c in f.colors.iter() {
                                        let c = color_to_bytes(*c);
                                        self.new_filament.colors.push(c);
                                    }
                                }
                            }
                        }
                        if ui.button("Load ID Only").clicked() {
                            if let Some(id) = self.new_filament.selected {
                                if let Ok(f) = self.db.get_filament(id) {
                                    self.new_filament.row_id = Some(f.id);
                                }
                            }
                        }
                    });

                    let button = if self.new_filament.confirm_delete.is_some() {
                        let but = egui::Button::new(
                            RichText::new("CONFIRM DELETE?").color(egui::Color32::RED),
                        );
                        ui.add(but)
                    } else {
                        ui.button("Delete Filament")
                    };

                    if self.new_filament.confirm_delete.is_some() && !button.hovered() {
                        self.new_filament.confirm_delete = None;
                    }
                    if button.clicked() {
                        if let Some(sel_id) = self.new_filament.selected {
                            if let Some(del_id) = self.new_filament.confirm_delete {
                                if del_id == sel_id {
                                    self.db.delete_filament(del_id).unwrap();
                                }
                            } else {
                                self.new_filament.confirm_delete = Some(sel_id);
                            }
                        }
                    }
                });

                ui.horizontal(|ui| {
                    if ui.button("Set as default White").clicked() {
                        if let Some(id) = self.new_filament.selected {
                            self.default_white = id;
                        }
                    }
                    if ui.button("Set as default black").clicked() {
                        if let Some(id) = self.new_filament.selected {
                            self.default_black = id;
                        }
                    }
                });

                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    // egui::Frame::none().show(ui, |ui| {
                    // for (f_id, (_, f)) in filaments.iter().enumerate() {
                    for (f_id, f) in filaments.1.iter().enumerate() {
                        ui.selectable_value(
                            &mut self.new_filament.selected,
                            Some(f.id),
                            f.colored_name(),
                        );
                    }
                    // });
                    ui.allocate_space(ui.available_size());
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // egui::Frame::none().show(ui, |ui| {
            // self.new_filament.picker.filament_picker(&filaments, ui);

            // ui.horizontal(|ui| {
            //   if ui.button("Load Filament").clicked() {
            //     if let Some(f) = &self.new_filament.picker.selected {
            //       self.new_filament.row_id = Some(f.id);
            //       self.new_filament.name = f.name.clone();
            //       self.new_filament.manufacturer = f.manufacturer.clone();
            //       self.new_filament.material = f.material.clone();
            //       self.new_filament.notes = f.notes.clone();

            //       self.new_filament.color_base = color_to_bytes(f.color_base);

            //       self.new_filament.colors.clear();
            //       for c in f.colors.iter() {
            //         let c = color_to_bytes(*c);
            //         self.new_filament.colors.push(c);
            //       }
            //     }
            //   }
            //   if ui.button("Load ID Only").clicked() {
            //     if let Some(f) = &self.new_filament.picker.selected {
            //       self.new_filament.row_id = Some(f.id);
            //     }
            //   }
            // });

            // let button = if self.new_filament.confirm_delete.is_some() {
            //   let but = egui::Button::new(RichText::new("CONFIRM DELETE?").color(egui::Color32::RED));
            //   ui.add(but)
            // } else {
            //   ui.button("Delete Filament")
            // };

            // if self.new_filament.confirm_delete.is_some() && !button.hovered() {
            //   self.new_filament.confirm_delete = None;
            // }
            // if button.clicked() {
            //   if let Some(f) = &self.new_filament.picker.selected {
            //     if let Some(id) = self.new_filament.confirm_delete {
            //       if id == f.id {
            //         self.db.delete_filament(id).unwrap();
            //       }
            //     } else {
            //       self.new_filament.confirm_delete = Some(f.id);
            //     }
            //   }
            // }

            ui.separator();

            if ui.button("Clear").clicked() {
                self.new_filament.clear();
            }

            egui::Grid::new("New Filament Grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(false)
                .show(ui, |ui| {
                    ui.label("Row ID (blank for new filament): ");
                    egui::Frame::none().show(ui, |ui| {
                        if let Some(id) = self.new_filament.row_id {
                            ui.label(format!("{}", id));
                            if ui.button("clear id").clicked() {
                                self.new_filament.row_id = None;
                            }
                        }
                    });
                    ui.end_row();

                    ui.label("Name: ");
                    ui.add(egui::TextEdit::singleline(&mut self.new_filament.name));
                    ui.end_row();

                    ui.label("Manufacturer: ");
                    ui.add(egui::TextEdit::singleline(
                        &mut self.new_filament.manufacturer,
                    ));
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

                    egui::Frame::none().show(ui, |ui| {
                        ui.label("Color 1: ");
                        color_edit_button(
                            ui,
                            &mut self.new_filament.color_base.0,
                            &mut self.new_filament.color_base.1,
                        );
                    });
                    ui.end_row();

                    for (i, c) in self.new_filament.colors.iter_mut().enumerate() {
                        egui::Frame::none().show(ui, |ui| {
                            ui.label(format!("Color {}: ", i + 2));
                            color_edit_button(ui, &mut c.0, &mut c.1);
                        });
                        ui.end_row();
                    }

                    ui.label("Material: ");
                    ui.end_row();

                    ui.label("Notes:");
                    ui.end_row();
                    ui.text_edit_multiline(&mut self.new_filament.notes);
                    //
                });

            let s = if let Some(id) = self.new_filament.row_id {
                "Update Existing Filament"
            } else {
                "Add New Filament"
            };

            if ui.add(egui::Button::new(s)).clicked() {
                if self.new_filament.not_empty() {
                    self.db
                        .add_filament(&self.new_filament, self.new_filament.row_id)
                        .unwrap();
                    ui.label(format!("Added Filament: {}", &self.new_filament.name));
                } else {
                    eprintln!("missing fields");
                }
            }
        });
    }

    //
}
