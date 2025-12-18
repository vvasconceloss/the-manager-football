use crate::domain::{
    clubs::entities::club::ClubId,
    competitions::value_objects::league::{
        name::LeagueName, row::LeagueTableRow, table::LeagueTable,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeagueId(pub u64);

#[derive(Debug, Clone)]
pub struct League {
    pub id: LeagueId,
    pub name: LeagueName,
    pub clubs: Vec<ClubId>,
    pub table: LeagueTable,
}

impl League {
    pub fn new(id: LeagueId, name: LeagueName, clubs: Vec<ClubId>) -> Self {
        let mut rows = Vec::new();

        for club in clubs.clone() {
            rows.push(LeagueTableRow::new(club));
        }

        Self {
            id,
            name,
            clubs,
            table: LeagueTable::new(rows),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_league_ok() {
        let league_id = LeagueId(1);
        let name = LeagueName::new("Premier League").unwrap();
        let clubs = vec![ClubId(1), ClubId(2), ClubId(3)];

        let league = League::new(league_id.clone(), name.clone(), clubs.clone());

        assert_eq!(league.id, league_id);
        assert_eq!(league.name, name);
        assert_eq!(league.clubs, clubs);
        assert_eq!(league.table.rows.len(), clubs.len());
    }
}
