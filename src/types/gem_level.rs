// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/7/2026.

use crate::util::errors::FgdbRangeError;
use anyhow::Result;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::{Result as RusqliteResult, ToSql};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::ops::RangeInclusive;

pub const MIN_GEM_LEVEL: u8 = 1;
pub const MAX_GEM_LEVEL: u8 = 21;
pub const GEM_LEVEL_RANGE: RangeInclusive<u8> = MIN_GEM_LEVEL..=MAX_GEM_LEVEL;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GemLevel {
    level: u8,
}

impl Display for GemLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.level)
    }
}

impl FromSql for GemLevel {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        GemLevel::new(u8::column_result(value)?).map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

impl ToSql for GemLevel {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(ToSqlOutput::from(self.level))
    }
}

impl GemLevel {
    pub fn new(level: u8) -> Result<Self, FgdbRangeError> {
        if !GEM_LEVEL_RANGE.contains(&level) {
            return Err(FgdbRangeError::GemLevel(MIN_GEM_LEVEL, MAX_GEM_LEVEL, level));
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
    use crate::util::errors::FgdbRangeError;

    #[test]
    fn test_gem_level_new_works_with_valid_level() {
        let gem_level = GemLevel::new(1).unwrap();
        assert_eq!(1, gem_level.level());
    }

    #[test]
    fn test_gem_level_new_generates_error_with_invalid_level() {
        let result = GemLevel::new(0);
        assert!(matches!(result, Err(FgdbRangeError::GemLevel(MIN_GEM_LEVEL, MAX_GEM_LEVEL, 0))));
    }
}
