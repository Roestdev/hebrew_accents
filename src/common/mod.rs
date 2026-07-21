/*
│
└── scholar/                         # [scholar] feature gate - academic tools
    ├── mod.rs
    ├── comparison.rs                # Cross-tradition analysis
    ├── statistics.rs                # Quantitative corpus analysis
    ├── attribution.rs               # Bibliographic metadata
    └── export.rs                    # CSV/XML/RDF output formats

*/
mod char;

pub(crate) use char::*;