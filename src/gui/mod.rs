pub mod enter_purge;
pub mod filament_grid;
pub mod filament_picker;
// pub mod filament_picker_widget;
// pub mod get_purge;
// pub mod edit_filament;
// pub mod dropdown;
pub mod history_tab;
pub mod new_filament;
pub mod text_val;

use crate::{db::Db, types::Filament};

use self::{enter_purge::EnterPurge, filament_grid::FilamentGrid, new_filament::NewFilament};

#[derive(PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Tab {
    // GetPurgeValues,
    EnterPurgeValues,
    NewFilament,
    // EditFilament,
    FilamentGrid,
    History,
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

    #[serde(skip)]
    pub history_sort: Option<(usize, history_tab::SortOrder)>,

    #[serde(skip)]
    pub filament_filter: String,

    // #[serde(skip)]
    // pub filament_regex: Option<regex::Regex>,
    #[serde(skip)]
    nucleo: Option<nucleo::Nucleo<(u32, Filament)>>,
    #[serde(skip)]
    injector: Option<nucleo::Injector<(u32, Filament)>>,
    #[serde(skip)]
    updated_filaments: bool,

    // #[serde(skip)]
    // matcher: Option<nucleo::Matcher>,
    default_white: u32,
    default_black: u32,
}

impl Default for App {
    fn default() -> Self {
        let db = Db::new().unwrap();
        // db.test_filaments().unwrap();

        // let filter = nucleo::Nucleo::new(
        //     nucleo::Config::DEFAULT,
        //     std::sync::Arc::new(|| {
        //         //
        //     }),
        //     Some(1),
        //     1,
        // );

        // let injector = filter.injector();

        Self {
            db,
            current_tab: Tab::default(),

            new_filament: NewFilament::default(),
            // edit_filament: EditFilament::default(),
            // get_purge: GetPurge::default(),
            enter_purge: EnterPurge::default(),
            filament_grid: FilamentGrid::default(),

            history_sort: None,

            filament_filter: String::new(),
            // filament_regex: None,

            // nucleo: Some(filter),
            // injector: Some(injector),
            nucleo: None,
            injector: None,
            updated_filaments: false,

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

        let mut out: Self = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        let filter = nucleo::Nucleo::new(
            nucleo::Config::DEFAULT,
            std::sync::Arc::new(|| {
                //
            }),
            Some(1),
            1,
        );

        let injector = filter.injector();

        out.nucleo = Some(filter);
        out.injector = Some(injector);

        out
    }

    pub fn update_filtered_filaments(&mut self, map: &crate::types::FilamentMap) {
        if self.updated_filaments {
            return;
        }

        self.nucleo.as_mut().unwrap().restart(true);
        self.injector = Some(self.nucleo.as_ref().unwrap().injector());

        let injector = self.injector.as_mut().unwrap();

        for (id, f) in map.filaments.iter() {
            let f = (id.clone(), f.clone());
            injector.push(f, |(_, filament), buf| {
                buf[0] = format!("{} {}", filament.manufacturer, filament.name).into();
                // buf[0] = filament.name.clone().into();
                // buf[1] = filament.manufacturer.clone().into();
            });
        }

        self.updated_filaments = true;
    }
}

// #[cfg(feature = "nope")]
impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ctx.set_visuals(egui::style::Visuals::dark());

        if cfg!(debug_assertions) && ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        self.nucleo.as_mut().unwrap().tick(10);

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
                ui.selectable_value(&mut self.current_tab, Tab::History, "History");
            });
            // ui.separator();
        });

        // #[cfg(feature = "nope")]
        egui::TopBottomPanel::bottom("bot_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("X").clicked() {
                    self.filament_filter.clear();
                    // self.filament_regex = None;
                    self.nucleo.as_mut().unwrap().restart(true);
                    self.injector = Some(self.nucleo.as_ref().unwrap().injector());
                }
                ui.label("Filter:");

                let resp = ui.add(egui::TextEdit::singleline(&mut self.filament_filter));

                if ctx.input(|i| i.key_pressed(egui::Key::F1)) {
                    resp.request_focus();
                    self.filament_filter.clear();
                    // self.filament_regex = None;
                    self.nucleo.as_mut().unwrap().restart(true);
                    self.injector = Some(self.nucleo.as_ref().unwrap().injector());
                    self.updated_filaments = false;
                }
                // if ctx.input(|i| i.key_pressed(egui::Key::F4)) {
                //     resp.request_focus();
                // }

                if resp.changed() {
                    self.nucleo.as_mut().unwrap().pattern.reparse(
                        0,
                        &self.filament_filter,
                        nucleo::pattern::CaseMatching::Smart,
                        nucleo::pattern::Normalization::Smart,
                        false,
                    );

                    #[cfg(feature = "nope")]
                    match regex::RegexBuilder::new(&self.filament_filter)
                        .case_insensitive(true)
                        .build()
                    {
                        Ok(r) => {
                            self.filament_regex = Some(r);
                        }
                        Err(_) => {
                            ui.label("Invalid Regex");
                        }
                    }
                    //
                }
            });
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
            Tab::History => {
                egui::CentralPanel::default().show(ctx, |ui| self.show_history_tab(ui));
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
