use std::collections::HashMap;

use egui::{
    text::{LayoutJob, TextWrapping},
    RichText,
};
use hex_color::HexColor;

#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Filament {
    pub id: u32,
    pub name: String,
    pub manufacturer: String,
    // pub color: (u8, u8, u8),
    pub color_base: HexColor,
    // pub material: Material,
    pub colors: Vec<HexColor>,
    pub material: String,
    pub notes: String,
}

impl Filament {
    pub fn new(
        id: u32,
        name: String,
        manufacturer: String,
        color_base: HexColor,
        colors: &[HexColor],
        material: String,
        notes: String,
    ) -> Self {
        Self {
            id,
            name,
            manufacturer,
            color_base,
            colors: colors.to_vec(),
            material,
            notes,
        }
    }

    pub fn colored_box(&self, vert: bool) -> LayoutJob {
        // RichText::new("\u{2B1B}").color(
        //   egui::Color32::from_rgb(self.color_base.r, self.color_base.g, self.color_base.b)
        // )

        let text = if vert { "\u{2B1B}\n" } else { "\u{2B1B}" };

        let mut job = LayoutJob::default();

        // job.wrap.max_width = 1.;
        // job.wrap.break_anywhere = true;

        if let Some(c) = self.colors.get(1) {
            job.append(
                text,
                0.0,
                egui::TextFormat {
                    color: egui::Color32::from_rgb(c.r, c.g, c.b),
                    ..Default::default()
                },
            );
        } else {
            job.append(
                text,
                0.0,
                egui::TextFormat {
                    color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 0),
                    ..Default::default()
                },
            );
        }

        if let Some(c) = self.colors.get(0) {
            job.append(
                text,
                0.0,
                egui::TextFormat {
                    color: egui::Color32::from_rgb(c.r, c.g, c.b),
                    ..Default::default()
                },
            );
        } else {
            job.append(
                text,
                0.0,
                egui::TextFormat {
                    color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 0),
                    ..Default::default()
                },
            );
        }

        job.append(
            text,
            0.0,
            egui::TextFormat {
                color: egui::Color32::from_rgb(
                    self.color_base.r,
                    self.color_base.g,
                    self.color_base.b,
                ),
                ..Default::default()
            },
        );

        job
    }

    pub fn colored_name(&self) -> LayoutJob {
        // let mut job = LayoutJob::default();
        let mut job = self.colored_box(false);

        job.append(
            &format!("{} {}", &self.manufacturer, &self.name),
            // &self.name,
            2.0,
            egui::TextFormat {
                // font_id: FontId::new(14.0, FontFamily::Proportional),
                color: egui::Color32::BLACK,
                ..Default::default()
            },
        );

        job
    }

    #[cfg(feature = "nope")]
    pub fn display(&self) -> String {
        let Self {
            name,
            manufacturer,
            color_base: color,
            ..
        } = self;
        let color = self.display_color();
        // format!("{manufacturer} {name} {color} ({material})")
        // format!("{manufacturer} {name} {color}")
        format!("{manufacturer} {name}")
    }

    #[cfg(feature = "nope")]
    pub fn display_color(&self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}",
            self.color_base.r, self.color_base.g, self.color_base.b
        )
    }
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Material {
    PLA,
    PETG,
    ABS,
    ASA,
}

#[derive(Debug, Default, Clone)]
pub struct FilamentMap {
    pub filaments: HashMap<u32, Filament>,
    // filters:
}

impl FilamentMap {
    pub fn new(filaments: HashMap<u32, Filament>) -> Self {
        Self { filaments }
    }

    pub fn get(&self, id: &u32) -> Option<&Filament> {
        self.filaments.get(id)
    }
}

// bitflags::bitflags! {
//   // Attributes can be applied to flags types
//   #[repr(transparent)]
//   #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
//   pub struct FilamentFlags: u32 {
//       const GLITTER   = 0b00000001;
//       const GRADIENT  = 0b00000010;
//       const MATTE     = 0b00000100;
//       const SILK      = 0b00001000;
//       const DUAL      = 0b00010000;
//       const TRIPLE    = 0b00100000;
//   }
// }

// pub struct FilamentFlags {
//   pub glitter: bool,
//   pub gradient: bool,
//   pub dual: bool,
//   pub triple: bool,
//   pub matte: bool,
// }
