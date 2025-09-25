#![doc = include_str!("../README.md")]

// static accent data 
mod accent;
mod accent_data;
mod accent_codepoints;
mod accent_display;
// finding accents
mod sentence_context;
mod regex;
mod char;

// export
pub use accent::*;
pub use sentence_context::*;
pub use accent_display::*;

