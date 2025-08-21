// SPDX-FileCopyrightText: Copyright (c) 2025 xtt28 and savant contributors
// SPDX-License-Identifier: GPL-3.0-or-later
use std::fs::File;

use anyhow::{Context, Result};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct FlashCardSet {
    pub flash_cards: Vec<FlashCard>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct FlashCard {
    pub front: String,
    pub back: String,
}

impl FlashCardSet {
    pub fn write_csv_to_file(&self, path: &std::path::PathBuf) -> Result<()> {
        let file =
            File::create(path).with_context(|| format!("could not create file {:?}", path))?;
        let mut wtr = csv::Writer::from_writer(file);
        for card in &self.flash_cards {
            wtr.write_record([&card.front, &card.back])
                .with_context(|| format!("could not write card {:?} to file {:?}", &card, &path))?;
        }
        wtr.flush()
            .with_context(|| format!("could not flush writer for {:?}", path))?;

        Ok(())
    }
}
