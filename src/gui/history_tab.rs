// use anyhow::{anyhow, bail, ensure, Context, Result};
use tracing::{debug, error, info, trace, warn};

use egui_extras::{Column, TableBuilder};

use super::App;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Debug, Clone)]
pub struct HistoryRow {
    pub id: u32,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub num_filaments: usize,
    pub filaments: Vec<u32>,
    pub multiplier: Option<f64>,
    pub offset: Option<f64>,
}

impl App {
    pub fn show_history_tab(&mut self, ui: &mut egui::Ui) {
        let history = self.db.fetch_history(self.history_sort).unwrap();
        let (filament_map, filaments) = self.db.get_all_filaments().unwrap();

        // debug!("history.len() = {}", history.len());

        ui.checkbox(&mut self.history_hide_duplicates, "Hide duplicates (TODO)");

        let row_height = 20.;
        // let row_height = 52.;

        TableBuilder::new(ui)
            .striped(true)
            // .sense(egui::Sense::click())
            // .column(Column::auto().at_least(220.)) // timestamp
            .column(Column::exact(200.)) // timestamp
            .column(Column::exact(250.)) // filaments
            // .column(Column::auto().at_least(100.)) // load button
            .column(Column::exact(100.)) // load button
            .column(Column::exact(100.)) // remove button
            .header(35., |mut header| {
                header.col(|ui| {
                    if ui.heading("Timestamp").clicked() {
                        match self.history_sort {
                            Some((0, SortOrder::Ascending)) => {
                                self.history_sort = Some((0, SortOrder::Descending));
                            }
                            _ => {
                                self.history_sort = Some((0, SortOrder::Ascending));
                            }
                        }
                    }
                });
                header.col(|ui| {
                    ui.heading("Colors");
                });
                header.col(|ui| {
                    // ui.heading("Colors");
                });
            })
            .body(|mut body| {
                for entry in history.iter() {
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label(format!("{}", entry.timestamp));
                        });
                        row.col(|ui| {
                            ui.horizontal(|ui| {
                                ui.separator();
                                for (i, f) in entry.filaments.iter().enumerate() {
                                    // eprintln!("i = {}, f = {:?}", i, f);
                                    let Some(filament) = filament_map.get(f) else {
                                        continue;
                                    };
                                    let resp = filament.stacked_colored_box(ui, 18.);

                                    resp.on_hover_text(format!(
                                        "{} {}",
                                        filament.manufacturer, filament.name
                                    ));

                                    ui.separator();
                                }
                            });

                            // ui.label("TODO");
                        });
                        row.col(|ui| {
                            if ui.button("Load").clicked() {
                                self.filament_grid
                                    .current
                                    .load_from_history(&filament_map, entry);
                                self.current_tab = crate::gui::Tab::FilamentGrid;
                            }
                        });
                        row.col(|ui| {
                            if ui.button("Remove").clicked() {
                                if let Err(e) = self.db.remove_history(entry.id) {
                                    error!("remove_history = {:?}", e);
                                }
                            }

                            // if self.new_filament.confirm_delete.is_some() && !button.hovered() {
                            //     self.new_filament.confirm_delete = None;
                            // }
                        });
                    });
                }

                //
            });
    }
}
