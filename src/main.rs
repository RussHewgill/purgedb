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
pub mod logging;

use anyhow::{Context, Result, anyhow, bail, ensure};
use tracing::{debug, error, info, trace, warn};

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

/// history test
#[cfg(feature = "nope")]
fn main() -> anyhow::Result<()> {
    // let n = std::mem::size_of::<gui::filament_grid::FilamentGridSave>();
    // let n = std::mem::size_of::<gui::filament_picker::FilamentPicker>();
    // eprintln!("n = {}", n);

    let mut db = db::Db::new()?;

    #[cfg(feature = "nope")]
    {
        // db.init_history()?;

        let mut grid = gui::filament_grid::FilamentGrid::default();

        *grid.num_filaments_mut() = 2;

        let f0 = types::Filament {
            id: 1,
            name: "PolyLite Red".to_string(),
            manufacturer: "Polymaker".to_string(),
            color_base: hex_color::HexColor {
                r: 255,
                g: 0,
                b: 0,
                a: 255,
            },
            colors: vec![hex_color::HexColor {
                r: 255,
                g: 0,
                b: 0,
                a: 255,
            }],
            material: "PLA".to_string(),
            notes: "".to_string(),
        };

        let f1 = types::Filament {
            id: 2,
            name: "PolyLite Green".to_string(),
            manufacturer: "Polymaker".to_string(),
            color_base: hex_color::HexColor {
                r: 0,
                g: 255,
                b: 0,
                a: 255,
            },
            colors: vec![hex_color::HexColor {
                r: 0,
                g: 255,
                b: 0,
                a: 255,
            }],
            material: "PLA".to_string(),
            notes: "".to_string(),
        };

        let p0 = &mut grid.pickers_mut()[0];
        p0.set_selected(Some(f0));

        let p1 = &mut grid.pickers_mut()[1];
        p1.set_selected(Some(f1));

        let i0 = grid.current.id(0);
        eprintln!("i0 = {:?}", i0);

        let i1 = grid.current.id(1);
        eprintln!("i1 = {:?}", i1);

        // db.add_to_history(&grid.current)?;
    }

    let history = db.fetch_history(None)?;

    eprintln!("history.len() = {}", history.len());

    for h in history.iter() {
        eprintln!("h = {:?}", h);
    }

    // let fs = vec![143, 2, 1, 72];

    // for id in fs.iter() {
    //     let f = db.get_filament(*id)?;
    //     eprintln!("f = {:?}", f);
    // }

    Ok(())
}

// #[cfg(feature = "nope")]
fn main() -> anyhow::Result<()> {
    input_sender::test_bs().unwrap();

    Ok(())
}

/// main app
// #[cfg(feature = "nope")]
fn _main() -> eframe::Result<()> {
    use gui::App;

    // env_logger::init();
    if cfg!(debug_assertions) {
        logging::init_logs();
    }

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
