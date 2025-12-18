use crate::domain::competitions::value_objects::league::row::LeagueTableRow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeagueTable {
    pub rows: Vec<LeagueTableRow>,
}

impl LeagueTable {
    pub fn new(rows: Vec<LeagueTableRow>) -> Self {
        Self { rows }
    }
}
