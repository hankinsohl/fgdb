// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/30/2026.

use crate::util::errors::FgdbParseError;
use anyhow::Result;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::{Result as RusqliteResult, ToSql};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Liquidity {
    Exchange,
    Async,
    Untradable,
}

impl Display for Liquidity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Liquidity::Exchange => "Exchange",
                Liquidity::Async => "Async",
                Liquidity::Untradable => "Untradable",
            }
        )
    }
}

impl FromSql for Liquidity {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        Liquidity::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

impl FromStr for Liquidity {
    type Err = FgdbParseError;
    fn from_str(s: &str) -> Result<Liquidity, FgdbParseError> {
        match s {
            "Exchange" => Ok(Liquidity::Exchange),
            "Async" => Ok(Liquidity::Async),
            "Untradable" => Ok(Liquidity::Untradable),
            _ => Err(FgdbParseError::InvalidLiquidity(s.to_string())),
        }
    }
}

impl ToSql for Liquidity {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(self.to_string().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_with_valid_input_works() {
        assert_eq!(Liquidity::from_str("Exchange").unwrap(), Liquidity::Exchange);
        assert_eq!(Liquidity::from_str("Async").unwrap(), Liquidity::Async);
        assert_eq!(Liquidity::from_str("Untradable").unwrap(), Liquidity::Untradable);
    }

    #[test]
    fn test_from_str_with_invalid_input_yields_parse_error_invalid_liquidity() {
        assert!(matches!(Liquidity::from_str("Auction"), Err(FgdbParseError::InvalidLiquidity(_))));
        assert!(matches!(Liquidity::from_str("in person"), Err(FgdbParseError::InvalidLiquidity(_))));
    }
}
