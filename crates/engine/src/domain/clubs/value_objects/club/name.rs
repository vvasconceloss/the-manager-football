use crate::domain::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClubName(pub String);

impl ClubName {
    pub fn new(name_str: &str) -> Result<Self, DomainError> {
        let name = name_str.trim().to_string();

        if name.is_empty() {
            return Err(DomainError::InputError(
                "The club name can't be empty".to_string(),
            ));
        }

        Ok(ClubName(name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_club_name_ok() {
        let name = ClubName::new("Real Madrid");
        assert!(name.is_ok());
        assert_eq!(name.unwrap(), ClubName("Real Madrid".to_string()));
    }

    #[test]
    fn new_club_name_empty() {
        let name = ClubName::new("");
        assert!(name.is_err());
    }
}
