use hebrew_accents::{Context, Match, PoetryAccent, ProseAccent, SentenceContext};

#[test]
fn test_find_prose_poetry_silluq() {
    // ProseAccent, with Soph Pasuq and Meteg, no Pey or Samech
    let sc = SentenceContext::new("הִי אֽוֹר׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO::Outermatch",
        start: 9,
        end: 19,
    };
    assert_eq!(sc.find_accent(ProseAccent::Silluq.into()), Some(expected));
    // ProseAccent, with Soph Pasuq, no Pey or Samech
    let sc = SentenceContext::new(
        "וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ׃",
        Context::Prosaic,
    );
    let expected = Match {
        haystack: "TODO::Outermatch",
        start: 159,
        end: 168,
    };
    assert_eq!(sc.find_accent(ProseAccent::Silluq.into()), Some(expected));
    // ProseAccent, no Soph Paseq, with Pey
    let sc = SentenceContext::new(
        "ס ס וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ס ",
        Context::Poetic,
    );
    let expected = Match {
        haystack: "TODO::Outermatch",
        start: 165,
        end: 175,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Silluq.into()), Some(expected));
    // PoetryAccent with Soph Paseq and Peh
    let sc = SentenceContext::new(
        "וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ׃ פ",
        Context::Poetic,
    );
    let expected = Match {
        haystack: "TODO::Outermatch",
        start: 159,
        end: 171,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Silluq.into()), Some(expected));
    // Meteg not in the last word of the sentence
    let sc = SentenceContext::new(
        "ס ס וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵׁם׃ ס ",
        Context::Poetic,
    );
    assert_eq!(sc.find_accent(PoetryAccent::Silluq.into()), None);
    // Meteg followed by Maqqeph (\u{05BE}) (meaning no Meteg in the last word)
    let sc1 = SentenceContext::new("וַ וַיִּצֹ֥ק שֶׁ֖מֶן עַֽל־עַל־רֹאשׁהּ׃ ׃ פ", Context::Poetic);
    assert_eq!(sc1.find_accent(PoetryAccent::Silluq.into()), None);
}
#[test]
fn test_find_prose_poetry_atnach() {
    // Atnach present
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 52,
        end: 54,
    };
    assert_eq!(sc.find_accent(ProseAccent::Atnach.into()), Some(expected));
    // No Atnach present
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Atnach.into()), None);
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Atnach.into()), None);
}
#[test]
fn test_find_prose_segolta() {
    let sc = SentenceContext::new(
        " וַיַּ֣עַשׂ אֱלֹהִים֮ אֶת־הָרָקִיעַ֒ וַיַּבְדֵּ֗ל בֵּ֤ין הַמַּ֨יִם֙ אֲשֶׁר֙ מִתַּ֣חַת לָרָקִ֔יעַ וּבֵ֣ין הַמַּ֔יִם אֲשֶׁ֖ר מֵעַ֣ל לָרָקִ֑יעַ וַֽיְהִי־כֵֽן׃",
        Context::Prosaic,
    );
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 67,
        end: 69,
    };
    assert_eq!(sc.find_accent(ProseAccent::Segolta.into()), Some(expected));
    let sc = SentenceContext::new(
        " וַיַּ֣עַשׂ אֱלֹהִים֮ אֶת־הָרָקִיעַ וַיַּבְדֵּ֗ל בֵּ֤ין הַמַּ֨יִם֙ אֲשֶׁר֙ מִתַּ֣חַת לָרָקִ֔יעַ וּבֵ֣ין הַמַּ֔יִם אֲשֶׁ֖ר מֵעַ֣ל לָרָקִ֑יעַ וַֽיְהִי־כֵֽן׃",
        Context::Prosaic,
    );
    assert_eq!(sc.find_accent(ProseAccent::Segolta.into()), None);
}
#[test]
fn test_find_prose_shalshelet() {
    // Shalshelet, with Paseq - no space
    let sc = SentenceContext::new("בְּהִ֑ים֓׀ אֵ֥ץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO::2CodePoints",
        start: 16,
        end: 20,
    };
    assert_eq!(
        sc.find_accent(ProseAccent::Shalshelet.into()),
        Some(expected)
    );
    // Shalshelet, with Paseq + one space
    let sc = SentenceContext::new("בְּהִ֑ים֓ ׀ אֵ֥ץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO::2CodePoints",
        start: 16,
        end: 21,
    };

    assert_eq!(
        sc.find_accent(ProseAccent::Shalshelet.into()),
        Some(expected)
    );
    // Shalshelet, with Vertical Bar - no space
    let sc = SentenceContext::new("בְּהִ֑ים֓| אֵ֥ץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO::2CodePoints",
        start: 16,
        end: 19,
    };
    assert_eq!(
        sc.find_accent(ProseAccent::Shalshelet.into()),
        Some(expected)
    );
    // Shalshelet, with Vertical Bar + one space
    let sc = SentenceContext::new("בְּהִ֑ים֓ | אֵ֥ץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO::2CodePoints",
        start: 16,
        end: 20,
    };
    assert_eq!(
        sc.find_accent(ProseAccent::Shalshelet.into()),
        Some(expected)
    );
    // Missing Paseq or Vertical Bar
    let sc = SentenceContext::new("בְּהִ֑ים֓ אֵ֥ץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Shalshelet.into()), None);
}
#[test]
fn test_find_prose_zaqeph_qaton() {
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֔ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 63,
        end: 65,
    };
    assert_eq!(
        sc.find_accent(ProseAccent::ZaqephQaton.into()),
        Some(expected)
    );
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::ZaqephQaton.into()), None);
}
#[test]
fn test_find_prose_zaqeph_gadol() {
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹ֕הִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 48,
        end: 50,
    };
    assert_eq!(
        sc.find_accent(ProseAccent::ZaqephGadol.into()),
        Some(expected)
    );
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::ZaqephGadol.into()), None);
}
#[test]
fn test_find_prose_revia() {
    let sc = SentenceContext::new("אלהים֮ את־הרקיע֒ ויּבדּ֗ל בּ֤ין", Context::Prosaic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 44,
        end: 46,
    };
    assert_eq!(sc.find_accent(ProseAccent::Revia.into()), Some(expected));
    let sc = SentenceContext::new("אלהים֮ את־הרקיע֒ ויּבדּל בּ֤ין", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Revia.into()), None);
}
#[test]
fn test_find_prose_tiphcha() {
    let sc = SentenceContext::new(
        "ויּ֣ר֖א עשׂ֔ו כּ֥י רע֖ות בּנ֣ות כּ֖נ֑ען בּעינ֖י יצח֥ק א֖בֽיו׃",
        Context::Prosaic,
    );
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 10,
        end: 12,
    };
    assert_eq!(sc.find_accent(ProseAccent::Tiphcha.into()), Some(expected));
    let sc = SentenceContext::new("אתך ר֖בך֑ אתך ו֖המֽים׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 9,
        end: 11,
    };
    assert_eq!(sc.find_accent(ProseAccent::Tiphcha.into()), Some(expected));
}
#[test]
fn test_find_prose_zarqa() {
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶ֘ץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 120,
        end: 122,
    };
    assert_eq!(sc.find_accent(ProseAccent::Zarqa.into()), Some(expected));
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Zarqa.into()), None);
}
#[test]
fn test_find_prose_pashta() {
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱ֙לֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 44,
        end: 46,
    };
    assert_eq!(sc.find_accent(ProseAccent::Pashta.into()), Some(expected));
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Pashta.into()), None);
}
#[test]
fn test_find_prose_yetiv() {
    let sc = SentenceContext::new("אֽת־יעקב֒ ושׁלּ֤ח א֚תו֙", Context::Prosaic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 36,
        end: 38,
    };
    assert_eq!(sc.find_accent(ProseAccent::Yetiv.into()), Some(expected));
    let sc = SentenceContext::new("אֽת־יעקב֒ ושׁלּ֤ח אתו֙", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Yetiv.into()), None);
}
#[test]
fn test_find_prose_tevir() {
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמצ֛יִם ד֛דד הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 84,
        end: 86,
    };
    assert_eq!(sc.find_accent(ProseAccent::Tevir.into()), Some(expected));
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמציִם דדד הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Tevir.into()), None);
}
#[test]
fn test_find_prose_geresh() {
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשּׁ֜מַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 78,
        end: 80,
    };
    assert_eq!(sc.find_accent(ProseAccent::Geresh.into()), Some(expected));
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Geresh.into()), None);
}
#[test]
fn test_find_prose_gershayim() {
    let sc = SentenceContext::new("בְּרֵאשִׁ֞ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 18,
        end: 20,
    };
    assert_eq!(
        sc.find_accent(ProseAccent::Gershayim.into()),
        Some(expected)
    );
    let sc = SentenceContext::new("בְּרֵאשִׁית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Gershayim.into()), None);
}
#[test]
fn test_find_prose_pazer() {
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְא֡ת הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 99,
        end: 101,
    };
    assert_eq!(sc.find_accent(ProseAccent::Pazer.into()), Some(expected));
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאת הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Pazer.into()), None);
}
#[test]
fn test_find_prose_pazer_gadol() {
    let sc = SentenceContext::new("בְּרֵא֟שִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 12,
        end: 14,
    };
    assert_eq!(
        sc.find_accent(ProseAccent::PazerGadol.into()),
        Some(expected)
    );
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::PazerGadol.into()), None);
}
#[test]
fn test_find_prose_telisha_gadolah() {
    let sc = SentenceContext::new("בְּרֵא֠ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 12,
        end: 14,
    };
    assert_eq!(
        sc.find_accent(ProseAccent::TelishaGedolah.into()),
        Some(expected)
    );
    let sc = SentenceContext::new("בְּרֵאית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::TelishaGedolah.into()), None);
}
#[test]
fn test_find_prose_legarmeh() {
    // Legarmeh, with Paseq
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים׀  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO::Inner Match",
        start: 52,
        end: 60,
    };
    assert_eq!(sc.find_accent(ProseAccent::Legarmeh.into()), Some(expected)); //  - 60
                                                                              // Legarmeh with a space + Paseq
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים ׀  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO::Inner Match",
        start: 52,
        end: 61,
    };
    assert_eq!(sc.find_accent(ProseAccent::Legarmeh.into()), Some(expected)); // 40 - 61
                                                                              // Legarmeh with two spaces + Paseq
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִים  ׀  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Legarmeh.into()), None);
    // Legarmeh, with Vertical Bar
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים|  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO::Inner Match",
        start: 52,
        end: 59,
    };
    assert_eq!(sc.find_accent(ProseAccent::Legarmeh.into()), Some(expected)); // 40 - 59
                                                                              // Legarmeh, with space + Vertical Bar
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים |  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO::Inner Match",
        start: 52,
        end: 60,
    };
    assert_eq!(sc.find_accent(ProseAccent::Legarmeh.into()), Some(expected)); // 40 - 60
                                                                              // Legarmeh, with two spaces + Vertical Bar
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִים  |  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Legarmeh.into()), None);
    // Paseq or Vertical Bar is missing
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Legarmeh.into()), None);
}
// Conjunctives
#[test]
fn test_find_prose_munnach() {
    // Single Munach
    let sc = SentenceContext::new("בּראשׁית בּרא א֣להים את השּׁמים ואת הארץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO::Outer Match",
        start: 28,
        end: 30,
    };
    assert_eq!(sc.find_accent(ProseAccent::Munach.into()), Some(expected));
    // Munach part of Legarmeh (Paseq)
    let sc = SentenceContext::new("בּראשׁית בּרא א֣להים׀  את השּׁמים ואת הארץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Munach.into()), None);
    // Munach part of Legarmeh (space + Paseq)
    let sc = SentenceContext::new("בּראשׁית בּרא א֣להים ׀  את השּׁמים ואת הארץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Munach.into()), None);
    // Munach part of Legarmeh (Vertical Bar)
    let sc = SentenceContext::new("בּראשׁית בּרא א֣להים|  את השּׁמים ואת הארץ׃׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Munach.into()), None);
    // Munach part of Legarmeh (space + Vertical Bar)
    let sc = SentenceContext::new("בּראשׁית בּרא א֣להים  |  את השּׁמים ואת הארץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Munach.into()), None);
}
#[test]
fn test_find_prose_mahpakh() {
    let sc = SentenceContext::new("בּאשׁ֤ית בּא אלֹהִים אֵת הַשָּׁמַיִם וְאת האץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 10,
        end: 12,
    };
    assert_eq!(sc.find_accent(ProseAccent::Mahpakh.into()), Some(expected));
    let sc = SentenceContext::new("בּאשׁית בּא אלֹהִים אֵת הַשָּׁמַיִם וְאת האץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Mahpakh.into()), None);
}
#[test]
fn test_find_prose_merkha() {
    let sc = SentenceContext::new("מזמ֥ור לדו֑ד יהו֥ה ר֝ע֗י ל֣א אחסֽר׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 6,
        end: 8,
    };
    assert_eq!(sc.find_accent(ProseAccent::Merkha.into()), Some(expected));
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵת הַשָּׁמַ֖יִם וְאֵת הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Merkha.into()), None);
}
#[test]
fn test_find_prose_merkha_kephulah() {
    let sc = SentenceContext::new("בְּרֵאשִׁ֦ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 18,
        end: 20,
    };
    assert_eq!(
        sc.find_accent(ProseAccent::MerkhaKephulah.into()),
        Some(expected)
    );
    let sc = SentenceContext::new("בְּרֵאשִׁית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::MerkhaKephulah.into()), None);
}
#[test]
fn test_find_prose_darga() {
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים֧ ואת הארץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 56,
        end: 58,
    };
    assert_eq!(sc.find_accent(ProseAccent::Darga.into()), Some(expected));
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Darga.into()), None);
}
#[test]
fn test_find_prose_azla() {
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים א֨ת השּׁמים ואת הארץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 39,
        end: 41,
    };
    assert_eq!(sc.find_accent(ProseAccent::Azla.into()), Some(expected));
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Azla.into()), None);
}
#[test]
fn test_find_prose_telisha_qetannah() {
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֩ת הארץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 61,
        end: 63,
    };
    assert_eq!(
        sc.find_accent(ProseAccent::TelishaQetannah.into()),
        Some(expected)
    );
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::TelishaQetannah.into()), None);
}
#[test]
fn test_find_prose_galgal() {
    let sc = SentenceContext::new("בּראשׁית בּר֪א אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 23,
        end: 25,
    };
    assert_eq!(sc.find_accent(ProseAccent::Galgal.into()), Some(expected));
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Galgal.into()), None);
}
#[test]
fn test_find_prose_meayla() {
    // Tiphcha followed by Atnach
    let sc = SentenceContext::new("וְבְּרֵאשִׁית בָּרָא אֱלֹ֖הִ֑ים אֵת הַשָּׁמַיִם וְאֵת הָאָֽרֶץ", Context::Prosaic);
    let expected = Match {
        haystack: "TODO::Outer Match",
        start: 48,
        end: 56,
    };
    assert_eq!(sc.find_accent(ProseAccent::Mayela.into()), Some(expected));
    // Tiphcha followed by Atnach, two words connected with a Maqqeph
    let sc = SentenceContext::new("ויּ֖צא־נ֑ח וּבנ֛יו ואשׁתּ֥ו וּנשֽׁי־בנ֖יו אתּֽו׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO::Outer Match",
        start: 6,
        end: 18,
    };
    assert_eq!(sc.find_accent(ProseAccent::Mayela.into()), Some(expected));
    // Tiphcha followed by silluq
    let sc = SentenceContext::new("וְבְּרֵאשִׁית בָּרָא אֱלֹהִ֑ים אֵת הַשָּׁמַיִם וְאֵת הָ֖אָֽרֶץ", Context::Prosaic);
    let expected = Match {
        haystack: "TODO::Outer Match",
        start: 104,
        end: 114,
    };
    assert_eq!(sc.find_accent(ProseAccent::Mayela.into()), Some(expected));
    // only Tiphcha
    let sc = SentenceContext::new("וְבְּרֵאשִׁית בָּרָא אֱלֹהִ֑ים אֵ֖ת הַשָּׁמַיִם וְאֵת הָאָֽרֶץ", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Mayela.into()), None);
}
#[test]
fn test_find_prose_poetry_meteg() {
    // Only Silluq, No Meteg
    let sc = SentenceContext::new("בּראשׁ֖ית בּר֣א אלה֑ים א֥ת השּׁמ֖ים וא֥ת האֽרץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Meteg.into()), None);
    // Meteg and Siluq, separated by a Maqqeph
    let sc = SentenceContext::new("ויּ֥אמר אלה֖ים יה֣י א֑ור וֽיהי־אֽור׃", Context::Prosaic);
    let expected = Match {
        haystack: "TODO::Outer Match",
        start: 48,
        end: 50,
    };
    assert_eq!(sc.find_accent(ProseAccent::Meteg.into()), Some(expected));
    // Meteg and Siluq in separate words
    let sc = SentenceContext::new(
        "ויּקר֧א אלה֛ים לֽרק֖יע שׁמ֑ים וֽיהי־ע֥רב וֽיהי־ב֖קר י֥ום שׁנֽי׃ פ",
        Context::Poetic,
    );
    let expected = Match {
        haystack: "TODO::Outer Match",
        start: 30,
        end: 32,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Meteg.into()), Some(expected));
    // Only Meteg, no Silluq
    let sc = SentenceContext::new("וֽיהי־ב֖קר י֥ום שׁני׃ פ", Context::Prosaic);
    let expected = Match {
        haystack: "TODO::Outer Match",
        start: 2,
        end: 4,
    };
    assert_eq!(sc.find_accent(ProseAccent::Meteg.into()), Some(expected));
    // Two Meteg's, no Silluq
    let sc = SentenceContext::new("ום וֽיהי־ע֥רב וֽיהי־ב֖קר י֥ום שׁני׃ פ", Context::Poetic);
    let expected = Match {
        haystack: "TODO::Outer Match",
        start: 7,
        end: 9,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Meteg.into()), Some(expected));
}
#[test]
fn test_find_prose_poetry_maqqeph() {
    // No Maqqeph
    let sc = SentenceContext::new("בּראשׁ֖ית בּר֣א אלה֑ים א֥ת השּׁמ֖ים וא֥ת האֽרץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(ProseAccent::Maqqeph.into()), None);
    // One Maqqeph
    let sc = SentenceContext::new("ויּ֥אמר אלה֖ים יה֣י א֑ור וֽיהי־אֽור׃", Context::Poetic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 56,
        end: 58,
    };
    assert_eq!(sc.find_accent(ProseAccent::Maqqeph.into()), Some(expected));
    // No Maqqeph
    let sc = SentenceContext::new("בּראשׁ֖ית בּר֣א אלה֑ים א֥ת השּׁמ֖ים וא֥ת האֽרץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Maqqeph.into()), None);
    // One Maqqeph
    let sc = SentenceContext::new("ויּ֥אמר אלה֖ים יה֣י א֑ור וֽיהי־אֽור׃", Context::Poetic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 56,
        end: 58,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Maqqeph.into()), Some(expected));
}
/* **********************************************************
 *                          POETRY
 * *********************************************************/
#[test]
fn test_find_poetry_oleh_we_yored() {
    // OlehWeYored, one word
    let sc = SentenceContext::new("בְּרֵעַֽל־פַּלְגֵ֫ימָ֥יִ", Context::Poetic);
    let expected = Match {
        haystack: "TODO::Outer Match",
        start: 34,
        end: 44,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::OlehWeYored.into()),
        Some(expected)
    );
    // OlehWeYored, one word - context: Prosaic
    let sc = SentenceContext::new("בְּרֵעַֽל־פַּלְגֵ֫ימָ֥יִ", Context::Prosaic);
    assert_eq!(sc.find_accent(PoetryAccent::OlehWeYored.into()), None);
    // OlehWeYored, two words
    let sc = SentenceContext::new("ועַֽל־פַּלְגֵ֫י מָ֥יִם וְעָלֵ֥הוּ ׃", Context::Poetic);
    let expected = Match {
        haystack: "TODO::Outer Match",
        start: 26,
        end: 37,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::OlehWeYored.into()),
        Some(expected)
    );
    // OlehWeYored, three words
    let sc = SentenceContext::new("ועַֽל־פַּלְגֵ֫י מָיִם וְעָ֥לֵ֥הוּ ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::OlehWeYored.into()), None);
}
#[test]
fn test_find_poetry_revia_gadol() {
    // No Revia at all
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::ReviaGadol.into()), None);
    // Two Revia's
    let sc = SentenceContext::new("בּר֗אשׁית בּרא אלהים את השּׁ֗מים ואת הארץ׃", Context::Poetic);
    let expected = Match {
        haystack: "\u{597}",
        start: 3,
        end: 5,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::ReviaGadol.into()),
        Some(expected)
    );
    // Revia followed by Oleh We Yored (1 word)
    let sc = SentenceContext::new("בּר֗אשׁית בּ֫ר֥א אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::ReviaGadol.into()), None);
    // Revia followed by Oleh We Yored (2 words)
    let sc = SentenceContext::new("בּר֗אשׁית בּ֫רא אלה֥ים את השּׁמים ואת הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::ReviaGadol.into()), None);
    // Revia followed by 'Oleh We Yored' (3 words)
    let sc = SentenceContext::new("בּר֗אשׁית בּ֫רא אלהים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
    let expected = Match {
        haystack: "\u{597}",
        start: 3,
        end: 5,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::ReviaGadol.into()),
        Some(expected)
    );
    // Revia not directly followed by Oleh We Yored (1 word)
    let sc = SentenceContext::new("בּר֗אשׁית בּרא אלה֫י֥ם את השּׁמים ואת הארץ׃", Context::Poetic);
    let expected = Match {
        haystack: "\u{597}",
        start: 3,
        end: 5,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::ReviaGadol.into()),
        Some(expected)
    );
}
#[test]
fn test_find_poetry_revia_mugrash() {
    // Revia and Geresh (Ps 32:3)
    let sc = SentenceContext::new("בְּ֝שַׁאֲגָתִ֗י", Context::Poetic);
    let expected = Match {
        haystack: "TODO::2CodePoints",
        start: 6,
        end: 28,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::ReviaMugrash.into()),
        Some(expected)
    );
    // Revia and Geresh (Ps 110:6) - accent on a single character
    let sc = SentenceContext::new("יָדִ֣ין בַּ֭גּוֹיִם מָלֵ֣א גְוִיּ֑וֹת מָ֥חַץ רֹ֝֗אשׁ עַל־אֶ֥רֶץ רַבָּֽה׃", Context::Poetic);
    let expected = Match {
        haystack: "TODO::2CodePoints",
        start: 89,
        end: 93,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::ReviaMugrash.into()),
        Some(expected)
    );
    // Only Revia
    let sc = SentenceContext::new(
        " שׁ֗יר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים מא֗ין יב֥א עזרֽי׃",
        Context::Poetic,
    );
    assert_eq!(sc.find_accent(PoetryAccent::ReviaMugrash.into()), None);
    // Only Geresh
    let sc = SentenceContext::new(
        " שׁ֗יר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים מ֝אין יב֥א עזרֽי׃",
        Context::Poetic,
    );
    assert_eq!(sc.find_accent(PoetryAccent::ReviaMugrash.into()), None);
}
#[test]
fn test_find_poetry_shalshelet_gadol() {
    // Shalshelet Gadol, with Paseq - no space
    let sc = SentenceContext::new("בְּהִ֑ים֓׀ אֵ֥ץ׃", Context::Poetic);
    let expected = Match {
        haystack: "TODO::2CodePoints",
        start: 16,
        end: 20,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::ShalsheletGadol.into()),
        Some(expected)
    );
    // Shalshelet Gadol, with Paseq + one space
    let sc = SentenceContext::new("בְּהִ֑ים֓ ׀ אֵ֥ץ׃", Context::Poetic);
    let expected = Match {
        haystack: "TODO::2CodePoints",
        start: 16,
        end: 21,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::ShalsheletGadol.into()),
        Some(expected)
    );
    // Shalshelet Gadol, with Vertical Bar - no space
    let sc = SentenceContext::new("בְּהִ֑ים֓| אֵ֥ץ׃", Context::Poetic);
    let expected = Match {
        haystack: "TODO::2CodePoints",
        start: 16,
        end: 19,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::ShalsheletGadol.into()),
        Some(expected)
    );
    // Shalshelet Gadol, with Vertical Bar + one space
    let sc = SentenceContext::new("בְּהִ֑ים֓ | אֵ֥ץ׃", Context::Poetic);
    let expected = Match {
        haystack: "TODO::2CodePoints",
        start: 16,
        end: 20,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::ShalsheletGadol.into()),
        Some(expected)
    );
    // Missing Paseq or Vertical Bar
    let sc = SentenceContext::new("בְּהִ֑ים֓ אֵ֥ץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::ShalsheletGadol.into()), None);
}
#[test]
fn test_find_poetry_tsinnor() {
    let sc = SentenceContext::new("את־אבר֮הם", Context::Poetic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 12,
        end: 14,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Tsinnor.into()), Some(expected));
    let sc = SentenceContext::new("את־אברהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Tsinnor.into()), None);
}
#[test]
fn test_find_poetry_revia_qaton() {
    // No revia at all
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::ReviaQaton.into()), None);
    // Revia, not followed by OleWe Yored
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים א֗ת השּׁמים ואת הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::ReviaQaton.into()), None);
    // Revia directly followed by Oleh We Yored (1 word)
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים א֗ת ה֫שּׁמי֥ם ואת הארץ׃", Context::Poetic);
    let expected = Match {
        haystack: "\u{597}",
        start: 21,
        end: 23,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::ReviaQaton.into()),
        Some(expected)
    );
    // Revia directly followed by Oleh We Yored (2 words)
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים א֗ת ה֫שּׁמים וא֥ת הארץ׃", Context::Poetic);
    let expected = Match {
        haystack: "\u{597}",
        start: 21,
        end: 23,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::ReviaQaton.into()),
        Some(expected)
    );
    // Revia directly followed by 'Oleh We Yored' (3 words)
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים א֗ת ה֫שּׁמים ואת האר֥ץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::ReviaQaton.into()), None);
    // Revia NOT directly followed by Oleh We Yored (2 words)
    let sc = SentenceContext::new("בּראשׁית בּרא א֗להים א֓ת ה֫שּׁמים וא֥ת הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::ReviaQaton.into()), None);
    // Revia is part of Revia Mugrash
    let sc = SentenceContext::new(
        " שׁיר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים מ֝א֗ין יב֥א עזרֽי׃",
        Context::Poetic,
    );
    assert_eq!(sc.find_accent(PoetryAccent::ReviaQaton.into()), None);
}
#[test]
fn test_find_poetry_dechi() {
    let sc = SentenceContext::new("את־אבר֭הם", Context::Poetic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 12,
        end: 14,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Dechi.into()), Some(expected));
    let sc = SentenceContext::new("את־אברהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Dechi.into()), None);
}
#[test]
fn test_find_poetry_pazer() {
    let sc = SentenceContext::new("את־אבר֡הם", Context::Poetic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 12,
        end: 14,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Pazer.into()), Some(expected));
    let sc = SentenceContext::new("את־אברהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Pazer.into()), None);
}
#[test]
fn test_find_poetry_mehuppakh_legarmeh() {
    // MehuppakhLegarmeh, with Paseq
    let sc = SentenceContext::new(" את־אברהם֤ ׀ מזמ֗ור", Context::Poetic);
    let expected = Match {
        haystack: "TODO::Outermatch",
        start: 17,
        end: 22,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::MehuppakhLegarmeh.into()),
        Some(expected)
    );
    // MehuppakhLegarmeh, with Vertical Bar
    let sc = SentenceContext::new(" את־אברהם֤ | מזמ֗ור", Context::Poetic);
    let expected = Match {
        haystack: "TODO::Outermatch",
        start: 17,
        end: 21,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::MehuppakhLegarmeh.into()),
        Some(expected)
    );
    // Mehuppakh only
    let sc = SentenceContext::new(" את־אברהם֤ מזמ֗ור", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::MehuppakhLegarmeh.into()), None);
}
#[test]
fn test_find_poetry_azla_legarmeh() {
    // AzlaLegarmeh, with Paseq + no space
    let sc = SentenceContext::new(" את־אברה֨ם׀ א־אם", Context::Poetic);
    let expected = Match {
        haystack: "TODO::Outermatch",
        start: 15,
        end: 21,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::AzlaLegarmeh.into()),
        Some(expected)
    );
    // AzlaLegarmeh, with Paseq + 1 space
    let sc = SentenceContext::new(" את־אברה֨ם ׀ א־אם", Context::Poetic);
    let expected = Match {
        haystack: "TODO::Outermatch",
        start: 15,
        end: 22,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::AzlaLegarmeh.into()),
        Some(expected)
    );
    // AzlaLegarmeh, with Vertical Bar + no space
    let sc = SentenceContext::new(" את־אברה֨ם| א־אם", Context::Poetic);
    let expected = Match {
        haystack: "TODO::Outermatch",
        start: 15,
        end: 20,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::AzlaLegarmeh.into()),
        Some(expected)
    );
    // AzlaLegarmeh, with Vertical Bar + 1 space
    let sc = SentenceContext::new(" את־אברה֨ם | א־אם", Context::Poetic);
    let expected = Match {
        haystack: "TODO::Outermatch",
        start: 15,
        end: 21,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::AzlaLegarmeh.into()),
        Some(expected)
    );
    // Azla only
    let sc = SentenceContext::new(" את־אברה֨ם  א־אם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::AzlaLegarmeh.into()), None);
}
#[test]
fn test_find_poetry_munnach() {
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 12,
        end: 14,
    };
    let sc = SentenceContext::new("את־אבר֣הם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Munach.into()), Some(expected));
    let sc = SentenceContext::new("את־אברהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Munach.into()), None);
}
#[test]
fn test_find_poetry_merkha() {
    // No Merkha
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Merkha.into()), None);
    // One Merkha
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
    let expected = Match {
        haystack: "\u{5a5}",
        start: 21,
        end: 23,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Merkha.into()), Some(expected));
    // Tsinnorit + Merkha (1w)
    let sc = SentenceContext::new("בּראשׁית בּרא א֘להי֥ם את השּׁמים ואת הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Merkha.into()), None);
    // Tsinnorit + Merkha (2w)
    let sc = SentenceContext::new("בּראשׁית בּרא אל֘הים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Merkha.into()), None);
    // Tsinnorit + Merkha (3w)
    let sc = SentenceContext::new("בּראשׁית בּר֘א אלהים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
    let expected = Match {
        haystack: "\u{5a5}",
        start: 22,
        end: 24,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Merkha.into()), Some(expected));
    // Oleh + Merkha (1w)
    let sc = SentenceContext::new("בּראשׁית בּרא א֫להי֥ם את השּׁמים ואת הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Merkha.into()), None);
    // Oleh + Merkha (2w)
    let sc = SentenceContext::new("בּראשׁית בּרא אלה֫ים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Merkha.into()), None);
    // Oleh + Merkha (3w)
    let sc = SentenceContext::new("בּראשׁית בּר֫א אלהים א֥ת השּׁ֥מים ואת הארץ׃", Context::Poetic);
    let expected = Match {
        haystack: "\u{5a5}",
        start: 22,
        end: 24,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Merkha.into()), Some(expected));
}

#[test]
fn test_find_poetry_illuy() {
    let sc = SentenceContext::new("את־אב֬רהם", Context::Poetic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 10,
        end: 12,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Illuy.into()), Some(expected));
    let sc = SentenceContext::new("את־אברהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Illuy.into()), None);
}
#[test]
fn test_find_poetry_tarkha() {
    let sc = SentenceContext::new("את־אבר֖הם", Context::Poetic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 12,
        end: 14,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Tarkha.into()), Some(expected));
    let sc = SentenceContext::new("את־אברהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Tarkha.into()), None);
}
#[test]
fn test_find_poetry_galgal() {
    let sc = SentenceContext::new("את־אבר֪הם", Context::Poetic);
    let expected = Match {
        haystack: "TODO: insert single character",
        start: 12,
        end: 14,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Galgal.into()), Some(expected));
    let sc = SentenceContext::new("את־אברהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Galgal.into()), None);
}
#[test]
fn test_find_poetry_mehuppakh() {
    // No Mehuppach
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Mehuppakh.into()), None);
    // One Mehuppach
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים א֤ת השּׁמים ואת הארץ׃", Context::Poetic);
    let expected = Match {
        haystack: "\u{5a4}",
        start: 21,
        end: 23,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::Mehuppakh.into()),
        Some(expected)
    );
    // One Mehuppach, part of Tsinnorit Mappach (one word)
    let sc = SentenceContext::new("בּראשׁית בּרא א֘להי֤ם את השּׁמים ואת הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Mehuppakh.into()), None);
    // One Mehuppach, part of Tsinnorit Mappach (two words)
    let sc = SentenceContext::new("בּראשׁית בּרא א֘להים א֤ת השּׁמים ואת הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Mehuppakh.into()), None);
    // One Mehuppach, part of Tsinnorit Mappach (three words)
    let sc = SentenceContext::new("בּראשׁית בּ֘רא אלהים א֤ת השּׁמים ואת הארץ׃", Context::Poetic);
    let expected = Match {
        haystack: "\u{5a4}",
        start: 22,
        end: 24,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::Mehuppakh.into()),
        Some(expected)
    );
    // One Mehuppach, part of Mehuppach Legarmeh (no space)
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת׀ הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Mehuppakh.into()), None);
    // One Mehuppach, part of Mehuppach Legarmeh (one space)
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת ׀ הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Mehuppakh.into()), None);
    // One Mehuppach, part of Mehuppach Legarmeh (no space - vertical line)
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת| הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Mehuppakh.into()), None);
    // One Mehuppach, part of Mehuppach Legarmeh (one space - vertical line)
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת | הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Mehuppakh.into()), None);
    // One Mehuppach, part of 'Mehuppach Legarmeh' (too many spaces)
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת    ׀ הארץ׃", Context::Poetic);
    let expected = Match {
        haystack: "\u{5a4}",
        start: 33,
        end: 35,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::Mehuppakh.into()),
        Some(expected)
    );
    // //One Mehuppach, part of Mehuppach Legarmeh (no space), followed with a Mehuppach
    let sc = SentenceContext::new("בּרא֤שׁית בּרא אלהים את השּׁמים וא֤ת׀ האר֤ץ׃", Context::Poetic);
    let expected = Match {
        haystack: "\u{5a4}",
        start: 4,
        end: 6,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::Mehuppakh.into()),
        Some(expected)
    );
    // One Mehuppach, part of Mehuppach Legarmeh (one space), followed with a Mehuppach
    let sc = SentenceContext::new("בּרא֤שׁית בּרא אלהים את השּׁמים וא֤ת ׀ האר֤ץ׃", Context::Poetic);
    let expected = Match {
        haystack: "\u{5a4}",
        start: 4,
        end: 6,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::Mehuppakh.into()),
        Some(expected)
    );
}

#[test]
fn test_find_poetry_azla() {
    // contains Azla
    let sc = SentenceContext::new(" את־אברה֨ם א־אם", Context::Poetic);
    let expected = Match {
        haystack: "TODO::Outermatch",
        start: 15,
        end: 17,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Azla.into()), Some(expected));
    // contains Azla and Azla Legarmeh
    let sc = SentenceContext::new(" אה֨ת־אברה֨ם ׀ א־אם", Context::Poetic);
    let expected = Match {
        haystack: "TODO::Outermatch",
        start: 5,
        end: 11,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Azla.into()), Some(expected));
    // Azla Legarmeh, with space + Paseq
    let sc = SentenceContext::new(" את־אברה֨ם ׀ א־אם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Azla.into()), None);
    // Azla Legarmeh, with Paseq
    let sc = SentenceContext::new(" את־אברה֨ם׀ א־אם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Azla.into()), None);
    // Azla Legarmeh, with space + Vertical Bar
    let sc = SentenceContext::new(" את־אברה֨ם | א־אם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Azla.into()), None);
    // Azla Legarmeh, with Vertical Bar
    let sc = SentenceContext::new(" את־אברה֨ם| א־אם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Azla.into()), None);
}
#[test]
fn test_find_poetry_shalshelet_qetannah() {
    // Shalshelet
    let sc = SentenceContext::new("יצחק אל־יעק֓ב ויברך", Context::Poetic);
    let expected = Match {
        haystack: "TODO::Outermatch",
        start: 21,
        end: 23,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::ShalsheletQetannah.into()),
        Some(expected)
    );
    // Shalshelet Gadol, with Paseq
    let sc = SentenceContext::new("יצחק אל־יעק֓ב ׀ ויברך", Context::Poetic);
    assert_eq!(
        sc.find_accent(PoetryAccent::ShalsheletQetannah.into()),
        None
    );
    // Shalshelet Gadol, with Vertical Bar
    let sc = SentenceContext::new("יצחק אל־יעק֓ב | ויברך", Context::Poetic);
    assert_eq!(
        sc.find_accent(PoetryAccent::ShalsheletQetannah.into()),
        None
    );
}
#[test]
fn test_find_poetry_tsinnorit_merkha() {
    // accent in a single word
    let sc = SentenceContext::new("אא֘תאב֥רהם", Context::Poetic);
    let expected = Match {
        haystack: "TODO::2CodePoints",
        start: 4,
        end: 14,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::TsinnoritMerkha.into()),
        Some(expected)
    );
    // accent in a single word, without Tsinnorit
    let sc = SentenceContext::new("את־אב֥רהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMerkha.into()), None);
    // accent in a single word, without Merkha
    let sc = SentenceContext::new("אא֘ת־אברהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMerkha.into()), None);
    // accent in two words seperated by Maqqeph
    let sc = SentenceContext::new("את־א֘ב֥רהם", Context::Poetic);
    let expected = Match {
        haystack: "TODO::2CodePoints",
        start: 8,
        end: 14,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::TsinnoritMerkha.into()),
        Some(expected)
    );
    // accent in two words seperated by Maqqeph, without Tsinnorit
    let sc = SentenceContext::new("את־אב֥רהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMerkha.into()), None);
    // accent in two words seperated by Maqqeph, without Merkha
    let sc = SentenceContext::new("את־א֘ברהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMerkha.into()), None);
    // accent in two words
    let sc = SentenceContext::new("את־א֘בם ב֥רהם", Context::Poetic);
    let expected = Match {
        haystack: "TODO::2CodePoints",
        start: 8,
        end: 19,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::TsinnoritMerkha.into()),
        Some(expected)
    );
    // accent in two words, without Tsinnorit
    let sc = SentenceContext::new("את־א֘בם ברהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMerkha.into()), None);
    // accent in two words, without Merkha
    let sc = SentenceContext::new("את־אבם ב֥רהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMerkha.into()), None);
    // accent in three words
    let sc = SentenceContext::new("את־א֘בם הם ב֥רהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMerkha.into()), None);
}
#[test]
fn test_find_poetry_tsinnorit_mahpakh() {
    // accent in a single word
    let sc = SentenceContext::new("את־א֘ב֤רהם אהם", Context::Poetic);
    let expected = Match {
        haystack: "TODO::2CodePoints",
        start: 8,
        end: 14,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()),
        Some(expected)
    );
    // accent in a single word
    let sc = SentenceContext::new("את־אב֤רהם אהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()), None);
    // accent in a single word, without Tsinnorit
    let sc = SentenceContext::new("את־א֘ברהם אהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()), None);
    // accent in two words seperated by Maqqeph, without Mahpakh
    let sc = SentenceContext::new("אא֘ת־אב֤רהם אהם", Context::Poetic);
    let expected = Match {
        haystack: "TODO::2CodePoints",
        start: 4,
        end: 16,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()),
        Some(expected)
    );
    // accent in two words seperated by Maqqeph, without Tsinnorit
    let sc = SentenceContext::new("את־אב֤רהם אהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()), None);
    // accent in two words seperated by Maqqeph, without Mahpakh
    let sc = SentenceContext::new("אא֘ת־אברהם אהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()), None);
    // accent in two words
    let sc = SentenceContext::new("את־א֘ברהם אהאב֤ם", Context::Poetic);
    let expected = Match {
        haystack: "TODO::2CodePoints",
        start: 8,
        end: 29,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()),
        Some(expected)
    );
    // accent in two words, without Tsinnorit
    let sc = SentenceContext::new("את־אברהם אהאב֤ם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()), None);
    // accent in two words, without Mahpakh
    let sc = SentenceContext::new("את־א֘ברהם אהאבם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()), None);
    // accent in three words
    let sc = SentenceContext::new("את־א֘ב רהם אהאב֤ם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()), None);
}
