// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/20/2026.

use serde::{Deserialize, Serialize};

// Item is an unofficial POE term used to distinguish uniques having the same base type, such as Headhunter and
// Waistgate which are both Heavy Belts.  Item cannot be used in filter blocks.  However, item is essential for
// pricing; item is one of the properties used to form price lookup keys.
// Item, on its own, is, not unique; two different base types may have items with the same item name.
// For example, Emerald, Ruby and Sapphire base types all have Grand Spectrum items.  The combination of base type
// and item is, however, unique.
#[derive(Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BaseTypeItemsRow {
    // The base_type associated with the item.
    pub base_type: String,

    // The name of the associated item, or None if this entry refers to the base type itself without regard to item.
    pub item: Option<String>,

    // True if the item is unique, false otherwise.
    pub is_unique: bool,
}

impl BaseTypeItemsRow {
    pub fn gen_key_from_parts(base_type: &str, item: &Option<String>) -> String {
        format!("{}::{}", base_type, item.as_ref().map_or("null".to_string(), |x| x.clone()))
    }

    pub fn gen_key(&self) -> String {
        BaseTypeItemsRow::gen_key_from_parts(&self.base_type, &self.item)
    }
}
