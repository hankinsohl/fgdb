// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/20/2026.

use crate::types::liquidity::Liquidity;
use crate::types::stack_size::StackSize;
use serde::{Deserialize, Serialize};
use url::Url;

// The base_types tables lists all base types in the game.
#[derive(Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BaseTypesRow {
    // The name of the base type.
    pub base_type: String,

    // The name of the class associated with the base type.  If no class is associated with the base type, "None" is
    // used.
    pub class: String,

    // The maximum number of items of this base type that can drop in a single stack.
    pub stack_size: StackSize,

    // Liquidity is a measure of how easy it is to trade the base type.  Liquidity is one of: Exchange, Async or
    // Untradable.
    //     Exchange means that items associated with the base type are tradable on the currency exchange.
    //     Async means that items associated with the base type are available for trade using the asynchronous trade
    //         system, but are not available on the auction house.
    //     Untradable means that items associated with the base type cannot be traded (e.g., quest items).
    pub liquidity: Liquidity,

    // Image URL obtained via the trade API to a PNG for the base type.  The image is used to determine the background
    // color for the item.  If the trade API does not provide an image URL for the base type, url is null.
    pub url: Option<Url>,
}
