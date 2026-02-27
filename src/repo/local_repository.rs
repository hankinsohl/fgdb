// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/25/2026.

use crate::config::fgdb_config::get_config;
use crate::fs::dir::Dir;
use crate::fs::paths::Paths;
use crate::repo::repository::Repository;
use crate::util::consts;
use crate::util::env::Env;
use crate::GameVariant;
use anyhow::Error;
use slitu::Timestamp;
use std::fs;
use std::path::PathBuf;

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
        let zip_path = self.repository_path.join(consts::REPOSITORY_ZIP_DIR).join(consts::REPOSITORY_ZIP_FILE_NAME);
        let cache_path = PathBuf::from(self.paths.lookup(Dir::CacheZip)).join(consts::REPOSITORY_ZIP_FILE_NAME);

        // Copy the zip file in the repository to the cache zip directory.
        fs::copy(&zip_path, &cache_path)?;

        // Copy the repository timestamp to the cache timestamp directory.
        let repository_timestamp_path = self.repository_path.join(consts::REPOSITORY_TIMESTAMP_DIR).join(consts::TIMESTAMP_FILE_NAME);
        let cache_timestamp_path = self.paths.lookup(Dir::CacheTimestamp).join(consts::TIMESTAMP_FILE_NAME);
        fs::copy(&repository_timestamp_path, &cache_timestamp_path)?;
        Ok(())
    }

    fn is_cache_current(&self) -> Result<bool, Error> {
        let repository_timestamp_path = self.repository_path.join(consts::REPOSITORY_TIMESTAMP_DIR).join(consts::TIMESTAMP_FILE_NAME);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download() {
        let repo = LocalRepository::new();
        repo.download().unwrap();
    }
}
