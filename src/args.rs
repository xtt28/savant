// SPDX-FileCopyrightText: Copyright (c) 2025 xtt28 and savant contributors
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    /// The name of the base URL to use for completions. The API at the URL provided must be
    /// compatible with the OpenAI API.
    #[arg(short, long, default_value_t = String::from("https://api.openai.com/v1"))]
    pub base_url: String,
    /// The name of the model to use. See https://platform.openai.com/docs/models for a list of
    /// acceptable model identifiers if using the OpenAI API.   
    #[arg(short, long, default_value_t = String::from("gpt-5-nano"))]
    pub model: String,
    /// The OpenAI API key to use.   
    #[arg(long)]
    pub openai_api_key: Option<String>,
    /// The path to the PDF file to source content from.
    #[arg(short, long)]
    pub input: std::path::PathBuf,
    /// The path to which the output CSV will be written.
    #[arg(short, long)]
    pub output: std::path::PathBuf,
    /// The maximum amount of cards to be generated.
    #[arg(short, long, default_value_t = 50)]
    pub card_limit: u32,
}
