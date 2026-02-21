// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/20/2026.

use crate::types::non_unique_rarity::NonUniqueRarity;
use serde::{Deserialize, Serialize};

// Class is the broadest category of item classification.  Example classes include rings and body armours.
// We obtain the complete list of classes using https://www.pathofexile.com/api/trade/data/items and then
// change class names to match class names used in item filters using information from DAT files.
#[derive(Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClassesRow {
    // The name of the class.  Note that some base types - perhaps by accident - are not associated with a class;
    // to accommodate this possibility, a class called "None" will be present in the table.
    pub class: String,

    // The highest item rarity (Normal < Magic < Rare) associated with the class.
    pub highest_rarity: Option<NonUniqueRarity>,
}
