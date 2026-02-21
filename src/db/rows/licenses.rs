// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/20/2026.

use serde::{Deserialize, Serialize};
use url::Url;

// Assets used by the filter generator such as sound files may be licensed.  The license table contains a list of
// all asset licenses; for each license a URL is provided to obtain more information about the license.
#[derive(Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct LicensesRow {
    // The name used to identify the license.
    pub license: String,

    // A URL used to obtain information about the license.
    pub url: Url,
}
