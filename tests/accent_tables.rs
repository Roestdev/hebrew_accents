use hebrew_accents::display_poetry_accent_table;
use hebrew_accents::display_prose_accent_table;
//use hebrew_accents::display_accent_table;

#[test]
fn show_tables() {
    // Direct calls
    display_prose_accent_table();
    display_poetry_accent_table();

    // Or the generic version
    //display_accent_table("PROSE (generic)", PROSE_ACCENT_TABLE.as_ref());
    //display_accent_table("POETRY (generic)", POETRY_ACCENT_TABLE.as_ref());
}
