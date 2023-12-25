use egui_extras::{Column, TableBuilder};

use super::{filament_picker::FilamentPicker, text_val::ValText, App};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FilamentGrid {
  pickers: [FilamentPicker; 16],
  num_filaments: usize,
  cells: [[ValText<u32>; 16]; 16],
}

impl Default for FilamentGrid {
  fn default() -> Self {
    let cells = std::array::from_fn(|_| {
      std::array::from_fn(|_| {
        ValText::with_validator(|text| text.parse::<u32>().ok().filter(|&n| n < 1000))
      })
    });

    Self {
      pickers: Default::default(),
      num_filaments: 2,
      cells,
    }
  }
}

impl App {
  pub fn show_filament_grid(&mut self, ui: &mut egui::Ui) {
    let filaments = self.db.get_all_filaments().unwrap();

    egui::Frame::none()
      .outer_margin(5.)
      .inner_margin(5.)
      .show(ui, |ui| {
        ui.visuals_mut().override_text_color = Some(egui::Color32::BLACK);

        ui.horizontal(|ui| {
          if ui.button("+").clicked() {
            if self.filament_grid.num_filaments < 16 {
              self.filament_grid.num_filaments += 1;
            }
          }
          ui.label(&format!("{}", self.filament_grid.num_filaments));
          if ui.button("-").clicked() {
            if self.filament_grid.num_filaments > 2 {
              self.filament_grid.num_filaments -= 1;
            }
          }
        });

        let mut any_changed = false;

        TableBuilder::new(ui)
          .striped(true)
          // .column(Column::auto().at_least(45.))
          .column(Column::auto().at_least(250.))
          .columns(Column::exact(40.), self.filament_grid.num_filaments)
          .header(50., |mut header| {
            header.col(|ui| {});

            for f in self.filament_grid.pickers[..self.filament_grid.num_filaments].iter() {
              if let Some(f) = &f.selected {
                header.col(|ui| {
                  ui.label(f.colored_box(true));
                });
              }
            }
          })
          .body(|mut body| {
            for from_id in 0..self.filament_grid.num_filaments {
              body.row(20., |mut row| {
                row.col(|ui| {
                  self.filament_grid.pickers[from_id].filament_picker(&filaments, ui);
                });

                for (to_id, to) in self.filament_grid.pickers[..self.filament_grid.num_filaments]
                  .iter()
                  .enumerate()
                {
                  row.col(|ui| {
                    if from_id == to_id {
                      egui::Frame::none()
                        .fill(egui::Color32::from_gray(32))
                        .show(ui, |ui| {
                          ui.allocate_space(ui.available_size());
                        });
                    }

                    match (
                      &self.filament_grid.pickers[from_id].selected,
                      &self.filament_grid.pickers[to_id].selected,
                    ) {
                      (Some(from), Some(to)) => {
                        if let Ok(purge) = self.db.get_purge_values(from.id, to.id) {
                          ui.label(format!("{purge}"));
                        }
                      }
                      _ => {}
                    }

                    // let cell = &mut self.filament_grid.cells[from_id][to_id];
                    // let edit = if !cell.is_valid() {
                    //   egui::TextEdit::singleline(cell)
                    //     .clip_text(false)
                    //     .text_color(egui::Color32::RED)
                    // } else {
                    //   egui::TextEdit::singleline(cell)
                    //     .clip_text(false)
                    // };

                    // if ui.add(edit).changed() {
                    // }

                    // let resp = ui.text_edit_singleline(&mut self.filament_grid.cells[from_id][to_id]);
                  });
                }
              });
            }
          });

        // if any_changed {
        //   if ui.button("Save Values").clicked() {
        //     // self.db.set_purge_values(id_from, id_to, purge)
        //   }
        // }
      });
  }
}
