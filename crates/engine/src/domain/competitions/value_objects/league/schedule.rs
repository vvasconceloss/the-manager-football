use crate::domain::{
    clubs::entities::club::ClubId,
    competitions::{entities::r#match::MatchId, value_objects::r#match::status::MatchStatus},
    errors::DomainError,
};
use chrono::{NaiveDate, Weekday};

#[derive(Debug, Clone)]
pub struct LeagueConfig {
    pub rounds_per_pair: u8, // 1 = single round-robin, 2 = double
    pub days_between_rounds: u16,
    pub blackout_dates: Vec<NaiveDate>,
    pub preferred_weekdays: Vec<Weekday>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeagueMatchId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeagueMatch {
    pub id: LeagueMatchId,
    pub home_club: ClubId,
    pub away_club: ClubId,
    pub status: MatchStatus,
    pub match_id: Option<MatchId>,
    pub scheduled_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchDay {
    pub day_index: u32,
    pub date: NaiveDate,
    pub matches: Vec<LeagueMatch>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeagueSchedule {
    pub end_date: NaiveDate,
    pub start_date: NaiveDate,
    pub matchdays: Vec<MatchDay>,
}

impl LeagueSchedule {
    pub fn new(end_date: NaiveDate, start_date: NaiveDate) -> Result<Self, DomainError> {
        if start_date > end_date {
            return Err(DomainError::InputError(
                "The start date of the schedule cannot be later than the end date".to_string(),
            ));
        }

        Ok(LeagueSchedule {
            end_date,
            start_date,
            matchdays: Vec::new(),
        })
    }
}
