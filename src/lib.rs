#![doc = include_str!("../README.md")]

// common
mod char;

// finding accents
mod sentence_context;
mod sentence_ctx_contains;
mod sentence_ctx_find;
mod sentence_ctx_funcs;
mod sentence_ctx_regex;

// static accent data
mod accent;
mod accent_codepoints;
mod accent_data;
mod accent_display;

// exports
pub use accent::*;
pub use accent_display::*;
pub use sentence_context::*;