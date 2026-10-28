//! The font families most often found in PDFs, and how a `/BaseFont` name maps to them.
//!
//! A subset font carries only the glyphs its document used. Typing a new character needs the
//! full program of the same face. When the family is proprietary (Calibri, Arial, Times New
//! Roman…) no open program of it exists, but metric-compatible open families do: same advance
//! widths, so the edited line keeps its length and the replaced glyphs are nearly identical.
//! Only such metric-compatible pairs are listed; families that merely look alike are not.
//!
//! Exact open families (Open Sans, Roboto, Lato, Montserrat…) need no entry: an installed or
//! craft-fonts face of the same family is matched by name alone.

use crate::embedding::face_key;

/// A proprietary family, the names PDFs give it, and its open metric-compatible replacements.
struct Family {
    /// [`face_key`] forms of the family part of `/BaseFont` names (after [`split`]).
    keys: &'static [&'static str],
    /// Open families with the same advance widths, in order of preference.
    substitutes: &'static [&'static str],
}

/// Metric-compatible replacements. Sources: the fonts' own documentation (Liberation and Croscore
/// fonts: Arial/Helvetica, Times New Roman, Courier New; Carlito: Calibri; Caladea: Cambria;
/// Gelasio: Georgia; Selawik: Segoe UI).
const FAMILIES: &[Family] = &[
    Family { keys: &["arial", "arialmt", "arialpsmt"], substitutes: &["Liberation Sans", "Arimo"] },
    // Arial was drawn on Helvetica's widths, so the same replacements fit.
    Family { keys: &["helvetica", "helveticamt"], substitutes: &["Liberation Sans", "Arimo"] },
    Family { keys: &["arialnarrow", "arialnarrowmt"], substitutes: &["Liberation Sans Narrow"] },
    Family {
        keys: &["timesnewroman", "timesnewromanps", "timesnewromanpsmt", "timesnewromanmt", "times", "timesroman"],
        substitutes: &["Liberation Serif", "Tinos"],
    },
    Family { keys: &["couriernew", "couriernewps", "couriernewpsmt", "couriernewmt", "courier"], substitutes: &["Liberation Mono", "Cousine"] },
    Family { keys: &["calibri"], substitutes: &["Carlito"] },
    Family { keys: &["cambria"], substitutes: &["Caladea"] },
    Family { keys: &["georgia"], substitutes: &["Gelasio"] },
    Family { keys: &["segoeui"], substitutes: &["Selawik"] },
];

/// Style words that may end a `/BaseFont` name without a separator ("ArialBold").
const STYLE_SUFFIXES: &[&str] = &["bolditalic", "boldoblique", "italic", "oblique", "bold", "regular"];

/// Split a `/BaseFont` or PostScript name into ([`face_key`] of the family, normalised style).
///
/// `ABCDEF+OpenSans-SemiBold` → (`opensans`, `semibold`); `TimesNewRomanPS-BoldItalicMT` →
/// (`timesnewromanps`, `bolditalic`); `Calibri,Bold` → (`calibri`, `bold`); `ArialMT` →
/// (`arialmt`, `regular`).
pub fn split(name: &str) -> (String, String) {
    let name = name.rsplit('+').next().unwrap_or(name);
    let (family, style) = match name.find(['-', ',']) {
        Some(at) => (name.get(..at).unwrap_or(name), name.get(at + 1..).unwrap_or("")),
        None => (name, ""),
    };
    let mut family = face_key(family);
    let mut style = face_key(style);
    if style.is_empty()
        && let Some(suffix) = STYLE_SUFFIXES.iter().find(|s| family.len() > s.len() && family.ends_with(*s))
    {
        family.truncate(family.len() - suffix.len());
        style = (*suffix).to_owned();
    }
    (family, style_key(&style))
}

/// One spelling per style: "Roman", "Book", "Normal" and "" are `regular`; "Oblique" is `italic`;
/// a trailing PostScript "MT"/"PS"/"PSMT" is dropped.
pub fn style_key(style: &str) -> String {
    let mut s = face_key(style);
    for tail in ["psmt", "mt", "ps"] {
        if s.len() > tail.len() && s.ends_with(tail) {
            s.truncate(s.len() - tail.len());
            break;
        }
    }
    let s = s.replace("oblique", "italic");
    match s.as_str() {
        "" | "roman" | "book" | "normal" | "regular" | "mt" | "psmt" => "regular".to_owned(),
        "italicbold" => "bolditalic".to_owned(),
        _ => s,
    }
}

/// The open metric-compatible families for a family key from [`split`], best first. Empty for
/// families with no such replacement (and for open families, which match themselves).
pub fn substitutes(family_key: &str) -> &'static [&'static str] {
    FAMILIES.iter().find(|f| f.keys.contains(&family_key)).map_or(&[], |f| f.substitutes)
}

/// Every open family [`substitutes`] can name, for documentation and the fonts guide.
pub fn all_substitutes() -> Vec<&'static str> {
    let mut all: Vec<&str> = FAMILIES.iter().flat_map(|f| f.substitutes.iter().copied()).collect();
    all.sort_unstable();
    all.dedup();
    all
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_font_names_split_into_family_and_style() {
        let s = |n: &str| {
            let (f, st) = split(n);
            (f, st)
        };
        assert_eq!(s("ABCDEF+OpenSans-SemiBold"), ("opensans".into(), "semibold".into()));
        assert_eq!(s("OpenSans-Regular"), ("opensans".into(), "regular".into()));
        assert_eq!(s("TimesNewRomanPS-BoldItalicMT"), ("timesnewromanps".into(), "bolditalic".into()));
        assert_eq!(s("TimesNewRomanPSMT"), ("timesnewromanpsmt".into(), "regular".into()));
        assert_eq!(s("Arial-BoldMT"), ("arial".into(), "bold".into()));
        assert_eq!(s("Calibri,Bold"), ("calibri".into(), "bold".into()));
        assert_eq!(s("Calibri,BoldItalic"), ("calibri".into(), "bolditalic".into()));
        assert_eq!(s("ArialBold"), ("arial".into(), "bold".into()));
        assert_eq!(s("Helvetica-Oblique"), ("helvetica".into(), "italic".into()));
        assert_eq!(s("Carlito"), ("carlito".into(), "regular".into()));
        // Names that only look like a style stay whole.
        assert_eq!(s("Bold"), ("bold".into(), "regular".into()));
        assert_eq!(s(""), (String::new(), "regular".into()));
        assert_eq!(s("+"), (String::new(), "regular".into()));
    }

    #[test]
    fn proprietary_families_name_metric_compatible_open_replacements() {
        assert_eq!(substitutes(&split("ABCDEF+Calibri").0), ["Carlito"]);
        assert_eq!(substitutes(&split("Calibri-Bold").0), ["Carlito"]);
        assert_eq!(substitutes(&split("ArialMT").0), ["Liberation Sans", "Arimo"]);
        assert_eq!(substitutes(&split("TimesNewRomanPSMT").0), ["Liberation Serif", "Tinos"]);
        assert_eq!(substitutes(&split("CourierNewPS-BoldMT").0), ["Liberation Mono", "Cousine"]);
        assert_eq!(substitutes(&split("Cambria").0), ["Caladea"]);
        assert_eq!(substitutes(&split("Georgia-Italic").0), ["Gelasio"]);
        assert_eq!(substitutes(&split("SegoeUI-Bold").0), ["Selawik"]);
        // No look-alike guesses: Futura and open families have none.
        assert!(substitutes(&split("Futura-Bold").0).is_empty());
        assert!(substitutes(&split("OpenSans-Regular").0).is_empty());
        assert!(all_substitutes().contains(&"Carlito"));
    }
}
