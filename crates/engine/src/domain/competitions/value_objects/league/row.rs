use crate::domain::clubs::entities::club::ClubId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeagueTableRow {
    pub club_id: ClubId,
    pub wins: u32,
    pub draws: u32,
    pub losses: u32,
    pub points: u32,
    pub goals_scored: u32,
    pub goals_conceded: u32,
}

impl LeagueTableRow {
    pub fn new(club_id: ClubId) -> Self {
        Self {
            club_id,
            wins: 0,
            draws: 0,
            losses: 0,
            points: 0,
            goals_scored: 0,
            goals_conceded: 0,
        }
    }
}
