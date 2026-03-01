// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 3/1/2026.

use crate::util::env::Env;
use crate::GameVariant;
use rusqlite::Transaction;
use std::ops::Deref;

pub struct Tx<'a> {
    pub tx: Transaction<'a>,
    pub env: Env,
    pub game_variant: GameVariant,
}

impl<'a> Deref for Tx<'a> {
    type Target = Transaction<'a>;
    fn deref(&self) -> &Self::Target {
        &self.tx
    }
}
