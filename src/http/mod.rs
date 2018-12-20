//! Control LIFX lights over the internet.

mod selector;
pub use self::selector::*;
mod state;
pub use self::state::Error as ColorValidationError;
pub use self::state::{ColorSetting, State, StateChange};
mod client;
pub use self::client::*;
