use anyhow::{Context, Result, anyhow, bail, ensure};

use uiautomation::{
    UIAutomation, UIElement, UIMatcher, UITreeWalker, core::UICondition, patterns::UIValuePattern,
};

pub fn get_num_filaments() -> Result<usize> {
    let automation = UIAutomation::new().unwrap();

    let w = find_purge_window(&automation)?;

    let p = automation.create_property_condition(
        uiautomation::types::UIProperty::ControlType,
        50016.into(), // spinner
        None,
    )?;

    let cs = w.find_all(uiautomation::types::TreeScope::Descendants, &p)?;

    eprintln!("cs.len() = {}", cs.len());

    Ok(cs.len() - 1)
}

pub fn send_purge_values(vals: &[u32]) -> Result<()> {
    let automation = UIAutomation::new().unwrap();
    // let walker = automation.get_control_view_walker().unwrap();
    // let root = automation.get_root_element().unwrap();

    // print_element(&walker, &root, 0).unwrap();
    // w.set_focus()?;

    let w = find_purge_window(&automation)?;

    let p = automation.create_property_condition(
        uiautomation::types::UIProperty::ControlType,
        50016.into(), // spinner
        None,
    )?;

    let cs = w.find_all(uiautomation::types::TreeScope::Descendants, &p)?;

    let num_cells = cs.len() - 1;
    eprintln!("cs.len() = {}", num_cells);

    let grid_size = (num_cells as f64).sqrt() as usize;

    let expected_values = num_cells - grid_size;

    ensure!(
        vals.len() == expected_values,
        "Expected {} values (grid size {} minus {} diagonal elements), got {}",
        expected_values,
        grid_size,
        grid_size,
        vals.len()
    );

    let mut val_index = 0;

    for (i, c) in (&cs[..cs.len() - 1]).iter().enumerate() {
        // let name = c.get_name()?;
        // let class = c.get_classname()?;
        // eprintln!("{}: {} - {}", i, name, class);

        // Calculate row and column in the grid
        let row = i / grid_size;
        let col = i % grid_size;

        // Skip diagonal elements (where row == col)
        if row == col {
            eprintln!("Skipping diagonal element at ({}, {})", row, col);
            continue;
        }

        let p = c.get_pattern::<UIValuePattern>()?;
        p.set_value(&format!("{}", vals[val_index]))?;

        val_index += 1;
        // break;
    }

    // walker.get_

    // eprintln!("w: {:?}", w);

    // let cond = UICondition

    // let e = w.find_first(uiautomation::types::TreeScope::Descendants, cond)?;

    Ok(())
}

#[cfg(feature = "nope")]
pub fn swap_extruder(automation: &UIAutomation, elem: &UIElement) -> Result<()> {
    Ok(())
}

pub fn swap_extruder() -> Result<()> {
    let automation = UIAutomation::new().unwrap();
    let w = find_purge_window(&automation)?;

    let p = automation.create_property_condition(
        uiautomation::types::UIProperty::ControlType,
        50003.into(), // ComboBox
        None,
    )?;

    let cs = w.find_all(uiautomation::types::TreeScope::Descendants, &p)?;

    eprintln!("cs.len() = {}", cs.len());

    let c = &cs[0];

    // let p = c.get_pattern::<UIValuePattern>()?;
    // let v = p.get_value()?;
    // eprintln!("v = {}", v);

    // c.set_focus()?;

    // c.send_keys("{down}", 10)?;

    Ok(())
}

fn find_purge_window(automation: &UIAutomation) -> Result<UIElement> {
    let matcher = UIMatcher::new(automation.clone());

    let w = matcher
        .match_name("Flushing Volumes for filament change")
        .find_first()?;

    Ok(w)
}
