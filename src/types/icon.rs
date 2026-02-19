// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/4/2026.

use crate::types::icon_shape::IconShape;
use crate::types::icon_size::IconSize;
use crate::types::stock_color::StockColor;
use crate::util::error::FromSqlError;
use anyhow::Result;
use rusqlite::Error as RusqliteError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Icon {
    // The shape of the minimap icon
    pub shape: IconShape,

    // The size of the minimap icon [0-2].  Note that 0 is the largest size.
    pub size: IconSize,

    // The stock color used for the minimap icon.
    pub color: StockColor,
}

impl Icon {
    // Constructs an Icon using arguments as stored in the database.
    pub fn from_sql(shape: Option<String>, size: Option<IconSize>, color: Option<String>) -> Result<Option<Self>, RusqliteError> {
        if shape.is_none() {
            if size.is_some() || color.is_some() {
                return Err(FromSqlError::Icon("shape is None, but size and/or color is Some".to_string()).into());
            }
            Ok(None)
        } else {
            if size.is_none() || color.is_none() {
                return Err(FromSqlError::Icon("shape is Some, but size and/or color is None".to_string()).into());
            }
            Ok(Some(Icon {
                shape: IconShape::from_str(&shape.unwrap())?,
                size: size.unwrap(),
                color: StockColor::from_str(&color.unwrap())?,
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_fails_with_invalid_parameters() {
        let square_shape = IconShape::Square;
        let icon_size = IconSize::new(0).unwrap();
        let color_red = StockColor::Red;

        // Shape is None, but color is Some
        let result = Icon::from_sql(None, None, Some(color_red.to_string()));
        assert!(matches!(result, Err(RusqliteError::FromSqlConversionFailure(..))));

        // Shape is None, but size is Some
        let result = Icon::from_sql(None, Some(icon_size), None);
        assert!(matches!(result, Err(RusqliteError::FromSqlConversionFailure(..))));

        // Shape is Some, but color is None
        let result = Icon::from_sql(Some(square_shape.to_string()), Some(icon_size), None);
        assert!(matches!(result, Err(RusqliteError::FromSqlConversionFailure(..))));

        // Shape is Some, but size is None
        let result = Icon::from_sql(Some(square_shape.to_string()), None, Some(color_red.to_string()));
        assert!(matches!(result, Err(RusqliteError::FromSqlConversionFailure(..))));
    }

    #[test]
    fn new_works_with_valid_parameters() {
        let opt_icon = Icon::from_sql(None, None, None).unwrap();
        assert!(opt_icon.is_none());

        let star_shape = IconShape::Star;
        let icon_size = IconSize::new(2).unwrap();
        let color_green = StockColor::Green;
        let opt_icon = Icon::from_sql(Some(star_shape.to_string()), Some(icon_size), Some(color_green.to_string())).unwrap();
        assert!(opt_icon.is_some());
        let icon = opt_icon.unwrap();
        assert_eq!(star_shape, icon.shape);
        assert_eq!(icon_size, icon.size);
        assert_eq!(color_green, icon.color);
    }
}
