// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/20/2026.

use crate::types::armor_type::ArmorType;
use serde::{Deserialize, Serialize};

// The armor types table is used to select the sound to play when an armor type drops.
#[derive(Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ArmorTypesRow {
    // The base type for the item.
    pub base_type: String,

    // The armor type for the item.
    pub armor_type: ArmorType,
}
