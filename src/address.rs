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
        return Address::new(s);
    }
}

impl From<String> for Address<'static> {
    fn from(s: String) -> Self {
        return Address::new(s);
    }
}

impl<'a> From<(&'a str, &'a str)> for Address<'a> {
    fn from((name, email): (&'a str, &'a str)) -> Self {
        return Address::with_name(name, email);
    }
}

impl<'a> From<(&'a str, String)> for Address<'a> {
    fn from((name, email): (&'a str, String)) -> Self {
        return Address::with_name(name, email);
    }
}

impl<'a> From<(String, &'a str)> for Address<'a> {
    fn from((name, email): (String, &'a str)) -> Self {
        return Address::with_name(name, email);
    }
}

impl<'a> From<(String, String)> for Address<'a> {
    fn from((name, email): (String, String)) -> Self {
        return Address::with_name(name, email);
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
        let addr = Address::from("test@example.com");
        assert!(addr.name.is_none());
        assert_eq!(addr.email, "test@example.com");
    }

    #[test]
    fn test_from_string() {
        let addr = Address::from("test@example.com".to_string());
        assert!(addr.name.is_none());
        assert_eq!(addr.email, "test@example.com");
    }

    #[test]
    fn test_from_tuple() {
        let addr = Address::from(("Test User", "test@example.com"));
        assert!(matches!(addr.name.as_deref(), Some("Test User")));
        assert_eq!(addr.email, "test@example.com");

        let addr = Address::from(("Test User", "test@example.com".to_string()));
        assert!(matches!(addr.name.as_deref(), Some("Test User")));
        assert_eq!(addr.email, "test@example.com");

        let addr = Address::from(("Test User".to_string(), "test@example.com"));
        assert!(matches!(addr.name.as_deref(), Some("Test User")));
        assert_eq!(addr.email, "test@example.com");

        let addr = Address::from(("Test User".to_string(), "test@example.com".to_string()));
        assert!(matches!(addr.name.as_deref(), Some("Test User")));
        assert_eq!(addr.email, "test@example.com");
    }
}
