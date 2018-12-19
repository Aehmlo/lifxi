//! Control LIFX lights over the internet.

mod selector;
pub use self::selector::*;
mod color;
pub use self::color::ColorSetting;
pub use self::color::Error as ColorValidationError;
