mod chars;
mod detector;
mod regex;
mod validator;

pub(crate) use chars::*;
pub(crate) use detector::detect_context_from_sentence;
pub(crate) use regex::poetry_patterns::*;
pub(crate) use regex::prose_patterns::*;
pub(crate) use regex::shared_patterns::*;
pub(crate) use validator::validate_sentence;
