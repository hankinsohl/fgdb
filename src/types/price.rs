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

pub const MIN_PRICE: f32 = 0.0;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Price {
    price: f32,
}

impl Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.price)
    }
}

impl FromSql for Price {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        Price::new(f32::column_result(value)?).map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

impl ToSql for Price {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(ToSqlOutput::from(self.price))
    }
}

impl Price {
    pub fn new(price: f32) -> Result<Self, RangeError> {
        if price < MIN_PRICE {
            return Err(RangeError::Price(price));
        }
        Ok(Self { price })
    }

    pub fn price(&self) -> f32 {
        self.price
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::error::RangeError;

    #[test]
    fn test_price_new_works_with_valid_price() {
        let price = Price::new(3.1).unwrap();
        assert_eq!(3.1, price.price());
    }

    #[test]
    fn test_price_new_generates_error_with_invalid_price() {
        let result = Price::new(-10.2);
        assert!(matches!(result, Err(RangeError::Price(-10.2))));
    }
}
