use crate::domain::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reputation(pub u8);

impl Reputation {
    const MIN: u8 = 1;
    const MAX: u8 = 100;

    pub fn new(value: u8) -> Result<Self, DomainError> {
        if !(Self::MIN..=Self::MAX).contains(&value) {
            return Err(DomainError::InputError(format!(
                "Reputation value needs a value between {0} and {1}",
                Self::MIN,
                Self::MAX
            )));
        }

        Ok(Reputation(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_reputation_err() {
        assert!(Reputation::new(0).is_err());
        assert!(Reputation::new(101).is_err());
    }

    #[test]
    fn new_reputation_ok() {
        assert_eq!(Reputation::new(1).unwrap(), Reputation::new(1).unwrap());
        assert_eq!(Reputation::new(100).unwrap(), Reputation::new(100).unwrap());
    }
}
