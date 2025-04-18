use std::collections::HashMap;

use egui::{Color32, FontFamily, FontId, Response, TextFormat, text::LayoutJob};

use crate::types::{Filament, FilamentMap};

use super::App;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FilamentPicker {
    id: u32,
    selected: Option<Filament>,
    buf: String,
}

impl Default for FilamentPicker {
    fn default() -> Self {
        Self {
            id: rand::random::<u32>(),
            selected: None,
            buf: String::with_capacity(128),
        }
    }
}

impl FilamentPicker {
    pub fn filament_id(&self) -> Option<u32> {
        self.selected.as_ref().map(|f| f.id)
    }

    pub fn to_saved(&self) -> Option<u32> {
        self.selected.as_ref().map(|f| f.id)
    }

    pub fn selected(&self) -> Option<&Filament> {
        self.selected.as_ref()
    }

    pub fn set_selected(&mut self, f: Option<Filament>) {
        self.selected = f;
        self.buf = match &self.selected {
            Some(f) => f.name.clone(),
            None => String::new(),
        };
    }

    pub fn set_buf(&mut self, buf: String) {
        self.buf = buf;
    }

    pub fn reset(&mut self) {
        self.selected = None;
        self.buf.clear();
    }

    pub fn filament_picker(
        &mut self,
        min_width: Option<f32>,
        // filaments_map: &HashMap<u32, Filament>,
        filaments_map: &FilamentMap,
        filaments: &[(u32, Filament)],
        // filter: &Option<regex::Regex>,
        snapshot: &nucleo::Snapshot<(u32, Filament)>,
        filter: bool,
        ui: &mut egui::Ui,
    ) -> Response {
        ui.horizontal(|ui| {
            // if ui.button("x").clicked() {
            //     self.reset();
            // }

            #[cfg(feature = "nope")]
            let filter_resp = ui.add(
                super::dropdown::DropDownBox::from_iter(
                    filaments.iter(),
                    // .map(|f| (f.name.as_str(), f.colored_name())),
                    // .map(|f| (f.name.as_str(), f.colored_name())),
                    // &items,
                    self.id,
                    &mut self.buf,
                    // |ui, filament| ui.label(filament.colored_name()),
                    |ui, f| {
                        //
                        ui.selectable_value(&mut self.selected, Some(f.clone()), f.colored_name())
                    },
                )
                .filter_by_input(false)
                .select_on_focus(true)
                // .desired_width(100.),
                .desired_width(if let Some(min_width) = min_width {
                    min_width
                } else {
                    ui.available_width()
                }),
            );

            // let response = egui::ComboBox::from_label("Select Filament")
            let mut response = egui::ComboBox::from_id_salt(self.id)
                // let mut response = super::dropdown::DropDownBox::from_id_source(self.id)
                .width(if let Some(min_width) = min_width {
                    min_width
                } else {
                    ui.available_width()
                })
                .height(250.)
                // response
                .selected_text(match &self.selected {
                    Some(f) => f.colored_name(ui.ctx()),
                    None => LayoutJob::default(),
                })
                .show_ui(ui, |ui| {
                    // eprintln!("ui.available_width() = {}", ui.available_width());

                    ui.selectable_value(&mut self.selected, None, "None");

                    if filter {
                        // log::debug!("matched_item_count = {}", n);
                        // log::debug!("snapshot.item_count = {}", snapshot.item_count());
                        // log::debug!("matched_item_count = {}", snapshot.matched_item_count());

                        for f in snapshot.matched_items(..) {
                            let f = &f.data;
                            ui.selectable_value(
                                &mut self.selected,
                                Some(f.1.clone()),
                                f.1.colored_name(ui.ctx()),
                            );
                        }
                    } else {
                        for (_, f) in filaments.iter() {
                            ui.selectable_value(
                                &mut self.selected,
                                Some(f.clone()),
                                f.colored_name(ui.ctx()),
                            );
                        }
                    }

                    #[cfg(feature = "nope")]
                    for (_, f) in filaments.iter() {
                        // let w = format!("{} {}", &f.name, &f.display_color());

                        // if let Some(re) = filter {
                        //     if re.is_match(&f.name) || re.is_match(&f.manufacturer) {
                        //         ui.selectable_value(
                        //             &mut self.selected,
                        //             Some(f.clone()),
                        //             f.colored_name(),
                        //         );
                        //     }
                        // } else {
                        //     ui.selectable_value(
                        //         &mut self.selected,
                        //         Some(f.clone()),
                        //         f.colored_name(),
                        //     );
                        // }

                        // if f.name.contains(filter) || f.manufacturer.contains(filter) {
                        // }
                    }
                });

            // filter_resp
            response.response
        })
        .response
    }
}
