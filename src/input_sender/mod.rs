pub mod automation;

use anyhow::{Context, Result, anyhow, bail, ensure};

use windows::Win32::Foundation::{HWND, LPARAM};
// use windows::{core::Result, Win32::System::Threading::*};
use windows::Win32::System::Threading::*;

use windows::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, INPUT_KEYBOARD, KEYBD_EVENT_FLAGS, KEYBDINPUT, SendInput, VIRTUAL_KEY,
};

pub const KEY_FLAGS_DOWN: KEYBD_EVENT_FLAGS = KEYBD_EVENT_FLAGS(0);
pub const KEY_FLAGS_UP: KEYBD_EVENT_FLAGS = KEYBD_EVENT_FLAGS(0x0002);

// const DELAY: u64 = 30;
const DELAY: u64 = 3;

pub fn test_bs() -> Result<()> {
    //

    // let n = automation::get_num_filaments()?;
    // eprintln!("n = {:?}", n);

    // automation::open_purge_window_bambu()?;

    // let purge_values = vec![011, 012, 013, 021, 022, 023, 031, 032, 033, 041, 042, 043];

    // let purge_values = vec![012, 013, 021, 023, 031, 032];

    #[rustfmt::skip]
    let purge_values = vec![
        0, 1,
        2, 3,
        4, 5
    ];

    #[rustfmt::skip]
    let purge_values = vec![
        0, 1, 2,
        3, 4, 5,
        6, 7, 8,
        9, 10, 11,
    ];

    // let purge_values = vec![270, 210, 999, 470, 270, 170, 160, 180, 160, 999, 270, 170];

    // automation::send_purge_values_bambu(&purge_values, false)?;
    automation::send_purge_values_orca(&purge_values)?;

    // automation::swap_extruder()?;

    Ok(())
}

#[cfg(feature = "nope")]
pub fn test_bs() -> Result<()> {
    use windows::{Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*, core::*};

    let parent_hwnd = get_window("Flushing Volumes for filament change").unwrap();

    eprintln!("hwnd = {:?}", parent_hwnd);

    unsafe {
        let _ = windows::Win32::UI::WindowsAndMessaging::SetForegroundWindow(parent_hwnd);
    };

    #[cfg(feature = "nope")]
    unsafe {
        let proc_id = GetWindowThreadProcessId(parent_hwnd, Some(std::ptr::null_mut()));
        // eprintln!("proc_id = {:08X}", proc_id);

        let current_thread = GetCurrentThreadId();
        let b = AttachThreadInput(current_thread, proc_id, true);
        // eprintln!("b = {:?}", b);

        let b = windows::Win32::UI::Input::KeyboardAndMouse::SetFocus(Some(FOUND_HWND));
    }

    #[cfg(feature = "nope")]
    unsafe {
        // let b = EnumChildWindows(
        //     Some(parent_hwnd),
        //     Some(enum_child_proc_bambu),
        //     // Some(enum_child_proc_orca),
        //     // (),
        //     LPARAM(0),
        // );

        // eprintln!("b = {:?}", b);

        // eprintln!("FOUND_HWND = {:?}", FOUND_HWND);

        // let hwnd = 0xf1814;
        // let hwnd = 0xe168e;
        // let hwnd = 0x121778;
        // let hwnd = 0xa17ae;
        // let hwnd = 0x122274;
        // let hwnd = 0xf1642;

        // let hwnd = HWND(hwnd as *mut std::ffi::c_void);
        // FOUND_HWND = hwnd;

        // let hwnd_edit = find_first_edit(FOUND_HWND).unwrap();
        // set_focus_to_edit(FOUND_HWND, hwnd_edit).unwrap();

        // #[allow(static_mut_refs)]
        // if !FOUND_HWND.is_invalid() {
        //     let hwnd_edit = find_first_edit(FOUND_HWND).unwrap();
        //     set_focus_to_edit(FOUND_HWND, hwnd_edit).unwrap();
        // }

        #[allow(static_mut_refs)]
        #[cfg(feature = "nope")]
        if !FOUND_HWND.is_invalid() {
            eprintln!("setting focus");

            let proc_id = GetWindowThreadProcessId(parent_hwnd, Some(std::ptr::null_mut()));
            // eprintln!("proc_id = {:08X}", proc_id);

            let current_thread = GetCurrentThreadId();
            let b = AttachThreadInput(current_thread, proc_id, true);
            // eprintln!("b = {:?}", b);

            let b = windows::Win32::UI::Input::KeyboardAndMouse::SetFocus(Some(FOUND_HWND));
            // eprint!("b = {:?}", b);

            // let res = SendMessageW(FOUND_HWND, WM_SETFOCUS, WPARAM(0), LPARAM(0));
            // eprintln!("res = {:?}", res);

            //
        } else {
            eprintln!("FOUND_HWND is invalid");
        }

        //
    }

    // focus_nested_element(parent_hwnd).unwrap();

    // let num_filaments = 4;
    // tabs(num_filaments + 5)?;

    // tabs(4)?;

    // std::thread::sleep(std::time::Duration::from_millis(400));

    Ok(())
}

#[cfg(feature = "nope")]
fn focus_nested_element(hwnd: HWND) -> Result<()> {
    use windows::{UI::UIAutomation::*, Win32::UI::Accessibility::*, core::*};

    // Initialize COM for this thread
    unsafe {
        windows::Win32::System::Com::CoInitializeEx(
            None,
            windows::Win32::System::Com::COINIT_MULTITHREADED,
        )
        .unwrap();
    }

    // Initialize UI Automation
    let automation: IUIAutomation = unsafe {
        windows::Win32::System::Com::CoCreateInstance(
            &windows::Win32::UI::Accessibility::CUIAutomation,
            None,
            windows::Win32::System::Com::CLSCTX_INPROC_SERVER,
        )?
    };

    // Get the element from the HWND
    let element: IUIAutomationElement = unsafe { automation.ElementFromHandle(hwnd)? };

    // Get the automation ID or name (for debugging)
    let name: BSTR = unsafe { element.CurrentName()? };
    let name_str = name.to_string();
    println!("Element name: {}", name_str);

    // Find the table element
    let table_condition = unsafe {
        automation.CreatePropertyCondition(
            UIA_ControlTypePropertyId,
            VARIANT::from(UIA_TableControlTypeId),
        )?
    };

    Ok(())
}

fn get_window(name: &str) -> Result<windows::Win32::Foundation::HWND> {
    use windows::{Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*, core::*};
    let window_name: HSTRING = name.into();
    let hwnd = unsafe {
        FindWindowW(
            PCWSTR::null(),
            // PCWSTR::null(),
            &window_name,
        )
    }?;
    if hwnd.is_invalid() {
        anyhow::bail!("Window not found");
    }
    Ok(hwnd)
}

fn get_sub_window(
    hwnd: windows::Win32::Foundation::HWND,
    prev_hwnd: Option<windows::Win32::Foundation::HWND>,
    name: &str,
) -> Result<windows::Win32::Foundation::HWND> {
    use windows::{Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*, core::*};
    match unsafe {
        FindWindowExW(
            Some(hwnd),
            // hwnd,
            // prev_hwnd.unwrap_or(HWND(std::ptr::null_mut())),
            prev_hwnd,
            // &text_box_class,
            // &window_name,
            PCWSTR::null(),
            PCWSTR::null(),
        )
    } {
        Ok(hwnd) => Ok(hwnd),
        Err(e) => {
            anyhow::bail!("Error: {:?}", e);
        }
    }
}

// Global variable to store the found HWND
static mut FOUND_HWND: windows::Win32::Foundation::HWND =
    windows::Win32::Foundation::HWND(std::ptr::null_mut());

unsafe extern "system" fn enum_child_proc_bambu(
    hwnd: windows::Win32::Foundation::HWND,
    lparam: windows::Win32::Foundation::LPARAM,
) -> windows::core::BOOL {
    use windows::{Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*, core::*};

    let mut class_name = [0u16; 256];
    unsafe {
        GetClassNameW(hwnd, &mut class_name);
    }

    // let hinstance = unsafe { GetWindowLongPtrW(hwnd, GWL_HINSTANCE) };
    // let hinstance = unsafe { GetWindowLongPtrW(hwnd, GWLP_ID) };

    // eprintln!("hinstance = {:?}", hinstance);

    // unsafe {
    //     GetClassInfoExW(hinstance, lpszclass, lpwcx);
    // }

    eprintln!("sub hwnd = {:?}", hwnd);

    let class_name = String::from_utf16_lossy(&class_name)
        .trim_end_matches('\0')
        .to_string();

    // eprintln!("class_name = {:?}", class_name);

    // if class_name != "wxWindowNR" {
    //     return true.into();
    // }
    if class_name != "Chrome_WidgetWin_0" {
        return true.into();
    }

    let mut caption = [0u16; 256];
    unsafe { GetWindowTextW(hwnd, &mut caption) };

    let caption = String::from_utf16_lossy(&caption)
        .trim_end_matches('\0')
        .to_string();

    // eprintln!("caption = {:?}", caption);

    // if caption != "" {
    //     return true.into();
    // } else {
    //     unsafe { FOUND_HWND = hwnd };
    //     return false.into();
    // }

    return true.into();
}

unsafe extern "system" fn enum_child_proc_orca(
    hwnd: windows::Win32::Foundation::HWND,
    lparam: windows::Win32::Foundation::LPARAM,
) -> windows::core::BOOL {
    use windows::{Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*, core::*};

    let mut class_name = [0u16; 256];
    unsafe {
        GetClassNameW(hwnd, &mut class_name);
    }

    // eprintln!("sub hwnd = {:?}", hwnd);

    let class_name = String::from_utf16_lossy(&class_name)
        .trim_end_matches('\0')
        .to_string();

    eprintln!("class_name = {:?}", class_name);

    if class_name != "wxWindowNR" {
        return true.into();
    }

    let mut caption = [0u16; 256];
    unsafe { GetWindowTextW(hwnd, &mut caption) };

    let caption = String::from_utf16_lossy(&caption)
        .trim_end_matches('\0')
        .to_string();

    if caption != "OK" {
        return true.into();
    } else {
        unsafe { FOUND_HWND = hwnd };
        return false.into();
    }

    // // Check if this is the text box we're looking for
    // if class_name == "Edit" {
    //     // You might want to add more conditions here to ensure it's the right text box
    //     FOUND_HWND = hwnd;
    //     false // Stop enumeration
    // } else {
    //     true // Continue enumeration
    // }
}

pub fn focus_first_input(num_filaments: usize) -> Result<()> {
    use windows::{Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*, core::*};

    let parent_hwnd = get_window("Flushing Volumes for filament change")?;

    // eprintln!("hwnd 0 = {:?}", parent_hwnd);

    unsafe {
        let _ = SetForegroundWindow(parent_hwnd);
    };

    unsafe {
        let b = EnumChildWindows(
            Some(parent_hwnd),
            Some(enum_child_proc_orca),
            // (),
            LPARAM(0),
        );

        // eprintln!("b = {:?}", b);

        // eprintln!("FOUND_HWND = {:?}", FOUND_HWND);

        #[allow(static_mut_refs)]
        if !FOUND_HWND.is_invalid() {
            // eprintln!("setting focus");

            let proc_id = GetWindowThreadProcessId(parent_hwnd, Some(std::ptr::null_mut()));
            // eprintln!("proc_id = {:08X}", proc_id);

            let current_thread = GetCurrentThreadId();
            let b = AttachThreadInput(current_thread, proc_id, true);
            // eprintln!("b = {:?}", b);

            let b = windows::Win32::UI::Input::KeyboardAndMouse::SetFocus(Some(FOUND_HWND));
            // eprint!("b = {:?}", b);

            // let res = SendMessageW(FOUND_HWND, WM_SETFOCUS, WPARAM(0), LPARAM(0));
            // eprintln!("res = {:?}", res);

            //
        }

        //
    }

    // match num_filaments {
    //     2 => shift_tab(4)?,
    //     3 => shift_tab(9)?,
    //     4 => shift_tab(16)?,
    //     _ => {}
    // }

    /// only works when Orca has 4 filaments
    // shift_tab(16)?;

    /// 4 filaments
    // tabs(9)?;
    tabs(num_filaments + 5)?;

    Ok(())
}

pub fn send_number(n: u32, next: bool) -> Result<()> {
    let del = 0x2E;
    let tab = 0x09;

    press_key(del)?;
    press_key(del)?;
    press_key(del)?;

    let n = format!("{}", n);

    for x in n.chars() {
        // eprintln!("x = {:x}", x as u8);
        press_key(x as u16)?;
    }

    if next {
        press_key(tab)?;
    }

    Ok(())
}

pub fn shift_tab(n: usize) -> Result<()> {
    let tab = 0x09;
    let shift = 0x10;

    send_key(shift, KEY_FLAGS_DOWN)?;
    for _ in 0..n {
        // press_key(tab)?;
        send_key(tab, KEY_FLAGS_DOWN)?;
        std::thread::sleep(std::time::Duration::from_millis(1));
        send_key(tab, KEY_FLAGS_UP)?;
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    send_key(shift, KEY_FLAGS_UP)?;

    Ok(())
}

pub fn tab() -> Result<()> {
    let tab = 0x09;
    press_key(tab)?;
    std::thread::sleep(std::time::Duration::from_millis(DELAY));
    Ok(())
}

pub fn tabs(n: usize) -> Result<()> {
    let tab = 0x09;
    for _ in 0..n {
        press_key(tab)?;
    }
    Ok(())
}

pub fn alt_tab() -> Result<()> {
    let alt = 0x12;
    let tab = 0x09;

    send_key(alt, KEY_FLAGS_DOWN)?;
    send_key(tab, KEY_FLAGS_DOWN)?;
    std::thread::sleep(std::time::Duration::from_millis(DELAY));
    send_key(tab, KEY_FLAGS_UP)?;
    send_key(alt, KEY_FLAGS_UP)?;

    Ok(())
}

fn press_key(key: u16) -> Result<()> {
    send_key(key, KEY_FLAGS_DOWN)?;
    std::thread::sleep(std::time::Duration::from_millis(DELAY));
    send_key(key, KEY_FLAGS_UP)?;
    std::thread::sleep(std::time::Duration::from_millis(DELAY));
    Ok(())
}

fn send_key(key: u16, flags: KEYBD_EVENT_FLAGS) -> Result<()> {
    let ki = KEYBDINPUT {
        wVk: VIRTUAL_KEY(key),
        wScan: 0,
        time: 0,
        dwExtraInfo: 0,
        dwFlags: flags,
    };

    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 { ki: ki },
    };

    let cbsize = std::mem::size_of::<INPUT>() as i32;

    unsafe {
        SendInput(&[input], cbsize);
    }

    Ok(())
}
