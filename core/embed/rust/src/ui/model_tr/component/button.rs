use crate::ui::{
    component::{Component, Event, EventCtx},
    display::{self, Color, Font},
    event::{ButtonEvent, PhysicalButton},
    geometry::{Offset, Point, Rect},
    model_tr::theme,
};

#[derive(PartialEq)]
pub enum ButtonMsg {
    Clicked,
    LongPressed,
}

#[derive(Copy, Clone)]
pub enum ButtonPos {
    Left,
    Middle,
    Right,
}

impl ButtonPos {
    pub fn hit(&self, b: &PhysicalButton) -> bool {
        matches!(
            (self, b),
            (Self::Left, PhysicalButton::Left)
                | (Self::Middle, PhysicalButton::Both)
                | (Self::Right, PhysicalButton::Right)
        )
    }
}

pub struct Button<T> {
    bounds: Rect,
    pos: ButtonPos,
    content: ButtonContent<T>,
    styles: ButtonStyleSheet,
    state: State,
}

impl<T: AsRef<str>> Button<T> {
    pub fn new(pos: ButtonPos, content: ButtonContent<T>, styles: ButtonStyleSheet) -> Self {
        Self {
            pos,
            content,
            styles,
            bounds: Rect::zero(),
            state: State::Released,
        }
    }

    pub fn with_text(pos: ButtonPos, text: T, styles: ButtonStyleSheet) -> Self {
        Self::new(pos, ButtonContent::Text(text), styles)
    }

    pub fn with_icon(pos: ButtonPos, image: &'static [u8], styles: ButtonStyleSheet) -> Self {
        Self::new(pos, ButtonContent::Icon(image), styles)
    }

    pub fn content(&self) -> &ButtonContent<T> {
        &self.content
    }

    fn style(&self) -> ButtonStyle {
        match self.state {
            State::Released => self.styles.normal,
            State::Pressed => self.styles.active,
        }
    }

    /// Changing the icon content of the button.
    pub fn set_icon(&mut self, image: &'static [u8]) {
        self.content = ButtonContent::Icon(image);
    }

    /// Changing the text content of the button.
    pub fn set_text(&mut self, text: T) {
        self.content = ButtonContent::Text(text);
    }

    /// Changing the style of the button.
    pub fn set_style(&mut self, styles: ButtonStyleSheet) {
        self.styles = styles;
    }

    // Setting the visual state of the button.
    fn set(&mut self, ctx: &mut EventCtx, state: State) {
        if self.state != state {
            self.state = state;
            ctx.request_paint();
        }
    }

    // Setting the visual state of the button.
    pub fn set_pressed(&mut self, ctx: &mut EventCtx, is_pressed: bool) {
        let new_state = if is_pressed {
            State::Pressed
        } else {
            State::Released
        };
        self.set(ctx, new_state);
    }

    /// Return the full area of the button according
    /// to its current style, content and position.
    fn get_current_area(&self) -> Rect {
        let style = self.style();
        // Button width may be forced. Otherwise calculate it.
        let button_width = if let Some(width) = style.force_width {
            width
        } else {
            let outline = if style.with_outline {
                theme::BUTTON_OUTLINE
            } else {
                0
            };
            let content_width = match &self.content {
                ButtonContent::Text(text) => style.font.text_width(text.as_ref()) - 1,
                ButtonContent::Icon(icon) => display::toif_dimensions(icon, true).0 as i32 - 1,
            };
            content_width + 2 * outline
        };

        match self.pos {
            ButtonPos::Left => self.bounds.split_left(button_width).0,
            ButtonPos::Right => self.bounds.split_right(button_width).1,
            ButtonPos::Middle => self.bounds.split_center(button_width),
        }
    }

    /// Determine baseline point for the text.
    fn get_baseline(&self, style: &ButtonStyle) -> Point {
        // Arms and outline require the text to be elevated.
        if style.with_arms || style.with_outline {
            let offset = theme::BUTTON_OUTLINE;
            self.get_current_area().bottom_left() + Offset::new(offset, -offset)
        } else {
            self.get_current_area().bottom_left()
        }
    }
}

impl<T> Component for Button<T>
where
    T: AsRef<str>,
{
    type Msg = ButtonMsg;

    fn place(&mut self, bounds: Rect) -> Rect {
        self.bounds = bounds;
        self.get_current_area()
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        // Everything should be handled by `ButtonController`
        // TODO: could be completely deleted, but `ResultPopup` is using Button.event()
        match event {
            Event::Button(ButtonEvent::ButtonPressed(which)) if self.pos.hit(&which) => {
                self.set(ctx, State::Pressed);
            }
            Event::Button(ButtonEvent::ButtonReleased(which)) if self.pos.hit(&which) => {
                if matches!(self.state, State::Pressed) {
                    self.set(ctx, State::Released);
                    return Some(ButtonMsg::Clicked);
                }
            }
            _ => {}
        };
        None
    }

    fn paint(&mut self) {
        let style = self.style();
        let text_color = style.text_color;
        let background_color = text_color.negate();
        let area = self.get_current_area();

        // TODO: support another combinations of text and icons
        // - text with OK icon on left

        // Optionally display "arms" at both sides of content, or create
        // a nice rounded outline around it.
        // By default just fill the content background.
        if style.with_arms {
            // Prepare space for both the arms and content with BG color.
            // Arms are icons 10*6 pixels.
            let area_to_fill = area.extend_left(10).extend_right(15);
            display::rect_fill(area_to_fill, background_color);

            // Paint both arms.
            // TODO: for "CONFIRM" there is one space at the right, but for "SELECT" there are two
            let left_arm_center = area.left_center() - Offset::x(3) + Offset::y(3);
            let right_arm_center = area.right_center() + Offset::x(9) + Offset::y(3);
            display::icon(
                left_arm_center,
                theme::ICON_ARM_LEFT,
                text_color,
                background_color,
            );
            display::icon(
                right_arm_center,
                theme::ICON_ARM_RIGHT,
                text_color,
                background_color,
            );
        } else if style.with_outline {
            display::rect_outline_rounded2(area, text_color, background_color);
        } else {
            display::rect_fill(area, background_color)
        }

        match &self.content {
            ButtonContent::Text(text) => {
                display::text(
                    self.get_baseline(&style),
                    text.as_ref(),
                    style.font,
                    text_color,
                    background_color,
                );
            }
            ButtonContent::Icon(icon) => {
                // Accounting for the 8*8 icon with empty left column and bottom row.
                let icon_center = area.center() + Offset::uniform(1);
                display::icon(icon_center, icon, text_color, background_color);
            }
        }
    }
}

#[cfg(feature = "ui_debug")]
impl<T> crate::trace::Trace for Button<T>
where
    T: AsRef<str> + crate::trace::Trace,
{
    fn trace(&self, t: &mut dyn crate::trace::Tracer) {
        t.open("Button");
        match &self.content {
            ButtonContent::Text(text) => t.field("text", text),
            ButtonContent::Icon(_) => t.symbol("icon"),
        }
        t.close();
    }
}

#[derive(PartialEq, Eq)]
enum State {
    Released,
    Pressed,
}

pub enum ButtonContent<T> {
    Text(T),
    Icon(&'static [u8]),
}

pub struct ButtonStyleSheet {
    pub normal: ButtonStyle,
    pub active: ButtonStyle,
}

#[derive(Clone, Copy)]
pub struct ButtonStyle {
    pub font: Font,
    pub text_color: Color,
    pub with_outline: bool,
    pub with_arms: bool,
    pub force_width: Option<i32>,
}

// TODO: currently `button_default` and `button_cancel`
// are the same - decide whether to differentiate them.
// In Figma, they are not differentiated.

impl ButtonStyleSheet {
    pub fn new(
        normal_color: Color,
        active_color: Color,
        with_outline: bool,
        with_arms: bool,
        force_width: Option<i32>,
    ) -> Self {
        Self {
            normal: ButtonStyle {
                font: theme::FONT_BUTTON,
                text_color: normal_color,
                with_outline,
                with_arms,
                force_width,
            },
            active: ButtonStyle {
                font: theme::FONT_BUTTON,
                text_color: active_color,
                with_outline,
                with_arms,
                force_width,
            },
        }
    }

    // White text in normal mode.
    pub fn default(with_outline: bool, with_arms: bool, force_width: Option<i32>) -> Self {
        Self::new(theme::FG, theme::BG, with_outline, with_arms, force_width)
    }

    // Black text in normal mode.
    pub fn cancel(with_outline: bool, with_arms: bool, force_width: Option<i32>) -> Self {
        Self::new(theme::FG, theme::BG, with_outline, with_arms, force_width)
        // Self::new(theme::BG, theme::FG, with_outline, with_arms)
    }
}
