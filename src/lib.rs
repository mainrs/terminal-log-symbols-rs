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
/// A symbol used to indicate a recoverable error.
pub const UNKNOWN_SYMBOL: Emoji = Emoji::new("?", "?");

#[cfg(feature = "colors")]
pub mod colored {
    use ansi_colors_macro::ansi_string;
    use terminal_emoji::Emoji;

    /// A symbol used for indicating error messages.
    pub const ERROR_SYMBOL: Emoji = Emoji::new(ansi_string!("{red ✖}"), ansi_string!("{red ×}"));
    /// A symbol used for indicating additional information to the user.
    pub const INFO_SYMBOL: Emoji = Emoji::new(ansi_string!("{blue ℹ}"), ansi_string!("{blue i}"));
    /// A symbol used for indicating a successful operation.
    pub const SUCCESS_SYMBOL: Emoji =
        Emoji::new(ansi_string!("{green ✔}"), ansi_string!("{green √}"));
    /// A symbol used to indicate a recoverable error.
    pub const WARNING_SYMBOL: Emoji =
        Emoji::new(ansi_string!("{yellow ⚠}"), ansi_string!("{yellow ‼}"));
    /// A symbol used to indicate a recoverable error.
    pub const UNKNOWN_SYMBOL: Emoji =
        Emoji::new(ansi_string!("{gray ?}"), ansi_string!("{gray ?}"));
}
