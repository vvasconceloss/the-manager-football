use crate::domain::{
    competitions::entities::league::LeagueId, state::value_objects::season::state::SeasonState,
};
use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeasonId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Season {
    pub id: SeasonId,
    pub state: SeasonState,
    pub leagues: Vec<LeagueId>,
    pub current_date: NaiveDate,
}

impl Season {
    pub fn new(
        id: SeasonId,
        state: SeasonState,
        leagues: Vec<LeagueId>,
        current_date: NaiveDate,
    ) -> Self {
        Season {
            id,
            state,
            leagues,
            current_date,
        }
    }
}
