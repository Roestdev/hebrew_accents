#![doc = include_str!("../README.md")]

// common
mod char;

// static accent data
mod accent;
mod accent_codepoints;
mod accent_data;
mod accent_display;

// finding accents
mod sentence_ctx_funcs;
mod sentence_ctx_regex;
mod sentence_context;

// exports
pub use accent::*;
pub use accent_display::*;
pub use sentence_context::*;
