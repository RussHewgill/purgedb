use egui::{text::LayoutJob, RichText};
use hex_color::HexColor;


#[derive(Debug, Default, Clone, PartialEq)]
pub struct Filament {
  pub id: u32,
  pub name: String,
  pub manufacturer: String,
  // pub color: (u8, u8, u8),
  pub color: HexColor,
  // pub material: Material,
}

impl Filament {
  pub fn new(
    id: u32, 
    name: String, 
    manufacturer: String, 
    color: HexColor,
  ) -> Self { 
      Self { id, name, manufacturer, color } 
    }
  
  pub fn colored_box(&self) -> RichText {
    RichText::new("\u{2B1B}").color(
      egui::Color32::from_rgb(self.color.r, self.color.g, self.color.b)
    )
  }


  // pub fn colored_box(&self) -> LayoutJob {
  //   let mut job = LayoutJob::default();
  //   job.append("\u{2B1B}", 0.0, egui::TextFormat {
  //     color: egui::Color32::from_rgb(self.color.r, self.color.g, self.color.b),
  //     ..Default::default()
  //   });
  //   job
  // }

  pub fn colored_name(&self) -> LayoutJob {
    let mut job = LayoutJob::default();

    job.append("\u{2B1B}", 0.0, egui::TextFormat {
      color: egui::Color32::from_rgb(self.color.r, self.color.g, self.color.b),
      ..Default::default()
    });

    job.append(
      // &format!("{} {}", &self.name, &self.display_color()),
      &self.name,
      2.0,
      egui::TextFormat {
          // font_id: FontId::new(14.0, FontFamily::Proportional),
          color: egui::Color32::BLACK,
          ..Default::default()
      },
    );

    job
  }

  // pub fn display(&self) -> LayoutJob {
  //   let Self { name, manufacturer, color, .. } = self;
  //   let color = self.display_color();
  //   // format!("{manufacturer} {name} {color} ({material})")
  //   // format!("{manufacturer} {name} {color}")
  //   // format!("{manufacturer} {name}")

  //   // RichText::new()
  //   let mut job = LayoutJob::default();

  //   job
  // }

  // pub fn display(&self) -> String {
  //   let Self { name, manufacturer, color, .. } = self;
  //   let color = self.display_color();
  //   // format!("{manufacturer} {name} {color} ({material})")
  //   // format!("{manufacturer} {name} {color}")
  //   format!("{manufacturer} {name}")
  // }

  pub fn display_color(&self) -> String {
    format!("#{:02X}{:02X}{:02X}", self.color.r, self.color.g, self.color.b)
  }
}

// pub enum Material {
//   PLA,
//   PETG,
//   ABS,
//   ASA
// }
