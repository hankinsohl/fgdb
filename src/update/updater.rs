// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/12/2026.

use crate::config::fgdb_config::get_config;
use crate::fs::paths::Paths;
use crate::update::policy::Policy;
use crate::util::env::Env;
use crate::GameVariant;
use anyhow::{Error, Result};

pub struct Updater {
    pub game_variant: GameVariant,
    pub paths: Paths,
}

impl Default for Updater {
    fn default() -> Self {
        Updater::new()
    }
}

impl Updater {
    pub fn new() -> Self {
        Updater::create(get_config().game_variant)
    }

    pub fn create(game_variant: GameVariant) -> Self {
        Self {
            game_variant,
            paths: Paths::create(game_variant, Env::Prod),
        }
    }

    // Updates the database according to policy.  If update was performed, true is returned.
    pub fn update(&self, _policy: Policy) -> Result<bool, Error> {
        todo!();
    }
}
