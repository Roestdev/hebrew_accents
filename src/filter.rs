use std::borrow::Cow;

use crate::BookGenre;

pub fn filters(_book_genre: BookGenre, _text: &str) -> Cow<'_, str> {
    "dsfsdf".into()
}
