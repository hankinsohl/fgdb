// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/19/2026.

use super::macros::*;
use super::table::GenericTable;
use super::table::Table;
use crate::db::rows::sounds_row::SoundsRow;
use crate::db::tables::names::SOUNDS;
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

pub struct SoundsTable {
    pub name: String,
}

impl_generic_table!(Sounds);

impl Table for SoundsTable {
    fn new() -> Self {
        Self { name: SOUNDS.to_string() }
    }

    fn create(&self, tx: &mut Transaction) -> Result<(), Error> {
        tx.execute(
            "CREATE TABLE IF NOT EXISTS sounds
                (
                    sound              TEXT NOT NULL PRIMARY KEY,
                    file_name          TEXT NOT NULL,
                    original_file_name TEXT NOT NULL,
                    source             TEXT NOT NULL,
                    composer           TEXT NOT NULL,
                    is_modified        INTEGER NOT NULL            CHECK (is_modified IN (0, 1)),
                    url                TEXT NOT NULL,
                    license            TEXT NOT NULL,
                    FOREIGN KEY (license) REFERENCES licenses (license)
                ) STRICT",
            (),
        )?;
        Ok(())
    }

    fn export(&self, writer: &mut dyn Write, tx: &mut Transaction) -> Result<(), Error> {
        let mut stmt = tx.prepare("SELECT * FROM sounds")?;
        let rows: Vec<SoundsRow> = stmt
            .query_map([], |row| {
                Ok(SoundsRow {
                    sound: row.get(0)?,
                    file_name: row.get(1)?,
                    original_file_name: row.get(2)?,
                    source: row.get(3)?,
                    composer: row.get(4)?,
                    is_modified: row.get(5)?,
                    url: row.get(6)?,
                    license: row.get(7)?,
                })
            })?
            .collect::<Result<Vec<SoundsRow>, RusqliteError>>()?;
        let json = JsonFormat::pretty().indent_width(Some(consts::JSON_TAB)).ascii(true).format_to_string(&rows)?;
        writer.write_all(json.as_bytes())?;
        Ok(())
    }

    fn import(&self, reader: &mut dyn Read, tx: &mut Transaction) -> Result<(), Error> {
        let rows: Vec<SoundsRow> = serde_json::from_reader(reader)?;
        let mut stmt = tx.prepare_cached(
            r#"INSERT OR IGNORE INTO sounds 
                (
                    sound, 
                    file_name, 
                    original_file_name, 
                    source, 
                    composer, 
                    is_modified,
                    url,
                    license
                ) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)"#,
        )?;
        for row in &rows {
            stmt.execute(params![
                row.sound,
                row.file_name,
                row.original_file_name,
                row.source,
                row.composer,
                row.is_modified,
                row.url,
                row.license
            ])?;
        }
        Ok(())
    }
}
