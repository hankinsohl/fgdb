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

pub const MIN_STACK_SIZE: u32 = 1;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct StackSize {
    size: u32,
}

impl Display for StackSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.size)
    }
}

impl FromSql for StackSize {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        StackSize::new(u32::column_result(value)?).map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

impl ToSql for StackSize {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(ToSqlOutput::from(self.size))
    }
}

impl StackSize {
    pub fn new(size: u32) -> Result<Self, RangeError> {
        if size < MIN_STACK_SIZE {
            return Err(RangeError::StackSize());
        }
        Ok(Self { size })
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::error::RangeError;

    #[test]
    fn test_stack_size_new_works_with_valid_size() {
        let stack_size = StackSize::new(31).unwrap();
        assert_eq!(31, stack_size.size());
    }

    #[test]
    fn test_stack_size_new_generates_error_with_invalid_size() {
        let result = StackSize::new(0);
        assert!(matches!(result, Err(RangeError::StackSize())));
    }
}
