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
use rusqlite::types::Type;
use rusqlite::{params, Error as RusqliteError, Transaction};
use serde::{Deserialize, Serialize};
use serde_json_fmt::JsonFormat;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Write};
use url::Url;

// Assets used by the filter generator such as sound files may be licensed.  The license table contains a list of
// all asset licenses; for each license a URL is provided to obtain more information about the license.
#[derive(Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct LicensesRow {
    // The name used to identify the license.
    pub license: String,

    // A URL used to obtain information about the license.
    pub url: Url,
}

pub struct LicensesTable {
    pub name: String,
}

impl_generic_table!(Licenses);

impl Table for LicensesTable {
    fn new() -> Self {
        Self { name: "licenses".to_string() }
    }

    fn create(&self, tx: &mut Transaction) -> Result<(), Error> {
        tx.execute(
            "CREATE TABLE IF NOT EXISTS licenses
                (
                    license TEXT NOT NULL PRIMARY KEY,
                    url     TEXT NOT NULL
                ) STRICT",
            (),
        )?;
        Ok(())
    }

    fn export(&self, writer: &mut dyn Write, tx: &mut Transaction) -> Result<(), Error> {
        let mut stmt = tx.prepare("SELECT * FROM licenses")?;
        let rows: Vec<LicensesRow> = stmt
            .query_map([], |row| {
                Ok(LicensesRow {
                    license: row.get(0)?,
                    url: Url::parse(&row.get::<usize, String>(1)?).map_err(|e| RusqliteError::FromSqlConversionFailure(1, Type::Text, Box::new(e)))?,
                })
            })?
            .collect::<Result<Vec<LicensesRow>, RusqliteError>>()?;
        let json = JsonFormat::pretty().indent_width(Some(consts::JSON_TAB)).ascii(true).format_to_string(&rows)?;
        writer.write_all(json.as_bytes())?;
        Ok(())
    }

    fn import(&self, reader: &mut dyn Read, tx: &mut Transaction) -> Result<(), Error> {
        let rows: Vec<LicensesRow> = serde_json::from_reader(reader)?;
        let mut stmt = tx.prepare_cached("INSERT OR IGNORE INTO licenses (license, url) VALUES (?1, ?2)")?;
        for row in &rows {
            stmt.execute(params![row.license, row.url.to_string()])?;
        }
        Ok(())
    }
}
