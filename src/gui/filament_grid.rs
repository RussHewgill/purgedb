use egui_extras::{Column, TableBuilder};

use super::{filament_picker::FilamentPicker, text_val::ValText, App};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FilamentGrid {
    current: usize,
    grids: Vec<FilamentGridData>,
}

impl Default for FilamentGrid {
    fn default() -> Self {
        Self {
            current: 0,
            grids: vec![Default::default()],
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FilamentGridData {
    pickers: [FilamentPicker; 16],
    num_filaments: usize,
    // cells: [[valtext<u32>; 16]; 16],
    pub multiplier: f32,
    pub use_multiplier: bool,
    pub offset: u32,
    pub use_offset: bool,
}

impl Default for FilamentGridData {
    fn default() -> Self {
        // let cells = std::array::from_fn(|_| {
        //   std::array::from_fn(|_| {
        //     ValText::with_validator(|text| text.parse::<u32>().ok().filter(|&n| n < 1000))
        //   })
        // });

        Self {
            pickers: Default::default(),
            num_filaments: 4,
            multiplier: 1.0,
            use_multiplier: false,
            offset: 0,
            use_offset: false,
            // cells,
        }
    }
}

impl FilamentGrid {
    pub fn pickers(&self) -> &[FilamentPicker] {
        &self.grids[self.current].pickers
    }

    pub fn pickers_mut(&mut self) -> &mut [FilamentPicker] {
        &mut self.grids[self.current].pickers
    }

    pub fn num_filaments(&self) -> usize {
        self.grids[self.current].num_filaments
    }

    pub fn num_filaments_mut(&mut self) -> &mut usize {
        &mut self.grids[self.current].num_filaments
    }

    pub fn offset(&self) -> u32 {
        self.grids[self.current].offset
    }

    pub fn offset_mut(&mut self) -> &mut u32 {
        &mut self.grids[self.current].offset
    }

    pub fn use_offset(&self) -> bool {
        self.grids[self.current].use_offset
    }

    pub fn use_offset_mut(&mut self) -> &mut bool {
        &mut self.grids[self.current].use_offset
    }

    pub fn multiplier(&self) -> f32 {
        self.grids[self.current].multiplier
    }

    pub fn multiplier_mut(&mut self) -> &mut f32 {
        &mut self.grids[self.current].multiplier
    }

    pub fn use_multiplier(&self) -> bool {
        self.grids[self.current].use_multiplier
    }

    pub fn use_multiplier_mut(&mut self) -> &mut bool {
        &mut self.grids[self.current].use_multiplier
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
                        for p in self.filament_grid.pickers_mut().iter_mut() {
                            p.reset();
                        }
                    }
                    if ui.button("White + Black").clicked() {
                        let f = self.db.get_filament(2).unwrap();
                        // *self.filament_grid.pickers_mut()[1].selected_mut() = Some(f);
                        self.filament_grid.pickers_mut()[1].set_selected(Some(f));
                        let f = self.db.get_filament(1).unwrap();
                        // *self.filament_grid.pickers_mut()[2].selected_mut() = Some(f);
                        self.filament_grid.pickers_mut()[2].set_selected(Some(f));
                    }
                });

                ui.horizontal(|ui| {
                    if ui.button("+").clicked() {
                        if self.filament_grid.num_filaments() < 16 {
                            *self.filament_grid.num_filaments_mut() += 1;
                        }
                    }
                    ui.label(&format!("{}", self.filament_grid.num_filaments()));
                    if ui.button("-").clicked() {
                        if self.filament_grid.num_filaments() > 2 {
                            *self.filament_grid.num_filaments_mut() -= 1;
                        }
                    }
                });

                ui.separator();

                let mut any_changed = false;

                TableBuilder::new(ui)
                    .striped(true)
                    // .column(Column::auto().at_least(45.))
                    .column(Column::auto().at_least(250.))
                    .columns(Column::exact(40.), self.filament_grid.num_filaments())
                    .header(50., |mut header| {
                        header.col(|ui| {});

                        for f in self.filament_grid.pickers()[..self.filament_grid.num_filaments()]
                            .iter()
                        {
                            if let Some(f) = &f.selected() {
                                header.col(|ui| {
                                    ui.label(f.colored_box(true));
                                });
                            } else {
                                header.col(|ui| {});
                            }
                        }
                    })
                    .body(|mut body| {
                        for from_id in 0..self.filament_grid.num_filaments() {
                            body.row(20., |mut row| {
                                row.col(|ui| {
                                    self.filament_grid.pickers_mut()[from_id].filament_picker(
                                        Some(400.),
                                        &filaments,
                                        ui,
                                    );
                                });

                                for (to_id, to) in self.filament_grid.pickers()
                                    [..self.filament_grid.num_filaments()]
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
                                            self.filament_grid.pickers()[from_id].selected(),
                                            self.filament_grid.pickers()[to_id].selected(),
                                        ) {
                                            (Some(from), Some(to)) => {
                                                if let Ok(purge) =
                                                    self.db.get_purge_values(from.id, to.id)
                                                {
                                                    let purge = match (
                                                        self.filament_grid.use_multiplier(),
                                                        self.filament_grid.use_offset(),
                                                    ) {
                                                        (true, true) => purge,
                                                        (true, false) => {
                                                            (purge as f32
                                                                * self.filament_grid.multiplier())
                                                                as u32
                                                        }
                                                        (false, true) => {
                                                            purge + self.filament_grid.offset()
                                                        }
                                                        (false, false) => purge,
                                                    };
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
                    self.send_purge_values(self.filament_grid.num_filaments())
                        .unwrap();
                }

                ui.separator();

                /// multiplier and offset
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

                // ui.separator();

                // /// saved grids
                // ui.horizontal(|ui| {
                //     /// 0, 1: rolling save for each send
                // });

                //
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
            let Some(from) = &self.filament_grid.pickers()[from_id].selected() else {
                panic!("missing from");
            };
            for to_id in 0..num_filaments {
                let Some(to) = &self.filament_grid.pickers()[to_id].selected() else {
                    panic!("missing to");
                };
                if from_id == to_id {
                    continue;
                }
                if let Ok(p) = self.db.get_purge_values(from.id, to.id) {
                    let purge = match (
                        self.filament_grid.use_multiplier(),
                        self.filament_grid.use_offset(),
                    ) {
                        (true, true) => p,
                        (true, false) => (p as f32 * self.filament_grid.multiplier()) as u32,
                        (false, true) => p + self.filament_grid.offset(),
                        (false, false) => p,
                    };
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
