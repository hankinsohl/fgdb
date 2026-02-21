// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/19/2026.

use super::macros::*;
use super::table::GenericTable;
use super::table::Table;
use crate::db::rows::exchange_prices_row::ExchangePricesRow;
use crate::db::tables::names::EXCHANGE_PRICES;
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

pub struct ExchangePricesTable {
    pub name: String,
}

impl_generic_table!(ExchangePrices);

impl Table for ExchangePricesTable {
    fn new() -> Self {
        Self {
            name: EXCHANGE_PRICES.to_string(),
        }
    }

    fn create(&self, tx: &mut Transaction) -> Result<(), Error> {
        tx.execute(
            "CREATE TABLE IF NOT EXISTS exchange_prices
                (
                    base_type           TEXT    NOT NULL PRIMARY KEY,
                    price               REAL    NOT NULL               CHECK (price >= 0),
                    FOREIGN KEY (base_type) REFERENCES base_types (base_type)
                ) STRICT",
            (),
        )?;
        Ok(())
    }

    fn export(&self, writer: &mut dyn Write, tx: &mut Transaction) -> Result<(), Error> {
        let mut stmt = tx.prepare("SELECT * FROM exchange_prices")?;
        let rows: Vec<ExchangePricesRow> = stmt
            .query_map([], |row| {
                Ok(ExchangePricesRow {
                    base_type: row.get(0)?,
                    price: row.get(1)?,
                })
            })?
            .collect::<Result<Vec<ExchangePricesRow>, RusqliteError>>()?;
        let json = JsonFormat::pretty().indent_width(Some(consts::JSON_TAB)).ascii(true).format_to_string(&rows)?;
        writer.write_all(json.as_bytes())?;
        Ok(())
    }

    fn import(&self, reader: &mut dyn Read, tx: &mut Transaction) -> Result<(), Error> {
        let rows: Vec<ExchangePricesRow> = serde_json::from_reader(reader)?;
        let mut stmt = tx.prepare_cached("INSERT OR IGNORE INTO exchange_prices (base_type, price) VALUES (?1, ?2)")?;
        for row in &rows {
            stmt.execute(params![row.base_type, row.price,])?;
        }
        Ok(())
    }
}
