use crate::domain::{
    clubs::value_objects::club::name::ClubName, shared::value_objects::reputation::Reputation,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClubId(pub u64);

#[derive(Debug, Clone)]
pub struct Club {
    pub id: ClubId,
    pub name: ClubName,
    pub reputation: Reputation,
}

impl Club {
    pub fn new(id: ClubId, name: ClubName, reputation: Reputation) -> Self {
        Club {
            id,
            name,
            reputation,
        }
    }
}
