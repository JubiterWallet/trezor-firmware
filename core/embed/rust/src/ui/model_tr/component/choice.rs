use crate::{
    time::Duration,
    ui::{
        component::{Component, Event, EventCtx, Pad},
        geometry::Rect,
    },
};

use super::{common::ChoiceItem, theme, BothButtonPressHandler, Button, ButtonMsg, ButtonPos};
use heapless::{LinearMap, Vec};

pub enum ChoicePageMsg {
    Choice(u8),
    LeftMost,
    RightMost,
}

/// Optional button on the very left or very right of the choice page
struct SideButton {
    button: Button<&'static str>,
    is_active: bool,
}

impl SideButton {
    pub fn new(pos: ButtonPos, text: &'static str) -> Self {
        Self {
            button: Button::with_text(pos, text, theme::button_default()),
            is_active: false,
        }
    }

    /// Update the current instance
    pub fn set(&mut self, text: &'static str, duration: Option<Duration>, button_area: Rect) {
        self.button.set_text(text, button_area);
        self.button.set_long_press(duration);
        self.is_active = true;
    }

    /// Deactivate the current instance
    pub fn unset(&mut self) {
        self.is_active = false;
    }

    pub fn got_triggered(&mut self, ctx: &mut EventCtx, event: Event) -> bool {
        let msg = self.button.event(ctx, event);
        msg == Some(ButtonMsg::LongPressed) && self.button.is_longpress()
            || msg == Some(ButtonMsg::Clicked) && !self.button.is_longpress()
    }
}

const MIDDLE_ROW: i32 = 72;

/// General component displaying a set of items on the screen
/// and allowing the user to select one of them.
///
/// To be used by other more specific components that will
/// supply a set of `ChoiceItem`s and will receive back
/// the index of the selected choice.
///
/// Using components can also specify the `leftmost` and `rightmost`
/// buttons and receive messages whenever they are triggered.
///
/// `select_button_map` allows for specifying custom texts of the middle
/// button according to the choice index.
pub struct ChoicePage<T, const N: usize> {
    choices: Vec<T, N>,
    select_button_map: Option<LinearMap<u8, &'static str, N>>,
    both_button_press: BothButtonPressHandler,
    pad: Pad,
    prev: Button<&'static str>,
    next: Button<&'static str>,
    select: Button<&'static str>,
    select_text: &'static str,
    rightmost: SideButton,
    leftmost: SideButton,
    button_area: Rect,
    page_counter: u8,
}

// TODO: consider creating some convenience constructors like
// impl<const N: usize> ChoicePage<StringChoiceItem, N> {
// pub fn from_str(choices: impl Iterator<Item = &'static str>) -> Self
// pub fn from_char(choices: impl Iterator<Item = &'static char>) -> Self
// }

impl<T, const N: usize> ChoicePage<T, N>
where
    T: ChoiceItem,
{
    pub fn new(choices: Vec<T, N>) -> Self {
        // Out of these button texts, only `select_text` is changeable
        // after placing the component, so we need to store only
        // `select_text` as an instance variable (at least right now).
        // (Text of select button may be changed dynamically by `select_button_map`.)
        let prev_text = "BACK";
        let select_text = "SELECT";
        let next_text = "NEXT";
        Self {
            choices,
            select_button_map: None,
            both_button_press: BothButtonPressHandler::new(),
            pad: Pad::with_background(theme::BG),
            prev: Button::with_text(ButtonPos::Left, prev_text, theme::button_default()),
            next: Button::with_text(ButtonPos::Right, next_text, theme::button_default()),
            select: Button::with_text(ButtonPos::Middle, select_text, theme::button_default()),
            select_text,
            // Side buttons need to be set from the beginning (in inactive state),
            // so they are placed correctly
            // (using Option<SideButton> was not working properly, so using
            // `is_active` for deciding whether to show it and handle their events)
            leftmost: SideButton::new(ButtonPos::Left, "LEFT"),
            rightmost: SideButton::new(ButtonPos::Right, "RIGHT"),
            // Button area is needed so the buttons
            // can be "re-placed" after their text is changed
            // Will be set in `place`
            button_area: Rect::zero(),
            page_counter: 0,
        }
    }

    /// Adding the optional button map with texts for the middle button
    pub fn with_select_button_map(
        mut self,
        select_button_map: LinearMap<u8, &'static str, N>,
    ) -> Self {
        self.select_button_map = Some(select_button_map);
        self
    }

    /// Change the default text of the middle button before placing it
    pub fn with_select_button_text(mut self, text: &'static str) -> Self {
        // Need to save it, in case the `select_button_map` would exist
        self.select_text = text;
        self.select = Button::with_text(ButtonPos::Middle, text, theme::button_default());
        self
    }

    /// Change the text of the next button before placing it
    pub fn with_next_button_text(mut self, text: &'static str) -> Self {
        self.next = Button::with_text(ButtonPos::Right, text, theme::button_default());
        self
    }

    /// Change the text of the previous button before placing it
    pub fn with_previous_button_text(mut self, text: &'static str) -> Self {
        self.prev = Button::with_text(ButtonPos::Left, text, theme::button_default());
        self
    }

    /// Resetting the component, which enables reusing the same instance
    /// for multiple choice categories.
    ///
    /// NOTE: from the client point of view, it would also be an option to
    /// always create a new instance with fresh setup, but I could not manage to
    /// properly clean up the previous instance - it would still be shown on
    /// screen and colliding with the new one.
    pub fn reset(
        &mut self,
        new_choices: Vec<T, N>,
        reset_page_counter: bool,
        reset_side_buttons: bool,
    ) {
        self.choices = new_choices;
        if reset_page_counter {
            self.set_page_counter(0);
        }
        if reset_side_buttons {
            self.unset_leftmost_button();
            self.unset_rightmost_button();
        }
    }

    pub fn set_page_counter(&mut self, page_counter: u8) {
        self.page_counter = page_counter;
    }

    pub fn set_rightmost_button(&mut self, text: &'static str) {
        self.rightmost.set(text, None, self.button_area);
    }

    pub fn set_leftmost_button(&mut self, text: &'static str) {
        self.leftmost.set(text, None, self.button_area);
    }

    pub fn set_rightmost_button_longpress(&mut self, text: &'static str, duration: Duration) {
        self.rightmost.set(text, Some(duration), self.button_area);
    }

    pub fn set_leftmost_button_longpress(&mut self, text: &'static str, duration: Duration) {
        self.leftmost.set(text, Some(duration), self.button_area);
    }

    pub fn unset_rightmost_button(&mut self) {
        self.rightmost.unset();
    }

    pub fn unset_leftmost_button(&mut self) {
        self.leftmost.unset();
    }

    /// Optionally changing the text of the middle button according to
    /// current page index and `self.select_button_map`
    fn handle_select_text(&mut self) {
        match &self.select_button_map {
            Some(select_button_map) => {
                match select_button_map.get(&self.page_counter) {
                    Some(text) => {
                        self.select.set_text(text, self.button_area);
                    }
                    None => self.select.set_text(self.select_text, self.button_area),
                };
            }
            None => {}
        }
    }

    fn update_situation(&mut self) {
        // So that only relevant buttons are visible
        self.pad.clear();

        // MIDDLE section above buttons
        self.show_current_choice();
        if self.has_previous_choice() {
            self.show_previous_choice();
        }
        if self.has_next_choice() {
            self.show_next_choice();
        }
    }

    fn last_page_index(&self) -> u8 {
        self.choices.len() as u8 - 1
    }

    fn has_previous_choice(&self) -> bool {
        self.page_counter > 0
    }

    fn has_next_choice(&self) -> bool {
        self.page_counter < self.last_page_index()
    }

    fn show_current_choice(&mut self) {
        self.choices[self.page_counter as usize].paint_center();
    }

    fn show_previous_choice(&mut self) {
        self.choices[(self.page_counter - 1) as usize].paint_left();
    }

    fn show_next_choice(&mut self) {
        self.choices[(self.page_counter + 1) as usize].paint_right();
    }

    fn decrease_page_counter(&mut self) {
        self.page_counter -= 1;
    }

    fn increase_page_counter(&mut self) {
        self.page_counter += 1;
    }

    /// Changing all non-middle button's visual state to "released" state
    /// (one of the buttons has a "pressed" state from
    /// the first press of the both-button-press)
    /// NOTE: does not cause any event to the button, it just repaints it
    fn set_right_and_left_buttons_as_released(&mut self, ctx: &mut EventCtx) {
        self.prev.set_released(ctx);
        self.next.set_released(ctx);
        self.leftmost.button.set_released(ctx);
        self.rightmost.button.set_released(ctx);
    }
}

impl<T, const N: usize> Component for ChoicePage<T, N>
where
    T: ChoiceItem,
{
    type Msg = ChoicePageMsg;

    fn place(&mut self, bounds: Rect) -> Rect {
        let button_height = theme::FONT_BOLD.line_height() + 2;
        let (_content_area, button_area) = bounds.split_bottom(button_height);
        self.pad.place(bounds);
        self.prev.place(button_area);
        self.next.place(button_area);
        self.select.place(button_area);
        self.leftmost.button.place(button_area);
        self.rightmost.button.place(button_area);
        // Saving button area so that we can re-place the buttons
        // when when they get updated
        self.button_area = button_area;
        bounds
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        // Possibly replacing or skipping an event because of both-button-press
        // aggregation
        let event = self.both_button_press.possibly_replace_event(event)?;

        // In case of both-button-press, changing all other buttons to released
        // state
        if self.both_button_press.are_both_buttons_pressed(event) {
            self.set_right_and_left_buttons_as_released(ctx);
        }

        // LEFT button clicks
        if self.has_previous_choice() {
            if let Some(ButtonMsg::Clicked) = self.prev.event(ctx, event) {
                // Clicked BACK. Decrease the page counter.
                self.decrease_page_counter();
                return None;
            }
        } else if self.leftmost.is_active && self.leftmost.got_triggered(ctx, event) {
            // Triggered LEFTmost button. Send event
            return Some(ChoicePageMsg::LeftMost);
        }

        // RIGHT button clicks
        if self.has_next_choice() {
            if let Some(ButtonMsg::Clicked) = self.next.event(ctx, event) {
                // Clicked NEXT. Increase the page counter.
                self.increase_page_counter();
                return None;
            }
        } else if self.rightmost.is_active && self.rightmost.got_triggered(ctx, event) {
            // Triggered RIGHTmost button. Send event
            return Some(ChoicePageMsg::RightMost);
        }

        // MIDDLE button clicks
        if let Some(ButtonMsg::Clicked) = self.select.event(ctx, event) {
            // Clicked SELECT. Send current choice index
            return Some(ChoicePageMsg::Choice(self.page_counter));
        }

        None
    }

    fn paint(&mut self) {
        self.pad.paint();

        // MIDDLE panel
        self.update_situation();

        // BOTTOM LEFT button
        if self.has_previous_choice() {
            self.prev.paint();
        } else if self.leftmost.is_active {
            self.leftmost.button.paint();
        }

        // BOTTOM RIGHT button
        if self.has_next_choice() {
            self.next.paint();
        } else if self.rightmost.is_active {
            self.rightmost.button.paint();
        }

        // BOTTOM MIDDLE button
        self.handle_select_text();
        self.select.paint();
    }
}

#[cfg(feature = "ui_debug")]
impl<T, const N: usize> crate::trace::Trace for ChoicePage<T, N>
where
    T: ChoiceItem,
{
    fn trace(&self, t: &mut dyn crate::trace::Tracer) {
        t.open("ChoicePage");
        t.close();
    }
}
