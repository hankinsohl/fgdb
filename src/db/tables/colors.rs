// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/19/2026.

use super::macros::*;
use super::table::GenericTable;
use super::table::Table;
use crate::fs::dir::Dir;
use crate::fs::paths::Paths;
use crate::util::consts;
use crate::util::env::Env;
use anyhow::{Error, Result};
use itertools::Itertools;
use paste::paste;
use rand::Rng;
use rgb::RGBA8 as Rgba8;
use rusqlite::{params, Error as RusqliteError, Transaction};
use serde::{Deserialize, Serialize};
use serde_json_fmt::JsonFormat;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Write};

// The colors table contains named 8-bit RGBA color values used for drops.
#[derive(Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ColorsRow {
    // The name of the color.
    pub color: String,

    // The url used to obtain RGBA value for the color.
    pub url: String,

    // The 8-bit RGBA value for the color.
    pub pixel: Rgba8,
}

pub struct ColorsTable {
    pub name: String,
}

impl_generic_table!(Colors);

impl Table for ColorsTable {
    fn new() -> Self {
        Self { name: "colors".to_string() }
    }

    fn create(&self, tx: &mut Transaction) -> Result<(), Error> {
        tx.execute(
            "CREATE TABLE IF NOT EXISTS colors
                (
                    color TEXT NOT NULL PRIMARY KEY,
                    url   TEXT NOT NULL,
                    red   INTEGER NOT NULL           CHECK (red >= 0 AND red <= 255),
                    green INTEGER NOT NULL           CHECK (green >= 0 AND green <= 255),
                    blue  INTEGER NOT NULL           CHECK (blue >= 0 AND blue <= 255),
                    alpha INTEGER NOT NULL           CHECK (alpha >= 0 AND alpha <= 255)
                ) STRICT",
            (),
        )?;
        Ok(())
    }

    fn export(&self, writer: &mut dyn Write, tx: &mut Transaction) -> Result<(), Error> {
        let mut stmt = tx.prepare("SELECT * FROM colors")?;
        let rows: Vec<ColorsRow> = stmt
            .query_map([], |row| {
                Ok(ColorsRow {
                    color: row.get(0)?,
                    url: row.get(1)?,
                    pixel: {
                        Rgba8 {
                            r: row.get(2)?,
                            g: row.get(3)?,
                            b: row.get(4)?,
                            a: row.get(5)?,
                        }
                    },
                })
            })?
            .collect::<Result<Vec<ColorsRow>, RusqliteError>>()?;
        let json = JsonFormat::pretty().indent_width(Some(consts::JSON_TAB)).ascii(true).format_to_string(&rows)?;
        writer.write_all(json.as_bytes())?;
        Ok(())
    }

    fn import(&self, reader: &mut dyn Read, tx: &mut Transaction) -> Result<(), Error> {
        let rows: Vec<ColorsRow> = serde_json::from_reader(reader)?;
        let mut stmt = tx.prepare_cached("INSERT OR IGNORE INTO colors (color, url, red, green, blue, alpha) VALUES (?1, ?2, ?3, ?4, ?5, ?6)")?;
        for row in &rows {
            stmt.execute(params![row.color, row.url, row.pixel.r, row.pixel.g, row.pixel.b, row.pixel.a])?;
        }
        Ok(())
    }
}
