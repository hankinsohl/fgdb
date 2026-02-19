// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/3/2026.

use super::macros::*;
use super::table::GenericTable;
use super::table::Table;
use crate::fs::dir::Dir;
use crate::fs::paths::Paths;
use crate::types::font_size::FontSize;
use crate::types::icon::Icon;
use crate::types::sound::{Sound, Type};
use crate::types::stock_color::StockColor;
use crate::util::consts;
use crate::util::env::Env;
use anyhow::{Error, Result};
use itertools::Itertools;
use paste::paste;
use rand::Rng;
use rusqlite::{params, Error as RusqliteError, Transaction};
use serde::{Deserialize, Serialize};
use serde_json_fmt::JsonFormat;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Write};

// The sounds table lists all custom sounds used for drops as well as their associated sound files and licenses.
#[derive(Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ActionSetsRow {
    // The name of the action set.
    pub action_set: String,

    // True if the action set is compatible with template application.  In general, action sets are
    // template compatible; however, action sets used to indicate extremely valuable drops, uniques or
    // quest items, for example, are not compatible because application of a template would obscure the
    // intended meaning.
    pub is_template_compatible: bool,

    // The font size [18-45] or None to use the default font size set by the filter generator.
    pub font_size: Option<FontSize>,

    // The text color for the drop.
    pub text_color: StockColor,

    // The border color for the drop.
    pub border_color: StockColor,

    // The play effect color for the drop or None if the drop lacks a play effect.
    pub play_effect_color: Option<StockColor>,

    // The name of RGBA color for the background.
    pub background_color: String,

    // The minimap icon associated with the drop, or None, if the drop has no minimap icon.
    pub icon: Option<Icon>,

    // The sound associated with the drop, or None, if the drop has no sound.
    pub sound: Option<Sound>,
}

impl ActionSetsRow {
    pub fn builder(action_set: String, stock_color: StockColor, rgb_color: String, icon: Option<Icon>, sound: Option<Sound>) -> ActionSetsRowBuilder {
        ActionSetsRowBuilder {
            action_set,
            is_template_compatible: true,
            font_size: None,
            text_color: stock_color,
            border_color: stock_color,
            play_effect_color: None,
            background_color: rgb_color,
            icon,
            sound,
        }
    }
}

// Builder for ActionSetsRow.
#[derive(Debug)]
pub struct ActionSetsRowBuilder {
    action_set: String,

    is_template_compatible: bool,

    font_size: Option<FontSize>,

    text_color: StockColor,

    border_color: StockColor,

    play_effect_color: Option<StockColor>,

    background_color: String,

    icon: Option<Icon>,

    sound: Option<Sound>,
}

impl ActionSetsRowBuilder {
    pub fn action_set(mut self, action_set: String) -> ActionSetsRowBuilder {
        self.action_set = action_set;
        self
    }

    pub fn background_color(mut self, background_color: String) -> ActionSetsRowBuilder {
        self.background_color = background_color;
        self
    }

    pub fn border_color(mut self, border_color: StockColor) -> ActionSetsRowBuilder {
        self.border_color = border_color;
        self
    }

    pub fn build(self) -> ActionSetsRow {
        ActionSetsRow {
            action_set: self.action_set,
            is_template_compatible: self.is_template_compatible,
            font_size: self.font_size,
            text_color: self.text_color,
            border_color: self.border_color,
            play_effect_color: self.play_effect_color,
            background_color: self.background_color,
            icon: self.icon,
            sound: self.sound,
        }
    }

    pub fn font_size(mut self, font_size: Option<FontSize>) -> ActionSetsRowBuilder {
        self.font_size = font_size;
        self
    }

    pub fn icon(mut self, icon: Option<Icon>) -> ActionSetsRowBuilder {
        self.icon = icon;
        self
    }

    pub fn is_template_compatible(mut self, is_template_compatible: bool) -> ActionSetsRowBuilder {
        self.is_template_compatible = is_template_compatible;
        self
    }

    pub fn play_effect_color(mut self, play_effect_color: Option<StockColor>) -> ActionSetsRowBuilder {
        self.play_effect_color = play_effect_color;
        self
    }

    pub fn sound(mut self, sound: Option<Sound>) -> ActionSetsRowBuilder {
        self.sound = sound;
        self
    }
}

pub struct ActionSetsTable {
    pub name: String,
}

impl_generic_table!(ActionSets);

impl Table for ActionSetsTable {
    fn new() -> Self {
        Self {
            name: "action_sets".to_string(),
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::icon_shape::IconShape;
    use crate::types::icon_size::IconSize;
    use crate::types::sound_volume::SoundVolume;

    #[test]
    fn test_builder_works() {
        let action_set = "Quest Items";
        let stock_color_green = StockColor::Green;
        let rgb_color_green = "Green";

        let cross_shape = IconShape::Cross;
        let icon_size_large = IconSize::new(0).unwrap();
        let icon_for_quests = Icon::from_sql(Some(cross_shape.to_string()), Some(icon_size_large), Some(stock_color_green.to_string()))
            .unwrap()
            .unwrap();

        let quest_sound_string = "Quest";
        let quest_sound_volume = SoundVolume::new(300).unwrap();
        let quest_sound = Sound::from_sql(Some(quest_sound_volume), None, Some(quest_sound_string.to_string()))
            .unwrap()
            .unwrap();

        let row = ActionSetsRow::builder(
            action_set.to_string(),
            stock_color_green,
            rgb_color_green.to_string(),
            Some(icon_for_quests),
            Some(quest_sound.clone()),
        )
            .is_template_compatible(false)
            .play_effect_color(Some(stock_color_green))
            .build();
        assert_eq!(action_set, row.action_set);
        assert_eq!(false, row.is_template_compatible);
        assert_eq!(stock_color_green, row.text_color);
        assert_eq!(stock_color_green, row.border_color);
        assert_eq!(Some(stock_color_green), row.play_effect_color);
        assert_eq!(rgb_color_green, row.background_color);
        assert_eq!(Some(icon_for_quests), row.icon);
        assert_eq!(Some(quest_sound), row.sound);
    }
}
