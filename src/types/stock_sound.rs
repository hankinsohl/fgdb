// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/3/2026.

use crate::util::error::ParseError;
use anyhow::Result;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::{Result as RusqliteResult, ToSql};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum StockSound {
    #[serde(rename = "1")]
    Sh1,
    #[serde(rename = "2")]
    Sh2,
    #[serde(rename = "3")]
    Sh3,
    #[serde(rename = "4")]
    Sh4,
    #[serde(rename = "5")]
    Sh5,
    #[serde(rename = "6")]
    Sh6,
    #[serde(rename = "7")]
    Sh7,
    #[serde(rename = "8")]
    Sh8,
    #[serde(rename = "9")]
    Sh9,
    #[serde(rename = "10")]
    Sh10,
    #[serde(rename = "11")]
    Sh11,
    #[serde(rename = "12")]
    Sh12,
    #[serde(rename = "13")]
    Sh13,
    #[serde(rename = "14")]
    Sh14,
    #[serde(rename = "15")]
    Sh15,
    #[serde(rename = "16")]
    Sh16,
    ShAlchemy,
    ShBlessed,
    ShChaos,
    ShDivine,
    ShExalted,
    ShFusing,
    ShGeneral,
    ShMirror,
    ShRegal,
    ShVaal,
}

impl Display for StockSound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StockSound::Sh1 => "1",
                StockSound::Sh2 => "2",
                StockSound::Sh3 => "3",
                StockSound::Sh4 => "4",
                StockSound::Sh5 => "5",
                StockSound::Sh6 => "6",
                StockSound::Sh7 => "7",
                StockSound::Sh8 => "8",
                StockSound::Sh9 => "9",
                StockSound::Sh10 => "10",
                StockSound::Sh11 => "11",
                StockSound::Sh12 => "12",
                StockSound::Sh13 => "13",
                StockSound::Sh14 => "14",
                StockSound::Sh15 => "15",
                StockSound::Sh16 => "16",
                StockSound::ShAlchemy => "ShAlchemy",
                StockSound::ShBlessed => "ShBlessed",
                StockSound::ShChaos => "ShChaos",
                StockSound::ShDivine => "ShDivine",
                StockSound::ShExalted => "ShExalted",
                StockSound::ShFusing => "ShFusing",
                StockSound::ShGeneral => "ShGeneral",
                StockSound::ShMirror => "ShMirror",
                StockSound::ShRegal => "ShRegal",
                StockSound::ShVaal => "ShVaal",
            }
        )
    }
}

impl FromSql for StockSound {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        StockSound::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

impl FromStr for StockSound {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<StockSound, ParseError> {
        match s {
            "1" => Ok(StockSound::Sh1),
            "2" => Ok(StockSound::Sh2),
            "3" => Ok(StockSound::Sh3),
            "4" => Ok(StockSound::Sh4),
            "5" => Ok(StockSound::Sh5),
            "6" => Ok(StockSound::Sh6),
            "7" => Ok(StockSound::Sh7),
            "8" => Ok(StockSound::Sh8),
            "9" => Ok(StockSound::Sh9),
            "10" => Ok(StockSound::Sh10),
            "11" => Ok(StockSound::Sh11),
            "12" => Ok(StockSound::Sh12),
            "13" => Ok(StockSound::Sh13),
            "14" => Ok(StockSound::Sh14),
            "15" => Ok(StockSound::Sh15),
            "16" => Ok(StockSound::Sh16),
            "ShAlchemy" => Ok(StockSound::ShAlchemy),
            "ShBlessed" => Ok(StockSound::ShBlessed),
            "ShChaos" => Ok(StockSound::ShChaos),
            "ShDivine" => Ok(StockSound::ShDivine),
            "ShExalted" => Ok(StockSound::ShExalted),
            "ShFusing" => Ok(StockSound::ShFusing),
            "ShGeneral" => Ok(StockSound::ShGeneral),
            "ShMirror" => Ok(StockSound::ShMirror),
            "ShRegal" => Ok(StockSound::ShRegal),
            "ShVaal" => Ok(StockSound::ShVaal),
            _ => Err(ParseError::InvalidStockSound(s.to_string())),
        }
    }
}

impl ToSql for StockSound {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(self.to_string().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_with_valid_input_works() {
        assert_eq!(StockSound::from_str("1").unwrap(), StockSound::Sh1);
        assert_eq!(StockSound::from_str("2").unwrap(), StockSound::Sh2);
        assert_eq!(StockSound::from_str("3").unwrap(), StockSound::Sh3);
        assert_eq!(StockSound::from_str("4").unwrap(), StockSound::Sh4);
        assert_eq!(StockSound::from_str("5").unwrap(), StockSound::Sh5);
        assert_eq!(StockSound::from_str("6").unwrap(), StockSound::Sh6);
        assert_eq!(StockSound::from_str("7").unwrap(), StockSound::Sh7);
        assert_eq!(StockSound::from_str("8").unwrap(), StockSound::Sh8);
        assert_eq!(StockSound::from_str("9").unwrap(), StockSound::Sh9);
        assert_eq!(StockSound::from_str("10").unwrap(), StockSound::Sh10);
        assert_eq!(StockSound::from_str("11").unwrap(), StockSound::Sh11);
        assert_eq!(StockSound::from_str("12").unwrap(), StockSound::Sh12);
        assert_eq!(StockSound::from_str("13").unwrap(), StockSound::Sh13);
        assert_eq!(StockSound::from_str("14").unwrap(), StockSound::Sh14);
        assert_eq!(StockSound::from_str("15").unwrap(), StockSound::Sh15);
        assert_eq!(StockSound::from_str("16").unwrap(), StockSound::Sh16);
        assert_eq!(StockSound::from_str("ShAlchemy").unwrap(), StockSound::ShAlchemy);
        assert_eq!(StockSound::from_str("ShBlessed").unwrap(), StockSound::ShBlessed);
        assert_eq!(StockSound::from_str("ShChaos").unwrap(), StockSound::ShChaos);
        assert_eq!(StockSound::from_str("ShDivine").unwrap(), StockSound::ShDivine);
        assert_eq!(StockSound::from_str("ShExalted").unwrap(), StockSound::ShExalted);
        assert_eq!(StockSound::from_str("ShFusing").unwrap(), StockSound::ShFusing);
        assert_eq!(StockSound::from_str("ShGeneral").unwrap(), StockSound::ShGeneral);
        assert_eq!(StockSound::from_str("ShMirror").unwrap(), StockSound::ShMirror);
        assert_eq!(StockSound::from_str("ShRegal").unwrap(), StockSound::ShRegal);
        assert_eq!(StockSound::from_str("ShVaal").unwrap(), StockSound::ShVaal);
    }

    #[test]
    fn test_from_str_with_invalid_input_yields_parse_error_invalid_stock_sound() {
        assert!(matches!(StockSound::from_str("17"), Err(ParseError::InvalidStockSound(_))));
        assert!(matches!(StockSound::from_str("ShGold"), Err(ParseError::InvalidStockSound(_))));
    }
}
