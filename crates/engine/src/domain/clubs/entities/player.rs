use crate::domain::{
    clubs::value_objects::player::{
        attributes::PlayerAttributes, position::positions::PlayerPositions,
    },
    shared::value_objects::person_name::PersonName,
};

#[derive(Debug, Clone)]
pub struct Player {
    pub id: u64,
    pub name: PersonName,
    pub positions: PlayerPositions,
    pub attributes: PlayerAttributes,
}

impl Player {
    pub fn new(
        id: u64,
        name: PersonName,
        positions: PlayerPositions,
        attributes: PlayerAttributes,
    ) -> Self {
        Player {
            id,
            name,
            positions,
            attributes,
        }
    }
}
