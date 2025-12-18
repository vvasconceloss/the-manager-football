#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchStatus {
    Finished,
    Canceled,
    Scheduled,
    Postponed,
    InProgress,
}
