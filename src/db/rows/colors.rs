// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/20/2026.

use rgb::RGBA8 as Rgba8;
use serde::{Deserialize, Serialize};

// The colors table contains named 8-bit RGBA color values used for drops.
#[derive(Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ColorsRow {
    // The name of the color.
    pub color: String,

    // The url used to obtain RGBA value for the color.
    pub url: String,

    // The 8-bit RGBA value for the color.
    pub pixel: Rgba8,
}
