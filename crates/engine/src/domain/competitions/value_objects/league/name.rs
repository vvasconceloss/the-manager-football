use crate::domain::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeagueName(pub String);

impl LeagueName {
    pub fn new(name_str: &str) -> Result<Self, DomainError> {
        let name = name_str.trim().to_string();

        if name.is_empty() {
            return Err(DomainError::InputError(
                "The league name can't be empty".to_string(),
            ));
        }

        Ok(LeagueName(name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_league_name_ok() {
        let name = LeagueName::new("Premier League");
        assert!(name.is_ok());
        assert_eq!(name.unwrap(), LeagueName("Premier League".to_string()));
    }

    #[test]
    fn new_league_name_empty() {
        let name = LeagueName::new("");
        assert!(name.is_err());
    }
}
