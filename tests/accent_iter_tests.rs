use hebrew_accents::Accent;
use hebrew_accents::{HebrewAccent, PoetryAccent, ProseAccent, PseudoAccent};
use strum::IntoEnumIterator;

#[test]
fn print_all_accents() {
    for accent in HebrewAccent::iter() {
        match accent {
            HebrewAccent::Prose(p) => println!("Prose variant {:?}", p),
            HebrewAccent::Poetry(p) => println!("Poetry variant {:?}", p),
            HebrewAccent::Pseudo(p) => println!("Pseudo variant {:?}", p),
        }
    }
}
#[test]
fn get_all_prose_accents() {
    let all: Vec<ProseAccent> = HebrewAccent::iter().filter_map(|a| a.as_prose()).collect();
    println!("{:?}", all);
}

#[test]
fn count_by_category() {
    let mut prose_count = 0;
    let mut poetry_count = 0;
    let mut pseudo_count = 0;

    for accent in HebrewAccent::iter() {
        match accent {
            HebrewAccent::Prose(_) => prose_count += 1,
            HebrewAccent::Poetry(_) => poetry_count += 1,
            HebrewAccent::Pseudo(_) => pseudo_count += 1,
        }
    }

    println!(
        "Prose: {}, Poetry: {}, Pseudo: {}",
        prose_count, poetry_count, pseudo_count
    );
}

#[test]
fn itearteall() {
    // 1. Create accents
    let prose = HebrewAccent::Prose(ProseAccent::Silluq);
    let poetry: HebrewAccent = PoetryAccent::Atnach.into();
    let pseudo: HebrewAccent = PseudoAccent::Maqqeph.into();

    // 2. Print info
    println!("Prose: {}", prose);
    println!("Poetry: {}", poetry);
    println!("Pseudo: {}", pseudo);

    // 3. Iterate all variants
    println!("\nAll Hebrew accents ({}):", HebrewAccent::iter().count());
    for accent in HebrewAccent::iter() {
        if let Some(strength) = match &accent {
            HebrewAccent::Prose(p) => p.relative_strength(),
            HebrewAccent::Poetry(p) => p.relative_strength(),
            HebrewAccent::Pseudo(_) => None,
        } {
            println!("  {:?} - strength: {}", accent, strength);
        } else {
            println!("  {:?}", accent);
        }
    }

    // 4. Filter by type
    let prose_count: usize = HebrewAccent::iter()
        .filter(|a| matches!(a, HebrewAccent::Prose(_)))
        .count();

    println!("\nProse accents: {}", prose_count);
}

#[test]
fn iterate_prose_accents() {
    println!("--- ProseAccent ---");
    for accent in ProseAccent::iter() {
        println!(
            "Name:{:?}\t\tconcept:{:?}.",
            accent,
            accent.hebrew_concept()
        );
    }
}

#[test]
fn iterate_poetry_accents() {
    for accent in PoetryAccent::iter() {
        println!("{:?}", accent);
    }
}

#[test]
fn iterate_pseudo_accents() {
    for accent in PseudoAccent::iter() {
        println!("{:?}", accent);
    }
}

#[test]
fn iterate_all_accents() {
    for accent in HebrewAccent::iter() {
        println!(
            "English name: {}, rank: {:?}",
            accent.english_name(),
            accent.relative_strength()
        );
    }
}
