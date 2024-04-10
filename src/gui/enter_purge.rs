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
                ui.horizontal(|ui| {
                    if ui.button("Black").clicked() {
                        let f = self.db.get_filament(2).unwrap();
                        // *self.enter_purge.picker1.selected_mut() = Some(f);
                        self.enter_purge.picker1.set_selected(Some(f));
                    }
                    if ui.button("White").clicked() {
                        let f = self.db.get_filament(1).unwrap();
                        // *self.enter_purge.picker1.selected_mut() = Some(f);
                        self.enter_purge.picker1.set_selected(Some(f));
                    }
                });

                ui.separator();

                let filaments = self.db.get_all_filaments().unwrap();

                ui.horizontal(|ui| {
                    if ui.button("Swap").clicked() {
                        std::mem::swap(
                            &mut self.enter_purge.picker1,
                            &mut self.enter_purge.picker2,
                        );
                    }
                    ui.vertical(|ui| {
                        let resp1 = self.enter_purge.picker1.filament_picker(
                            None,
                            &filaments.0,
                            &filaments.1,
                            ui,
                        );
                        let resp2 = self.enter_purge.picker2.filament_picker(
                            None,
                            &filaments.0,
                            &filaments.1,
                            ui,
                        );
                    });
                });

                if self.enter_purge.picker1.selected() != self.enter_purge.prev1.as_ref() {
                    self.enter_purge.prev1 = self.enter_purge.picker1.selected().cloned();
                    self.enter_purge.purge1.clear();
                    self.enter_purge.purge2.clear();
                }
                if self.enter_purge.picker2.selected() != self.enter_purge.prev2.as_ref() {
                    self.enter_purge.prev2 = self.enter_purge.picker2.selected().cloned();
                    self.enter_purge.purge1.clear();
                    self.enter_purge.purge2.clear();
                }

                // if resp1.changed() || resp2.changed() {
                //   self.enter_purge.purge1.clear();
                //   self.enter_purge.purge2.clear();
                // }

                ui.separator();

                match (
                    &self.enter_purge.picker1.selected(),
                    &self.enter_purge.picker2.selected(),
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
                                    ui.visuals_mut().override_text_color =
                                        Some(egui::Color32::BLACK);
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
                                    ui.visuals_mut().override_text_color =
                                        Some(egui::Color32::BLACK);
                                    // ui.label(format!("Existing Value: {}", v));
                                    ui.label(format!("({})", v));
                                }
                                _ => {}
                            }
                            let resp = ui.text_edit_singleline(&mut self.enter_purge.purge2);
                        });

                        if ui.button("Save Vaules").clicked() {
                            if let Ok(p) = self.enter_purge.purge1.parse::<u32>() {
                                let p = match (
                                    self.filament_grid.use_multiplier(),
                                    self.filament_grid.use_offset(),
                                ) {
                                    (true, true) => p,
                                    (true, false) => {
                                        (p as f32 * self.filament_grid.multiplier()) as u32
                                    }
                                    (false, true) => p - self.filament_grid.offset(),
                                    (false, false) => p,
                                };
                                self.db.set_purge_values(f1.id, f2.id, p).unwrap();
                            }
                            if let Ok(p) = self.enter_purge.purge2.parse::<u32>() {
                                let p = match (
                                    self.filament_grid.use_multiplier(),
                                    self.filament_grid.use_offset(),
                                ) {
                                    (true, true) => p,
                                    (true, false) => {
                                        (p as f32 * self.filament_grid.multiplier()) as u32
                                    }
                                    (false, true) => p - self.filament_grid.offset(),
                                    (false, false) => p,
                                };
                                self.db.set_purge_values(f2.id, f1.id, p).unwrap();
                            }
                        }
                    }
                    _ => {}
                }

                ui.separator();
                ui.horizontal(|ui| {
                    ui.checkbox(self.filament_grid.use_multiplier_mut(), "Use multiplier");
                    let drag = egui::DragValue::new(self.filament_grid.multiplier_mut())
                        .update_while_editing(false)
                        .max_decimals(3);
                    ui.add(drag);
                });
                ui.horizontal(|ui| {
                    ui.checkbox(self.filament_grid.use_offset_mut(), "Use offset");
                    let drag = egui::DragValue::new(self.filament_grid.offset_mut())
                        .update_while_editing(false)
                        .max_decimals(0);
                    ui.add(drag);
                });
            });
    }
}
