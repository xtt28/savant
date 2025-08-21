# Welcome to Savant's source repository.

![Crates.io Version](https://img.shields.io/crates/v/savant)
![GitHub License](https://img.shields.io/github/license/xtt28/savant)


Savant is a CLI tool that lets you generate flash cards from PDF files using a
large language model. It supports all language model providers that are
compatible with the OpenAI API and support structured outputs.

## Usage

### Build yourself

The Rust toolchain must be installed on your machine.

Run
```shell
git clone https://github.com/xtt28/savant.git
```
to clone the Git repository onto your machine.

Then run
```shell
cargo build --release
```
to compile a binary of the program. The binary will appear at
`target/release/savant`.

### From Cargo

Install with Cargo:
```shell
cargo install savant
```

### Usage

1. Set the OPENAI_API_KEY environment variable, or refer to "Non-OpenAI
   providers" section below if not using the OpenAI API.
2. Run `savant --help` to view all available command-line options.

### Non-OpenAI providers

Savant uses the OpenAI API scheme.

If your AI model provider is not OpenAI, but they provide a compatibility layer
with the OpenAI API, then you can use this compatibility layer with Savant.
Simply specify the `base-url` and `model` command-line options at runtime.

Your AI model provider must support structured responses. If it doesn't, you can
change `SYSTEM_PROMPT_TEMPLATE` in [model.rs](src/model.rs) to instruct the LLM
to output only valid JSON, although your mileage may vary.

## License

This project is licensed under:

    SPDX-License-Identifier: GPL-3.0-or-later

being in concordance with the terms in the LICENSE file in the root of this
repository.