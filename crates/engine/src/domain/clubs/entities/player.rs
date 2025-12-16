use crate::domain::{
    clubs::value_objects::player::{
        attributes::PlayerAttributes, position::positions::PlayerPositions,
    },
    shared::value_objects::person_name::PersonName,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerId(pub u64);

#[derive(Debug, Clone)]
pub struct Player {
    pub id: PlayerId,
    pub name: PersonName,
    pub positions: PlayerPositions,
    pub attributes: PlayerAttributes,
}

impl Player {
    pub fn new(
        id: PlayerId,
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
