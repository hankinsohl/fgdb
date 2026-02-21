// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/20/2026.

use crate::types::non_unique_rarity::NonUniqueRarity;
use serde::{Deserialize, Serialize};

// Crafting categories for item classes.
#[derive(Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CraftingCategoriesRow {
    // Crafting category.
    pub crafting_category: String,

    // The highest non-unique rarity obtainable for items in this category.
    pub highest_rarity: NonUniqueRarity,
}
