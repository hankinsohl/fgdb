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
pub enum IconShape {
    Circle,
    Cross,
    Diamond,
    Hexagon,
    Kite,
    Moon,
    Pentagon,
    Raindrop,
    Square,
    Star,
    Triangle,
    UpsideDownHouse,
}

impl Display for IconShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IconShape::Circle => "Circle",
                IconShape::Cross => "Cross",
                IconShape::Diamond => "Diamond",
                IconShape::Hexagon => "Hexagon",
                IconShape::Kite => "Kite",
                IconShape::Moon => "Moon",
                IconShape::Pentagon => "Pentagon",
                IconShape::Raindrop => "Raindrop",
                IconShape::Square => "Square",
                IconShape::Star => "Star",
                IconShape::Triangle => "Triangle",
                IconShape::UpsideDownHouse => "UpsideDownHouse",
            }
        )
    }
}

impl FromSql for IconShape {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        IconShape::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

impl FromStr for IconShape {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<IconShape, ParseError> {
        match s {
            "Circle" => Ok(IconShape::Circle),
            "Cross" => Ok(IconShape::Cross),
            "Diamond" => Ok(IconShape::Diamond),
            "Hexagon" => Ok(IconShape::Hexagon),
            "Kite" => Ok(IconShape::Kite),
            "Moon" => Ok(IconShape::Moon),
            "Pentagon" => Ok(IconShape::Pentagon),
            "Raindrop" => Ok(IconShape::Raindrop),
            "Square" => Ok(IconShape::Square),
            "Star" => Ok(IconShape::Star),
            "Triangle" => Ok(IconShape::Triangle),
            "UpsideDownHouse" => Ok(IconShape::UpsideDownHouse),
            _ => Err(ParseError::InvalidIconShape(s.to_string())),
        }
    }
}

impl ToSql for IconShape {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(self.to_string().into())
    }
}
