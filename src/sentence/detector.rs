use crate::sentence::PASHTA_AS_CHAR;
use crate::sentence::validator::validate_sentence;
use crate::sentence::ATNAH_HAFUKH_AS_CHAR;
use crate::sentence::DARGA_AS_CHAR;
use crate::sentence::DECHI_AS_CHAR;
use crate::sentence::GERESH_MUQDAM_AS_CHAR;
use crate::sentence::GERSHAYIM_AS_CHAR;
use crate::sentence::ILUY_AS_CHAR;
use crate::sentence::MERKHA_KEFULA_AS_CHAR;
use crate::sentence::OLEH_AS_CHAR;
use crate::sentence::QARNEY_PARA_AS_CHAR;
use crate::sentence::SEGOL_AS_CHAR;
use crate::sentence::TELISHA_GEDOLAH_AS_CHAR;
use crate::sentence::TELISHA_QETANA_AS_CHAR;
use crate::sentence::TEVIR_AS_CHAR;
use crate::sentence::TSINNORIT_AS_CHAR;
use crate::sentence::YETIV_AS_CHAR;
use crate::sentence::ZAQEF_GADOL_AS_CHAR;
use crate::sentence::ZAQEF_QATAN_AS_CHAR;
use crate::Context;
use crate::SentenceContextError;

pub(crate) fn detect_context_from_sentence(
    sentence: &str,
) -> Result<Context, SentenceContextError> {
    validate_sentence(sentence)?;

    let mut could_be_prose = false;
    let mut could_be_poetry = false;

    // Single pass through characters
    for c in sentence.chars() {
        // Check if this char is a prose-exclusive accent
        if matches!(
            c,
            SEGOL_AS_CHAR // Segol
                | ZAQEF_QATAN_AS_CHAR // Zaqeph Qatan
                | ZAQEF_GADOL_AS_CHAR // Zaqef Gadol
                | PASHTA_AS_CHAR // Pashta
                | TEVIR_AS_CHAR // Tevir
                | YETIV_AS_CHAR // Yetiv
                | GERSHAYIM_AS_CHAR // Gershayim
                | QARNEY_PARA_AS_CHAR // Qarney Para
                | TELISHA_GEDOLAH_AS_CHAR // Telisha Gedolah
                | MERKHA_KEFULA_AS_CHAR
                | DARGA_AS_CHAR // Darga
                | TELISHA_QETANA_AS_CHAR
        ) {
            could_be_prose = true;
        }

        // Check if this char is a poetry-exclusive accent
        if matches!(
            c,
            TSINNORIT_AS_CHAR  // \u{0598}
            | GERESH_MUQDAM_AS_CHAR // Geresh Muqdam
            | ATNAH_HAFUKH_AS_CHAR  // Atnach Hafukh 
            | OLEH_AS_CHAR  // \u{05AB}
            | ILUY_AS_CHAR  // \u{05AC}
            | DECHI_AS_CHAR // \u{05AD};
        ) {
            could_be_poetry = true;
        }
    }

    match (could_be_prose, could_be_poetry) {
        (false, false) => Err(SentenceContextError::InsufficientData("// TODO")),
        (false, true) => Ok(Context::Poetic),
        (true, false) => Ok(Context::Prosaic),
        (true, true) => Err(SentenceContextError::Ambiguous("// TODO")),
    }
}

#[cfg(test)]
mod prose_only_detection {

    // ============================================================
    // PROSE-ONLY DETECTION PATHS
    // ============================================================
}

#[cfg(test)]
mod poetry_only_detection {
    // ============================================================
    // POETRY-ONLY DETECTION PATHS
    // ============================================================
}

#[cfg(test)]
mod prose_and_poetry_detection {
    // ============================================================
    // BOTH DETECTED (EARLY EXIT)
    // ============================================================
}
#[cfg(test)]
mod no_signal_detection {
    // ============================================================
    // VALIDATION ERROR PROPAGATION
    // ============================================================
}

#[cfg(test)]
mod edge_cases {
    // ============================================================
    // EDGE CASES AND BOUNDARIES
    // ============================================================
}
