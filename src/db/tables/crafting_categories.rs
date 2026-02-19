// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/17/2026.

use super::macros::*;
use super::table::GenericTable;
use super::table::Table;
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

// Crafting categories for item classes.
#[derive(Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CraftingCategoriesRow {
    // Crafting category.
    pub crafting_category: String,

    // The highest non-unique rarity obtainable for items in this category.
    pub highest_rarity: NonUniqueRarity,
}

pub struct CraftingCategoriesTable {
    pub name: String,
}

impl_generic_table!(CraftingCategories);

impl Table for CraftingCategoriesTable {
    fn new() -> Self {
        Self {
            name: "crafting_categories".to_string(),
        }
    }

    fn create(&self, tx: &mut Transaction) -> Result<(), Error> {
        tx.execute(
            "CREATE TABLE IF NOT EXISTS crafting_categories
                (
                    crafting_category TEXT NOT NULL PRIMARY KEY,
                    highest_rarity    TEXT NOT NULL
                ) STRICT",
            (),
        )?;
        Ok(())
    }

    fn export(&self, writer: &mut dyn Write, tx: &mut Transaction) -> Result<(), Error> {
        let mut stmt = tx.prepare("SELECT * FROM crafting_categories")?;
        let rows: Vec<CraftingCategoriesRow> = stmt
            .query_map([], |row| {
                Ok(CraftingCategoriesRow {
                    crafting_category: row.get(0)?,
                    highest_rarity: row.get(1)?,
                })
            })?
            .collect::<Result<Vec<CraftingCategoriesRow>, RusqliteError>>()?;
        let json = JsonFormat::pretty().indent_width(Some(consts::JSON_TAB)).ascii(true).format_to_string(&rows)?;
        writer.write_all(json.as_bytes())?;
        Ok(())
    }

    fn import(&self, reader: &mut dyn Read, tx: &mut Transaction) -> Result<(), Error> {
        let rows: Vec<CraftingCategoriesRow> = serde_json::from_reader(reader)?;
        let mut stmt = tx.prepare_cached("INSERT OR IGNORE INTO crafting_categories (crafting_category, highest_rarity) VALUES (?1, ?2)")?;
        for row in &rows {
            stmt.execute(params![row.crafting_category, row.highest_rarity.to_string()])?;
        }
        Ok(())
    }
}
