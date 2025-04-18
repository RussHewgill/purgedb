use anyhow::{Context, Result, anyhow, bail, ensure};
use tracing::{debug, error, info, trace, warn};

use std::collections::HashMap;

use egui::{
    Color32, Rect, RichText,
    text::{LayoutJob, TextWrapping},
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
    pub default_white: bool,
    pub default_black: bool,
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
        default_white: bool,
        default_black: bool,
    ) -> Self {
        Self {
            id,
            name,
            manufacturer,
            color_base,
            colors: colors.to_vec(),
            material,
            notes,
            default_white,
            default_black,
        }
    }

    pub fn set_default_white(&mut self) {
        self.default_white = true;
        self.default_black = false;
    }

    pub fn set_default_black(&mut self) {
        self.default_black = true;
        self.default_white = false;
    }

    pub fn colored_box_vert(&self, ui: &mut egui::Ui) {
        let square_size = 14.0;
        let transparent = egui::Color32::from_rgba_premultiplied(0, 0, 0, 0);
        // let stroke = (1.0, egui::Color32::BLACK);

        let layout = egui::Layout::top_down(egui::Align::Min);

        ui.with_layout(layout, |ui| {
            let (response, painter) =
                ui.allocate_painter(egui::vec2(square_size, square_size), egui::Sense::hover());
            let rect = Rect::from_min_size(response.rect.min, egui::vec2(square_size, square_size));
            if let Some(c) = self.colors.get(1) {
                painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(c.r, c.g, c.b));
                // painter.rect_stroke(rect.shrink(1.0), 0.0, stroke);
            } else {
                painter.rect_filled(rect, 0.0, transparent);
            }

            let (response, painter) =
                ui.allocate_painter(egui::vec2(square_size, square_size), egui::Sense::hover());
            let rect = Rect::from_min_size(response.rect.min, egui::vec2(square_size, square_size));
            if let Some(c) = self.colors.get(0) {
                painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(c.r, c.g, c.b));
                // painter.rect_stroke(rect.shrink(1.0), 0.0, stroke);
            } else {
                painter.rect_filled(rect, 0.0, transparent);
            }

            let (response, painter) =
                ui.allocate_painter(egui::vec2(square_size, square_size), egui::Sense::hover());
            let rect = Rect::from_min_size(response.rect.min, egui::vec2(square_size, square_size));
            painter.rect_filled(
                rect,
                0.0,
                egui::Color32::from_rgb(self.color_base.r, self.color_base.g, self.color_base.b),
            );
            // painter.rect_stroke(rect.shrink(1.0), 0.0, stroke);
        });
    }

    #[cfg(feature = "nope")]
    pub fn colored_box(&self, ui: &mut egui::Ui, vert: bool) {
        if vert {
            self.colored_box_vert(ui);
        } else {
            // self.colored_box_horiz(ui);
        }
    }

    pub fn colored_box_text(&self, vert: bool) -> LayoutJob {
        if vert {
            panic!("vert colored text not implemented");
        }
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

    pub fn colored_name(&self, ctx: &egui::Context) -> LayoutJob {
        // let mut job = LayoutJob::default();
        let mut job = self.colored_box_text(false);

        let col = if ctx.style().visuals.dark_mode {
            egui::Color32::WHITE
        } else {
            egui::Color32::BLACK
        };

        job.append(
            &format!("{} {}", &self.manufacturer, &self.name),
            // &self.name,
            2.0,
            egui::TextFormat {
                // font_id: FontId::new(14.0, FontFamily::Proportional),
                color: col,
                ..Default::default()
            },
        );

        job
    }

    #[cfg(feature = "nope")]
    pub fn colored_name_with_def(&self, ctx: &egui::Context) -> LayoutJob {
        let mut job = self.colored_name(ctx);

        let text = "\u{2B1B}";

        if self.default_black {
            job.append(
                text,
                0.0,
                egui::TextFormat {
                    color: egui::Color32::BLACK,
                    ..Default::default()
                },
            );
        }

        if self.default_white {
            debug!("default_white = {}", self.default_white);

            job.append(
                text,
                0.0,
                egui::TextFormat {
                    color: egui::Color32::WHITE,
                    ..Default::default()
                },
            );
        }

        job
    }

    pub fn stacked_colored_box(&self, ui: &mut egui::Ui, size: f32) -> egui::Response {
        let size = egui::Vec2::splat(size);

        let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());
        let rect = response.rect;

        match self.colors.len() {
            0 => {
                let c = self.color_base;
                painter.rect_filled(rect, 0., egui::Color32::from_rgb(c.r, c.g, c.b));
            }
            1 => {
                let c0 = self.color_base;
                let r0 = Rect::from_min_max(rect.min, rect.center_bottom());
                painter.rect_filled(r0, 0., egui::Color32::from_rgb(c0.r, c0.g, c0.b));

                let c1 = self.colors[0];
                let r1 = Rect::from_min_max(rect.center_top(), rect.max);
                painter.rect_filled(r1, 0., egui::Color32::from_rgb(c1.r, c1.g, c1.b));
                //
            }
            2 => {
                let x = rect.width() / 3.;

                let c0 = self.color_base;
                let r0 = Rect::from_min_max(rect.min, rect.left_bottom() + egui::Vec2::new(x, 0.));
                painter.rect_filled(r0, 0., egui::Color32::from_rgb(c0.r, c0.g, c0.b));

                let c1 = self.colors[0];
                let r1 = Rect::from_min_max(
                    rect.left_top() + egui::Vec2::new(x, 0.),
                    rect.left_bottom() + egui::Vec2::new(2. * x, 0.),
                );
                painter.rect_filled(r1, 0., egui::Color32::from_rgb(c1.r, c1.g, c1.b));

                let c2 = self.colors[1];
                let r2 = Rect::from_min_max(rect.right_top() - egui::Vec2::new(x, 0.), rect.max);
                painter.rect_filled(r2, 0., egui::Color32::from_rgb(c2.r, c2.g, c2.b));
            }
            n => eprintln!("unexpected number of colors: {}", n),
        }
        painter.rect_stroke(
            rect,
            0.,
            (1., egui::Color32::BLACK),
            egui::StrokeKind::Middle,
        );

        response
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
