use std::borrow::Cow;
use std::fmt;

use crate::utils::encode_mime_b;

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
            Some(name) => write!(f, "{} <{}>", quote_display_name(name), self.email),
            None => write!(f, "{}", self.email),
        };
    }
}

fn quote_display_name(name: &str) -> String {
    if name.bytes().all(is_safe_ascii) {
        // 2 for the quotes, and another 2 as a buffer for the escaped chars
        let mut result = String::with_capacity(name.len() + 2 + 2);

        result.push('"');
        for byte in name.bytes() {
            match byte {
                b'\\' => result.push_str("\\\\"),
                b'"' => result.push_str("\\\""),
                _ => result.push(byte as char),
            }
        }
        result.push('"');

        return result;
    } else {
        return encode_mime_b(name);
    }
}

fn is_safe_ascii(b: u8) -> bool {
    return b' ' <= b && b <= b'~';
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

impl From<(String, String)> for Address<'static> {
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
        assert_eq!(addr.to_string(), "\"Test User\" <test@example.com>");
    }

    #[test]
    fn test_quote_display_name() {
        let quoted = quote_display_name("Mary");
        assert_eq!(quoted, "\"Mary\"");

        let quoted = quote_display_name("John Doe");
        assert_eq!(quoted, "\"John Doe\"");

        let quoted = quote_display_name("Marshall \"Eminem\" Mathers");
        assert_eq!(quoted, "\"Marshall \\\"Eminem\\\" Mathers\"");

        let quoted = quote_display_name("Saul\\Hudson");
        assert_eq!(quoted, "\"Saul\\\\Hudson\"");

        let quoted = quote_display_name("María Clara de Tolitol");
        assert_eq!(quoted, "=?UTF-8?B?TWFyw61hIENsYXJhIGRlIFRvbGl0b2w=?=");

        let quoted = quote_display_name("孫悟空");
        assert_eq!(quoted, "=?UTF-8?B?5a2r5oKf56m6?=");
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
