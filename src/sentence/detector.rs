use crate::sentence::validator::validate_sentence;
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
            '\u{0592}' // Segol
                | '\u{0594}' // Zaqeph Qatan
                | '\u{0595}' // Zaqef Gadol
                | '\u{0599}' // Pashta
                | '\u{059B}' // Tevir
                | '\u{059A}' // Yetiv
                | '\u{059E}' // Gershayim
                | '\u{059F}' // Qarney Para
                | '\u{05A0}' // Telisha Gedolah
                | '\u{05A6}' // Merkha Khfula
                | '\u{05A7}' // Darga
                | '\u{05A9}' // Qadma
        ) {
            could_be_prose = true;
        }

        // Check if this char is a poetry-exclusive accent
        if matches!(
            c,
            '\u{0598}' | // Tsinnorit
            '\u{059D}' | // Geresh Muqdam
            '\u{05A2}' | // Atnach Hafukh 
            '\u{05AB}' | // Ole
            '\u{05AC}' | // Illuy
            '\u{05AD}' // Dechi
        ) {
            could_be_poetry = true;
        }

        // ⭐ EARLY EXIT HERE ⭐
        // If both flags are true, we can STOP immediately
        if could_be_prose && could_be_poetry {
            return Err(SentenceContextError::DerivationFailed(
                "Unique prose and poetry accent markers identified",
            ));
        }
    }

    match (could_be_prose, could_be_poetry) {
        (true, false) => Ok(Context::Prosaic),
        (false, true) => Ok(Context::Poetic),
        (false, false) => Err(SentenceContextError::DerivationFailed(
            "No unique prose or poetry accent markers identified",
        )),
        // Unreachable due to early exit above
        _ => unreachable!(),
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
mod no_ganrantee_detection {
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