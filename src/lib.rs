#![doc = include_str!("../README.md")]

// static accent data
mod accent;
mod accent_codepoints;
mod accent_data;
mod accent_display;
// finding accents
mod char;
mod regex;
mod sentence_context;

// export
pub use accent::*;
pub use accent_display::*;
pub use sentence_context::*;
