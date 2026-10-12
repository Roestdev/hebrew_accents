// TODO

mod prose_notes {
    use hebrew_accents::{Accent, ProseAccent};

    /// Test all prose disjunctive accents (should return Some(strength))
    #[test]
    fn test_prose_disjunctive_notes() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(
            ProseAccent::Silluq.notes(),
            Some(
                r#"The terms Silluq and Sof Pasuq are indifferently used for the final accent of the verse.
            It is possible to have both a Silluq and Meteg in one word,
            where the Meteg comes before the Silluq.
            The Silluq appears once per verse.
"#
            )
        );
        assert_eq!(
            ProseAccent::Atnach.notes(),
            Some(
                r#"The major disjunctive accent dividing the verse in two halves.
    It appears only once per verse"#
            )
        );
        assert_eq!(
            ProseAccent::Segolta.notes(),
            Some(r#"Appears frequently in prose, preceding Atnach or Silluq."#)
        );
        assert_eq!(
            ProseAccent::Shalshelet.notes(),
            Some(r#"Rare accent occurring only four times in the Torah."#)
        );
        assert_eq!(
            ProseAccent::ZaqephQatan.notes(),
            Some(r#"Common disjunctive, often appearing before Zaqeph Gadol or Atnach."#)
        );
        assert_eq!(
            ProseAccent::ZaqephGadol.notes(),
            Some(r#"Less common than Zaqeph Qaton; appears after Qadma or Azla."#)
        );
        assert_eq!(
            ProseAccent::Revia.notes(),
            Some(r#"probably due to its four-note tune."#)
        );
        assert_eq!(
            ProseAccent::Tiphcha.notes(),
            Some(r#"before Atnach and Silluq"#)
        );
        assert_eq!(ProseAccent::Zarqa.notes(), Some(r#"before Segolta"#));
        assert_eq!(
            ProseAccent::Pashta.notes(),
            Some(r#"appears on the last syllable of the word, extending rightward."#)
        );
        assert_eq!(
            ProseAccent::Yetiv.notes(),
            Some(r#"occasionally substitutes for a Pashta"#)
        );
        assert_eq!(
            ProseAccent::Tevir.notes(),
            Some(r#"Often followed by Revia; indicates a melodic break."#)
        );
        assert_eq!(
            ProseAccent::Geresh.notes(),
            Some(r#"Functions as Level 4 disjunctive; often preceded by Azla."#)
        );
        assert_eq!(
            ProseAccent::Gershayim.notes(),
            Some(r#"Double Geresh; rarer variant appearing in specific syntactic contexts."#)
        );
        assert_eq!(
            ProseAccent::Pazer.notes(),
            Some(
                r#"Level 4 disjunctive; can appear in place of Zaqeph Qaton in certain conditions."#,
            )
        );
        assert_eq!(
            ProseAccent::PazerGadol.notes(),
            Some(r#"Also known as Qarne Pharah; rare variant resembling horns."#)
        );
        assert_eq!(
            ProseAccent::TelishaGedolah.notes(),
            Some(r#"Level 4 disjunctive; distinct from Telisha Qetannah."#)
        );
        assert_eq!(
            ProseAccent::Legarmeh.notes(),
            Some(r#"Munach with Passeq; Before Revia"#)
        );
    }

    /// Test all prose conjunctive accents (should return None)
    #[test]
    fn test_prose_conjunctive_notes() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(
            ProseAccent::Munach.notes(),
            Some(r#"Most common conjunctive; precedes disjunctive accents."#)
        );
        assert_eq!(
            ProseAccent::Mahpakh.notes(),
            Some(r#"Conjunctive accent; in poetry can function as Mehuppakh Legarmeh."#)
        );
        assert_eq!(
            ProseAccent::Merkha.notes(),
            Some(r#"Lengthens the phrase; often precedes Revia or Telisha."#)
        );
        assert_eq!(
            ProseAccent::MerkhaKephulah.notes(),
            Some(r#"Merkha duplex"#)
        );
        assert_eq!(
            ProseAccent::Darga.notes(),
            Some(r#"Conjunctive accent; synonymous with Qadma in some traditions."#)
        );
        assert_eq!(ProseAccent::Azla.notes(), Some(r#"When Geresh: Qadma"#));
        assert_eq!(
            ProseAccent::TelishaQetannah.notes(),
            Some(r#"Short form of Telisha; conjunctive function."#)
        );
        assert_eq!(
            ProseAccent::Galgal.notes(),
            Some(r#"Conjunctive accent; also known as Yerach Ben Yomo."#)
        );
        assert_eq!(
            ProseAccent::Meayla.notes(),
            Some(
                r#"Name given to a Tiphcha, when in the same word as Atnach or Silluq.
    This accent is historically the same glyph as Tipcha (U+0596) but functions as a secondary conjunctive 
    in specific syntactic contexts.
    Different traditions use different names: 
        Tipcha (common), Tarḥa, or Me'ayla/Mayela (rare scholarly names)."#,
            )
        );
        assert_eq!(
            ProseAccent::Meteg.notes(),
            Some(
                r#"In Masoretic Hebrew grammar, גַּעְיָה (gaʿyah) is a technical term referring to a secondary vocal stress,
    an additional emphasis placed on a syllable beyond the primary word stress. 
    It is physically marked by the מֶתֶג (meteg), a short vertical line (U+05BD) placed to the left of a vowel point.
    The metaphor is vivid: just as an ox lows or bellows to draw attention, 
    the gaʿyah mark draws the reader's attention to a secondary stressed syllable 
    that might otherwise be swallowed or de-emphasized. It "makes the syllable cry out," so to speak.
    It is possible to have both a Meteg and Silluq in one word,
    where the Meteg comes before the Silluq.!"#,
            )
        );
    }
}
mod poetry_notes {
    use hebrew_accents::{Accent, PoetryAccent};
    /// Test all poetry disjunctive accents (should return Some(strength))
    #[test]
    fn test_poetry_disjunctive_notes() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(
            PoetryAccent::Silluq.notes(),
            Some(
                r#"The terms Silluq and Sof Pasuq are indifferently used for the final accent of the verse.
            It is possible to have both a Silluq and Meteg in one word,
            where the Meteg comes before the Silluq.
            The Silluq appears once per verse.
"#
            )
        );
        assert_eq!(
            PoetryAccent::OlehWeYored.notes(),
            Some(
                r#"The primary_mark looks like a Mehuppakh, 
but is located above the consonant in stead of below.
It is then called OLE. When preceded by no place to attach, Yored remains alone.
The alternate BHS naming is fabricated"#,
            )
        );
        assert_eq!(
            PoetryAccent::Atnach.notes(),
            Some(
                r#"The major disjunctive accent dividing the verse in two halves.
    It appears only once per verse"#
            )
        );
        assert_eq!(
            PoetryAccent::ReviaGadol.notes(),
            Some(r#"Poetry Level 2 disjunctive; equivalent to Revia in prose."#)
        );
        assert_eq!(
            PoetryAccent::ReviaMugrash.notes(),
            Some(r#"Revia with Geresh secondary mark; indicates exiled position."#)
        );
        assert_eq!(
            PoetryAccent::ShalsheletGadol.notes(),
            Some(r#"Poetic variant of Shalshelet; larger chain form."#)
        );
        assert_eq!(
            PoetryAccent::Tsinnor.notes(),
            Some(r#"Poetry equivalent of Zarqa; appears in poetic books."#)
        );
        assert_eq!(
            PoetryAccent::ReviaQaton.notes(),
            Some(r#"After that occurs Oleh Weyored"#)
        );
        assert_eq!(
            PoetryAccent::Dechi.notes(),
            Some(r#"Poetry Level 2 disjunctive; related to Tiphcha."#)
        );
        assert_eq!(
            PoetryAccent::Pazer.notes(),
            Some(
                r#"Level 4 disjunctive; can appear in place of Zaqeph Qaton in certain conditions."#,
            )
        ); // TODO otherlevelin poetry
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.notes(),
            Some(
                r#"While the search results suggest that Mahpach Legarmeh as a complete compound term 
        is most distinctly associated with the Ashkenazi tradition, 
        all four traditions recognize the same grammatical function (a Mahpakh functioning disjunctively with a Paseq). 
        The other traditions may use the base name Mahpakh, and determine the Legarmeh function from context, 
        rather than using the full compound name systematically. 
        This reflects a broader pattern where Ashkenazi terminology tends to be more explicit with compound names 
        (e.g., Munach Legarmeh), while Sephardi and other traditions may use simpler designations."#,
            )
        );
        assert_eq!(
            PoetryAccent::AzlaLegarmeh.notes(),
            Some(r#"Compound disjunctive in poetry; Qadma with Paseq."#)
        );
    }

    /// Test all poetry conjunctive accents (should return None)
    #[test]
    fn test_poetry_conjunctive_notes() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(
            PoetryAccent::Munach.notes(),
            Some(r#"Most common conjunctive; precedes disjunctive accents."#)
        );
        assert_eq!(
            PoetryAccent::Merkha.notes(),
            Some(r#"Lengthens the phrase; often precedes Revia or Telisha."#)
        );
        assert_eq!(
            PoetryAccent::Illuy.notes(),
            Some(r#"also called Munach superior"#)
        );
        assert_eq!(
            PoetryAccent::Tarcha.notes(),
            Some(
                r#"it refers to the effort, strain, or inconvenience involved in doing something."#
            )
        );
        assert_eq!(
            PoetryAccent::Galgal.notes(),
            Some(r#"Conjunctive accent; also known as Yerach Ben Yomo."#)
        );
        assert_eq!(
            PoetryAccent::Mehuppakh.notes(),
            Some(r#"Poetic conjunctive; variant spelling of Mahpakh."#)
        );
        assert_eq!(PoetryAccent::Azla.notes(), Some(r#"When Geresh: Qadma"#));
        assert_eq!(
            PoetryAccent::ShalsheletQetannah.notes(),
            Some(r#"Poetic conjunctive; smaller chain form."#)
        );
        assert_eq!(
            PoetryAccent::TsinnoritMerkha.notes(),
            Some(r#"Compound conjunctive in poetry; Tsinnor with Merkha secondary."#)
        );
        assert_eq!(
            PoetryAccent::TsinnoritMahpakh.notes(),
            Some(r#"Compound conjunctive in poetry; Tsinnor with Mahpakh secondary."#)
        );
        assert_eq!(
            PoetryAccent::Meteg.notes(),
            Some(
                r#"In Masoretic Hebrew grammar, גַּעְיָה (gaʿyah) is a technical term referring to a secondary vocal stress,
    an additional emphasis placed on a syllable beyond the primary word stress. 
    It is physically marked by the מֶתֶג (meteg), a short vertical line (U+05BD) placed to the left of a vowel point.
    The metaphor is vivid: just as an ox lows or bellows to draw attention, 
    the gaʿyah mark draws the reader's attention to a secondary stressed syllable 
    that might otherwise be swallowed or de-emphasized. It "makes the syllable cry out," so to speak.
    It is possible to have both a Meteg and Silluq in one word,
    where the Meteg comes before the Silluq.!"#,
            )
        );
    }
}
mod pseudo_notes {
    use hebrew_accents::{Accent, PseudoAccent};
    #[test]
    fn test_pseudo_accent_notes() {
        // Pseudo accents never have strength hierarchy
        assert_eq!(PseudoAccent::SophPasuq.notes(), Some(
        "it doesn't carry any theological or interpretive meaning beyond marking a boundary",
    ));
        assert_eq!(
            PseudoAccent::Maqqaph.notes(),
            Some(
                "Can link two (or more) short words together after which 
    they function as a single compound word bearing a single Hebrew accent.",
            )
        );
        assert_eq!(PseudoAccent::Paseq.notes(), Some(
        "It's indicating that someone or something is stopping temporarily or creating a pause.",
    ));
    }
}
