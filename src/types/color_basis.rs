// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 3/2/2026.

use crate::util::errors::FgdbParseError;
use anyhow::Result;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::{Result as RusqliteResult, ToSql};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

/// The basis for choosing the background color for drops.  Most drops are colored on the basis of their class, but some
/// drops such as essences, are colored on the basis of their base type.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ColorBasis {
    BaseType,
    Class,
}

impl Display for ColorBasis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ColorBasis::BaseType => "BaseType",
                ColorBasis::Class => "Class",
            }
        )
    }
}

impl FromSql for ColorBasis {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        ColorBasis::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

impl FromStr for ColorBasis {
    type Err = FgdbParseError;
    fn from_str(s: &str) -> Result<ColorBasis, FgdbParseError> {
        match s {
            "BaseType" => Ok(ColorBasis::BaseType),
            "Class" => Ok(ColorBasis::Class),
            _ => Err(FgdbParseError::InvalidColorBasis(s.to_string())),
        }
    }
}

impl ToSql for ColorBasis {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(self.to_string().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_with_valid_input_works() {
        assert_eq!(ColorBasis::from_str("BaseType").unwrap(), ColorBasis::BaseType);
        assert_eq!(ColorBasis::from_str("Class").unwrap(), ColorBasis::Class);
    }

    #[test]
    fn test_from_str_with_invalid_input_yields_parse_error_invalid_color_basis() {
        assert!(matches!(ColorBasis::from_str("Item"), Err(FgdbParseError::InvalidColorBasis(_))));
        assert!(matches!(ColorBasis::from_str("BaseTypeItem"), Err(FgdbParseError::InvalidColorBasis(_))));
    }
}
