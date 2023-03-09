use std::fmt::{self, Display, Formatter};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::{error, io};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub source: Option<String>,
    pub text: String,
}

#[derive(Debug)]
pub enum Error {
    EOI,
    Serialize(bincode::Error),
    IO(io::Error),
}

impl From<bincode::Error> for Error {
    fn from(err: bincode::Error) -> Self {
        Self::Serialize(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IO(err)
    }
}

impl error::Error for Error {}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::EOI => write!(f, "Unexpected end of input"),
            Self::IO(e) => write!(f, "IO error: {}", e),
            Self::Serialize(e) => write!(f, "Serialization error: {}", e),
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

pub struct QuoteManager {
    cache_path: PathBuf,
    quotes_path: PathBuf,
}

impl QuoteManager {
    pub fn new(quotes_path: &Path, cache_path: &Path) -> Self {
        Self {
            cache_path: cache_path.into(),
            quotes_path: quotes_path.into(),
        }
    }

    pub fn rebuild_cache(&self) -> Result<(), Error> {
        let quotes_dir = fs::read_dir(&self.quotes_path)?;
        let cache_file = File::create(&self.cache_path)?;

        let mut quotes = Vec::new();

        for file in quotes_dir {
            let file = file?;

            if file.path().extension() != Some("txt".as_ref()) {
                continue;
            }

            let content = fs::read_to_string(file.path())?;

            for quote in content.split_terminator("\n%") {
                quotes.push(Quote::try_from(quote)?);
            }
        }

        bincode::serialize_into(cache_file, &quotes).map_err(Error::Serialize)
    }

    pub fn get_cached(&self) -> Result<Vec<Quote>, Error> {
        let cache_file = File::open(&self.cache_path)?;

        bincode::deserialize_from(cache_file).map_err(Error::Serialize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let quote = Quote::try_from("Lorem ipsum dolor sit amet\n\t\t-- Lorem Ipsum");
        assert!(quote.is_ok());

        let quote = quote.unwrap();
        assert_eq!(quote.source, Some("Lorem Ipsum".into()));
        assert_eq!(quote.text, "Lorem ipsum dolor sit amet".to_string());
    }

    #[test]
    fn test_parse_invalid() {
        let quote = Quote::try_from("");
        assert!(quote.is_err());
    }

    #[test]
    fn test_display() {
        let quote = Quote {
            source: None,
            text: "Lorem ipsum dolor sit amet".into(),
        };

        assert_eq!(
            quote.to_string(),
            "\u{2018}Lorem ipsum dolor sit amet\u{2019}\n\t\u{2014} Unknown"
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
            "\u{2018}Lorem ipsum dolor sit amet\u{2019}\n\t\u{2014} Example Author"
        );
    }
}
