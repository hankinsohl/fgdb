// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/16/2026.

use rusqlite::Error as RusqliteError;
use thiserror::Error;

const UNKNOWN_COLUMN: usize = usize::MAX;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Command output failed: {0}")]
    Failed(String),
}

#[derive(Debug, Error)]
pub enum DatFileError {
    #[error("Lookup failed.  {0}")]
    LookupFailed(String),
}

#[derive(Debug, Error)]
pub enum FromSqlError {
    #[error("Error constructing Icon: {0}.")]
    Icon(String),

    #[error("Error constructing Sound: {0}.")]
    Sound(String),
}

impl From<FromSqlError> for RusqliteError {
    fn from(e: FromSqlError) -> RusqliteError {
        RusqliteError::FromSqlConversionFailure(UNKNOWN_COLUMN, rusqlite::types::Type::Text, Box::new(e))
    }
}

#[derive(Debug, Error)]
pub enum NetError {
    #[error("Invalid update data: {0}.")]
    InvalidUpdateData(String),

    #[error("Invalid URL: {0}.")]
    InvalidUrl(String),
}

#[derive(Debug, Error)]
pub enum ParseError {
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

impl From<ParseError> for RusqliteError {
    fn from(e: ParseError) -> RusqliteError {
        RusqliteError::FromSqlConversionFailure(UNKNOWN_COLUMN, rusqlite::types::Type::Text, Box::new(e))
    }
}

#[derive(Debug, Error)]
pub enum RangeError {
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

impl From<RangeError> for RusqliteError {
    fn from(e: RangeError) -> RusqliteError {
        RusqliteError::FromSqlConversionFailure(UNKNOWN_COLUMN, rusqlite::types::Type::Text, Box::new(e))
    }
}
