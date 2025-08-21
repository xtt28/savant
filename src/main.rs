// SPDX-FileCopyrightText: Copyright (c) 2025 xtt28 and savant contributors
// SPDX-License-Identifier: GPL-3.0-or-later
mod args;
mod model;
mod output;

use std::fs;

use anyhow::{Context, Result};
use clap::Parser;
use tysm::chat_completions::ChatClient;

use crate::args::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let args = Cli::parse();
    let client = ChatClient::new(
        args.openai_api_key.clone().unwrap_or_default(),
        args.model.clone(),
    )
    .with_url(args.base_url.clone());

    let sys_prompt = model::system_prompt(&args);

    let file_bytes =
        fs::read(&args.input).with_context(|| format!("could not read file {:?}", &args.input))?;
    let file_text = pdf_extract::extract_text_from_mem(&file_bytes)
        .with_context(|| format!("could not read PDF text from file {:?}", &args.input))?;

    let card_set = model::generate_guide(client, sys_prompt, file_text)
        .await
        .with_context(|| "could not generate chat completion")?;

    card_set
        .write_csv_to_file(&args.output)
        .with_context(|| format!("could not write to file {:?}", &args.output))?;

    println!("{:?}", card_set);
    Ok(())
}
