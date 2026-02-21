// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/15/2026.

#[cfg(feature = "std")]
pub mod consts;
#[cfg(feature = "std")]
pub mod env;
#[cfg(any(feature = "std", feature = "errors"))]
pub mod error;
