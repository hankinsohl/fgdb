// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 2/12/2026.

pub enum Policy {
    // Skip the update
    Skip,

    // Automatically update out-of-date data
    Auto,

    // Remove all existing data and replace with up-to-date data
    Force,
}
