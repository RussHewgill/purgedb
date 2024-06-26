pub mod enter_purge;
pub mod filament_grid;
pub mod filament_picker;
// pub mod filament_picker_widget;
// pub mod get_purge;
// pub mod edit_filament;
pub mod dropdown;
pub mod new_filament;
pub mod text_val;

use crate::db::Db;

use self::{enter_purge::EnterPurge, filament_grid::FilamentGrid, new_filament::NewFilament};

#[derive(PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Tab {
    // GetPurgeValues,
    EnterPurgeValues,
    NewFilament,
    // EditFilament,
    FilamentGrid,
}

impl Default for Tab {
    fn default() -> Self {
        Self::FilamentGrid
        // Self::NewFilament
        // Self::GetPurgeValues
        // Self::EnterPurgeValues
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct App {
    #[serde(skip)]
    db: Db,
    current_tab: Tab,

    new_filament: NewFilament,
    // edit_filament: EditFilament,
    // get_purge: GetPurge,
    enter_purge: EnterPurge,
    filament_grid: FilamentGrid,

    default_white: u32,
    default_black: u32,
}

impl Default for App {
    fn default() -> Self {
        let db = Db::new().unwrap();
        db.test_filaments().unwrap();
        Self {
            db,
            current_tab: Tab::default(),

            new_filament: NewFilament::default(),
            // edit_filament: EditFilament::default(),
            // get_purge: GetPurge::default(),
            enter_purge: EnterPurge::default(),
            filament_grid: FilamentGrid::default(),

            default_white: 1,
            default_black: 2,
        }
    }
}

/// new
impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

// #[cfg(feature = "nope")]
impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ctx.set_visuals(egui::style::Visuals::dark());

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // ui.separator();
            ui.horizontal(|ui| {
                // ui.selectable_value(
                //   &mut self.current_tab,
                //   Tab::GetPurgeValues,
                //   "Get Purge Values",
                // );
                ui.selectable_value(&mut self.current_tab, Tab::NewFilament, "New Filament");
                ui.selectable_value(
                    &mut self.current_tab,
                    Tab::EnterPurgeValues,
                    "Enter Purge Values",
                );
                ui.selectable_value(&mut self.current_tab, Tab::FilamentGrid, "Filament Grid");
            });
            // ui.separator();
        });

        match self.current_tab {
            // Tab::GetPurgeValues => self.show_get_purge(ui),
            Tab::EnterPurgeValues => {
                egui::CentralPanel::default().show(ctx, |ui| self.show_enter_purge(ui));
                // self.show_enter_purge(ui)
            }
            Tab::NewFilament => {
                // egui::CentralPanel::default().show(ctx, |ui| self.show_new_filament(ui));
                self.show_new_filament(ctx);
            }
            // Tab::EditFilament => self.show_edit_filament(ui),
            Tab::FilamentGrid => {
                egui::CentralPanel::default().show(ctx, |ui| self.show_filament_grid(ui));
            }
        }

        // egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        //   ui.horizontal(|ui| {
        //     if ui.button("Reload Database").clicked() {

        //     }
        //   });
        // });
    }
}
