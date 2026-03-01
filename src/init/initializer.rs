// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/23/2026.

#[cfg(test)]
use crate::concurrency::env_pool::EnvPool;

use crate::db::conn::Conn;
use crate::db::database::{Database, TABLES};
use crate::fs::dir::Dir;
use crate::fs::paths;
use crate::fs::paths::Paths;
use crate::types::game_variant::GameVariant;
use crate::util::env::Env;
use std::fs::File;
use std::io::BufReader;
use strum::IntoEnumIterator;

pub struct Initializer;

impl Default for Initializer {
    fn default() -> Self {
        Self::new()
    }
}

impl Initializer {
    pub fn new() -> Self {
        Initializer::remove_env_out_dirs();
        Initializer::create_dirs();
        #[cfg(test)]
        Initializer::init_env_pool();
        Initializer::init_database();
        Self {}
    }

    fn create_dirs() {
        for game_variant in GameVariant::iter() {
            paths::create_cache_dirs(game_variant).unwrap();
            for env in Env::iter() {
                paths::create_env_dirs(game_variant, env).unwrap();
                paths::create_env_out_dirs(game_variant, env).unwrap();
            }
        }
    }

    #[cfg(test)]
    fn init_env_pool() {
        EnvPool::init();
    }

    // Ensures that each table in the database exists, creating a new table if the table does not currently exist.  If
    // an extant table is found, no changes to the table are made.
    fn init_database() {
        let db = Database::new().unwrap();
        for game_variant in GameVariant::iter() {
            for env in Env::iter() {
                let mut conn = Conn::create(game_variant, env).unwrap();
                let mut tx = conn.create_tx().unwrap();
                db.create(&mut tx).unwrap();

                // For test environments, initialize each table with test data.
                if Env::is_test_env(env) {
                    for table in TABLES.iter() {
                        table.delete(&mut tx).unwrap();
                        let paths = Paths::new(Env::Prod);
                        let src_path = paths.lookup(Dir::AssetsJsonTest).join(format!("{}.json", table.name()));
                        let file = File::open(&src_path).unwrap();
                        let mut reader = BufReader::new(file);
                        table.import(&mut reader, &mut tx).unwrap();
                    }
                }
            }
        }
    }

    fn remove_env_out_dirs() {
        for game_variant in GameVariant::iter() {
            for env in Env::iter() {
                paths::remove_env_out_dirs(game_variant, env).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_init::dynamic;

    // The static initializer is used to ensure that the test databases are reset to an initial, known
    // condition prior to running any tests.
    #[dynamic]
    static INITIALIZER: Initializer = Initializer::new();
}
