use std::collections::HashMap;

use egui::{text::LayoutJob, Color32, FontFamily, FontId, Response, TextFormat};

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
        filaments: &[Filament],
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
            let mut response = egui::ComboBox::from_id_source(self.id)
                // let mut response = super::dropdown::DropDownBox::from_id_source(self.id)
                .width(if let Some(min_width) = min_width {
                    min_width
                } else {
                    ui.available_width()
                })
                // response
                .selected_text(match &self.selected {
                    Some(f) => f.colored_name(),
                    None => LayoutJob::default(),
                })
                .show_ui(ui, |ui| {
                    // eprintln!("ui.available_width() = {}", ui.available_width());
                    for (_, f) in filaments.iter().enumerate() {
                        // let w = format!("{} {}", &f.name, &f.display_color());
                        ui.selectable_value(&mut self.selected, Some(f.clone()), f.colored_name());
                    }
                });

            // filter_resp
            response.response
        })
        .response
    }
}
