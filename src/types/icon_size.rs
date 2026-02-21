// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/6/2026.

use crate::util::errors::FgdbRangeError;
use anyhow::Result;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::{Result as RusqliteResult, ToSql};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::ops::RangeInclusive;

pub const MIN_ICON_SIZE: u8 = 0;
pub const MAX_ICON_SIZE: u8 = 2;
pub const ICON_SIZE_RANGE: RangeInclusive<u8> = MIN_ICON_SIZE..=MAX_ICON_SIZE;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IconSize {
    size: u8,
}

impl Display for IconSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.size)
    }
}

impl FromSql for IconSize {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        IconSize::new(u8::column_result(value)?).map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

impl ToSql for IconSize {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(ToSqlOutput::from(self.size))
    }
}

impl IconSize {
    pub fn new(size: u8) -> Result<Self, FgdbRangeError> {
        if !ICON_SIZE_RANGE.contains(&size) {
            return Err(FgdbRangeError::IconSize(MIN_ICON_SIZE, MAX_ICON_SIZE, size));
        }
        Ok(Self { size })
    }

    pub fn size(&self) -> u8 {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::errors::FgdbRangeError;

    #[test]
    fn test_icon_size_new_works_with_valid_icon_size() {
        let icon_size = IconSize::new(1).unwrap();
        assert_eq!(1, icon_size.size());
    }

    #[test]
    fn test_icon_size_new_generates_error_with_invalid_icon_size() {
        let result = IconSize::new(6);
        assert!(matches!(result, Err(FgdbRangeError::IconSize(MIN_ICON_SIZE, MAX_ICON_SIZE, 6))));
    }
}
