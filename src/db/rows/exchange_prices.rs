// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/20/2026.

use crate::types::price::Price;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

// The exchange_prices table contains estimated prices for items available on the in-game auction house.  Since
// GGG does not, at present, have an API for the auction house, this table is created manually.  The
// exchange_prices table is updated infrequently; thus in-game prices are likely to differ from values in this
// table.  The filter generator uses this table to filter items available for trade on the auction house using
// estimated price thresholds.
#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangePricesRow {
    // The name of the base type.
    pub base_type: String,

    // The price of the item in units of the game variant's basis currency (chaos orbs for POE 1; exalted orbs for POE 2).
    pub price: Price,
}

impl Eq for ExchangePricesRow {}

impl Ord for ExchangePricesRow {
    fn cmp(&self, other: &ExchangePricesRow) -> Ordering {
        self.base_type.cmp(&other.base_type)
    }
}

impl PartialEq for ExchangePricesRow {
    fn eq(&self, other: &ExchangePricesRow) -> bool {
        self.base_type == other.base_type
    }
}

impl PartialOrd for ExchangePricesRow {
    fn partial_cmp(&self, other: &ExchangePricesRow) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
