// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/6/2026.

use crate::util::errors::FgdbRangeError;
use anyhow::Result;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::{Result as RusqliteResult, ToSql};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::ops::RangeInclusive;

pub const MIN_SOUND_VOLUME: u16 = 0;
pub const MAX_SOUND_VOLUME: u16 = 300;
pub const SOUND_VOLUME_RANGE: RangeInclusive<u16> = MIN_SOUND_VOLUME..=MAX_SOUND_VOLUME;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SoundVolume {
    volume: u16,
}

impl Display for SoundVolume {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.volume)
    }
}

impl FromSql for SoundVolume {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        SoundVolume::new(u16::column_result(value)?).map_err(|e| FromSqlError::Other(Box::new(e)))
    }
}

impl ToSql for SoundVolume {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(ToSqlOutput::from(self.volume))
    }
}

impl SoundVolume {
    pub fn new(volume: u16) -> Result<Self, FgdbRangeError> {
        if !SOUND_VOLUME_RANGE.contains(&volume) {
            return Err(FgdbRangeError::SoundVolume(MIN_SOUND_VOLUME, MAX_SOUND_VOLUME, volume));
        }
        Ok(Self { volume })
    }

    pub fn volume(&self) -> u16 {
        self.volume
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::errors::FgdbRangeError;

    #[test]
    fn test_sound_volume_new_works_with_valid_volume() {
        let sound_volume = SoundVolume::new(29).unwrap();
        assert_eq!(29, sound_volume.volume());
    }

    #[test]
    fn test_sound_volume_new_generates_error_with_invalid_volume() {
        let result = SoundVolume::new(666);
        assert!(matches!(result, Err(FgdbRangeError::SoundVolume(MIN_SOUND_VOLUME, MAX_SOUND_VOLUME, 666))));
    }
}
