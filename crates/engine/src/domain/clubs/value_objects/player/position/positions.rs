use crate::domain::{
    clubs::value_objects::player::position::position_type::PositionType, errors::DomainError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    pub proficiency: u8,
    pub position: PositionType,
}

impl Position {
    const MIN_VALUE: u8 = 1;
    const MAX_VALUE: u8 = 5;

    pub fn new(position: PositionType, proficiency: u8) -> Result<Self, DomainError> {
        if !(Self::MIN_VALUE..=Self::MAX_VALUE).contains(&proficiency) {
            return Err(DomainError::InputError(format!(
                "Proficiency must be between {0} and {1}",
                Self::MIN_VALUE,
                Self::MAX_VALUE
            )));
        }

        Ok(Position {
            position,
            proficiency,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerPositions {
    pub positions: Vec<Position>,
}

impl PlayerPositions {
    pub fn new(positions: Vec<Position>) -> Self {
        PlayerPositions { positions }
    }

    pub fn add_position(&mut self, position: Position) {
        self.positions.push(position);
    }

    pub fn has_position(&self, position: PositionType) -> bool {
        self.positions.iter().any(|p| p.position == position)
    }

    pub fn position_proficiency(&self, position: PositionType) -> u8 {
        match self.positions.iter().find(|p| p.position == position) {
            Some(p) => p.proficiency,
            None => 0,
        }
    }
}
