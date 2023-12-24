use crate::types::Filament;

use super::App;
use super::filament_picker::FilamentPicker;

use egui::{RichText, Stroke};

use egui_extras::{TableBuilder, Column};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

#[derive(Debug, Default)]
pub struct GetPurge {
  filaments: Vec<Filament>,
  picker: FilamentPicker,
}

impl App {
  pub fn show_get_purge(&mut self, ui: &mut egui::Ui) {

    // filament picker
    egui::Frame::none()
      // .stroke(Stroke::new(1.0, egui::Color32::from_gray(65)))
      .outer_margin(5.)
      .inner_margin(5.)
      .show(ui, |ui| {

        let filaments = self.db.get_all_filaments().unwrap();
        let response = self.get_purge.picker.filament_picker(&filaments, ui);
        
        let add_filament_button = egui::Button::new(RichText::new("Add Filament").size(15.));
        if ui.add(add_filament_button).clicked() {
          if let Some(f) = &self.get_purge.picker.selected {
            self.get_purge.filaments.push(f.clone());
          }
        }

        // ui.allocate_space(ui.available_size());
      });

    // table
    egui::Frame::none()
    .stroke(Stroke::new(1., egui::Color32::from_gray(30)))
    // .fill(egui::Color32::from_gray(220))
    .show(ui, |ui| {
      ui.visuals_mut().override_text_color = Some(egui::Color32::BLACK);
      TableBuilder::new(ui)
        .striped(true)
        .column(Column::auto().at_least(150.))
        .columns(Column::exact(60.), self.get_purge.filaments.len())
        .header(30., |mut header| {
          header.col(|ui| {});
          for f in self.get_purge.filaments.iter().rev() {
            header.col(|ui| {
              ui.heading(f.colored_box());
            });
          }
        })
        .body(|mut body| {
          for from in self.get_purge.filaments.iter() {
            body.row(20., |mut row| {
              row.col(|ui| {
                ui.label(from.colored_name());
                // ui.fonts(|fonts| fonts.layout_job(from.colored_name()));
              });
              for to in self.get_purge.filaments.iter() {
                row.col(|ui| {
                  if from == to {
                    egui::Frame::none()
                      .fill(egui::Color32::from_gray(32))
                      .show(ui, |ui| {
                        ui.allocate_space(ui.available_size());
                      });
                  } else if let Ok(purge) = self.db.get_purge_values(from.id, to.id) {
                    ui.label(format!("{purge}"));
                  }
                  // ui.label("123");
                });
              }
            })
          }
        })
        ;
    });
  }
}
