//! The crate provides four symbols that can be used for printing process
//! messages to the terminal: error, info, success and warning. The emojis used
//! support fallback values for OS' that have no Emoji support.

use terminal_emoji::Emoji;

/// A symbol used for indicating error messages.
pub const ERROR_SYMBOL: Emoji = Emoji::new("✖", "×");
/// A symbol used for indicating additional information to the user.
pub const INFO_SYMBOL: Emoji = Emoji::new("ℹ", "i");
/// A symbol used for indicating a successful operation.
pub const SUCCESS_SYMBOL: Emoji = Emoji::new("✔", "√");
/// A symbol used to indicate a recoverable error.
pub const WARNING_SYMBOL: Emoji = Emoji::new("⚠", "‼");
