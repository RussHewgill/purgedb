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

pub fn main() -> Result<()> {
  // let key = 0x41; // a

  // press_key(key, KEY_FLAGS_DOWN)?;
  // std::thread::sleep(std::time::Duration::from_millis(30));
  // press_key(key, KEY_FLAGS_UP)?;

  // send_number(123)?;

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
