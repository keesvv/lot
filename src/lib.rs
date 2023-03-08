use std::error;
use std::fmt::{self, Display, Formatter};

pub struct Quote {
    pub author: Option<String>,
    pub text: String,
}

#[derive(Debug)]
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
        let mut quote = value.split_terminator('\n');

        Ok(Self {
            text: quote.next().ok_or(Error::EOI)?.into(),
            author: quote.next().map(ToString::to_string),
        })
    }
}

impl Display for Quote {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "\"{}\"\n\t- {}",
            self.text,
            self.author.clone().unwrap_or("Unknown".into())
        )
    }
}

pub trait Quoter: Iterator<Item = Quote> {}
