use std::{fmt::Display, str::FromStr};

use eframe::egui::TextBuffer;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ValText<T> {
  text: String,
  val: Option<T>,
}

impl<T: Copy> ValText<T> {
  pub fn with_validator(validator: impl Fn(&str) -> Option<T> + 'static) -> Self {
    Self {
      text: Default::default(),
      val: Default::default(),
    }
  }

  pub fn get_val(&self) -> Option<T> {
    self.val
  }

  pub fn is_valid(&self) -> bool {
    self.val.is_some()
  }
}

impl<T: Display> ValText<T> {
  pub fn set_val(&mut self, val: T) {
    self.text = val.to_string();
    self.val = Some(val);
  }
}

impl<T: FromStr> Default for ValText<T> {
  fn default() -> Self {
    Self {
      text: Default::default(),
      val: Default::default(),
    }
  }
}

impl TextBuffer for ValText<u32> {
  fn is_mutable(&self) -> bool {
    true
  }

  fn as_str(&self) -> &str {
    self.text.as_str()
  }

  fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
    let n = self.text.insert_text(text, char_index);
    // self.val = (self.validator)(&self.text);
    self.val = (|text: &str| text.parse().ok())(&self.text);
    n
  }

  fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
    self.text.delete_char_range(char_range);
    self.val = (|text: &str| text.parse().ok())(&self.text);
  }
}

// impl<T> TextBuffer for ValText<T> {
//   fn is_mutable(&self) -> bool {
//       true
//   }

//   fn as_str(&self) -> &str {
//       self.text.as_str()
//   }

//   // fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
//   //     let n = self.text.insert_text(text, char_index);
//   //     // self.val = (self.validator)(&self.text);
//   //     // n
//   //     unimplemented!()
//   // }

//   // fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
//   //     self.text.delete_char_range(char_range);
//   //     // self.val = (self.validator)(&self.text);
//   //     unimplemented!()
//   // }
// }

#[cfg(feature = "nope")]
mod generic {
  #[derive(Debug, serde::Serialize, serde::Deserialize)]
  pub struct ValText<T> {
    text: String,
    val: Option<T>,
    #[allow(clippy::type_complexity)]
    validator: Box<dyn Fn(&str) -> Option<T>>,
  }

  impl<T: Copy> ValText<T> {
    pub fn with_validator(validator: impl Fn(&str) -> Option<T> + 'static) -> Self {
      Self {
        text: Default::default(),
        val: Default::default(),
        validator: Box::new(validator),
      }
    }

    pub fn get_val(&self) -> Option<T> {
      self.val
    }

    pub fn is_valid(&self) -> bool {
      self.val.is_some()
    }
  }

  impl<T: Display> ValText<T> {
    pub fn set_val(&mut self, val: T) {
      self.text = val.to_string();
      self.val = Some(val);
    }
  }

  impl<T: FromStr> Default for ValText<T> {
    fn default() -> Self {
      Self {
        text: Default::default(),
        val: Default::default(),
        validator: Box::new(|text| text.parse().ok()),
      }
    }
  }

  impl<T> TextBuffer for ValText<T> {
    fn is_mutable(&self) -> bool {
      true
    }

    fn as_str(&self) -> &str {
      self.text.as_str()
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
      let n = self.text.insert_text(text, char_index);
      self.val = (self.validator)(&self.text);
      n
    }

    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
      self.text.delete_char_range(char_range);
      self.val = (self.validator)(&self.text);
    }
  }
}
