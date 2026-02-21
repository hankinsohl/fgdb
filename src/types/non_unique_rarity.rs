// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/7/2026.

// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/19/2026.

use crate::util::errors::FgdbParseError;
use anyhow::Result;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::{Result as RusqliteResult, ToSql};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum NonUniqueRarity {
    Normal,
    Magic,
    Rare,
}

impl Display for NonUniqueRarity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NonUniqueRarity::Normal => "Normal",
                NonUniqueRarity::Magic => "Magic",
                NonUniqueRarity::Rare => "Rare",
            }
        )
    }
}

impl FromSql for NonUniqueRarity {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        NonUniqueRarity::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

impl FromStr for NonUniqueRarity {
    type Err = FgdbParseError;
    fn from_str(s: &str) -> Result<NonUniqueRarity, FgdbParseError> {
        match s {
            "Normal" => Ok(NonUniqueRarity::Normal),
            "Magic" => Ok(NonUniqueRarity::Magic),
            "Rare" => Ok(NonUniqueRarity::Rare),
            _ => Err(FgdbParseError::InvalidRarity(s.to_string())),
        }
    }
}

impl ToSql for NonUniqueRarity {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(self.to_string().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_with_valid_input_works() {
        assert_eq!(NonUniqueRarity::from_str("Normal").unwrap(), NonUniqueRarity::Normal);
        assert_eq!(NonUniqueRarity::from_str("Magic").unwrap(), NonUniqueRarity::Magic);
        assert_eq!(NonUniqueRarity::from_str("Rare").unwrap(), NonUniqueRarity::Rare);
    }

    #[test]
    fn test_from_str_with_invalid_input_yields_parse_error_invalid_rarity() {
        assert!(matches!(NonUniqueRarity::from_str("normal"), Err(FgdbParseError::InvalidRarity(_))));
        assert!(matches!(NonUniqueRarity::from_str("Epic"), Err(FgdbParseError::InvalidRarity(_))));
        assert!(matches!(NonUniqueRarity::from_str("Unique"), Err(FgdbParseError::InvalidRarity(_))));
    }
}
