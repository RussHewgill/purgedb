use std::collections::{HashMap, HashSet};

use egui_extras::{Column, TableBuilder};

use crate::types::{Filament, FilamentMap};

use super::{filament_picker::FilamentPicker, history_tab::SortOrder, App};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ListCalibrations {
    picker: FilamentPicker,
    sort: Option<(usize, SortOrder)>,
    last_selected_filament: Option<u32>,
    filament_table: Option<(u32, Vec<(u32, (u32, u32))>)>,
}

impl App {
    pub fn show_calibration_list(&mut self, ui: &mut egui::Ui) {
        egui::Frame::none().show(ui, |ui| {
            let filaments = self.db.get_all_filaments().unwrap();
            self.update_filtered_filaments(&filaments.0);

            let mut filament_changed = false;

            ui.horizontal(|ui| {
                let resp = self.calibration_list.picker.filament_picker(
                    None,
                    &filaments.0,
                    &filaments.1,
                    self.nucleo.as_ref().unwrap().snapshot(),
                    !self.filament_filter.is_empty(),
                    ui,
                );

                // Check if selected filament has changed
                let current_selection: Option<u32> = self.calibration_list.picker.filament_id();
                if current_selection != self.calibration_list.last_selected_filament {
                    self.calibration_list.last_selected_filament = current_selection;
                    filament_changed = true;
                }
            });
            ui.separator();

            if filament_changed {
                self.update_calibration_table_data();
            }

            self.show_calibration_list_table(ui, &filaments.0);
        });
    }

    fn update_calibration_table_data(&mut self) {
        let Some(id) = self.calibration_list.last_selected_filament else {
            return;
        };
        let table = match self.db.get_all_purge_for_filament(id) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Error fetching purge data: {}", e);
                return;
            }
        };

        self.calibration_list.filament_table = Some((id, table));
    }

    fn show_calibration_list_table(&mut self, ui: &mut egui::Ui, filament_map: &FilamentMap) {
        let row_height = 20.;

        let mut sort_changed = false;

        TableBuilder::new(ui)
            .striped(true)
            .column(Column::exact(400.)) // filament
            .column(Column::exact(100.)) // from
            .column(Column::exact(100.)) // to
            .header(30., |mut header| {
                header.col(|ui| {
                    if ui.heading("Timestamp").clicked() {
                        match self.list_sort {
                            Some((0, SortOrder::Ascending)) => {
                                self.list_sort = Some((0, SortOrder::Descending));
                                sort_changed = true;
                            }
                            _ => {
                                self.list_sort = Some((0, SortOrder::Ascending));
                                sort_changed = true;
                            }
                        }
                    }
                });
                header.col(|ui| {
                    if ui.heading("From").clicked() {
                        match self.list_sort {
                            Some((1, SortOrder::Ascending)) => {
                                self.list_sort = Some((1, SortOrder::Descending));
                                sort_changed = true;
                            }
                            _ => {
                                self.list_sort = Some((1, SortOrder::Ascending));
                                sort_changed = true;
                            }
                        }
                    }
                });
                header.col(|ui| {
                    if ui.heading("To").clicked() {
                        match self.list_sort {
                            Some((2, SortOrder::Ascending)) => {
                                self.list_sort = Some((2, SortOrder::Descending));
                                sort_changed = true;
                            }
                            _ => {
                                self.list_sort = Some((2, SortOrder::Ascending));
                                sort_changed = true;
                            }
                        }
                    }
                });
            })
            .body(|mut body| {
                let Some((id, table)) = &mut self.calibration_list.filament_table else {
                    return;
                };

                if sort_changed {
                    match self.list_sort {
                        Some((0, SortOrder::Ascending)) => {
                            table.sort_by(|a, b| {
                                match (filament_map.get(&a.0), filament_map.get(&b.0)) {
                                    (Some(f1), Some(f2)) => f1.name.cmp(&f2.name),
                                    (Some(_), None) => std::cmp::Ordering::Less,
                                    (None, Some(_)) => std::cmp::Ordering::Greater,
                                    (None, None) => a.0.cmp(&b.0), // Fall back to comparing IDs when names unavailable
                                }
                            });
                        }
                        Some((0, SortOrder::Descending)) => {
                            table.sort_by(|a, b| {
                                match (filament_map.get(&a.0), filament_map.get(&b.0)) {
                                    (Some(f1), Some(f2)) => f2.name.cmp(&f1.name),
                                    (Some(_), None) => std::cmp::Ordering::Less,
                                    (None, Some(_)) => std::cmp::Ordering::Greater,
                                    (None, None) => b.0.cmp(&a.0), // Fall back to comparing IDs when names unavailable
                                }
                            });
                        }
                        Some((1, SortOrder::Ascending)) => {
                            table.sort_by(|a, b| a.1 .0.cmp(&b.1 .0));
                        }
                        Some((1, SortOrder::Descending)) => {
                            table.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));
                        }
                        Some((2, SortOrder::Ascending)) => {
                            table.sort_by(|a, b| a.1 .1.cmp(&b.1 .1));
                        }
                        Some((2, SortOrder::Descending)) => {
                            table.sort_by(|a, b| b.1 .1.cmp(&a.1 .1));
                        }
                        _ => {
                            //
                        }
                    }
                }

                let snapshot = self.nucleo.as_ref().unwrap().snapshot();

                let snapshot = snapshot
                    .matched_items(..)
                    .map(|f| f.data.0)
                    .collect::<HashSet<_>>();

                // for f in snapshot.matched_items(..) {
                //     let Some((id, (from, to))) = table.iter().find(|(id, _)| *id == f.data.0)
                //     else {
                //         continue;
                //     };
                for (id, (from, to)) in table.iter() {
                    if !snapshot.contains(id) {
                        continue;
                    }

                    let Some(filament2) = filament_map.get(id) else {
                        continue;
                    };

                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label(filament2.colored_name(ui.ctx()));
                        });

                        row.col(|ui| {
                            ui.label(format!("{}", from));
                        });

                        row.col(|ui| {
                            ui.label(format!("{}", to));
                        });
                    });
                }

                // for (from_id, to, filament) in self.calibration_list.filament_table.iter() {
                //     if *id == picked_id {
                //         continue;
                //     }
                // }
            });
    }
}
