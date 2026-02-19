// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/17/2026.

use crate::config::fgdb_config::get_config;
use crate::fs::dir::{Dir, CACHE_DIRS, ENV_DIRS, ENV_OUT_DIRS};
use crate::types::game_variant::GameVariant;
use crate::util::consts::*;
use crate::util::env::Env;
use anyhow::{Error, Result};
use const_format::formatcp;
use std::fs;
use std::path::Path;

// Note: 2 macros are used to work around RustRover parsing bug ("attempting to use nonexistent positional argument 1").
macro_rules! gen_poe1_registry_element {
    ($root:ident) => {
        [
            formatcp!("{ASSETS_DIR}/poe1/{ASSETS_JSON_TEST_DIR}"),
            formatcp!("{CACHE_DIR}/poe1/{CACHE_JSON_DIR}"),
            formatcp!("{CACHE_DIR}/poe1/{CACHE_VER_DIR}"),
            formatcp!("{ENV_DIR}/poe1/{}/{ENV_DB_DIR}", $root),
            formatcp!("{ENV_DIR}/poe1/{}/{ENV_OUT_DIR}", $root),
        ]
    };
}
macro_rules! gen_poe2_registry_element {
    ($root:ident) => {
        [
            formatcp!("{ASSETS_DIR}/poe2/{ASSETS_JSON_TEST_DIR}"),
            formatcp!("{CACHE_DIR}/poe2/{CACHE_JSON_DIR}"),
            formatcp!("{CACHE_DIR}/poe2/{CACHE_VER_DIR}"),
            formatcp!("{ENV_DIR}/poe2/{}/{ENV_DB_DIR}", $root),
            formatcp!("{ENV_DIR}/poe2/{}/{ENV_OUT_DIR}", $root),
        ]
    };
}

// Directory registries ordered by [Env][Dir].  These arrays must be kept in sync with changes to the Env
// and Dir enums.
const POE1_REGISTRY: [[&str; 5]; 6] = [
    gen_poe1_registry_element!(PROD_RELATIVE_PATH),
    gen_poe1_registry_element!(TEST1_RELATIVE_PATH),
    gen_poe1_registry_element!(TEST2_RELATIVE_PATH),
    gen_poe1_registry_element!(TEST3_RELATIVE_PATH),
    gen_poe1_registry_element!(TEST4_RELATIVE_PATH),
    gen_poe1_registry_element!(TEST5_RELATIVE_PATH),
];
const POE2_REGISTRY: [[&str; 5]; 6] = [
    gen_poe2_registry_element!(PROD_RELATIVE_PATH),
    gen_poe2_registry_element!(TEST1_RELATIVE_PATH),
    gen_poe2_registry_element!(TEST2_RELATIVE_PATH),
    gen_poe2_registry_element!(TEST3_RELATIVE_PATH),
    gen_poe2_registry_element!(TEST4_RELATIVE_PATH),
    gen_poe2_registry_element!(TEST5_RELATIVE_PATH),
];

#[derive(Copy, Clone)]
pub struct Paths {
    env: Env,
    game_variant: GameVariant,
}

impl Paths {
    /// Creates a Paths object for the current game variant.  A Paths object facilitates path lookup.
    pub fn new(env: Env) -> Self {
        Paths::create(get_config().game_variant, env)
    }

    /// Creates a Paths object for a specified game variant.  A Paths object facilitates path lookup.
    pub fn create(game_variant: GameVariant, env: Env) -> Self {
        Self { env, game_variant }
    }

    /// Returns the path to the specified directory for the environment.
    pub fn lookup(&self, dir: Dir) -> &Path {
        match self.game_variant {
            GameVariant::Poe1 => Path::new(POE1_REGISTRY[self.env as usize][dir as usize]),
            GameVariant::Poe2 => Path::new(POE2_REGISTRY[self.env as usize][dir as usize]),
        }
    }
}

pub fn create_cache_dirs(game_variant: GameVariant) -> Result<(), Error> {
    create_dirs(game_variant, Env::Prod, CACHE_DIRS.iter())
}

pub fn create_env_dirs(game_variant: GameVariant, env: Env) -> Result<(), Error> {
    create_dirs(game_variant, env, ENV_DIRS.iter())
}

pub fn create_env_out_dirs(game_variant: GameVariant, env: Env) -> Result<(), Error> {
    create_dirs(game_variant, env, ENV_OUT_DIRS.iter())
}

pub fn remove_cache_dirs(game_variant: GameVariant) -> Result<(), Error> {
    remove_dirs(game_variant, Env::Prod, CACHE_DIRS.iter())
}

pub fn remove_env_dirs(game_variant: GameVariant, env: Env) -> Result<(), Error> {
    remove_dirs(game_variant, env, ENV_DIRS.iter())
}

pub fn remove_env_out_dirs(game_variant: GameVariant, env: Env) -> Result<(), Error> {
    remove_dirs(game_variant, env, ENV_OUT_DIRS.iter())
}

fn create_dirs<'a>(game_variant: GameVariant, env: Env, dirs: impl IntoIterator<Item=&'a Dir>) -> Result<(), Error> {
    let paths = Paths::create(game_variant, env);
    for dir in dirs {
        let path = paths.lookup(*dir);
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
    }
    Ok(())
}

fn remove_dirs<'a>(game_variant: GameVariant, env: Env, dirs: impl IntoIterator<Item=&'a Dir>) -> Result<(), Error> {
    let paths = Paths::create(game_variant, env);
    for dir in dirs {
        let path = paths.lookup(*dir);
        if path.exists() {
            fs::remove_dir_all(path)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::dir::Dir;
    use crate::fs::paths::Paths;
    use crate::util::env::Env;
    use strum::IntoEnumIterator;

    #[test]
    fn test_lookup_works() {
        let registry = match get_config().game_variant {
            GameVariant::Poe1 => &POE1_REGISTRY,
            GameVariant::Poe2 => &POE2_REGISTRY,
        };
        for env in Env::iter() {
            for dir in Dir::iter() {
                assert_eq!(
                    Paths::new(env).lookup(dir).to_string_lossy(),
                    registry[env as usize][dir as usize],
                    "Expected '{}': Actual '{}'",
                    Paths::new(env).lookup(dir).to_string_lossy(),
                    registry[env as usize][dir as usize]
                );
            }
        }
    }
}
