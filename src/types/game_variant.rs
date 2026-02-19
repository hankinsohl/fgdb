// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/16/2026.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, Default, Deserialize, EnumIter, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum GameVariant {
    #[default]
    Poe1,
    Poe2,
}

impl Display for GameVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GameVariant::Poe1 => "POE 1",
                GameVariant::Poe2 => "POE 2",
            }
        )
    }
}
