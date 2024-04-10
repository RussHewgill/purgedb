use egui::{
    epaint,
    style::WidgetVisuals,
    text::{CCursor, CCursorRange, LayoutJob},
    vec2, AboveOrBelow, Align2, IconPainter, Id, InnerResponse, NumExt, Painter, Rect, Response,
    ScrollArea, Sense, Shape, Stroke, TextEdit, TextStyle, Ui, Vec2, Widget, WidgetInfo,
    WidgetText, WidgetType,
};
use std::hash::Hash;

use crate::types::Filament;

pub struct DropDownBox {
    id_source: Id,
    label: Option<WidgetText>,
    selected_text: WidgetText,
    width: Option<f32>,
    height: Option<f32>,
    icon: Option<IconPainter>,
    wrap_enabled: bool,
}

impl DropDownBox {
    pub fn new(id_source: impl std::hash::Hash, label: impl Into<WidgetText>) -> Self {
        Self {
            id_source: Id::new(id_source),
            label: Some(label.into()),
            selected_text: Default::default(),
            width: None,
            height: None,
            icon: None,
            wrap_enabled: false,
        }
    }

    /// Without label.
    pub fn from_id_source(id_source: impl std::hash::Hash) -> Self {
        Self {
            id_source: Id::new(id_source),
            label: Default::default(),
            selected_text: Default::default(),
            width: None,
            height: None,
            icon: None,
            wrap_enabled: false,
        }
    }

    /// Set the outer width of the button and menu.
    ///
    /// Default is [`Spacing::combo_width`].
    #[inline]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the maximum outer height of the menu.
    ///
    /// Default is [`Spacing::combo_height`].
    #[inline]
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// What we show as the currently selected value
    #[inline]
    pub fn selected_text(mut self, selected_text: impl Into<WidgetText>) -> Self {
        self.selected_text = selected_text.into();
        self
    }

    /// Show the combo box, with the given ui code for the menu contents.
    ///
    /// Returns `InnerResponse { inner: None }` if the combo box is closed.
    pub fn show_ui<R>(
        self,
        ui: &mut Ui,
        menu_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<Option<R>> {
        self.show_ui_dyn(ui, Box::new(menu_contents))
    }

    fn show_ui_dyn<'c, R>(
        self,
        ui: &mut Ui,
        menu_contents: Box<dyn FnOnce(&mut Ui) -> R + 'c>,
    ) -> InnerResponse<Option<R>> {
        let Self {
            id_source,
            label,
            selected_text,
            width,
            height,
            icon,
            wrap_enabled,
        } = self;

        let button_id = ui.make_persistent_id(id_source);

        ui.horizontal(|ui| {
            let mut ir = combo_box_dyn(
                ui,
                button_id,
                selected_text,
                menu_contents,
                icon,
                wrap_enabled,
                (width, height),
            );
            if let Some(label) = label {
                ir.response
                    .widget_info(|| WidgetInfo::labeled(WidgetType::ComboBox, label.text()));
                ir.response |= ui.label(label);
            } else {
                ir.response
                    .widget_info(|| WidgetInfo::labeled(WidgetType::ComboBox, ""));
            }
            ir
        })
        .inner
    }
}

fn combo_box_dyn<'c, R>(
    ui: &mut Ui,
    button_id: Id,
    selected_text: WidgetText,
    menu_contents: Box<dyn FnOnce(&mut Ui) -> R + 'c>,
    icon: Option<IconPainter>,
    wrap_enabled: bool,
    (width, height): (Option<f32>, Option<f32>),
) -> InnerResponse<Option<R>> {
    let popup_id = button_id.with("popup");

    let is_popup_open = ui.memory(|m| m.is_popup_open(popup_id));

    // let popup_height = ui.memory(|m| m.areas().get(popup_id).map_or(100.0, |state| state.size.y));
    let popup_height = 10.0;

    let above_or_below =
        if ui.next_widget_position().y + ui.spacing().interact_size.y + popup_height
            < ui.ctx().screen_rect().bottom()
        {
            AboveOrBelow::Below
        } else {
            AboveOrBelow::Above
        };

    let margin = ui.spacing().button_padding;
    let button_response = button_frame(ui, button_id, is_popup_open, Sense::click(), |ui| {
        let icon_spacing = ui.spacing().icon_spacing;
        // We don't want to change width when user selects something new
        let full_minimum_width = if wrap_enabled {
            // Currently selected value's text will be wrapped if needed, so occupy the available width.
            ui.available_width()
        } else {
            // Occupy at least the minimum width assigned to ComboBox.
            let width = width.unwrap_or_else(|| ui.spacing().combo_width);
            width - 2.0 * margin.x
        };
        let icon_size = Vec2::splat(ui.spacing().icon_width);
        let wrap_width = if wrap_enabled {
            // Use the available width, currently selected value's text will be wrapped if exceeds this value.
            ui.available_width() - icon_spacing - icon_size.x
        } else {
            // Use all the width necessary to display the currently selected value's text.
            f32::INFINITY
        };

        let galley =
            selected_text.into_galley(ui, Some(wrap_enabled), wrap_width, TextStyle::Button);

        // The width necessary to contain the whole widget with the currently selected value's text.
        let width = if wrap_enabled {
            full_minimum_width
        } else {
            // Occupy at least the minimum width needed to contain the widget with the currently selected value's text.
            galley.size().x + icon_spacing + icon_size.x
        };

        // Case : wrap_enabled : occupy all the available width.
        // Case : !wrap_enabled : occupy at least the minimum width assigned to Slider and ComboBox,
        // increase if the currently selected value needs additional horizontal space to fully display its text (up to wrap_width (f32::INFINITY)).
        let width = width.at_least(full_minimum_width);
        let height = galley.size().y.max(icon_size.y);

        let (_, rect) = ui.allocate_space(Vec2::new(width, height));
        let button_rect = ui.min_rect().expand2(ui.spacing().button_padding);
        let response = ui.interact(button_rect, button_id, Sense::click());
        // response.active |= is_popup_open;

        if ui.is_rect_visible(rect) {
            let icon_rect = Align2::RIGHT_CENTER.align_size_within_rect(icon_size, rect);
            let visuals = if is_popup_open {
                &ui.visuals().widgets.open
            } else {
                ui.style().interact(&response)
            };

            if let Some(icon) = icon {
                icon(
                    ui,
                    icon_rect.expand(visuals.expansion),
                    visuals,
                    is_popup_open,
                    above_or_below,
                );
            } else {
                paint_default_icon(
                    ui.painter(),
                    icon_rect.expand(visuals.expansion),
                    visuals,
                    above_or_below,
                );
            }

            let text_rect = Align2::LEFT_CENTER.align_size_within_rect(galley.size(), rect);
            ui.painter()
                .galley(text_rect.min, galley, visuals.text_color());
        }
    });

    if button_response.clicked() {
        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
    }

    let height = height.unwrap_or_else(|| ui.spacing().combo_height);

    let inner = egui::popup::popup_above_or_below_widget(
        ui,
        popup_id,
        &button_response,
        above_or_below,
        |ui| {
            ScrollArea::vertical()
                .max_height(height)
                .show(ui, |ui| {
                    // Often the button is very narrow, which means this popup
                    // is also very narrow. Having wrapping on would therefore
                    // result in labels that wrap very early.
                    // Instead, we turn it off by default so that the labels
                    // expand the width of the menu.
                    ui.style_mut().wrap = Some(false);
                    menu_contents(ui)
                })
                .inner
        },
    );

    InnerResponse {
        inner,
        response: button_response,
    }
}

fn button_frame(
    ui: &mut Ui,
    id: Id,
    is_popup_open: bool,
    sense: Sense,
    add_contents: impl FnOnce(&mut Ui),
) -> Response {
    let where_to_put_background = ui.painter().add(Shape::Noop);

    let margin = ui.spacing().button_padding;
    let interact_size = ui.spacing().interact_size;

    let mut outer_rect = ui.available_rect_before_wrap();
    outer_rect.set_height(outer_rect.height().at_least(interact_size.y));

    let inner_rect = outer_rect.shrink2(margin);
    let mut content_ui = ui.child_ui(inner_rect, *ui.layout());
    add_contents(&mut content_ui);

    let mut outer_rect = content_ui.min_rect().expand2(margin);
    outer_rect.set_height(outer_rect.height().at_least(interact_size.y));

    let response = ui.interact(outer_rect, id, sense);

    if ui.is_rect_visible(outer_rect) {
        let visuals = if is_popup_open {
            &ui.visuals().widgets.open
        } else {
            ui.style().interact(&response)
        };

        ui.painter().set(
            where_to_put_background,
            epaint::RectShape::new(
                outer_rect.expand(visuals.expansion),
                visuals.rounding,
                visuals.weak_bg_fill,
                visuals.bg_stroke,
            ),
        );
    }

    ui.advance_cursor_after_rect(outer_rect);

    response
}

fn paint_default_icon(
    painter: &Painter,
    rect: Rect,
    visuals: &WidgetVisuals,
    above_or_below: AboveOrBelow,
) {
    let rect = Rect::from_center_size(
        rect.center(),
        vec2(rect.width() * 0.7, rect.height() * 0.45),
    );

    match above_or_below {
        AboveOrBelow::Above => {
            // Upward pointing triangle
            painter.add(Shape::convex_polygon(
                vec![rect.left_bottom(), rect.right_bottom(), rect.center_top()],
                visuals.fg_stroke.color,
                Stroke::NONE,
            ));
        }
        AboveOrBelow::Below => {
            // Downward pointing triangle
            painter.add(Shape::convex_polygon(
                vec![rect.left_top(), rect.right_top(), rect.center_bottom()],
                visuals.fg_stroke.color,
                Stroke::NONE,
            ));
        }
    }
}

#[cfg(feature = "nope")]
mod dropdown2 {

    /// Dropdown widget
    pub struct DropDownBox<
        'a,
        F: FnMut(&mut Ui, &Filament) -> Response,
        // V: AsRef<str>,
        // I: Iterator<Item = V>,
        // I: Iterator<Item = (&'a str, LayoutJob)>,
        I: Iterator<Item = &'a Filament>,
    > {
        buf: &'a mut String,
        popup_id: Id,
        display: F,
        it: I,
        hint_text: WidgetText,
        filter_by_input: bool,
        select_on_focus: bool,
        desired_width: Option<f32>,
    }

    // impl<'a, F: FnMut(&mut Ui, &str) -> Response, I: Iterator<Item = (&'a str, LayoutJob)>>
    impl<'a, F: FnMut(&mut Ui, &Filament) -> Response, I: Iterator<Item = &'a Filament>>
        DropDownBox<'a, F, I>
    {
        /// Creates new dropdown box.
        pub fn from_iter(
            it: impl IntoIterator<IntoIter = I>,
            id_source: impl Hash,
            buf: &'a mut String,
            display: F,
        ) -> Self {
            Self {
                popup_id: Id::new(id_source),
                it: it.into_iter(),
                display,
                buf,
                hint_text: WidgetText::default(),
                filter_by_input: true,
                select_on_focus: false,
                desired_width: None,
            }
        }

        /// Add a hint text to the Text Edit
        pub fn hint_text(mut self, hint_text: impl Into<WidgetText>) -> Self {
            self.hint_text = hint_text.into();
            self
        }

        /// Determine whether to filter box items based on what is in the Text Edit already
        pub fn filter_by_input(mut self, filter_by_input: bool) -> Self {
            self.filter_by_input = filter_by_input;
            self
        }

        /// Determine whether to select the text when the Text Edit gains focus
        pub fn select_on_focus(mut self, select_on_focus: bool) -> Self {
            self.select_on_focus = select_on_focus;
            self
        }

        /// Passes through the desired width value to the underlying Text Edit
        pub fn desired_width(mut self, desired_width: f32) -> Self {
            self.desired_width = desired_width.into();
            self
        }
    }

    // impl<'a, F: FnMut(&mut Ui, &str) -> Response, I: Iterator<Item = (&'a str, LayoutJob)>> Widget
    impl<'a, F: FnMut(&mut Ui, &Filament) -> Response, I: Iterator<Item = &'a Filament>> Widget
        for DropDownBox<'a, F, I>
    {
        fn ui(self, ui: &mut Ui) -> Response {
            let Self {
                popup_id,
                buf,
                it,
                mut display,
                hint_text,
                filter_by_input,
                select_on_focus,
                desired_width,
            } = self;

            if ui.button("x").clicked() {
                buf.clear();
            }

            let mut edit = TextEdit::singleline(buf).hint_text(hint_text);
            if let Some(dw) = desired_width {
                edit = edit.desired_width(dw);
            }
            let mut edit_output = edit.show(ui);
            let mut r = edit_output.response;
            if r.gained_focus() {
                if select_on_focus {
                    edit_output
                        .state
                        .cursor
                        .set_char_range(Some(CCursorRange::two(
                            CCursor::new(0),
                            CCursor::new(buf.len()),
                        )));
                    edit_output.state.store(ui.ctx(), r.id);
                }
                ui.memory_mut(|m| m.open_popup(popup_id));
            }

            let mut changed = false;
            egui::popup_below_widget(ui, popup_id, &r, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for filament in it {
                        let text = &filament.name;

                        if filter_by_input
                            && !buf.is_empty()
                            && !text.to_lowercase().contains(&buf.to_lowercase())
                        {
                            continue;
                        }

                        if display(ui, &filament).clicked() {
                            *buf = text.to_owned();
                            changed = true;
                            eprintln!("changed");

                            ui.memory_mut(|m| m.close_popup());
                        }
                    }
                });
            });

            if changed {
                r.mark_changed();
            }

            r
        }
    }
}

#[cfg(feature = "nope")]
mod dropdown {
    /// Dropdown widget
    pub struct DropDownBox<
        'a,
        F: FnMut(&mut Ui, &str) -> Response,
        V: AsRef<str>,
        I: Iterator<Item = V>,
    > {
        buf: &'a mut String,
        popup_id: Id,
        display: F,
        it: I,
        hint_text: WidgetText,
        filter_by_input: bool,
        select_on_focus: bool,
        desired_width: Option<f32>,
    }

    impl<'a, F: FnMut(&mut Ui, &str) -> Response, V: AsRef<str>, I: Iterator<Item = V>>
        DropDownBox<'a, F, V, I>
    {
        /// Creates new dropdown box.
        pub fn from_iter(
            it: impl IntoIterator<IntoIter = I>,
            id_source: impl Hash,
            buf: &'a mut String,
            display: F,
        ) -> Self {
            Self {
                popup_id: Id::new(id_source),
                it: it.into_iter(),
                display,
                buf,
                hint_text: WidgetText::default(),
                filter_by_input: true,
                select_on_focus: false,
                desired_width: None,
            }
        }

        /// Add a hint text to the Text Edit
        pub fn hint_text(mut self, hint_text: impl Into<WidgetText>) -> Self {
            self.hint_text = hint_text.into();
            self
        }

        /// Determine whether to filter box items based on what is in the Text Edit already
        pub fn filter_by_input(mut self, filter_by_input: bool) -> Self {
            self.filter_by_input = filter_by_input;
            self
        }

        /// Determine whether to select the text when the Text Edit gains focus
        pub fn select_on_focus(mut self, select_on_focus: bool) -> Self {
            self.select_on_focus = select_on_focus;
            self
        }

        /// Passes through the desired width value to the underlying Text Edit
        pub fn desired_width(mut self, desired_width: f32) -> Self {
            self.desired_width = desired_width.into();
            self
        }
    }

    impl<'a, F: FnMut(&mut Ui, &str) -> Response, V: AsRef<str>, I: Iterator<Item = V>> Widget
        for DropDownBox<'a, F, V, I>
    {
        fn ui(self, ui: &mut Ui) -> Response {
            let Self {
                popup_id,
                buf,
                it,
                mut display,
                hint_text,
                filter_by_input,
                select_on_focus,
                desired_width,
            } = self;

            let mut edit = TextEdit::singleline(buf).hint_text(hint_text);
            if let Some(dw) = desired_width {
                edit = edit.desired_width(dw);
            }
            let mut edit_output = edit.show(ui);
            let mut r = edit_output.response;
            if r.gained_focus() {
                if select_on_focus {
                    edit_output
                        .state
                        .cursor
                        .set_char_range(Some(CCursorRange::two(
                            CCursor::new(0),
                            CCursor::new(buf.len()),
                        )));
                    edit_output.state.store(ui.ctx(), r.id);
                }
                ui.memory_mut(|m| m.open_popup(popup_id));
            }

            let mut changed = false;
            egui::popup_below_widget(ui, popup_id, &r, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for var in it {
                        let text = var.as_ref();
                        if filter_by_input
                            && !buf.is_empty()
                            && !text.to_lowercase().contains(&buf.to_lowercase())
                        {
                            continue;
                        }

                        if display(ui, text).clicked() {
                            *buf = text.to_owned();
                            changed = true;

                            ui.memory_mut(|m| m.close_popup());
                        }
                    }
                });
            });

            if changed {
                r.mark_changed();
            }

            r
        }
    }
}
