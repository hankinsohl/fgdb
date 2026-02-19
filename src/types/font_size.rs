// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/6/2026.

use crate::util::error::RangeError;
use anyhow::Result;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::{Result as RusqliteResult, ToSql};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::ops::RangeInclusive;

pub const MIN_FONT_SIZE: u8 = 18;
pub const MAX_FONT_SIZE: u8 = 45;
pub const FONT_SIZE_RANGE: RangeInclusive<u8> = MIN_FONT_SIZE..=MAX_FONT_SIZE;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct FontSize {
    size: u8,
}

impl Display for FontSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.size)
    }
}

impl FromSql for FontSize {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        FontSize::new(u8::column_result(value)?).map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

impl ToSql for FontSize {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(ToSqlOutput::from(self.size))
    }
}

impl FontSize {
    pub fn new(size: u8) -> Result<Self, RangeError> {
        if !FONT_SIZE_RANGE.contains(&size) {
            return Err(RangeError::FontSize(MIN_FONT_SIZE, MAX_FONT_SIZE, size));
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
    use crate::util::error::RangeError;

    #[test]
    fn test_font_size_new_works_with_valid_font_size() {
        let font_size = FontSize::new(29).unwrap();
        assert_eq!(29, font_size.size());
    }

    #[test]
    fn test_font_size_new_generates_error_with_invalid_font_size() {
        let result = FontSize::new(66);
        assert!(matches!(result, Err(RangeError::FontSize(MIN_FONT_SIZE, MAX_FONT_SIZE, 66))));
    }
}
