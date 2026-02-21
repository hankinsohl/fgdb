// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/20/2026.

use crate::types::gem_level::GemLevel;
use crate::types::item_level::ItemLevel;
use crate::types::price::Price;
use crate::types::rarity::Rarity;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

// async_prices
//     The async_prices table contains a list of in-game prices for items available for instant buyout or in-person
//     trades.  It does not contain prices for items available on the auction house; these prices are recorded in the
//     exchange_prices table.
// async_prices::async_price_key
//     Trade_price_key is a composite formed as follows:
//         <base_type>::<item>::<minimum_item_level>::<gem_level>::<rarity>
//      If one of the key parts is null, the text "null" is used when forming the composite.
#[derive(Debug, Deserialize, Serialize)]
pub struct AsyncPricesRow {
    // The name of the base type.
    pub base_type: String,

    // The name of the item or None if not applicable.
    pub item: Option<String>,

    // The minimum item level associated with the price, or None if not applicable.
    pub minimum_item_level: Option<ItemLevel>,

    // The gem level associated with the price, or None if not applicable.
    pub gem_level: Option<GemLevel>,

    // The rarity associated with the price, or None if not applicable
    pub rarity: Option<Rarity>,

    // The price of the item in units of the game variant's basis currency (chaos orbs for POE 1; exalted orbs for POE 2).
    pub price: Price,
}

impl Eq for AsyncPricesRow {}

impl Ord for AsyncPricesRow {
    fn cmp(&self, other: &AsyncPricesRow) -> Ordering {
        self.gen_key().cmp(&other.gen_key())
    }
}

impl PartialEq for AsyncPricesRow {
    fn eq(&self, other: &AsyncPricesRow) -> bool {
        self.gen_key() == other.gen_key()
    }
}

impl PartialOrd for AsyncPricesRow {
    fn partial_cmp(&self, other: &AsyncPricesRow) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl AsyncPricesRow {
    pub fn gen_key_from_parts(
        base_type: &String,
        item: &Option<String>,
        minimum_item_level: &Option<ItemLevel>,
        gem_level: &Option<GemLevel>,
        rarity: &Option<Rarity>,
    ) -> String {
        format!(
            "{}::{}::{}::{}::{}",
            base_type,
            item.as_ref().unwrap_or(&"null".to_string()),
            minimum_item_level.map(|l| l.to_string()).unwrap_or("null".to_string()),
            gem_level.map(|l| l.to_string()).unwrap_or("null".to_string()),
            rarity.map(|r| r.to_string()).unwrap_or("null".to_string())
        )
    }

    pub fn gen_key(&self) -> String {
        AsyncPricesRow::gen_key_from_parts(&self.base_type, &self.item, &self.minimum_item_level, &self.gem_level, &self.rarity)
    }
}
