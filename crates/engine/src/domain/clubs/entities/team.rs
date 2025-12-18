use crate::domain::clubs::entities::{club::ClubId, player::PlayerId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Team {
    pub id: TeamId,
    pub club_id: ClubId,
    pub squad: Vec<PlayerId>,
}

impl Team {
    pub fn new(id: TeamId, club_id: ClubId, squad: Vec<PlayerId>) -> Self {
        Team { id, club_id, squad }
    }

    pub fn add_player(&mut self, player_id: PlayerId) {
        self.squad.push(player_id);
    }

    pub fn remove_player(&mut self, player_id: PlayerId) {
        self.squad.retain(|id| *id != player_id);
    }
}
