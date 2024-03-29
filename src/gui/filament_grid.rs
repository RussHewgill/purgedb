use egui_extras::{Column, TableBuilder};

use super::{filament_picker::FilamentPicker, text_val::ValText, App};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FilamentGrid {
  pickers: [FilamentPicker; 16],
  num_filaments: usize,
  // cells: [[ValText<u32>; 16]; 16],
}

impl Default for FilamentGrid {
  fn default() -> Self {
    // let cells = std::array::from_fn(|_| {
    //   std::array::from_fn(|_| {
    //     ValText::with_validator(|text| text.parse::<u32>().ok().filter(|&n| n < 1000))
    //   })
    // });

    Self {
      pickers: Default::default(),
      num_filaments: 4,
      // cells,
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
          if ui.button("Clear").clicked() {
            for p in self.filament_grid.pickers.iter_mut() {
              p.reset();
            }
          }
          if ui.button("White + Black").clicked() {
            let f = self.db.get_filament(2).unwrap();
            self.filament_grid.pickers[1].selected = Some(f);
            let f = self.db.get_filament(1).unwrap();
            self.filament_grid.pickers[2].selected = Some(f);
          }
        });

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
              } else {
                header.col(|ui| {});
              }
            }
          })
          .body(|mut body| {
            for from_id in 0..self.filament_grid.num_filaments {
              body.row(20., |mut row| {
                row.col(|ui| {
                  self.filament_grid.pickers[from_id].filament_picker(Some(400.), &filaments, ui);
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

        if ui.button("send to orca").clicked() {
          self
            .send_purge_values(self.filament_grid.num_filaments)
            .unwrap();
        }
      });
  }

  fn send_purge_values(&self, num_filaments: usize) -> anyhow::Result<()> {
    // if num_filaments != 4 {
    //   panic!("num_filaments TODO");
    // }

    crate::input_sender::alt_tab()?;
    std::thread::sleep(std::time::Duration::from_millis(400));
    // eprintln!("alt-tab");

    for from_id in 0..num_filaments {
      let Some(from) = &self.filament_grid.pickers[from_id].selected else {
        panic!("missing from");
      };
      for to_id in 0..num_filaments {
        let Some(to) = &self.filament_grid.pickers[to_id].selected else {
          panic!("missing to");
        };
        if from_id == to_id {
          continue;
        }
        if let Ok(purge) = self.db.get_purge_values(from.id, to.id) {
          // eprintln!("sending purge = {:?}", purge);
          crate::input_sender::send_number(purge, true)?;
        } else {
          crate::input_sender::send_number(999, true)?;
        }
      }
      for _ in 0..4 - num_filaments {
        // eprintln!("tab");
        crate::input_sender::tab()?;
      }
      // eprintln!("tab");
      crate::input_sender::tab()?;
    }

    Ok(())
  }
}
