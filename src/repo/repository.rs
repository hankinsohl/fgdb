// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/25/2026.

use crate::GameVariant;
use anyhow::{Error, Result};

pub trait Repository {
    /// Downloads a zip file containing all the JSON files in the repository to the cache/zip directory.
    fn download(&self) -> Result<(), Error>;

    /// Compares the timestamp in the cache/timestamp directory against the timestamp in the repository.  If
    /// the cache is at least as new as the repository, true is returned, otherwise false is returned.  If the
    /// timestamp file is missing from the cache/timestamp directory, false is returned.
    fn is_cache_current(&self) -> Result<bool, Error>;
}

/// Obtains settings from fgdb.toml and then creates a compatible repository.
pub fn create_repository(_game_variant: GameVariant) -> Result<Box<dyn Repository>, Error> {
    todo!("create_repository not yet implemented");
}
