use crate::ui::{
    component::text::layout::DefaultTextTheme,
    display::{Color, Font},
};

// Font constants.
// TODO: add some bigger fonts for PIN etc.
pub const FONT_NORMAL: Font = Font::new(-1);
pub const FONT_MEDIUM: Font = Font::new(-5);
pub const FONT_BOLD: Font = Font::new(-2);
pub const FONT_MONO: Font = Font::new(-3);

pub const FONT_BUTTON: Font = FONT_MONO;
pub const FONT_HEADER: Font = FONT_MONO;

// Color palette.
pub const WHITE: Color = Color::rgb(255, 255, 255);
pub const BLACK: Color = Color::rgb(0, 0, 0);
pub const FG: Color = WHITE; // Default foreground (text & icon) color.
pub const BG: Color = BLACK; // Default background color.

pub const ICON_SUCCESS: &[u8] = include_res!("model_tr/res/success.toif");
pub const ICON_FAIL: &[u8] = include_res!("model_tr/res/fail.toif");
pub const ICON_CANCEL: &[u8] = include_res!("model_tr/res/cancel_for_outline.toif"); // 8*8
pub const ICON_ARM_LEFT: &[u8] = include_res!("model_tr/res/arm_left.toif"); // 6*10
pub const ICON_ARM_RIGHT: &[u8] = include_res!("model_tr/res/arm_right.toif"); // 6*10
pub const ICON_ARROW_LEFT: &[u8] = include_res!("model_tr/res/arrow_left.toif"); // 6*10
pub const ICON_ARROW_RIGHT: &[u8] = include_res!("model_tr/res/arrow_right.toif"); // 6*10
pub const ICON_ARROW_UP: &[u8] = include_res!("model_tr/res/arrow_up.toif"); // 10*6
pub const ICON_ARROW_DOWN: &[u8] = include_res!("model_tr/res/arrow_down.toif"); // 10*6
pub const ICON_BIN: &[u8] = include_res!("model_tr/res/bin.toif"); // 10*10
pub const ICON_AMOUNT: &[u8] = include_res!("model_tr/res/amount.toif"); // 10*10
pub const ICON_LOCK: &[u8] = include_res!("model_tr/res/lock.toif"); // 10*10
pub const ICON_PARAM: &[u8] = include_res!("model_tr/res/param.toif"); // 10*10
pub const ICON_USER: &[u8] = include_res!("model_tr/res/user.toif"); // 10*10
pub const ICON_WALLET: &[u8] = include_res!("model_tr/res/wallet.toif"); // 10*10
pub const ICON_WARNING: &[u8] = include_res!("model_tr/res/warning.toif"); // 12*12

// Button height is constant for both text and icon buttons.
// It is a combination of content and (optional) outline/border.
// It is not possible to have icons 7*7, therefore having 8*8
// with empty LEFT column and BOTTOM row.
pub const BUTTON_CONTENT_HEIGHT: i32 = 7;
pub const BUTTON_OUTLINE: i32 = 3;
pub const BUTTON_HEIGHT: i32 = BUTTON_CONTENT_HEIGHT + 2 * BUTTON_OUTLINE;

pub struct TRDefaultText;

impl DefaultTextTheme for TRDefaultText {
    const BACKGROUND_COLOR: Color = BG;
    const TEXT_FONT: Font = FONT_NORMAL;
    const TEXT_COLOR: Color = FG;
    const HYPHEN_FONT: Font = FONT_NORMAL;
    const HYPHEN_COLOR: Color = FG;
    const ELLIPSIS_FONT: Font = FONT_NORMAL;
    const ELLIPSIS_COLOR: Color = FG;

    const NORMAL_FONT: Font = FONT_NORMAL;
    const MEDIUM_FONT: Font = FONT_MEDIUM;
    const BOLD_FONT: Font = FONT_BOLD;
    const MONO_FONT: Font = FONT_MONO;
}
