// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/19/2026.

use super::macros::*;
use super::table::GenericTable;
use super::table::Table;
use crate::db::rows::classes::ClassesRow;
use crate::db::tables::names::CLASSES;
use crate::fs::dir::Dir;
use crate::fs::paths::Paths;
use crate::util::consts;
use crate::util::env::Env;
use anyhow::{Error, Result};
use itertools::Itertools;
use paste::paste;
use rand::Rng;
use rusqlite::{params, Error as RusqliteError, Transaction};
use serde_json_fmt::JsonFormat;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Write};

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
