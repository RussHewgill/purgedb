#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_doc_comments)]
// #![windows_subsystem = "windows"]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unexpected_cfgs)]

mod db;
mod gui;
mod search;
mod types;

#[cfg(target_os = "windows")]
mod input_sender;

#[cfg(feature = "nope")]
fn main() {
    let _ = crate::input_sender::main();
}

// #[cfg(feature = "nope")]
fn main() -> eframe::Result<()> {
    use gui::App;

    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([400.0, 300.0]),
        ..Default::default()
    };
    eframe::run_native(
        "PurgeDB",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    )
}

#[cfg(feature = "nope")]
fn main() {
    use db::Db;

    let db = Db::new().unwrap();

    // db.test_filaments().unwrap();

    // let xs = db.get_all_filaments().unwrap();

    // let xs = db.get_all_searchable_keywords().unwrap();

    let xs = db.get_purge_values(1, 2).unwrap();

    // eprintln!("wat 0");
    // let r = xs.search_names("poly");
    eprintln!("xs = {:?}", xs);
}
