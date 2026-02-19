// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/2/2026.

use crate::util::error::ParseError;
use anyhow::Result;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::{Result as RusqliteResult, ToSql};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ArmorType {
    // Strength/Intelligence (Armor/Energy Shield)
    Chain,

    // Intelligence (Energy Shield)
    Cloth,

    // Dexterity (Evasion)
    Leather,

    // Strength/Dexterity/Intelligence (Armor/Evasion/Energy Shield)
    Mail,

    // Dexterity/Intelligence (Evasion/Energy Shield)
    Padded,

    // Strength (Armor)
    Plate,

    // Strength/Dexterity (Armor/Evasion)
    Scale,
}

impl Display for ArmorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ArmorType::Chain => "Chain",
                ArmorType::Cloth => "Cloth",
                ArmorType::Leather => "Leather",
                ArmorType::Mail => "Mail",
                ArmorType::Padded => "Padded",
                ArmorType::Plate => "Plate",
                ArmorType::Scale => "Scale",
            }
        )
    }
}

impl FromSql for ArmorType {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        ArmorType::from_str(value.as_str()?).map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

impl FromStr for ArmorType {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<ArmorType, ParseError> {
        match s {
            "Chain" => Ok(ArmorType::Chain),
            "Cloth" => Ok(ArmorType::Cloth),
            "Leather" => Ok(ArmorType::Leather),
            "Mail" => Ok(ArmorType::Mail),
            "Padded" => Ok(ArmorType::Padded),
            "Plate" => Ok(ArmorType::Plate),
            "Scale" => Ok(ArmorType::Scale),
            _ => Err(ParseError::InvalidArmorType(s.to_string())),
        }
    }
}

impl ToSql for ArmorType {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(self.to_string().into())
    }
}

impl ArmorType {
    pub fn from_stats(armour: u32, evasion: u32, energy_shield: u32) -> ArmorType {
        match (armour, evasion, energy_shield) {
            (a, e, es) if a == 0 && e == 0 && es == 0 => ArmorType::Cloth,

            (a, e, es) if a > 0 && e == 0 && es == 0 => ArmorType::Plate,
            (a, e, es) if a == 0 && e > 0 && es == 0 => ArmorType::Leather,
            (a, e, es) if a == 0 && e == 0 && es > 0 => ArmorType::Cloth,

            (a, e, es) if a > 0 && e > 0 && es == 0 => ArmorType::Scale,
            (a, e, es) if a > 0 && e == 0 && es > 0 => ArmorType::Chain,

            (a, e, es) if a == 0 && e > 0 && es > 0 => ArmorType::Padded,

            (a, e, es) if a > 0 && e > 0 && es > 0 => ArmorType::Mail,

            _ => panic!("All 8 cases should have been covered above."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_with_valid_input_works() {
        assert_eq!(ArmorType::from_str("Chain").unwrap(), ArmorType::Chain);
        assert_eq!(ArmorType::from_str("Cloth").unwrap(), ArmorType::Cloth);
        assert_eq!(ArmorType::from_str("Leather").unwrap(), ArmorType::Leather);
        assert_eq!(ArmorType::from_str("Mail").unwrap(), ArmorType::Mail);
        assert_eq!(ArmorType::from_str("Padded").unwrap(), ArmorType::Padded);
        assert_eq!(ArmorType::from_str("Plate").unwrap(), ArmorType::Plate);
        assert_eq!(ArmorType::from_str("Scale").unwrap(), ArmorType::Scale);
    }

    #[test]
    fn test_from_str_with_invalid_input_yields_parse_error_invalid_liquidity() {
        assert!(matches!(ArmorType::from_str("mail"), Err(ParseError::InvalidArmorType(_))));
        assert!(matches!(ArmorType::from_str("Ringmail"), Err(ParseError::InvalidArmorType(_))));
    }
}
