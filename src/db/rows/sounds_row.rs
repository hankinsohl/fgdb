// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/20/2026.

use serde::{Deserialize, Serialize};

// The sounds table lists all custom sounds used for drops as well as their associated sound files and licenses.
#[derive(Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SoundsRow {
    // The name of the sound.
    pub sound: String,

    // The name of the file associated with the sound.
    pub file_name: String,

    // The original name of the sound file.  Note that sound files are often renamed to better reflect their use in
    // the filter.
    pub original_file_name: String,

    // The source from which the sound was obtained.
    pub source: String,

    // The name of the composer of the sound.
    pub composer: String,

    // Whether the original sound file was modified.
    pub is_modified: bool,

    // URL from which the sound was obtained.
    pub url: String,

    // The name of the license governing use of the sound.
    pub license: String,
}
