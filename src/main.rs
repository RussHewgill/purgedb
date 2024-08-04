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

use anyhow::{anyhow, bail, ensure, Context, Result};
use log::{debug, error, info, trace, warn};

#[cfg(feature = "nope")]
fn main() -> Result<()> {
    env_logger::init();

    let db = db::Db::new().unwrap();

    let names = db.get_all_names().unwrap();
    // let names = names.iter().map(|(_, n)| n).collect::<Vec<_>>();

    let (tx, rx) = std::sync::mpsc::channel::<()>();

    // #[cfg(feature = "nope")]
    let filter: nucleo::Nucleo<u32> = nucleo::Nucleo::new(
        nucleo::Config::DEFAULT,
        std::sync::Arc::new(move || {
            tx.send(()).unwrap();
        }),
        Some(1),
        1,
    );

    let injector = filter.injector();

    // for (i, name) in names.iter() {
    //     // injector.insert(*i, name.clone());
    //     injector.push(i, |i|)
    // }

    debug!("looping");
    loop {
        match rx.recv() {
            Ok(_) => {
                debug!("got recv");
            }
            Err(e) => {
                error!("recv error: {:?}", e);
                break;
            }
        }

        // break;
    }

    debug!("done");
    Ok(())
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
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
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
