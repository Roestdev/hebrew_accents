use hebrew_accents::{
    display_poetry_accent_table, display_prose_accent_table, display_pseudo_accent_table,
};

#[test]
fn show_tables() {
    display_prose_accent_table();
    display_poetry_accent_table();
    display_pseudo_accent_table();
}
