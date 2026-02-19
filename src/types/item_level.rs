// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/7/2026.

use crate::util::error::RangeError;
use anyhow::Result;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::{Result as RusqliteResult, ToSql};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::ops::RangeInclusive;

pub const MIN_ITEM_LEVEL: u8 = 0;
pub const MAX_ITEM_LEVEL: u8 = 100;
pub const ITEM_LEVEL_RANGE: RangeInclusive<u8> = MIN_ITEM_LEVEL..=MAX_ITEM_LEVEL;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ItemLevel {
    level: u8,
}

impl Display for ItemLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.level)
    }
}

impl FromSql for ItemLevel {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        ItemLevel::new(u8::column_result(value)?).map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

impl ToSql for ItemLevel {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(ToSqlOutput::from(self.level))
    }
}

impl ItemLevel {
    pub fn new(level: u8) -> Result<Self, RangeError> {
        if !ITEM_LEVEL_RANGE.contains(&level) {
            return Err(RangeError::ItemLevel(MIN_ITEM_LEVEL, MAX_ITEM_LEVEL, level));
        }
        Ok(Self { level })
    }

    pub fn level(&self) -> u8 {
        self.level
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::error::RangeError;

    #[test]
    fn test_item_level_new_works_with_valid_level() {
        let item_level = ItemLevel::new(31).unwrap();
        assert_eq!(31, item_level.level());
    }

    #[test]
    fn test_item_level_new_generates_error_with_invalid_level() {
        let result = ItemLevel::new(102);
        assert!(matches!(result, Err(RangeError::ItemLevel(MIN_ITEM_LEVEL, MAX_ITEM_LEVEL, 102))));
    }
}
