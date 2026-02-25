// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/16/2026.

use crate::types::game_variant::GameVariant;
use crate::types::repository::Repository;
use crate::util::consts;
use anyhow::{Error, Result};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

static FGDB_CONFIG: OnceLock<FgdbConfig> = OnceLock::new();

#[derive(Clone, Debug, Deserialize)]
pub struct FgdbConfig {
    pub game_variant: GameVariant,

    pub repository: Repository,

    pub root_path: PathBuf,
}

impl FgdbConfig {
    pub fn new() -> Result<FgdbConfig, Error> {
        let contents = fs::read_to_string(consts::CONFIG_TOML)?;
        Ok(toml::from_str(&contents)?)
    }
}

pub fn get_config() -> &'static FgdbConfig {
    FGDB_CONFIG.get_or_init(|| FgdbConfig::new().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fgen_config_works() {
        let _config = FgdbConfig::new().unwrap();
    }

    #[test]
    fn test_get_config_works() {
        let _config = get_config();
    }
}
