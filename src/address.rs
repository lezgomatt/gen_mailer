use std::borrow::Cow;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address<'a> {
    pub name: Option<Cow<'a, str>>,
    pub email: Cow<'a, str>,
}

impl<'a> Address<'a> {
    pub fn new(email: impl Into<Cow<'a, str>>) -> Self {
        return Self {
            name: None,
            email: email.into(),
        };
    }

    pub fn with_name(name: impl Into<Cow<'a, str>>, email: impl Into<Cow<'a, str>>) -> Self {
        return Self {
            name: Some(name.into()),
            email: email.into(),
        };
    }
}

impl fmt::Display for Address<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match &self.name {
            Some(name) => write!(f, "{} <{}>", name, self.email),
            None => write!(f, "{}", self.email),
        };
    }
}

impl<'a> From<&'a str> for Address<'a> {
    fn from(s: &'a str) -> Self {
        if s.ends_with(">") {
            if let Some(i) = s.find("<") {
                let name = s[..i].trim();
                let email = &s[i + 1..s.len() - 1];

                return Address::with_name(name, email);
            }
        }

        return Address::new(s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_name() {
        let addr = Address::new("test@example.com");
        assert!(addr.name.is_none());
        assert_eq!(addr.email, "test@example.com");
        assert_eq!(addr.to_string(), "test@example.com");
    }

    #[test]
    fn test_with_name() {
        let addr = Address::with_name("Test User", "test@example.com");
        assert!(matches!(addr.name.as_deref(), Some("Test User")));
        assert_eq!(addr.email, "test@example.com");
        assert_eq!(addr.to_string(), "Test User <test@example.com>");
    }

    #[test]
    fn test_from_str() {
        let no_name = Address::from("test@example.com");
        assert!(no_name.name.is_none());
        assert_eq!(no_name.email, "test@example.com");

        let with_name = Address::from("Test User <test@example.com>");
        assert!(matches!(with_name.name.as_deref(), Some("Test User")));
        assert_eq!(with_name.email, "test@example.com");
    }
}
