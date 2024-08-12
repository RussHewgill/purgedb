use std::collections::HashMap;

use egui_extras::{Column, TableBuilder};

use crate::types::{Filament, FilamentMap};

use super::{filament_picker::FilamentPicker, history_tab::HistoryRow, text_val::ValText, App};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FilamentGrid {
    // current: usize,
    // current: FilamentGridData,
    pub current: FilamentGridData,
    // grids: Vec<FilamentGridData>,
    // #[serde(skip)]
    grids: [Option<FilamentGridSave>; 4],
}

impl Default for FilamentGrid {
    fn default() -> Self {
        Self {
            // current: 0,
            current: FilamentGridData::default(),
            grids: std::array::from_fn(|_| None),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FilamentGridData {
    pickers: [FilamentPicker; 16],
    num_filaments: usize,
    // cells: [[valtext<u32>; 16]; 16],
    pub multiplier: f32,
    pub use_multiplier: bool,
    pub offset: u32,
    pub use_offset: bool,
}

impl FilamentGridData {
    pub fn id(&self, n: usize) -> Option<u32> {
        if n >= self.pickers.len() {
            None
        } else {
            self.pickers[n].selected().map(|f| f.id)
            // Some(self.pickers[n].id())
        }
    }
    pub fn num_filaments(&self) -> usize {
        self.num_filaments
    }

    pub fn load_from_history(&mut self, filaments: &FilamentMap, history: &HistoryRow) {
        self.num_filaments = history.num_filaments;
        for (i, id) in history.filaments.iter().enumerate() {
            if let Some(f) = filaments.get(id) {
                self.pickers[i].set_selected(Some(f.clone()));
            } else {
                self.pickers[i].reset();
            }
        }
        if let Some(m) = history.multiplier {
            self.multiplier = m as f32;
            self.use_multiplier = true;
        } else {
            self.multiplier = 0.;
            self.use_multiplier = false;
        }

        if let Some(o) = history.offset {
            self.offset = o as u32;
            self.use_offset = true;
        } else {
            self.offset = 0;
            self.use_offset = false;
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FilamentGridSave {
    // pickers: [(Option<Filament>, String); 16],
    pub pickers: [Option<u32>; 16],
    pub num_filaments: usize,
    pub multiplier: f32,
    pub use_multiplier: bool,
    pub offset: u32,
    pub use_offset: bool,
}

impl FilamentGridSave {
    pub fn new() -> Self {
        Self {
            pickers: [None; 16],
            num_filaments: 4,
            multiplier: 1.0,
            use_multiplier: false,
            offset: 0,
            use_offset: false,
        }
    }

    pub fn from_grid(grid: &FilamentGridData) -> Self {
        let pickers = std::array::from_fn(|i| grid.pickers[i].to_saved());
        Self {
            pickers,
            num_filaments: grid.num_filaments,
            multiplier: grid.multiplier,
            use_multiplier: grid.use_multiplier,
            offset: grid.offset,
            use_offset: grid.use_offset,
        }
    }

    pub fn to_grid(&self, filaments: &FilamentMap, grid: &mut FilamentGridData) {
        for i in 0..16 {
            if let Some(id) = self.pickers[i] {
                let f = filaments.get(&id).unwrap();
                grid.pickers[i].set_selected(Some(f.clone()));
            } else {
                grid.pickers[i].reset();
            }
            // grid.pickers[i].set_selected(self.pickers[i].0.clone());
            // grid.pickers[i].set_buf(self.pickers[i].1.clone());
        }
        grid.num_filaments = self.num_filaments;
        grid.multiplier = self.multiplier;
        grid.use_multiplier = self.use_multiplier;
        grid.offset = self.offset;
        grid.use_offset = self.use_offset;
    }
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
    pub fn save_picker(&mut self, index: usize) {
        // self.grids[index] = Some(self.current.clone());
        self.grids[index] = Some(FilamentGridSave::from_grid(&self.current));
    }

    pub fn load_picker(&mut self, filaments: &FilamentMap, index: usize) {
        if let Some(data) = &self.grids[index] {
            data.to_grid(filaments, &mut self.current);
        }
    }

    pub fn pickers(&self) -> &[FilamentPicker] {
        // &self.grids[self.current].pickers
        &self.current.pickers
    }

    pub fn pickers_mut(&mut self) -> &mut [FilamentPicker] {
        // &mut self.grids[self.current].pickers
        &mut self.current.pickers
    }

    pub fn num_filaments(&self) -> usize {
        // self.grids[self.current].num_filaments
        self.current.num_filaments
    }

    pub fn num_filaments_mut(&mut self) -> &mut usize {
        // &mut self.grids[self.current].num_filaments
        &mut self.current.num_filaments
    }

    pub fn offset(&self) -> u32 {
        // self.grids[self.current].offset
        self.current.offset
    }

    pub fn offset_mut(&mut self) -> &mut u32 {
        // &mut self.grids[self.current].offset
        &mut self.current.offset
    }

    pub fn use_offset(&self) -> bool {
        // self.grids[self.current].use_offset
        self.current.use_offset
    }

    pub fn use_offset_mut(&mut self) -> &mut bool {
        // &mut self.grids[self.current].use_offset
        &mut self.current.use_offset
    }

    pub fn multiplier(&self) -> f32 {
        // self.grids[self.current].multiplier
        self.current.multiplier
    }

    pub fn multiplier_mut(&mut self) -> &mut f32 {
        // &mut self.grids[self.current].multiplier
        &mut self.current.multiplier
    }

    pub fn use_multiplier(&self) -> bool {
        // self.grids[self.current].use_multiplier
        self.current.use_multiplier
    }

    pub fn use_multiplier_mut(&mut self) -> &mut bool {
        // &mut self.grids[self.current].use_multiplier
        &mut self.current.use_multiplier
    }
}

impl App {
    pub fn show_filament_grid(&mut self, ui: &mut egui::Ui) {
        let filaments = self.db.get_all_filaments().unwrap();
        // let keywords = self.db.get_all_searchable_keywords().unwrap();
        self.update_filtered_filaments(&filaments.0);

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
                        let f = self.db.get_filament(self.default_black).unwrap();
                        // *self.filament_grid.pickers_mut()[1].selected_mut() = Some(f);
                        self.filament_grid.pickers_mut()[1].set_selected(Some(f));
                        let f = self.db.get_filament(self.default_white).unwrap();
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
                                        &filaments.0,
                                        &filaments.1,
                                        // &self.filament_regex,
                                        self.nucleo.as_ref().unwrap().snapshot(),
                                        !self.filament_filter.is_empty(),
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
                                                let button = if let Ok(purge) =
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
                                                    // ui.label(format!("{purge}"));
                                                    egui::Label::new(format!("{:?}", purge))
                                                } else {
                                                    egui::Label::new(format!("---"))
                                                };
                                                if ui.add(button).clicked() {
                                                    // eprintln!("clicked");
                                                    self.enter_purge.set_picker1(
                                                        &self.filament_grid.pickers()[from_id],
                                                    );
                                                    self.enter_purge.set_picker2(
                                                        &self.filament_grid.pickers()[to_id],
                                                    );
                                                    self.current_tab = super::Tab::EnterPurgeValues;
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

                #[cfg(target_os = "windows")]
                if ui.button("send to orca").clicked() {
                    if let Err(e) = self.send_purge_values(self.filament_grid.num_filaments()) {
                        eprintln!("send_purge_values error: {:?}", e);
                    }
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

                ui.separator();

                ui.horizontal(|ui| {
                    'outer: for i in 0..self.filament_grid.grids.len() {
                        let b = ui.vertical(|ui| {
                            let mut b = false;
                            if ui.button(format!("Save {}", i)).clicked() {
                                self.filament_grid.save_picker(i);
                            }
                            if self.filament_grid.grids[i].is_some() {
                                if ui.button(format!("Load {}", i)).clicked() {
                                    self.filament_grid.load_picker(&filaments.0, i);
                                    // break 'outer;
                                    b = true;
                                }
                            }
                            if let Some(data) = &self.filament_grid.grids[i] {
                                for k in 0..data.num_filaments {
                                    if let Some(f) = data.pickers[k] {
                                        if let Some(f) = filaments.0.get(&f) {
                                            ui.label(f.colored_box(false));
                                        }
                                    }
                                }
                            }
                            if self.filament_grid.grids[i].is_some() {
                                if ui.button(format!("Delete {}", i)).clicked() {
                                    self.filament_grid.grids[i] = None;
                                }
                            }
                            b
                        });
                        if b.inner {
                            break 'outer;
                        }
                    }
                });

                // /// saved grids
                // ui.horizontal(|ui| {
                //     /// 0, 1: rolling save for each send
                // });

                //
            });
    }

    #[cfg(target_os = "windows")]
    fn send_purge_values(&mut self, num_filaments: usize) -> anyhow::Result<()> {
        // if num_filaments != 4 {
        //   panic!("num_filaments TODO");
        // }

        /// save history
        self.db.add_to_history(&self.filament_grid.current)?;

        // crate::input_sender::alt_tab()?;
        crate::input_sender::focus_first_input(num_filaments)?;
        std::thread::sleep(std::time::Duration::from_millis(400));
        // eprintln!("alt-tab");

        // #[cfg(feature = "nope")]
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
