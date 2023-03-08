use crate::value_structs::id::ParkId;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParkEntity {
    pub id: ParkId,
    pub name: String,
    pub name_ja: String,
}

impl ParkEntity {
    pub fn new(id: &ParkId, name: &str, name_ja: &str) -> Self {
        Self {
            id: id.to_owned(),
            name: name.to_string(),
            name_ja: name_ja.to_string(),
        }
    }
}
