/*
├── data/                            # Static lookup tables
│   ├── mod.rs                       # Public data exports
│   ├── prose_table.rs               # PROSE_ACCENT_TABLE
│   ├── poetry_table.rs              # POETRY_ACCENT_TABLE
│   ├── pseudo_table.rs              # PSEUDO_ACCENT_TABLE
│   └── poetry_rank_map.rs           # BHS_POETRY_RANK_MAP
*/

mod accent_information;
mod poetry_table;
mod prose_table;
mod pseudo_table;
mod rank_map;

pub(crate) use poetry_table::POETRY_ACCENT_TABLE;
pub(crate) use prose_table::PROSE_ACCENT_TABLE;
pub(crate) use pseudo_table::PSEUDO_ACCENT_TABLE;
pub(crate) use rank_map::BHS_POETRY_RANK_MAP;
