use hebrew_accents::{
    unicode_version, unicode_version_info, UnicodeVersionInfo, UNICODE_REFERENCE, UNICODE_VERSION,
};

#[test]
fn public_api_exposes_all_constants() {
    // Verify all public exports are accessible
    let _version: &str = UNICODE_VERSION;
    let _reference: UnicodeVersionInfo = UNICODE_REFERENCE;
    let _version_fn: &str = unicode_version();
    let _info_fn: &UnicodeVersionInfo = unicode_version_info();
}

#[test]
fn display_implementations_work_together() {
    use hebrew_accents::unicode_version_info;

    let info = unicode_version_info();

    // All three Display implementations should work
    let v_str: String = unicode_version().to_string();
    let ref_str: String = UNICODE_REFERENCE.to_string();
    let info_str: String = info.to_string();

    assert_eq!(v_str, "18.0");
    assert!(ref_str.contains("Unicode"));
    assert_eq!(info_str, "Unicode 18.0 (Hebrew)");
}
