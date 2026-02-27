// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/12/2026.

#[cfg(any(feature = "std", feature = "errors"))]
pub use util::errors::{FgdbDatFileError, FgdbFromSqlError, FgdbParseError, FgdbRangeError};

#[cfg(any(feature = "std", feature = "names"))]
pub use db::tables::names::*;

#[cfg(any(feature = "std", feature = "rows"))]
pub use db::rows::{
    action_sets_row::ActionSetsRow, action_sets_row::ActionSetsRowBuilder, armor_types_row::ArmorTypesRow, async_prices_row::AsyncPricesRow,
    base_type_items_row::BaseTypeItemsRow, base_types_row::BaseTypesRow, classes_row::ClassesRow, exchange_prices_row::ExchangePricesRow,
    licenses_row::LicensesRow, sounds_row::SoundsRow,
};

#[cfg(any(feature = "std", feature = "types"))]
pub use types::{
    armor_type::ArmorType, font_size::FontSize, game_variant::GameVariant, gem_level::GemLevel, icon::Icon, icon_shape::IconShape, icon_size::IconSize,
    item_level::ItemLevel, liquidity::Liquidity, non_unique_rarity::NonUniqueRarity, price::Price, rarity::Rarity, repository::Repository, sound::Sound,
    sound_volume::SoundVolume, stack_size::StackSize, stock_color::StockColor, stock_sound::StockSound,
};

#[cfg(feature = "std")]
pub use db::tables::{
    action_sets_table::ActionSetsTable, armor_types_table::ArmorTypesTable, async_prices_table::AsyncPricesTable, base_type_items_table::BaseTypeItemsTable,
    base_types_table::BaseTypesTable, classes_table::ClassesTable, exchange_prices_table::ExchangePricesTable, licenses_table::LicensesTable,
    sounds_table::SoundsTable,
};

#[cfg(feature = "std")]
pub mod concurrency;
#[cfg(feature = "std")]
pub mod config;
#[cfg(any(feature = "std", feature = "names", feature = "rows"))]
pub mod db;
#[cfg(feature = "std")]
pub mod fs;
#[cfg(feature = "std")]
pub mod init;
#[cfg(feature = "std")]
pub mod net;
#[cfg(feature = "std")]
pub mod repo;
#[cfg(any(feature = "std", feature = "types"))]
pub mod types;
#[cfg(feature = "std")]
pub mod update;
#[cfg(any(feature = "std", feature = "errors"))]
pub mod util;
