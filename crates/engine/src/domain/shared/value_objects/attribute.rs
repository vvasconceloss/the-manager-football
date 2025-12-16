use crate::domain::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute(pub u8);

impl Default for Attribute {
    fn default() -> Self {
        Attribute(10)
    }
}

impl Attribute {
    const MIN: u8 = 1;
    const MAX: u8 = 20;

    pub fn new(value: u8) -> Result<Self, DomainError> {
        if !(Self::MIN..=Self::MAX).contains(&value) {
            return Err(DomainError::InputError(format!(
                "Attribute needs a value between {0} and {1}",
                Self::MIN,
                Self::MAX
            )));
        }

        Ok(Attribute(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_attribute_err() {
        assert!(Attribute::new(0).is_err());
        assert!(Attribute::new(101).is_err());
    }

    #[test]
    fn new_attribute_ok() {
        assert_eq!(Attribute::new(1).unwrap(), Attribute::new(1).unwrap());
        assert_eq!(Attribute::new(20).unwrap(), Attribute::new(20).unwrap());
    }
}
