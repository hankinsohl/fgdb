// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/19/2026.

use crate::config::fgdb_config::get_config;
use crate::fs::dir::Dir;
use crate::fs::paths::Paths;
use crate::types::game_variant::GameVariant;
use crate::util::consts;
use crate::util::env::Env;
use anyhow::{Error, Result};
use rusqlite::{Connection, DropBehavior, Transaction};

pub struct Conn {
    pub conn: Connection,
    pub env: Env,
}

impl Conn {
    pub fn new(env: Env) -> Result<Self, Error> {
        Conn::create(get_config().game_variant, env)
    }

    pub fn create(game_variant: GameVariant, env: Env) -> Result<Self, Error> {
        let paths = Paths::create(game_variant, env);
        Ok(Self {
            conn: Connection::open(paths.lookup(Dir::EnvDb).join(consts::DB_NAME))?,
            env,
        })
    }

    pub fn test_transaction(&mut self) -> Result<Transaction, Error> {
        // Note: The default Rusqlite transaction drop mode is rollback.
        Ok(self.conn.transaction()?)
    }

    pub fn transaction(&mut self) -> Result<Transaction, Error> {
        let mut tx = self.conn.transaction()?;
        tx.set_drop_behavior(DropBehavior::Commit);
        Ok(tx)
    }
}
