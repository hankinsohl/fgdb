// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/18/2026.

use crate::util::env::Env;
use anyhow::{Error, Result};
use rusqlite::Transaction;
use std::io::{Read, Write};

/// Trait for the generic methods of the Table interface.  Splitting the Table interface into generic and
/// non-generic parts enables the use of a macro to implement generic functionality.
pub trait GenericTable {
    /// Returns the number of rows in the table.
    fn count(&self, tx: &mut Transaction) -> Result<usize, Error>;

    /// Generates initial JSON for the tables and writes it to writer.
    ///
    /// The create_initial_data method does not rely upon data, if any, currently in the table.  Instead, data is
    /// generated from some other source such as an XML file or a vector of data hard-coded into the table's
    /// implementation of this method, or using a website query, etc. The env parameter is used in case
    /// the implementation relies on environment-specific details in its implementation.
    fn create_initial_data(&self, writer: &mut dyn Write, env: Env) -> Result<(), Error>;

    /// Deletes all rows in the table and returns the number of rows deleted.
    fn delete(&self, tx: &mut Transaction) -> Result<usize, Error>;

    /// Drops the table if it exists.
    fn drop_table(&self, tx: &mut Transaction) -> Result<(), Error>;

    /// Returns true if the table is empty; false otherwise.
    fn is_empty(&self, tx: &mut Transaction) -> Result<bool, Error> {
        Ok(self.count(tx)? == 0)
    }

    /// Returns the name of the table.
    fn name(&self) -> &str;

    /// Reads JSON from reader, randomly removes elements, sorts the remaining elements and then writes the resulting
    /// collection to writer.  The purpose of this method is to generate sorted, partial JSON for use in unit testing.
    fn partial(&self, reader: &mut dyn Read, writer: &mut dyn Write) -> Result<(), Error>;
}

/// Trait for the full Table interface.  Note that Table requires implementation of GenericTable.
pub trait Table: GenericTable + Send + Sync {
    /// Creates an instance of the tables.
    fn new() -> Self
    where
        Self: Sized;

    /// Creates the table.
    fn create(&self, tx: &mut Transaction) -> Result<(), Error>;

    /// Exports all data in the table to writer as JSON.
    fn export(&self, writer: &mut dyn Write, tx: &mut Transaction) -> Result<(), Error>;

    /// Imports JSON from reader.
    fn import(&self, reader: &mut dyn Read, tx: &mut Transaction) -> Result<(), Error>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concurrency::env_pool::EnvPoolGuard;
    use crate::config::fgdb_config::get_config;
    use crate::db::conn::Conn;
    use crate::db::tables::action_sets_table::ActionSetsTable;
    use crate::db::tables::armor_types_table::ArmorTypesTable;
    use crate::db::tables::async_prices_table::AsyncPricesTable;
    use crate::db::tables::base_type_items_table::BaseTypeItemsTable;
    use crate::db::tables::base_types_table::BaseTypesTable;
    use crate::db::tables::classes_table::ClassesTable;
    use crate::db::tables::colors_table::ColorsTable;
    use crate::db::tables::crafting_categories_table::CraftingCategoriesTable;
    use crate::db::tables::exchange_prices_table::ExchangePricesTable;
    use crate::db::tables::licenses_table::LicensesTable;
    use crate::db::tables::names::*;
    use crate::db::tables::sounds_table::SoundsTable;
    use crate::fs::dir::Dir;
    use crate::fs::paths::Paths;
    use crate::types::game_variant::GameVariant;
    use slitu::fs as tu;
    use static_init::dynamic;
    use std::fs::{File, OpenOptions};
    use std::io::{BufReader, BufWriter};

    #[dynamic]
    static TABLE_REGISTRY: [TableInfo; 11] = [
        TableInfo {
            table: Box::new(ActionSetsTable::new()),
            name: ACTION_SETS.to_string(),
            expected_row_count_poe1: 1,
            expected_row_count_poe2: 1,
        },
        TableInfo {
            table: Box::new(ArmorTypesTable::new()),
            name: ARMOR_TYPES.to_string(),
            expected_row_count_poe1: 7,
            expected_row_count_poe2: 7,
        },
        TableInfo {
            table: Box::new(AsyncPricesTable::new()),
            name: ASYNC_PRICES.to_string(),
            expected_row_count_poe1: 4,
            expected_row_count_poe2: 5,
        },
        TableInfo {
            table: Box::new(BaseTypeItemsTable::new()),
            name: BASE_TYPE_ITEMS.to_string(),
            expected_row_count_poe1: 11,
            expected_row_count_poe2: 12,
        },
        TableInfo {
            table: Box::new(BaseTypesTable::new()),
            name: BASE_TYPES.to_string(),
            expected_row_count_poe1: 17,
            expected_row_count_poe2: 16,
        },
        TableInfo {
            table: Box::new(ClassesTable::new()),
            name: CLASSES.to_string(),
            expected_row_count_poe1: 13,
            expected_row_count_poe2: 13,
        },
        TableInfo {
            table: Box::new(ColorsTable::new()),
            name: COLORS.to_string(),
            expected_row_count_poe1: 14,
            expected_row_count_poe2: 14,
        },
        TableInfo {
            table: Box::new(CraftingCategoriesTable::new()),
            name: CRAFTING_CATEGORIES.to_string(),
            expected_row_count_poe1: 20,
            expected_row_count_poe2: 18,
        },
        TableInfo {
            table: Box::new(ExchangePricesTable::new()),
            name: EXCHANGE_PRICES.to_string(),
            expected_row_count_poe1: 1,
            expected_row_count_poe2: 1,
        },
        TableInfo {
            table: Box::new(LicensesTable::new()),
            name: LICENSES.to_string(),
            expected_row_count_poe1: 6,
            expected_row_count_poe2: 6,
        },
        TableInfo {
            table: Box::new(SoundsTable::new()),
            name: SOUNDS.to_string(),
            expected_row_count_poe1: 6,
            expected_row_count_poe2: 7,
        },
    ];

    struct TableInfo {
        table: Box<dyn Table>,
        name: String,
        expected_row_count_poe1: usize,
        expected_row_count_poe2: usize,
    }

    fn call_for_each_table(test: fn(table: &dyn Table, expected_count: usize, tx: &mut Transaction, env: Env)) {
        let env_guard = EnvPoolGuard::new();
        let env = env_guard.env;
        for info in TABLE_REGISTRY.iter() {
            let mut conn = Conn::new(env).unwrap();
            let mut tx = conn.test_transaction().unwrap();
            let expected_row_count = match get_config().game_variant {
                GameVariant::Poe1 => info.expected_row_count_poe1,
                GameVariant::Poe2 => info.expected_row_count_poe2,
            };
            test(&*info.table, expected_row_count, &mut tx, env);
        }
    }

    #[test]
    fn test_count_works() {
        call_for_each_table(|table, expected_count, tx, _| {
            let actual_count = table.count(tx).expect(&format!("Count failed for table {}.", table.name()));
            assert_eq!(expected_count, actual_count, "Incorrect initial count for table {}.", table.name());
        });
    }

    #[test]
    fn test_create_works() {
        call_for_each_table(|table, _, tx, _| {
            // Drop and then create the table, panicking on error.
            table.drop_table(tx).unwrap();
            table.create(tx).unwrap();

            // This call too shouldn't panic.  In other words, create should succeed even if the table already exists.
            table.create(tx).unwrap();
        });
    }

    #[test]
    fn test_create_initial_data_works() {
        // TODO
    }

    #[test]
    fn test_delete_works() {
        call_for_each_table(|table, expected_count, tx, _| {
            // Check existing count.
            let actual_count = table.count(tx).expect(&format!("Count failed for table {}.", table.name()));
            assert_eq!(expected_count, actual_count, "Incorrect initial count for table {}.", table.name());

            // Delete content and verify that row count is zero.
            table.delete(tx).unwrap();
            let actual_count = table.count(tx).expect(&format!("Count failed for table {}.", table.name()));
            assert_eq!(0, actual_count, "Incorrect count for table {} following delete.", table.name());
        });
    }

    #[test]
    fn test_drop_table_works() {
        call_for_each_table(|table, expected_count, tx, _| {
            // Check that table exists initially by calling count.
            let actual_count = table.count(tx).expect(&format!("Count failed for table {}.", table.name()));
            assert_eq!(expected_count, actual_count, "Incorrect initial count for table {}.", table.name());

            // Drop the table.  Then call count confirming that the table doesn't exist.
            table.drop_table(tx).unwrap();
            let result = table.count(tx);
            assert!(result.is_err(), "Drop_table failed for table {}.", table.name());

            // Call drop_table on a table that doesn't exist.  This should succeed.
            table.drop_table(tx).unwrap();
        });
    }

    #[test]
    fn test_export_works() {
        call_for_each_table(|table, _, tx, env| {
            let env_paths = Paths::new(env);
            let export_path = env_paths.lookup(Dir::EnvOut).join(format!("test_export_works_{}.json", table.name()));
            {
                let file = OpenOptions::new().write(true).create(true).open(&export_path).unwrap();
                let mut writer = BufWriter::new(file);
                table.export(&mut writer, tx).unwrap();
            }
            let asset_path = env_paths.lookup(Dir::AssetsJsonTest).join(format!("{}.json", table.name()));
            let result = tu::compare_text_files(&asset_path, &export_path, None);
            assert!(result.is_ok(), "{}-{}: {}", get_config().game_variant, env, result.unwrap_err());
        });
    }

    #[test]
    fn test_import_works() {
        call_for_each_table(|table, _, tx, env| {
            // First, delete all data currently in the table.
            table.delete(tx).unwrap();

            // Next import test data into the table.
            let env_paths = Paths::new(env);
            let asset_path = env_paths.lookup(Dir::AssetsJsonTest).join(format!("{}.json", table.name()));
            {
                let file = File::open(&asset_path).unwrap();
                let mut reader = BufReader::new(file);
                table.import(&mut reader, tx).unwrap();
            }

            // Then, export the imported data
            let export_path = env_paths.lookup(Dir::EnvOut).join(format!("test_import_works_{}.json", table.name()));
            {
                let file = OpenOptions::new().write(true).create(true).open(&export_path).unwrap();
                let mut writer = BufWriter::new(file);
                table.export(&mut writer, tx).unwrap();
            }

            // Finally, compare the exported JSON against the imported JSON.
            let result = tu::compare_text_files(&asset_path, &export_path, None);
            assert!(result.is_ok(), "{}-{}: {}", get_config().game_variant, env, result.unwrap_err());
        });
    }

    #[test]
    fn test_is_empty_works() {
        call_for_each_table(|table, _, tx, _| {
            assert_eq!(
                table.is_empty(tx).unwrap(),
                false,
                "Is_empty() returned true for non-empty table {}.",
                table.name()
            );

            table.delete(tx).unwrap();
            assert_eq!(table.is_empty(tx).unwrap(), true, "Is_empty() returned false for empty table {}.", table.name());
        });
    }

    #[test]
    fn test_name_works() {
        fn test(table: &dyn Table, name: &str) {
            assert_eq!(table.name(), name, "Name failed.  Expected name '{}'; actual name '{}'.", name, table.name());
        }

        for info in TABLE_REGISTRY.iter() {
            test(&*info.table, &info.name);
        }
    }

    #[test]
    fn test_partial_works() {
        call_for_each_table(|table, full_row_count, tx, env| {
            // Create the partial data in the out directory.
            let env_paths = Paths::new(env);
            let asset_path = env_paths.lookup(Dir::AssetsJsonTest).join(format!("{}.json", table.name()));
            let partial_path = env_paths.lookup(Dir::EnvOut).join(format!("test_partial_works_{}.json", table.name()));
            {
                let asset_file = File::open(&asset_path).unwrap();
                let mut reader = BufReader::new(asset_file);

                let partial_file = OpenOptions::new().write(true).create(true).open(&partial_path).unwrap();
                let mut writer = BufWriter::new(partial_file);
                table.partial(&mut reader, &mut writer).unwrap();
            }

            // Delete all data currently in the table.
            table.delete(tx).unwrap();

            // Import the partial data and confirm that partial_row_count < full_row_count
            {
                let file = File::open(&partial_path).unwrap();
                let mut reader = BufReader::new(file);
                table.import(&mut reader, tx).unwrap();
            }
            let partial_row_count = table.count(tx).unwrap();
            assert!(
                partial_row_count < full_row_count,
                "{}-{}: Partial row count {} >= full row count {} for table {}.",
                get_config().game_variant,
                env,
                partial_row_count,
                full_row_count,
                table.name()
            );

            // Import partial data again and confirm that count = previous count
            {
                let file = File::open(&partial_path).unwrap();
                let mut reader = BufReader::new(file);
                table.import(&mut reader, tx).unwrap();
            }
            let second_partial_row_count = table.count(tx).unwrap();
            assert_eq!(
                partial_row_count,
                second_partial_row_count,
                "{}-{}: Partial row count {} != second partial row count {} for table {}.",
                get_config().game_variant,
                env,
                partial_row_count,
                second_partial_row_count,
                table.name()
            );

            // Import full data and confirm that row count becomes full row count.
            let asset_path = env_paths.lookup(Dir::AssetsJsonTest).join(format!("{}.json", table.name()));
            {
                let file = File::open(&asset_path).unwrap();
                let mut reader = BufReader::new(file);
                table.import(&mut reader, tx).unwrap();
            }
            let actual_full_row_count = table.count(tx).unwrap();
            assert_eq!(
                full_row_count,
                actual_full_row_count,
                "{}-{}: Expected full row count {} != actual full row count {} for table {}.",
                get_config().game_variant,
                env,
                full_row_count,
                actual_full_row_count,
                table.name()
            );

            // Import the partial data a second time and confirm that actual_full_row_count == full_row_count
            {
                let file = File::open(&partial_path).unwrap();
                let mut reader = BufReader::new(file);
                table.import(&mut reader, tx).unwrap();
            }
            let second_actual_full_row_count = table.count(tx).unwrap();
            assert_eq!(
                actual_full_row_count,
                full_row_count,
                "{}-{}: Actual full row count {} != full row count {} after 2nd import for table {}.",
                get_config().game_variant,
                env,
                second_actual_full_row_count,
                full_row_count,
                table.name()
            );
        });
    }
}
