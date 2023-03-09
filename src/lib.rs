use std::error;
use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub source: Option<String>,
    pub text: String,
}

#[derive(Debug, Clone)]
pub enum Error {
    EOI,
}

impl error::Error for Error {}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::EOI => write!(f, "Unexpected end of input"),
        }
    }
}

impl TryFrom<&str> for Quote {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut quote = value.split_terminator("\n\t\t-- ");

        Ok(Self {
            text: quote.next().ok_or(Error::EOI)?.trim().into(),
            source: quote.next().map(ToString::to_string),
        })
    }
}

impl Display for Quote {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\u{2018}{}\u{2019}\n\t\u{2014} {}",
            self.text,
            self.source.clone().unwrap_or("Unknown".into())
        )
    }
}

pub trait Quoter: Iterator<Item = Quote> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let quote = Quote::try_from("Lorem ipsum dolor sit amet\n- Lorem Ipsum");
        assert!(quote.clone().is_ok());

        let quote = quote.unwrap();
        assert_eq!(quote.source, Some("Lorem Ipsum".into()));
        assert_eq!(quote.text, "Lorem ipsum dolor sit amet".to_string());
    }

    #[test]
    fn test_parse_invalid() {
        let quote = Quote::try_from("");
        assert!(quote.clone().is_err());
    }

    #[test]
    fn test_display() {
        let quote = Quote {
            source: None,
            text: "Lorem ipsum dolor sit amet".into(),
        };

        assert_eq!(
            quote.to_string(),
            "\"Lorem ipsum dolor sit amet\"\n\t- Unknown"
        );
    }

    #[test]
    fn test_display_with_source() {
        let quote = Quote {
            source: Some("Example Author".into()),
            text: "Lorem ipsum dolor sit amet".into(),
        };

        assert_eq!(
            quote.to_string(),
            "\"Lorem ipsum dolor sit amet\"\n\t- Example Author"
        );
    }
}
