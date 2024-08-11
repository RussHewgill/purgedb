use egui_extras::{Column, TableBuilder};

use super::App;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Debug, Clone)]
pub struct HistoryRow {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub num_filaments: usize,
    pub filaments: Vec<u32>,
}

impl App {
    pub fn show_history_tab(&mut self, ui: &mut egui::Ui) {
        let history = self.db.fetch_history(self.history_sort).unwrap();
        let filaments = self.db.get_all_filaments().unwrap();

        let row_height = 20.;

        TableBuilder::new(ui)
            .striped(true)
            .column(Column::auto()) // timestamp
            .column(Column::auto()) // filaments
            .header(35., |mut header| {
                header.col(|ui| {
                    ui.heading("Timestamp");
                });
                header.col(|ui| {
                    ui.heading("Colors");
                });
            })
            .body(|mut body| {
                for entry in history.iter() {
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label(format!("{}", entry.timestamp));
                        });
                        row.col(|ui| {
                            ui.label("TODO");
                        });
                    });
                }

                //
            });
    }
}
