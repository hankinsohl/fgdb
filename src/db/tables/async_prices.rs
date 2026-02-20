// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/19/2026.

use super::macros::*;
use super::table::GenericTable;
use super::table::Table;
use crate::db::tables::base_type_items::BaseTypeItemsRow;
use crate::db::tables::names::ASYNC_PRICES;
use crate::fs::dir::Dir;
use crate::fs::paths::Paths;
use crate::types::gem_level::GemLevel;
use crate::types::item_level::ItemLevel;
use crate::types::price::Price;
use crate::types::rarity::Rarity;
use crate::util::consts;
use crate::util::env::Env;
use anyhow::{Error, Result};
use itertools::Itertools;
use paste::paste;
use rand::Rng;
use rusqlite::{params, Error as RusqliteError, Transaction};
use serde::{Deserialize, Serialize};
use serde_json_fmt::JsonFormat;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Write};

// async_prices
//     The async_prices table contains a list of in-game prices for items available for instant buyout or in-person
//     trades.  It does not contain prices for items available on the auction house; these prices are recorded in the
//     exchange_prices table.
// async_prices::async_price_key
//     Trade_price_key is a composite formed as follows:
//         <base_type>::<item>::<minimum_item_level>::<gem_level>::<rarity>
//      If one of the key parts is null, the text "null" is used when forming the composite.
#[derive(Debug, Deserialize, Serialize)]
pub struct AsyncPricesRow {
    // The name of the base type.
    pub base_type: String,

    // The name of the item or None if not applicable.
    pub item: Option<String>,

    // The minimum item level associated with the price, or None if not applicable.
    pub minimum_item_level: Option<ItemLevel>,

    // The gem level associated with the price, or None if not applicable.
    pub gem_level: Option<GemLevel>,

    // The rarity associated with the price, or None if not applicable
    pub rarity: Option<Rarity>,

    // The price of the item in units of the game variant's basis currency (chaos orbs for POE 1; exalted orbs for POE 2).
    pub price: Price,
}

impl Eq for AsyncPricesRow {}

impl Ord for AsyncPricesRow {
    fn cmp(&self, other: &AsyncPricesRow) -> Ordering {
        self.gen_key().cmp(&other.gen_key())
    }
}

impl PartialEq for AsyncPricesRow {
    fn eq(&self, other: &AsyncPricesRow) -> bool {
        self.gen_key() == other.gen_key()
    }
}

impl PartialOrd for AsyncPricesRow {
    fn partial_cmp(&self, other: &AsyncPricesRow) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl AsyncPricesRow {
    pub fn gen_key_from_parts(
        base_type: &String,
        item: &Option<String>,
        minimum_item_level: &Option<ItemLevel>,
        gem_level: &Option<GemLevel>,
        rarity: &Option<Rarity>,
    ) -> String {
        format!(
            "{}::{}::{}::{}::{}",
            base_type,
            item.as_ref().unwrap_or(&"null".to_string()),
            minimum_item_level.map(|l| l.to_string()).unwrap_or("null".to_string()),
            gem_level.map(|l| l.to_string()).unwrap_or("null".to_string()),
            rarity.map(|r| r.to_string()).unwrap_or("null".to_string())
        )
    }

    pub fn gen_key(&self) -> String {
        AsyncPricesRow::gen_key_from_parts(&self.base_type, &self.item, &self.minimum_item_level, &self.gem_level, &self.rarity)
    }
}

pub struct AsyncPricesTable {
    pub name: String,
}

impl_generic_table!(AsyncPrices);

impl Table for AsyncPricesTable {
    fn new() -> Self {
        Self {
            name: ASYNC_PRICES.to_string(),
        }
    }

    fn create(&self, tx: &mut Transaction) -> Result<(), Error> {
        tx.execute(
            "CREATE TABLE IF NOT EXISTS async_prices
                (
                    -- Async_price_key is a composite formed as follows:
                    --     <base_type>::<item>::<minimum_item_level>::<gem_level>::<rarity>
                    -- If a key part is None, the text null is used when forming the composite.
                    async_price_key     TEXT    NOT NULL PRIMARY KEY,

                    -- Base_type_item is a composite formed as follows:
                    --     <base_type>::<item>.
                    -- using null for <item> if item is None.
                    base_type_item      TEXT    NOT NULL,
                    base_type           TEXT    NOT NULL,
                    item                TEXT,
                    minimum_item_level  INTEGER                        CHECK (minimum_item_level >= 0 AND minimum_item_level <= 100),
                    gem_level           INTEGER                        CHECK (gem_level >= 1 AND gem_level <= 21),
                    rarity              TEXT,
                    price               REAL    NOT NULL               CHECK (price >= 0),
                    FOREIGN KEY (base_type_item) REFERENCES base_type_items (base_type_item),
                    FOREIGN KEY (base_type) REFERENCES base_types (base_type)
                ) STRICT",
            (),
        )?;
        Ok(())
    }

    fn export(&self, writer: &mut dyn Write, tx: &mut Transaction) -> Result<(), Error> {
        let mut stmt = tx.prepare("SELECT * FROM async_prices")?;
        let rows: Vec<AsyncPricesRow> = stmt
            .query_map([], |row| {
                Ok(AsyncPricesRow {
                    base_type: row.get(2)?,
                    item: row.get(3)?,
                    minimum_item_level: row.get(4)?,
                    gem_level: row.get(5)?,
                    rarity: row.get(6)?,
                    price: row.get(7)?,
                })
            })?
            .collect::<Result<Vec<AsyncPricesRow>, RusqliteError>>()?;
        let json = JsonFormat::pretty().indent_width(Some(consts::JSON_TAB)).ascii(true).format_to_string(&rows)?;
        writer.write_all(json.as_bytes())?;
        Ok(())
    }

    fn import(&self, reader: &mut dyn Read, tx: &mut Transaction) -> Result<(), Error> {
        let rows: Vec<AsyncPricesRow> = serde_json::from_reader(reader)?;
        let mut stmt = tx.prepare_cached("INSERT OR IGNORE INTO async_prices (async_price_key, base_type_item, base_type, item, minimum_item_level, gem_level, rarity, price) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)")?;
        for row in &rows {
            stmt.execute(params![
                row.gen_key(),
                BaseTypeItemsRow::gen_key_from_parts(&row.base_type, &row.item),
                row.base_type,
                row.item,
                row.minimum_item_level,
                row.gem_level,
                row.rarity,
                row.price,
            ])?;
        }
        Ok(())
    }
}
