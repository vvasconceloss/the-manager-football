use crate::domain::{
    clubs::entities::team::TeamId,
    competitions::value_objects::r#match::{statistics::MatchStatistics, status::MatchStatus},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchId(pub u64);

#[derive(Debug, Clone)]
pub struct Match {
    pub id: MatchId,
    pub home_team: TeamId,
    pub away_team: TeamId,
    pub status: MatchStatus,
    pub statistics: MatchStatistics,
}

impl Match {
    pub fn new(id: MatchId, home_team: TeamId, away_team: TeamId) -> Self {
        Self {
            id,
            home_team,
            away_team,
            status: MatchStatus::Scheduled,
            statistics: MatchStatistics::new(),
        }
    }
}
