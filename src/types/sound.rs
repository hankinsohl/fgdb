// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/4/2026.

use crate::types::sound_volume::SoundVolume;
use crate::types::stock_sound::StockSound;
use crate::util::error::FromSqlError;
use anyhow::Result;
use rusqlite::Error as RusqliteError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Type {
    Custom,
    Stock,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Sound {
    pub volume: Option<SoundVolume>,
    pub sound_type: Type,
    pub sound: String,
}

impl Sound {
    // Constructs a Sound using arguments as stored in the database.
    pub fn from_sql(volume: Option<SoundVolume>, stock_sound: Option<String>, custom_sound: Option<String>) -> Result<Option<Self>, RusqliteError> {
        if stock_sound.is_none() && custom_sound.is_none() {
            if volume.is_some() {
                return Err(FromSqlError::Sound("stock_sound and custom_sound are None but volume is Some".to_string()).into());
            }
            Ok(None)
        } else if stock_sound.is_some() && custom_sound.is_some() {
            Err(FromSqlError::Sound("stock_sound and custom_sound are both Some".to_string()).into())
        } else if let Some(s) = stock_sound {
            Ok(Some(Sound {
                volume,
                sound_type: Type::Stock,
                sound: StockSound::from_str(&s)?.to_string(),
            }))
        } else {
            let custom_sound_string = custom_sound.unwrap();
            Ok(Some(Sound {
                volume,
                sound_type: Type::Custom,
                sound: custom_sound_string,
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Error as RusqliteError;

    #[test]
    fn test_new_fails_with_invalid_parameters() {
        // Some(Volume) but None stock and custom sounds.
        let sound_volume_100 = SoundVolume::new(100).unwrap();
        let result = Sound::from_sql(Some(sound_volume_100), None, None);
        assert!(matches!(result, Err(RusqliteError::FromSqlConversionFailure(..))));

        // Stock and custom sound both Some.
        let stock_sound_string = "10".to_string();
        let custom_sound_string = "Angry Cat".to_string();
        let result = Sound::from_sql(
            Some(sound_volume_100),
            Some(stock_sound_string.to_string()),
            Some(custom_sound_string.to_string()),
        );
        assert!(matches!(result, Err(RusqliteError::FromSqlConversionFailure(..))));
    }

    #[test]
    fn test_new_works_with_valid_parameters() {
        let opt_sound = Sound::from_sql(None, None, None).unwrap();
        assert!(opt_sound.is_none());

        let sound_volume_10 = SoundVolume::new(10).unwrap();
        let stock_sound_string = "3".to_string();
        let opt_sound = Sound::from_sql(Some(sound_volume_10), Some(stock_sound_string.clone()), None).unwrap();
        assert!(opt_sound.is_some());
        let sound = opt_sound.unwrap();
        assert_eq!(sound_volume_10, sound.volume.unwrap());
        assert_eq!(Type::Stock, sound.sound_type);
        assert_eq!(stock_sound_string, sound.sound);

        let sound_volume_300 = SoundVolume::new(300).unwrap();
        let custom_sound_string = "Alarm".to_string();
        let opt_sound = Sound::from_sql(Some(sound_volume_300), None, Some(custom_sound_string.clone())).unwrap();
        assert!(opt_sound.is_some());
        let sound = opt_sound.unwrap();
        assert_eq!(sound_volume_300, sound.volume.unwrap());
        assert_eq!(Type::Custom, sound.sound_type);
        assert_eq!(custom_sound_string, sound.sound);
    }
}
