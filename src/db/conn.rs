// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/19/2026.

use crate::config::fgdb_config::get_config;
use crate::db::tx::Tx;
use crate::fs::dir::Dir;
use crate::fs::paths::Paths;
use crate::types::game_variant::GameVariant;
use crate::util::consts;
use crate::util::env::Env;
use anyhow::{Error, Result};
use rusqlite::{Connection, DropBehavior};

pub struct Conn {
    pub conn: Connection,
    pub env: Env,
    pub game_variant: GameVariant,
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
            game_variant,
        })
    }

    pub fn create_test_tx(&mut self) -> Result<Tx, Error> {
        // Note: The default Rusqlite transaction drop mode is rollback.
        Ok(Tx {
            tx: self.conn.transaction()?,
            env: self.env,
            game_variant: self.game_variant,
        })
    }

    pub fn create_tx(&mut self) -> Result<Tx, Error> {
        let mut tx = self.conn.transaction()?;
        tx.set_drop_behavior(DropBehavior::Commit);
        Ok(Tx {
            tx,
            env: self.env,
            game_variant: self.game_variant,
        })
    }
}
