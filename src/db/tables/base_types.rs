// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/19/2026.

use super::macros::*;
use super::table::GenericTable;
use super::table::Table;
use crate::db::rows::base_types::BaseTypesRow;
use crate::db::tables::names::BASE_TYPES;
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
use serde_json_fmt::JsonFormat;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Write};
use url::Url;

pub struct BaseTypesTable {
    pub name: String,
}

impl_generic_table!(BaseTypes);

impl Table for BaseTypesTable {
    fn new() -> Self {
        Self { name: BASE_TYPES.to_string() }
    }

    fn create(&self, tx: &mut Transaction) -> Result<(), Error> {
        tx.execute(
            "CREATE TABLE IF NOT EXISTS base_types
                (
                    base_type    TEXT    NOT NULL PRIMARY KEY,
                    class        TEXT    NOT NULL,
                    stack_size   INTEGER NOT NULL               CHECK (stack_size > 0),
                    liquidity    TEXT    NOT NULL,
                    url          TEXT
                ) STRICT",
            (),
        )?;
        Ok(())
    }

    fn export(&self, writer: &mut dyn Write, tx: &mut Transaction) -> Result<(), Error> {
        let mut stmt = tx.prepare("SELECT * FROM base_types")?;
        let rows: Vec<BaseTypesRow> = stmt
            .query_map([], |row| {
                Ok(BaseTypesRow {
                    base_type: row.get(0)?,
                    class: row.get(1)?,
                    stack_size: row.get(2)?,
                    liquidity: row.get(3)?,
                    url: row
                        .get::<usize, Option<String>>(4)?
                        .map(|s| Url::parse(&s).map_err(|e| RusqliteError::FromSqlConversionFailure(4, Type::Text, Box::new(e))))
                        .transpose()?,
                })
            })?
            .collect::<Result<Vec<BaseTypesRow>, RusqliteError>>()?;
        let json = JsonFormat::pretty().indent_width(Some(consts::JSON_TAB)).ascii(true).format_to_string(&rows)?;
        writer.write_all(json.as_bytes())?;
        Ok(())
    }

    fn import(&self, reader: &mut dyn Read, tx: &mut Transaction) -> Result<(), Error> {
        let rows: Vec<BaseTypesRow> = serde_json::from_reader(reader)?;
        let mut stmt = tx.prepare_cached("INSERT OR IGNORE INTO base_types (base_type, class, stack_size, liquidity, url) VALUES (?1, ?2, ?3, ?4, ?5)")?;
        for row in &rows {
            stmt.execute(params![
                row.base_type,
                row.class,
                row.stack_size,
                row.liquidity,
                row.url.as_ref().map(|u| u.as_str()),
            ])?;
        }
        Ok(())
    }
}
