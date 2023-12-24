use crate::types::Filament;

use super::App;
use super::filament_picker::FilamentPicker;

use egui::{RichText, Stroke, CursorIcon, LayerId, Order, Sense};

use egui_extras::{TableBuilder, Column};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

#[derive(Debug, Default, serde::Serialize,serde::Deserialize)]
pub struct GetPurge {
  filaments: Vec<Filament>,
  picker: FilamentPicker,
}

fn drag_source(ui: &mut egui::Ui, id: egui::Id, body: impl FnOnce(&mut egui::Ui)) {
  let is_being_dragged = ui.memory(|mem| mem.is_being_dragged(id));

  if !is_being_dragged {
      let response = ui.scope(body).response;

      // Check for drags:
      let response = ui.interact(response.rect, id, Sense::drag());
      if response.hovered() {
          ui.ctx().set_cursor_icon(CursorIcon::Grab);
      }
  } else {
      ui.ctx().set_cursor_icon(CursorIcon::Grabbing);

      // Paint the body to a new layer:
      let layer_id = LayerId::new(Order::Tooltip, id);
      let response = ui.with_layer_id(layer_id, body).response;

      // Now we move the visuals of the body to where the mouse is.
      // Normally you need to decide a location for a widget first,
      // because otherwise that widget cannot interact with the mouse.
      // However, a dragged component cannot be interacted with anyway
      // (anything with `Order::Tooltip` always gets an empty [`Response`])
      // So this is fine!

      if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
          let delta = pointer_pos - response.rect.center();
          ui.ctx().translate_layer(layer_id, delta);
      }
  }
}

impl App {
  pub fn show_get_purge(&mut self, ui: &mut egui::Ui) {

    // filament picker
    egui::Frame::none()
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
        .column(Column::auto().at_least(15.))
        .column(Column::auto().at_least(45.))
        .column(Column::auto().at_least(150.))
        .columns(Column::exact(40.), self.get_purge.filaments.len())
        .header(50., |mut header| {
          header.col(|ui| {});
          header.col(|ui| {});
          header.col(|ui| {});
          for f in self.get_purge.filaments.iter().rev() {
            header.col(|ui| {
              // ui.vertical(|ui| {
                ui.label(f.colored_box(true));
              // });
              // ui.heading(f.colored_box());
            });
          }
        })
        .body(|mut body| {
          let mut to_remove = vec![];
          for (i, from) in self.get_purge.filaments.iter().enumerate() {
            body.row(20., |mut row| {
              row.col(|ui| {
                if ui.button("X").clicked() {
                  // self.get_purge.filaments.remove(i);
                  to_remove.push(i);
                }
              });
              row.col(|ui| {
                ui.label(from.colored_box(false));
              });
              row.col(|ui| {
                ui.label(&from.display());
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
          for i in to_remove.into_iter() {
            self.get_purge.filaments.remove(i);
          }
        })
        ;
    });
  }
}
