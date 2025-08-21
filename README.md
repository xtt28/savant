# Welcome to Savant's source repository.

Savant is a CLI tool that lets you generate flash cards from PDF files using a
large language model.

## Usage

1. Set the OPENAI_API_KEY environment variable, or refer to "Non-OpenAI
   providers" section below if not using the OpenAI API.
2. Compile a binary (`cargo build --release`)
3. 

### Non-OpenAI providers

Savant uses the OpenAI API scheme.

If your AI model provider is not OpenAI, but they provide a compatibility layer
with the OpenAI API, then you can use this compatibility layer with Savant.
Simply specify the `base-url` and `model` command-line options at runtime.

## License

This project is licensed under:

    SPDX-License-Identifier: GPL-3.0-or-later

being in concordance with the terms in the LICENSE file in the root of this
repository.