// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/21/2026.

use crate::db::tables::action_sets_table::ActionSetsTable;
use crate::db::tables::armor_types_table::ArmorTypesTable;
use crate::db::tables::async_prices_table::AsyncPricesTable;
use crate::db::tables::base_type_items_table::BaseTypeItemsTable;
use crate::db::tables::base_types_table::BaseTypesTable;
use crate::db::tables::classes_table::ClassesTable;
use crate::db::tables::colors_table::ColorsTable;
use crate::db::tables::exchange_prices_table::ExchangePricesTable;
use crate::db::tables::licenses_table::LicensesTable;
use crate::db::tables::sounds_table::SoundsTable;
use crate::db::tables::table::Table;
use anyhow::{Error, Result};
use rusqlite::Transaction;
use static_init::dynamic;

// N.B.: The order of tables in this array is important.  It must be possible to create each table
// in order of appearance and to drop/delete each table in reverse order of appearance.
#[dynamic]
pub static TABLES: [Box<dyn Table>; 10] = [
    Box::new(ClassesTable::new()),
    Box::new(BaseTypesTable::new()),
    Box::new(BaseTypeItemsTable::new()),
    Box::new(ArmorTypesTable::new()),
    Box::new(AsyncPricesTable::new()),
    Box::new(ExchangePricesTable::new()),
    Box::new(ColorsTable::new()),
    Box::new(LicensesTable::new()),
    Box::new(SoundsTable::new()),
    Box::new(ActionSetsTable::new()),
];

pub struct Database;

impl Database {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {})
    }

    pub fn create(&self, tx: &mut Transaction) -> Result<(), Error> {
        for table in TABLES.iter() {
            table.create(tx)?;
        }
        Ok(())
    }

    pub fn delete(&self, tx: &mut Transaction) -> Result<(), Error> {
        for table in TABLES.iter().rev() {
            table.delete(tx)?;
        }
        Ok(())
    }

    pub fn drop_tables(&self, tx: &mut Transaction) -> Result<(), Error> {
        for table in TABLES.iter().rev() {
            table.drop_table(tx)?;
        }
        Ok(())
    }
}
