//! Static data, used in various places
//!
//! If the UTF-8 naming is inline with the Hebrew Accents,
//! then these will be used.
//! Otherwise an alias will be used.
//!
//! This is done to keep all code in line from the perspective
//! of the Hebrew Accents according the BHS
//!
//!

/// --------------------------------------------------
///  HEBREW characters ; UTF-8 codepoints as '&str'
///  
/// Naming according UNICODE
/// --------------------------------------------------
/// accents
pub(crate) const ETNAHTA: &str = "\u{0591}";
pub(crate) const SEGOL: &str = "\u{0592}";
pub(crate) const SHALSHELET: &str = "\u{0593}";
pub(crate) const ZAQEF_QATAN: &str = "\u{0594}";
pub(crate) const ZAQEF_GADOL: &str = "\u{0595}";
pub(crate) const TIPEHA: &str = "\u{0596}";
pub(crate) const REVIA: &str = "\u{0597}";
pub(crate) const ZARQA: &str = "\u{0598}";
pub(crate) const PASHTA: &str = "\u{0599}";
pub(crate) const YETIV: &str = "\u{059A}";
pub(crate) const TEVIR: &str = "\u{059B}";
pub(crate) const GERESH: &str = "\u{059C}";
///pub(crate) const GERESH_MUQDAM:&str = "\u{059D}";
pub(crate) const GERSHAYIM: &str = "\u{059E}";
pub(crate) const QARNEY_PARA: &str = "\u{059F}";
pub(crate) const TELISHA_GEDOLAH: &str = "\u{05A0}";
pub(crate) const PAZER: &str = "\u{05A1}";
///pub(crate) const ATNAH_HAFUKH:&str = "\u{05A2}";
pub(crate) const MUNAH: &str = "\u{05A3}";
pub(crate) const MAHPAKH: &str = "\u{05A4}";
pub(crate) const MERKHA: &str = "\u{05A5}";
pub(crate) const MERKHA_KEFULA: &str = "\u{05A6}";
pub(crate) const DARGA: &str = "\u{05A7}";
pub(crate) const QADMA: &str = "\u{05A8}";
pub(crate) const TELISHA_QETANA: &str = "\u{05A9}";
pub(crate) const YERAH_BEN_YOMO: &str = "\u{05AA}";
pub(crate) const OLEH: &str = "\u{05AB}";
pub(crate) const ILUY: &str = "\u{05AC}";
pub(crate) const DECHI: &str = "\u{05AD}";
pub(crate) const ZINOR: &str = "\u{05AE}";
/// misceleneous
pub(crate) const METEG: &str = "\u{05BD}";
pub(crate) const MAQAF: &str = "\u{05BE}";
///pub(crate) const RAFE:&str = "\u{05BF}";
pub(crate) const PASEQ: &str = "\u{05C0}";
pub(crate) const SOF_PASUQ: &str = "\u{05C3}";
/// pub(crate) const VERTICAL_LINE: &str = "\u{007C}";

/// --------------------------------------------------
///  HEBREW characters ; UTF-8 codepoints as 'char'
///  
/// Naming according UNICODE
/// --------------------------------------------------
/// accents
//pub(crate) const ETNAHTA_AS_CHAR: char = '\u{0591}';
pub(crate) const SEGOL_AS_CHAR: char = '\u{0592}';
//pub(crate) const SHALSHELET_AS_CHAR: char = '\u{0593}';
pub(crate) const ZAQEF_QATAN_AS_CHAR: char = '\u{0594}';
pub(crate) const ZAQEF_GADOL_AS_CHAR: char = '\u{0595}';
//pub(crate) const TIPEHA_AS_CHAR: char = '\u{0596}';
//pub(crate) const REVIA_AS_CHAR: char = '\u{0597}';
pub(crate) const ZARQA_AS_CHAR: char = '\u{0598}';
pub(crate) const PASHTA_AS_CHAR: char = '\u{0599}';
pub(crate) const YETIV_AS_CHAR: char = '\u{059A}';
pub(crate) const TEVIR_AS_CHAR: char = '\u{059B}';
pub(crate) const GERESH_AS_CHAR: char = '\u{059C}';
pub(crate) const GERESH_MUQDAM_AS_CHAR: char = '\u{059D}';
pub(crate) const GERSHAYIM_AS_CHAR: char = '\u{059E}';
pub(crate) const QARNEY_PARA_AS_CHAR: char = '\u{059F}';
pub(crate) const TELISHA_GEDOLAH_AS_CHAR: char = '\u{05A0}';
//pub(crate) const PAZER_AS_CHAR: char = '\u{05A1}';
pub(crate) const ATNAH_HAFUKH_AS_CHAR: char = '\u{05A2}';
//pub(crate) const MUNAH_AS_CHAR: char = '\u{05A3}';
//pub(crate) const MAHPAKH_AS_CHAR: char = '\u{05A4}';
pub(crate) const MERKHA_AS_CHAR: char = '\u{05A5}';
pub(crate) const MERKHA_KEFULA_AS_CHAR: char = '\u{05A6}';
pub(crate) const DARGA_AS_CHAR: char = '\u{05A7}';
pub(crate) const QADMA_AS_CHAR: char = '\u{05A8}';
pub(crate) const TELISHA_QETANA_AS_CHAR: char = '\u{05A9}';
//pub(crate) const YERAH_BEN_YOMO_AS_CHAR: char = '\u{05AA}';
pub(crate) const OLEH_AS_CHAR: char = '\u{05AB}';
pub(crate) const ILUY_AS_CHAR: char = '\u{05AC}';
pub(crate) const DECHI_AS_CHAR: char = '\u{05AD}';
//pub(crate) const ZINOR_AS_CHAR: char = '\u{05AE}';
/// misceleneous
//pub(crate) const METEG_AS_CHAR: char = '\u{05BD}';
pub(crate) const MAQAF_AS_CHAR: char = '\u{05BE}';
//pub(crate) const RAFE_AS_CHAR: char = '\u{05BF}';
pub(crate) const PASEQ_AS_CHAR: char = '\u{05C0}';
//pub(crate) const SOF_PASUQ_AS_CHAR: char = '\u{05C3}';
pub(crate) const VERTICAL_LINE_AS_CHAR: char = '\u{007C}';

// ---------------------------------------------------
//  ALIAS
// --------------------------------------------------
/// alias for ZARQA
pub(crate) const TSINNORIT: &str = ZARQA;
pub(crate) const TSINNORIT_AS_CHAR: char = ZARQA_AS_CHAR;
/// alias for MERKHA
pub(crate) const YORED: &str = MERKHA;
pub(crate) const YORED_AS_CHAR: char = MERKHA_AS_CHAR;
/// alias for QADMA
pub(crate) const AZLA: &str = QADMA;
/// alias for METEG
pub(crate) const SILLUQ: &str = METEG;
/// aliases for TIPEHA
pub(crate) const MEAYLA: &str = TIPEHA;
pub(crate) const TARCHA: &str = TIPEHA;
/// alias for ETNAHTA
pub(crate) const ATNACH: &str = ETNAHTA;
/// alias for MAQAF
pub(crate) const MAQQAPH: &str = MAQAF;
pub(crate) const MAQQAPH_AS_CHAR: char = MAQAF_AS_CHAR;
/// alias for YERAH_BEN_YOMO
pub(crate) const GALGAL: &str = YERAH_BEN_YOMO;
// alias for MUNAH
//pub(crate) const MUNACH: &str = MUNAH;
/// alias for SEGOL
pub(crate) const SEGOLTA: &str = SEGOL;
/// alias for QARNEY_PARA
pub(crate) const PAZER_GADOL: &str = QARNEY_PARA;
/// alias for TIPEHA
pub(crate) const TIPHCHA: &str = TIPEHA;
