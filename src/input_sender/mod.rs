use anyhow::Result;
// use windows::{core::Result, Win32::System::Threading::*};
use windows::Win32::System::Threading::*;

use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, VIRTUAL_KEY,
};

pub const KEY_FLAGS_DOWN: KEYBD_EVENT_FLAGS = KEYBD_EVENT_FLAGS(0);
pub const KEY_FLAGS_UP: KEYBD_EVENT_FLAGS = KEYBD_EVENT_FLAGS(0x0002);

// const DELAY: u64 = 30;
const DELAY: u64 = 3;

fn get_window(name: &str) -> Result<windows::Win32::Foundation::HWND> {
    use windows::{core::*, Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*};
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
    use windows::{core::*, Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*};
    match unsafe {
        FindWindowExW(
            hwnd,
            // hwnd,
            prev_hwnd.unwrap_or(HWND(std::ptr::null_mut())),
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

unsafe extern "system" fn enum_child_proc(
    hwnd: windows::Win32::Foundation::HWND,
    lparam: windows::Win32::Foundation::LPARAM,
) -> windows::Win32::Foundation::BOOL {
    use windows::{core::*, Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*};

    let mut class_name = [0u16; 256];
    GetClassNameW(hwnd, &mut class_name);

    // eprintln!("sub hwnd = {:?}", hwnd);

    let class_name = String::from_utf16_lossy(&class_name)
        .trim_end_matches('\0')
        .to_string();

    if class_name != "wxWindowNR" {
        return TRUE;
    }

    let mut caption = [0u16; 256];
    GetWindowTextW(hwnd, &mut caption);

    let caption = String::from_utf16_lossy(&caption)
        .trim_end_matches('\0')
        .to_string();

    if caption != "OK" {
        return TRUE;
    } else {
        FOUND_HWND = hwnd;
        return FALSE;
    }

    // // Check if this is the text box we're looking for
    // if class_name == "Edit" {
    //     // You might want to add more conditions here to ensure it's the right text box
    //     FOUND_HWND = hwnd;
    //     FALSE // Stop enumeration
    // } else {
    //     TRUE // Continue enumeration
    // }
}

pub fn focus_first_input() -> Result<()> {
    use windows::{core::*, Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*};

    let parent_hwnd = get_window("Flushing Volumes for filament change")?;

    eprintln!("hwnd 0 = {:?}", parent_hwnd);

    unsafe {
        let _ = SetForegroundWindow(parent_hwnd);
    };

    unsafe {
        let b = EnumChildWindows(
            parent_hwnd,
            Some(enum_child_proc),
            // (),
            LPARAM(0),
        );

        eprintln!("b = {:?}", b);

        eprintln!("FOUND_HWND = {:?}", FOUND_HWND);

        if !FOUND_HWND.is_invalid() {
            eprintln!("setting focus");

            let proc_id = GetWindowThreadProcessId(parent_hwnd, Some(std::ptr::null_mut()));
            eprintln!("proc_id = {:08X}", proc_id);

            let current_thread = GetCurrentThreadId();
            let b = AttachThreadInput(current_thread, proc_id, TRUE);
            eprintln!("b = {:?}", b);

            let b = windows::Win32::UI::Input::KeyboardAndMouse::SetFocus(FOUND_HWND);
            eprint!("b = {:?}", b);

            // let res = SendMessageW(FOUND_HWND, WM_SETFOCUS, WPARAM(0), LPARAM(0));
            // eprintln!("res = {:?}", res);

            //
        }

        //
    }

    shift_tab(16)?;

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
