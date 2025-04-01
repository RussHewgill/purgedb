use anyhow::{Context, Result, anyhow, bail, ensure};

use uiautomation::{
    UIAutomation, UIElement, UIMatcher, UITreeWalker, core::UICondition, patterns::UIValuePattern,
};

#[derive(Debug, Clone, Copy)]
enum Extruder {
    Left,
    Right,
}

#[cfg(feature = "nope")]
pub fn open_purge_window_bambu() -> Result<()> {
    let automation = UIAutomation::new().unwrap();

    let root = automation.get_root_element().unwrap();

    let w = automation
        .create_matcher()
        .from(root)
        .timeout(1000)
        .depth(2)
        .contains_name("BambuStudio")
        .find_first()?;

    // eprintln!("w: {:?}", w);

    let ws = automation
        .create_matcher()
        .from(w)
        // .timeout(2000)
        .name("Flushing volumes")
        .control_type(uiautomation::controls::ControlType::Pane)
        // .match_name("Flushing Volumes for filament change")
        .find_all()?;

    eprintln!("ws.len() = {}", ws.len());

    let w = &ws[0];

    w.set_focus()?;
    w.send_keys("{enter}", 10)?;

    Ok(())
}

#[cfg(feature = "nope")]
pub fn get_num_filaments() -> Result<usize> {
    let automation = UIAutomation::new().unwrap();

    let w = find_purge_window_bambu(&automation)?;

    let p = automation.create_property_condition(
        uiautomation::types::UIProperty::ControlType,
        50016.into(), // spinner
        None,
    )?;

    let cs = w.find_all(uiautomation::types::TreeScope::Descendants, &p)?;

    eprintln!("cs.len() = {}", cs.len());

    Ok(cs.len() - 1)
}

pub fn send_purge_values_orca(vals: &[u32]) -> Result<()> {
    let automation = UIAutomation::new().unwrap();
    let w = find_purge_window_orca(&automation)?;

    // eprintln!("w: {:?}", w);

    _send_purge_values_orca(&automation, &w, vals)?;

    Ok(())
}

/// for a given row and column in a grid of a certain size, get the index in the 1D array
fn get_cell_index(grid_size: usize, row: usize, col: usize) -> usize {
    row * grid_size + col
}

fn _send_purge_values_orca(automation: &UIAutomation, w: &UIElement, vals: &[u32]) -> Result<()> {
    let p_edit = automation.create_property_condition(
        uiautomation::types::UIProperty::ControlType,
        50004.into(), // edit
        None,
    )?;

    let cs = w.find_all(uiautomation::types::TreeScope::Descendants, &p_edit)?;

    let num_cells = cs.len() - 1;
    // eprintln!("cs.len() = {}", num_cells);

    let grid_size = (num_cells as f64).sqrt() as usize;
    let ideal_grid_size = (vals.len() as f64).sqrt() as usize + 1;

    // eprintln!("grid_size = {}", grid_size);
    // eprintln!("ideal_grid_size = {}", ideal_grid_size);

    let mut val_index = 0;

    for row in 0..ideal_grid_size {
        for col in (0..ideal_grid_size).rev() {
            let index = get_cell_index(grid_size, row, ideal_grid_size - col);

            let p = cs[index].get_pattern::<UIValuePattern>()?;

            if p.is_readonly()? {
                // eprintln!("Skipping readonly cell at ({}, {})", row, col);
                continue;
            }

            let v = vals[val_index];
            eprintln!(
                "Setting value for cell with index[{}] at ({}, {}): {}",
                index, row, col, v
            );
            p.set_value(&format!("{}", v))?;

            val_index += 1;
        }
    }

    Ok(())
}

pub fn send_purge_values_bambu(vals: &[u32], both: bool) -> Result<()> {
    let automation = UIAutomation::new().unwrap();
    let w = find_purge_window_bambu(&automation)?;

    if both {
        set_extruder(&automation, &w, Extruder::Left)?;
    }

    _send_purge_values_bambu(&automation, &w, vals)?;

    if both {
        set_extruder(&automation, &w, Extruder::Right)?;

        _send_purge_values_bambu(&automation, &w, vals)?;
    }

    Ok(())
}

fn _send_purge_values_bambu(automation: &UIAutomation, w: &UIElement, vals: &[u32]) -> Result<()> {
    let p = automation.create_property_condition(
        uiautomation::types::UIProperty::ControlType,
        50016.into(), // spinner
        None,
    )?;

    let cs = w.find_all(uiautomation::types::TreeScope::Descendants, &p)?;

    let num_cells = cs.len() - 1;
    eprintln!("cs.len() = {}", num_cells);

    let grid_size = (num_cells as f64).sqrt() as usize;

    let ideal_grid_size = (vals.len() as f64).sqrt() as usize + 1;

    let mut val_index = 0;

    for (i, c) in cs.iter().enumerate() {
        let row = i / grid_size;
        let col = i % grid_size;

        if col >= ideal_grid_size || row >= ideal_grid_size {
            // eprintln!("Skipping cell at ({}, {})", row, col);
            continue;
        }

        let p = c.get_pattern::<UIValuePattern>()?;

        if p.is_readonly()? {
            continue;
        }

        p.set_value(&format!("{}", vals[val_index]))?;
        val_index += 1;
    }

    Ok(())
}

fn set_extruder(automation: &UIAutomation, w: &UIElement, extruder: Extruder) -> Result<()> {
    let p = automation.create_property_condition(
        uiautomation::types::UIProperty::ControlType,
        50003.into(), // ComboBox
        None,
    )?;

    let cs = w.find_all(uiautomation::types::TreeScope::Descendants, &p)?;

    eprintln!("cs.len() = {}", cs.len());

    let c = &cs[0];

    let p = c.get_pattern::<UIValuePattern>()?;
    let v = p.get_value()?;
    // eprintln!("v = {}", v);

    let s_left = "Left extruder";
    let s_right = "Right extruder";

    match extruder {
        Extruder::Left => {
            if v == s_left {
                // eprintln!("Already on left extruder, skipping");
                return Ok(());
            } else if v == s_right {
                c.set_focus()?;
                c.send_keys("{up}", 10)?;
            } else {
                bail!("Unexpected value: {}", v);
            }
        }
        Extruder::Right => {
            if v == s_right {
                // eprintln!("Already on right extruder, skipping");
                return Ok(());
            } else if v == s_left {
                c.set_focus()?;
                c.send_keys("{down}", 10)?;
            } else {
                bail!("Unexpected value: {}", v);
            }
        }
    }

    Ok(())
}

fn find_purge_window_orca(automation: &UIAutomation) -> Result<UIElement> {
    let root = automation.get_root_element().unwrap();

    let walker = automation.create_tree_walker()?;

    // eprintln!("Finding first Orca window");
    let mut orca_window = automation
        .create_matcher()
        .from(root)
        .timeout(300)
        .depth(2)
        .contains_name("OrcaSlicer")
        .find_first()?;

    // eprintln!("id = {}", orca_window.get_automation_id()?);
    // eprintln!("process_id = {}", orca_window.get_process_id()?);
    // eprintln!("name = {}", orca_window.get_name()?);
    // eprintln!("class = {}", orca_window.get_classname()?);

    loop {
        // eprintln!("Finding Flushing window");

        let Ok(ws) = automation
            .create_matcher()
            .from(orca_window.clone())
            .timeout(100)
            .depth(2)
            .name("Flushing volumes for filament change")
            .classname("#32770")
            .find_all()
        else {
            // eprintln!("Finding next Orca window");
            let mut w = orca_window.clone();
            orca_window = 'inner: loop {
                w = walker.get_next_sibling(&w)?;
                if w.get_name()?.contains("OrcaSlicer") {
                    break 'inner w;
                }
            };

            continue;
            // bail!("Could not find purge window");
        };

        // eprintln!("ws.len() = {}", ws.len());
        // break 'outer;
        return Ok(ws[0].clone());
    }
}

fn find_purge_window_bambu(automation: &UIAutomation) -> Result<UIElement> {
    let root = automation.get_root_element().unwrap();

    eprintln!("Finding BambuStudio window");
    let ws = automation
        .create_matcher()
        .from(root)
        .timeout(300)
        .depth(2)
        .contains_name("BambuStudio")
        // .find_all()?;
        .find_first()?;

    // eprintln!("ws: {:?}", ws.len());

    // let w = &ws[0];

    // eprintln!("id = {}", w.get_automation_id()?);
    // eprintln!("process_id = {}", w.get_process_id()?);
    // eprintln!("name = {}", w.get_name()?);
    // eprintln!("class = {}", w.get_classname()?);

    let Ok(ws) = automation
        .create_matcher()
        .from(ws.clone())
        .timeout(1000)
        .depth(2)
        .name("Flushing volumes for filament change")
        .find_all()
    else {
        bail!("Could not find purge window");
    };

    if ws.len() == 0 {
        // continue;
        bail!("Could not find purge window");
    } else if ws.len() == 1 {
        return Ok(ws[0].clone());
    } else {
        eprintln!("Found multiple windows?");
        return Ok(ws[0].clone());
    }

    #[cfg(feature = "nope")]
    for bs in ws.iter() {
        let Ok(ws) = automation
            .create_matcher()
            .from(bs.clone())
            .timeout(1000)
            .depth(2)
            .name("Flushing volumes for filament change")
            .find_all()
        else {
            continue;
        };

        if ws.len() == 0 {
            continue;
        } else if ws.len() == 1 {
            return Ok(ws[0].clone());
        } else {
            panic!("Found multiple windows?");
        }
    }

    // let w = matcher
    //     .match_name("Flushing volumes for filament change")
    //     .find_first()?;

    // Ok(w)
    // bail!("Could not find purge window")
}
