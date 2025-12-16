use crate::domain::{
    clubs::value_objects::club::name::ClubName, shared::value_objects::reputation::Reputation,
};

#[derive(Debug, Clone)]
pub struct Club {
    pub id: u64,
    pub name: ClubName,
    pub reputation: Reputation,
}

impl Club {
    pub fn new(id: u64, name: ClubName, reputation: Reputation) -> Self {
        Club {
            id,
            name,
            reputation,
        }
    }
}
