// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/3/2026.

use super::macros::*;
use super::table::GenericTable;
use super::table::Table;
use crate::db::rows::action_sets_row::ActionSetsRow;
use crate::db::tables::names::ACTION_SETS;
use crate::fs::dir::Dir;
use crate::fs::paths::Paths;
use crate::types::icon::Icon;
use crate::types::sound::{Sound, Type};
use crate::util::consts;
use crate::util::env::Env;
use anyhow::{Error, Result};
use itertools::Itertools;
use paste::paste;
use rand::Rng;
use rusqlite::{params, Error as RusqliteError, Transaction};
use serde_json_fmt::JsonFormat;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Write};

pub struct ActionSetsTable {
    pub name: String,
}

impl_generic_table!(ActionSets);

impl Table for ActionSetsTable {
    fn new() -> Self {
        Self { name: ACTION_SETS.to_string() }
    }

    fn create(&self, tx: &mut Transaction) -> Result<(), Error> {
        tx.execute(
            "CREATE TABLE IF NOT EXISTS action_sets
                (
                    action_set              TEXT NOT NULL PRIMARY KEY,
                    is_template_compatible  INTEGER NOT NULL            CHECK (is_template_compatible IN (0, 1)),
                    font_size               INTEGER                     CHECK (font_size >= 18 AND font_size <= 45),
                    text_color              TEXT NOT NULL,
                    border_color            TEXT NOT NULL,
                    play_effect_color       TEXT,
                    background_color        TEXT NOT NULL,
                    minimap_icon_shape      TEXT,
                    minimap_icon_size       INTEGER                     CHECK (minimap_icon_size >= 0 AND minimap_icon_size <= 2),
                    minimap_icon_color      TEXT,
                    volume                  INTEGER                     CHECK (volume >= 0 AND volume <= 300),
                    stock_sound             TEXT,
                    custom_sound            TEXT,
                    FOREIGN KEY (background_color) REFERENCES colors (color),
                    FOREIGN KEY (custom_sound) REFERENCES sounds (sound)
                ) STRICT",
            (),
        )?;
        Ok(())
    }

    fn export(&self, writer: &mut dyn Write, tx: &mut Transaction) -> Result<(), Error> {
        let mut stmt = tx.prepare("SELECT * FROM action_sets")?;
        let rows: Vec<ActionSetsRow> = stmt
            .query_map([], |row| {
                Ok(ActionSetsRow {
                    action_set: row.get(0)?,
                    is_template_compatible: row.get(1)?,
                    font_size: row.get(2)?,
                    text_color: row.get(3)?,
                    border_color: row.get(4)?,
                    play_effect_color: row.get(5)?,
                    background_color: row.get(6)?,
                    icon: Icon::from_sql(row.get(7)?, row.get(8)?, row.get(9)?)?,
                    sound: Sound::from_sql(row.get(10)?, row.get(11)?, row.get(12)?)?,
                })
            })?
            .collect::<Result<Vec<ActionSetsRow>, RusqliteError>>()?;
        let json = JsonFormat::pretty().indent_width(Some(consts::JSON_TAB)).ascii(true).format_to_string(&rows)?;
        writer.write_all(json.as_bytes())?;
        Ok(())
    }

    fn import(&self, reader: &mut dyn Read, tx: &mut Transaction) -> Result<(), Error> {
        let rows: Vec<ActionSetsRow> = serde_json::from_reader(reader)?;
        let mut stmt = tx.prepare_cached(
            r#"INSERT OR IGNORE INTO action_sets (
                action_set,
                is_template_compatible,
                font_size,
                text_color,
                border_color,
                play_effect_color,
                background_color,
                minimap_icon_size,
                minimap_icon_shape,
                minimap_icon_color,
                volume,
                stock_sound,
                custom_sound)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)"#,
        )?;
        for row in &rows {
            stmt.execute(params![
                row.action_set,
                row.is_template_compatible,
                row.font_size,
                row.text_color,
                row.border_color,
                row.play_effect_color,
                row.background_color,
                row.icon.as_ref().map(|i| i.size),
                row.icon.as_ref().map(|i| i.shape),
                row.icon.as_ref().map(|i| i.color),
                row.sound.as_ref().map(|s| s.volume),
                if let Some(s) = row.sound.as_ref() {
                    if s.sound_type == Type::Stock { Some(s.sound.clone()) } else { None }
                } else {
                    None
                },
                if let Some(s) = row.sound.as_ref() {
                    if s.sound_type == Type::Custom { Some(s.sound.clone()) } else { None }
                } else {
                    None
                },
            ])?;
        }
        Ok(())
    }
}
