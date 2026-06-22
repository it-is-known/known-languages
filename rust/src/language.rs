// This is free and unencumbered software released into the public domain.

use core::str::FromStr;

#[cfg(feature = "alloc")]
use alloc::string::String;

/// An enumerated language.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[non_exhaustive]
pub enum Language {
    #[default]
    English,

    #[cfg(feature = "alloc")]
    Other(String),
}

impl Language {
    pub const ALL: &'static [Self] = &[Self::English];

    pub fn as_str(&self) -> &str {
        use Language::*;
        match self {
            English => "en",

            #[cfg(feature = "alloc")]
            Other(input) => input.as_str(),
        }
    }

    #[cfg(feature = "alloc")]
    pub fn into_string(self) -> String {
        self.as_str().into()
    }

    #[cfg(all(feature = "serde", feature = "alloc"))]
    pub fn to_json(&self) -> Option<serde_json::Value> {
        Some(self.clone().into_json())
    }

    #[cfg(all(feature = "serde", feature = "alloc"))]
    pub fn into_json(self) -> serde_json::Value {
        serde_json::Value::String(self.into_string())
    }

    #[cfg(all(feature = "bson", feature = "alloc"))]
    pub fn to_bson(&self) -> Option<bson::Bson> {
        Some(self.clone().into_bson())
    }

    #[cfg(all(feature = "bson", feature = "alloc"))]
    pub fn into_bson(self) -> bson::Bson {
        bson::Bson::String(self.into_string())
    }
}

impl FromStr for Language {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Language::*;
        Ok(match input {
            "en" => English,

            #[cfg(feature = "alloc")]
            input => Other(input.into()),

            #[cfg(not(feature = "alloc"))]
            _ => return Err(()),
        })
    }
}

impl core::fmt::Display for Language {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(all(feature = "serde", feature = "alloc"))]
impl TryFrom<serde_json::Value> for Language {
    type Error = ();

    fn try_from(input: serde_json::Value) -> Result<Self, Self::Error> {
        use serde_json::Value;
        match input {
            Value::String(input) => Ok(input.parse().unwrap_or_else(|_| Self::Other(input))),
            _ => Err(()),
        }
    }
}
