// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/19/2026.

use super::macros::*;
use super::table::GenericTable;
use super::table::Table;
use crate::db::rows::base_type_items::BaseTypeItemsRow;
use crate::db::tables::names::BASE_TYPE_ITEMS;
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

pub struct BaseTypeItemsTable {
    pub name: String,
}

impl_generic_table!(BaseTypeItems);

impl Table for BaseTypeItemsTable {
    fn new() -> Self {
        Self {
            name: BASE_TYPE_ITEMS.to_string(),
        }
    }

    fn create(&self, tx: &mut Transaction) -> Result<(), Error> {
        tx.execute(
            "CREATE TABLE IF NOT EXISTS base_type_items
                (
                    -- Base_type_item is the combination of base type and item name for a drop.  The value of this field is formed
                    -- as follows: <base_type>::<item>.  For example, Heavy Belt::Headhunter.  Note that this table will contain
                    -- an entry used to refer to the base_type itself without regard to item.  Such entries will use null for the
                    -- item name.
                    -- Since the base_type_item field is derived based on base_type and item, it is not contained in the BaseTypeItemsRow
                    -- struct.
                    base_type_item TEXT    NOT NULL PRIMARY KEY,
                    base_type      TEXT    NOT NULL,
                    item           TEXT,
                    is_unique      INTEGER NOT NULL               CHECK (is_unique IN (0, 1)),
                    FOREIGN KEY (base_type) REFERENCES base_types (base_type)
                ) STRICT",
            (),
        )?;
        Ok(())
    }

    fn export(&self, writer: &mut dyn Write, tx: &mut Transaction) -> Result<(), Error> {
        let mut stmt = tx.prepare("SELECT * FROM base_type_items")?;
        let rows: Vec<BaseTypeItemsRow> = stmt
            .query_map([], |row| {
                Ok(BaseTypeItemsRow {
                    base_type: row.get(1)?,
                    item: row.get(2)?,
                    is_unique: row.get(3)?,
                })
            })?
            .collect::<Result<Vec<BaseTypeItemsRow>, RusqliteError>>()?;
        let json = JsonFormat::pretty().indent_width(Some(consts::JSON_TAB)).ascii(true).format_to_string(&rows)?;
        writer.write_all(json.as_bytes())?;
        Ok(())
    }

    fn import(&self, reader: &mut dyn Read, tx: &mut Transaction) -> Result<(), Error> {
        let rows: Vec<BaseTypeItemsRow> = serde_json::from_reader(reader)?;
        let mut stmt = tx.prepare_cached("INSERT OR IGNORE INTO base_type_items (base_type_item, base_type, item, is_unique) VALUES (?1, ?2, ?3, ?4)")?;
        for row in &rows {
            stmt.execute(params![
                BaseTypeItemsRow::gen_key_from_parts(&row.base_type, &row.item),
                row.base_type,
                row.item,
                row.is_unique,
            ])?;
        }
        Ok(())
    }
}
