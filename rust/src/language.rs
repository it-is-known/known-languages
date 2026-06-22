// This is free and unencumbered software released into the public domain.

use alloc::{fmt, string::String};
use core::str::FromStr;

/// An enumerated language.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[non_exhaustive]
pub enum Language {
    English,
    Other(String),
}

impl Language {
    pub const ALL: &'static [Self] = &[
        Self::English,
        //Self::Other(String),
    ];

    pub fn as_str(&self) -> &str {
        use Language::*;
        match self {
            English => "en",
            Other(scheme) => scheme.as_str(),
        }
    }
}

impl FromStr for Language {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Language::*;
        Ok(match input {
            "en" => English,
            scheme => Other(scheme.into()),
        })
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
