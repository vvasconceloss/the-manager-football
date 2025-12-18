#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchStatistics {
    pub home_goals: u8,
    pub away_goals: u8,
}

impl MatchStatistics {
    pub fn new() -> Self {
        Self {
            home_goals: 0,
            away_goals: 0,
        }
    }

    pub fn update(&mut self, home_goals: u8, away_goals: u8) {
        self.home_goals = home_goals;
        self.away_goals = away_goals;
    }
}

impl Default for MatchStatistics {
    fn default() -> Self {
        Self::new()
    }
}
