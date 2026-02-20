// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/2/2026.

use super::macros::*;
use super::table::GenericTable;
use super::table::Table;
use crate::db::tables::names::ARMOR_TYPES;
use crate::fs::dir::Dir;
use crate::fs::paths::Paths;
use crate::types::armor_type::ArmorType;
use crate::util::consts;
use crate::util::env::Env;
use anyhow::{Error, Result};
use itertools::Itertools;
use paste::paste;
use rand::Rng;
use rusqlite::{params, Error as RusqliteError, Transaction};
use serde::{Deserialize, Serialize};
use serde_json_fmt::JsonFormat;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Write};

// The armor types table is used to select the sound to play when an armor type drops.
#[derive(Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ArmorTypesRow {
    // The base type for the item.
    pub base_type: String,

    // The armor type for the item.
    pub armor_type: ArmorType,
}

pub struct ArmorTypesTable {
    pub name: String,
}

impl_generic_table!(ArmorTypes);

impl Table for ArmorTypesTable {
    fn new() -> Self {
        Self { name: ARMOR_TYPES.to_string() }
    }

    fn create(&self, tx: &mut Transaction) -> Result<(), Error> {
        tx.execute(
            "CREATE TABLE IF NOT EXISTS armor_types
                (
                    base_type   TEXT NOT NULL PRIMARY KEY,
                    armor_type  TEXT,
                    FOREIGN KEY (base_type) REFERENCES base_types (base_type)
                ) STRICT",
            (),
        )?;
        Ok(())
    }

    fn export(&self, writer: &mut dyn Write, tx: &mut Transaction) -> Result<(), Error> {
        let mut stmt = tx.prepare("SELECT * FROM armor_types")?;
        let rows: Vec<ArmorTypesRow> = stmt
            .query_map([], |row| {
                Ok(ArmorTypesRow {
                    base_type: row.get(0)?,
                    armor_type: row.get(1)?,
                })
            })?
            .collect::<Result<Vec<ArmorTypesRow>, RusqliteError>>()?;
        let json = JsonFormat::pretty().indent_width(Some(consts::JSON_TAB)).ascii(true).format_to_string(&rows)?;
        writer.write_all(json.as_bytes())?;
        Ok(())
    }

    fn import(&self, reader: &mut dyn Read, tx: &mut Transaction) -> Result<(), Error> {
        let rows: Vec<ArmorTypesRow> = serde_json::from_reader(reader)?;
        let mut stmt = tx.prepare_cached("INSERT OR IGNORE INTO armor_types (base_type, armor_type) VALUES (?1, ?2)")?;
        for row in &rows {
            stmt.execute(params![row.base_type, row.armor_type])?;
        }
        Ok(())
    }
}
