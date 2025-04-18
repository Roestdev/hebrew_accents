pub enum BookGenre {
    Prose,
    NonProse,
}

pub enum AccentType {
    Conjuntive,
    Disjuntive { group: u8 },
}
pub struct AccentProperty {
    _name: String,
    _alias: String,
    _code_points: u8,
}
