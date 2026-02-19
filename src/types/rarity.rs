// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/19/2026.

use crate::util::error::ParseError;
use anyhow::Result;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::{Result as RusqliteResult, ToSql};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Rarity {
    Normal,
    Magic,
    Rare,
    Unique,
}

impl Display for Rarity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rarity::Normal => "Normal",
                Rarity::Magic => "Magic",
                Rarity::Rare => "Rare",
                Rarity::Unique => "Unique",
            }
        )
    }
}

impl FromSql for Rarity {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        Rarity::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

impl FromStr for Rarity {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Rarity, ParseError> {
        match s {
            "Normal" => Ok(Rarity::Normal),
            "Magic" => Ok(Rarity::Magic),
            "Rare" => Ok(Rarity::Rare),
            "Unique" => Ok(Rarity::Unique),
            _ => Err(ParseError::InvalidRarity(s.to_string())),
        }
    }
}

impl ToSql for Rarity {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(self.to_string().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_with_valid_input_works() {
        assert_eq!(Rarity::from_str("Normal").unwrap(), Rarity::Normal);
        assert_eq!(Rarity::from_str("Magic").unwrap(), Rarity::Magic);
        assert_eq!(Rarity::from_str("Rare").unwrap(), Rarity::Rare);
        assert_eq!(Rarity::from_str("Unique").unwrap(), Rarity::Unique);
    }

    #[test]
    fn test_from_str_with_invalid_input_yields_parse_error_invalid_rarity() {
        assert!(matches!(Rarity::from_str("normal"), Err(ParseError::InvalidRarity(_))));
        assert!(matches!(Rarity::from_str("Epic"), Err(ParseError::InvalidRarity(_))));
    }
}
