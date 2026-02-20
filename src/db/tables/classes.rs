// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/19/2026.

use super::macros::*;
use super::table::GenericTable;
use super::table::Table;
use crate::db::tables::names::CLASSES;
use crate::fs::dir::Dir;
use crate::fs::paths::Paths;
use crate::types::non_unique_rarity::NonUniqueRarity;
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

// Class is the broadest category of item classification.  Example classes include rings and body armours.
// We obtain the complete list of classes using https://www.pathofexile.com/api/trade/data/items and then
// change class names to match class names used in item filters using information from DAT files.
#[derive(Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClassesRow {
    // The name of the class.  Note that some base types - perhaps by accident - are not associated with a class;
    // to accommodate this possibility, a class called "None" will be present in the table.
    pub class: String,

    // The highest item rarity (Normal < Magic < Rare) associated with the class.
    pub highest_rarity: Option<NonUniqueRarity>,
}

pub struct ClassesTable {
    pub name: String,
}

impl_generic_table!(Classes);

impl Table for ClassesTable {
    fn new() -> Self {
        Self { name: CLASSES.to_string() }
    }

    fn create(&self, tx: &mut Transaction) -> Result<(), Error> {
        tx.execute(
            "CREATE TABLE IF NOT EXISTS classes
                (
                    class          TEXT NOT NULL PRIMARY KEY,
                    highest_rarity TEXT
                ) STRICT",
            (),
        )?;
        Ok(())
    }

    fn export(&self, writer: &mut dyn Write, tx: &mut Transaction) -> Result<(), Error> {
        let mut stmt = tx.prepare("SELECT * FROM classes")?;
        let rows: Vec<ClassesRow> = stmt
            .query_map([], |row| {
                Ok(ClassesRow {
                    class: row.get(0)?,
                    highest_rarity: row.get(1)?,
                })
            })?
            .collect::<Result<Vec<ClassesRow>, RusqliteError>>()?;
        let json = JsonFormat::pretty().indent_width(Some(consts::JSON_TAB)).ascii(true).format_to_string(&rows)?;
        writer.write_all(json.as_bytes())?;
        Ok(())
    }

    fn import(&self, reader: &mut dyn Read, tx: &mut Transaction) -> Result<(), Error> {
        let rows: Vec<ClassesRow> = serde_json::from_reader(reader)?;
        let mut stmt = tx.prepare_cached("INSERT OR IGNORE INTO classes (class, highest_rarity) VALUES (?1, ?2)")?;
        for row in &rows {
            stmt.execute(params![row.class, row.highest_rarity])?;
        }
        Ok(())
    }
}
