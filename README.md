# mdbook-llms-txt

A tool to convert mdbook documents to llmstxt.org format.

## Overview

This tool is a renderer that converts documents created with [mdbook](https://rust-lang.github.io/mdBook/) into [llmstxt.org](https://llmstxt.org/) format. This allows you to share technical documentation created with mdbook in a format optimized for LLMs.

## Installation

```bash
cargo install mdbook-llms-txt-tools
```

## Usage

1. Add the following configuration to your project's `book.toml`:

```toml
# Basic llmstxt.org format output
[output.llms-txt]

# Detailed llmstxt.org format output with additional information
[output.llms-full-txt]
```

2. Run `mdbook build` as usual, and files will be generated in your chosen output format.

## Output Formats

- `llms-txt`: Generates basic llmstxt.org format output. Suitable for general use.
- `llms-full-txt`: Generates llmstxt.org format output with more detailed information. Use this when more advanced analysis or processing is needed.

## License

MIT

## Contributing

Please report bugs and feature requests on [GitHub Issues](https://github.com/higumachan/mdbook-llms-txt/issues).
Pull requests are welcome. 