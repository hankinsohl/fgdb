// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/3/2026.

use crate::util::error::ParseError;
use anyhow::Result;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::Result as RusqliteResult;
use rusqlite::ToSql;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum StockColor {
    Blue,
    Brown,
    Cyan,
    Green,
    Grey,
    Orange,
    Pink,
    Purple,
    Red,
    White,
    Yellow,
}

impl Display for StockColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StockColor::Blue => "Blue",
                StockColor::Brown => "Brown",
                StockColor::Cyan => "Cyan",
                StockColor::Green => "Green",
                StockColor::Grey => "Grey",
                StockColor::Orange => "Orange",
                StockColor::Pink => "Pink",
                StockColor::Purple => "Purple",
                StockColor::Red => "Red",
                StockColor::White => "White",
                StockColor::Yellow => "Yellow",
            }
        )
    }
}

impl FromSql for StockColor {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        StockColor::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

impl FromStr for StockColor {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<StockColor, ParseError> {
        match s {
            "Blue" => Ok(StockColor::Blue),
            "Brown" => Ok(StockColor::Brown),
            "Cyan" => Ok(StockColor::Cyan),
            "Green" => Ok(StockColor::Green),
            "Grey" => Ok(StockColor::Grey),
            "Orange" => Ok(StockColor::Orange),
            "Pink" => Ok(StockColor::Pink),
            "Purple" => Ok(StockColor::Purple),
            "Red" => Ok(StockColor::Red),
            "White" => Ok(StockColor::White),
            "Yellow" => Ok(StockColor::Yellow),
            _ => Err(ParseError::InvalidStockColor(s.to_string())),
        }
    }
}

impl ToSql for StockColor {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(self.to_string().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_with_valid_input_works() {
        assert_eq!(StockColor::from_str("Blue").unwrap(), StockColor::Blue);
        assert_eq!(StockColor::from_str("Brown").unwrap(), StockColor::Brown);
        assert_eq!(StockColor::from_str("Cyan").unwrap(), StockColor::Cyan);
        assert_eq!(StockColor::from_str("Green").unwrap(), StockColor::Green);
        assert_eq!(StockColor::from_str("Grey").unwrap(), StockColor::Grey);
        assert_eq!(StockColor::from_str("Orange").unwrap(), StockColor::Orange);
        assert_eq!(StockColor::from_str("Pink").unwrap(), StockColor::Pink);
        assert_eq!(StockColor::from_str("Purple").unwrap(), StockColor::Purple);
        assert_eq!(StockColor::from_str("Red").unwrap(), StockColor::Red);
        assert_eq!(StockColor::from_str("White").unwrap(), StockColor::White);
        assert_eq!(StockColor::from_str("Yellow").unwrap(), StockColor::Yellow);
    }

    #[test]
    fn test_from_str_with_invalid_input_yields_parse_error_invalid_stock_color() {
        assert!(matches!(StockColor::from_str("Black"), Err(ParseError::InvalidStockColor(_))));
        assert!(matches!(StockColor::from_str("green"), Err(ParseError::InvalidStockColor(_))));
    }
}
