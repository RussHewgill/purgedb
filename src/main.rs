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

    // let names = db.get_all_names().unwrap();
    // let names = names.iter().map(|(_, n)| n).collect::<Vec<_>>();
    let (filament_map, filaments) = db.get_all_filaments().unwrap();

    let (tx, rx) = std::sync::mpsc::channel::<()>();

    // #[cfg(feature = "nope")]
    let mut filter: nucleo::Nucleo<(u32, types::Filament)> = nucleo::Nucleo::new(
        nucleo::Config::DEFAULT,
        std::sync::Arc::new(move || {
            tx.send(()).unwrap();
        }),
        Some(1),
        2,
    );

    let injector = filter.injector();

    for f in filaments.iter() {
        injector.push(f.clone(), |(_, filament), buf| {
            buf[0] = filament.name.clone().into();
            buf[1] = filament.manufacturer.clone().into();
        });
    }

    filter.pattern.reparse(
        // column,
        0,
        "poly",
        nucleo::pattern::CaseMatching::Smart,
        nucleo::pattern::Normalization::Never,
        false,
    );
    filter.tick(50);

    debug!("looping");
    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));

        filter.tick(50);

        match rx.try_recv() {
            Ok(_) => {
                debug!("got recv");
                let snapshot = filter.snapshot();
                debug!("snapshot.item_count = {}", snapshot.item_count());
                debug!("matched_item_count = {}", snapshot.matched_item_count());

                let mut n = 0;
                for f in snapshot.matched_items(..) {
                    n += 1;
                    let f = &f.data;
                    debug!("f = {:?}", f);
                }
                debug!("n = {}", n);

                break;
            }
            Err(e) => {
                error!("recv error: {:?}", e);
                // break;
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
