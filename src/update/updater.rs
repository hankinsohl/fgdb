// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/12/2026.

use crate::config::fgdb_config::get_config;
use crate::db::conn::Conn;
use crate::db::database::Database;
use crate::fs::dir::Dir;
use crate::fs::paths::Paths;
use crate::repo::local_repository::LocalRepository;
use crate::repo::repository::Repository;
use crate::update::policy::Policy;
use crate::util::env::Env;
use crate::GameVariant;
use anyhow::{Error, Result};
use zip_extensions::zip_extract::zip_extract;

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
    pub fn update(&self, policy: Policy) -> Result<bool, Error> {
        match policy {
            Policy::Skip => Ok(false),
            Policy::Auto => {
                if !self.is_cache_current()? {
                    self.update_impl()?;
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Policy::Force => {
                self.update_impl()?;
                Ok(true)
            }
        }
    }

    fn is_cache_current(&self) -> Result<bool, Error> {
        // TODO - replace repository creation with call to factory function create_repository.
        let repo = LocalRepository::create(get_config().root_path.clone(), get_config().game_variant);
        repo.is_cache_current()
    }

    fn update_cache(&self) -> Result<(), Error> {
        // TODO - replace repository creation with call to factory function create_repository.
        let repo = LocalRepository::create(get_config().root_path.clone(), get_config().game_variant);
        repo.download()?;

        let zip_file = repo.get_zip_path();
        let cache_json_dir = self.paths.lookup(Dir::CacheJson).to_path_buf();
        zip_extract(&zip_file, &cache_json_dir)?;
        Ok(())
    }

    fn update_database(&self) -> Result<(), Error> {
        let db = Database::new()?;
        let mut conn = Conn::create(self.game_variant, Env::Prod)?;
        let mut tx = conn.create_tx()?;
        db.drop_tables(&mut tx)?;
        db.create(&mut tx)?;

        todo!(); // import data - we should be able to do this from a single func
    }

    fn update_impl(&self) -> Result<(), Error> {
        self.update_cache()?;
        self.update_database()?;
        Ok(())
    }
}
