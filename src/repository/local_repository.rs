// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/25/2026.

use std::path::PathBuf;
use anyhow::Error;
use slitu::Timestamp;
use crate::fs::dir::Dir;
use crate::fs::paths::Paths;
use crate::repository::repository::Repository;
use crate::util::consts;
use std::fs;
use crate::config::fgdb_config::get_config;
use crate::GameVariant;
use crate::util::env::Env;

pub struct LocalRepository {
    paths: Paths,
    repository_path: PathBuf,
}

impl Default for LocalRepository {
    fn default() -> Self {
        LocalRepository::new()
    }
}

impl Repository for LocalRepository {
    fn download(&self) -> Result<(), Error> {
        let mut zip_path = self.repository_path.join(consts::REPOSITORY_ZIP_DIR);
        zip_path.push(consts::REPOSITORY_ZIP_FILE_NAME);
        let cache_dir = PathBuf::from(self.paths.lookup(Dir::CacheZip));

        // Copy the zip file in the repository to the cache zip directory.
        fs::copy(&zip_path, &cache_dir)?;

        // Copy the repository timestamp to the cache timestamp directory.
        let mut repository_timestamp_path = self.repository_path.join(consts::REPOSITORY_TIMESTAMP_DIR);
        repository_timestamp_path.push(consts::TIMESTAMP_FILE_NAME);
        let cache_timestamp_path = self.paths.lookup(Dir::CacheTimestamp).join(consts::TIMESTAMP_FILE_NAME);
        fs::copy(&repository_timestamp_path, &cache_timestamp_path)?;
        Ok(())
    }

    fn is_cache_current(&self) -> Result<bool, Error> {
        let mut repository_timestamp_path = self.repository_path.join(consts::REPOSITORY_TIMESTAMP_DIR);
        repository_timestamp_path.push(consts::TIMESTAMP_FILE_NAME);
        let repository_timestamp = Timestamp::from_path(&repository_timestamp_path)?;

        let cache_timestamp_path = self.paths.lookup(Dir::CacheTimestamp).join(consts::TIMESTAMP_FILE_NAME);
        let is_current = if fs::exists(&cache_timestamp_path)? {
            let cache_timestamp = Timestamp::from_path(&cache_timestamp_path)?;
            cache_timestamp.is_current(&repository_timestamp)
        } else {
            false
        };
        Ok(is_current)
    }
}

impl LocalRepository {
    pub fn new() -> Self {
        LocalRepository::create(get_config().root_path.clone(), get_config().game_variant)
    }

    pub fn create(root_path: PathBuf, game_variant: GameVariant) -> Self {
        let paths = Paths::create(game_variant, Env::Prod);
        let repository_path = root_path.join(game_variant.to_string());
        Self { paths, repository_path }
    }
}
