use crate::domain::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersonName {
    pub nickname: String,
    pub last_name: String,
    pub first_name: String,
}

impl PersonName {
    pub fn new(
        last_name_str: &str,
        first_name_str: &str,
        nickname_str: &str,
    ) -> Result<Self, DomainError> {
        let nickname = nickname_str.trim().to_string();
        let last_name = last_name_str.trim().to_string();
        let first_name = first_name_str.trim().to_string();

        if last_name.is_empty() {
            return Err(DomainError::InputError(
                "The person last name can't be empty".to_string(),
            ));
        }

        if first_name.is_empty() {
            return Err(DomainError::InputError(
                "The person first name can't be empty".to_string(),
            ));
        }

        if nickname.is_empty() {
            return Err(DomainError::InputError(
                "The person nickname can't be empty".to_string(),
            ));
        }

        Ok(PersonName {
            nickname,
            last_name,
            first_name,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_person_name_ok() {
        let person_name = PersonName::new("Doe", "John", "Johnny");
        assert!(person_name.is_ok());
    }

    #[test]
    fn new_person_name_with_empty_last_name() {
        let person_name = PersonName::new("", "John", "Johnny");
        assert!(person_name.is_err());
    }

    #[test]
    fn new_person_name_with_empty_first_name() {
        let person_name = PersonName::new("Doe", "", "Johnny");
        assert!(person_name.is_err());
    }
}
