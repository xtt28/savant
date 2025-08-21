// SPDX-FileCopyrightText: Copyright (c) 2025 xtt28 and savant contributors
// SPDX-License-Identifier: GPL-3.0-or-later
use tysm::chat_completions::{ChatClient, ChatError};

use crate::{args::Cli, output::FlashCardSet};

const SYSTEM_PROMPT_TEMPLATE: &str = "You are an AI study guide. You will be given the text \
    extracted from a PDF file. With this text, you must generate no more than {max} flashcards. \
    The flashcards should comprehensively cover the most important points of the text. The \
    front of each card should consist of a term or phrase, and the back of each card should \
    consist of a definition or description.";

pub fn system_prompt(config: &Cli) -> String {
    SYSTEM_PROMPT_TEMPLATE
        .replace("{max}", &config.card_limit.to_string())
        .to_string()
}

pub async fn generate_guide(
    client: ChatClient,
    sys_prompt: String,
    text: String,
) -> Result<FlashCardSet, ChatError> {
    let card_set: FlashCardSet = client.chat_with_system_prompt(sys_prompt, text).await?;

    Ok(card_set)
}
