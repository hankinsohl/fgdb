// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/17/2026.

use std::fmt;
use std::fmt::Display;
use strum_macros::EnumIter;

pub static CACHE_DIRS: [Dir; 3] = [Dir::CacheJson, Dir::CacheTimestamp, Dir::CacheZip];
pub static ENV_DIRS: [Dir; 2] = [Dir::EnvDb, Dir::EnvOut];
pub static ENV_OUT_DIRS: [Dir; 1] = [Dir::EnvOut];

#[derive(Copy, Clone, Debug, EnumIter, PartialEq)]
pub enum Dir {
    AssetsJsonTest,
    CacheJson,
    CacheTimestamp,
    CacheZip,
    EnvDb,
    EnvOut,
}

impl Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Dir::AssetsJsonTest => "AssetsJsonTest",
                Dir::CacheJson => "CacheJson",
                Dir::CacheTimestamp => "CacheTimestamp",
                Dir::CacheZip => "CacheZip",
                Dir::EnvDb => "EnvDb",
                Dir::EnvOut => "EnvOut",
            }
        )
    }
}
