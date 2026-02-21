// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/16/2026.

use rusqlite::Error as RusqliteError;
use thiserror::Error;

const UNKNOWN_COLUMN: usize = usize::MAX;

#[derive(Debug, Error)]
pub enum FgdbDatFileError {
    #[error("Lookup failed.  {0}")]
    LookupFailed(String),
}

#[derive(Debug, Error)]
pub enum FgdbFromSqlError {
    #[error("Error constructing Icon: {0}.")]
    Icon(String),

    #[error("Error constructing Sound: {0}.")]
    Sound(String),
}

impl From<FgdbFromSqlError> for RusqliteError {
    fn from(e: FgdbFromSqlError) -> RusqliteError {
        RusqliteError::FromSqlConversionFailure(UNKNOWN_COLUMN, rusqlite::types::Type::Text, Box::new(e))
    }
}

#[derive(Debug, Error)]
pub enum FgdbParseError {
    #[error("Invalid armor type '{0}'.")]
    InvalidArmorType(String),

    #[error("Invalid icon shape '{0}'.")]
    InvalidIconShape(String),

    #[error("Invalid liquidity '{0}'.")]
    InvalidLiquidity(String),

    #[error("Invalid rarity '{0}'.")]
    InvalidRarity(String),

    #[error("Invalid stock color '{0}'.")]
    InvalidStockColor(String),

    #[error("Invalid stock sound '{0}'.")]
    InvalidStockSound(String),
}

impl From<FgdbParseError> for RusqliteError {
    fn from(e: FgdbParseError) -> RusqliteError {
        RusqliteError::FromSqlConversionFailure(UNKNOWN_COLUMN, rusqlite::types::Type::Text, Box::new(e))
    }
}

#[derive(Debug, Error)]
pub enum FgdbRangeError {
    #[error("Font size out of range [{0}-{1}]: {2}.")]
    FontSize(u8, u8, u8),

    #[error("Gem level out of range [{0}-{1}]: {2}.")]
    GemLevel(u8, u8, u8),

    #[error("Icon size out of range [{0}-{1}]: {2}.")]
    IconSize(u8, u8, u8),

    #[error("Item level out of range [{0}-{1}]: {2}.")]
    ItemLevel(u8, u8, u8),

    #[error("Price cannot be negative: {0}.")]
    Price(f32),

    #[error("Sound volume out of range [{0}-{1}]: {2}.")]
    SoundVolume(u16, u16, u16),

    #[error("Stack size cannot be zero.")]
    StackSize(),
}

impl From<FgdbRangeError> for RusqliteError {
    fn from(e: FgdbRangeError) -> RusqliteError {
        RusqliteError::FromSqlConversionFailure(UNKNOWN_COLUMN, rusqlite::types::Type::Text, Box::new(e))
    }
}
