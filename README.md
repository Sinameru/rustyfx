# RustyFX

![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)

RustyFX is a comprehensive Rust ecosystem for financial applications, providing tools for fiat and cryptocurrency conversion, with the flexibility to expand into trading strategies and other financial modules.

## Ecosystem Structure

RustyFX is modular, allowing you to import only what you need:

- **`rustyfx-core`** – Core types, structs, and traits for currencies and assets.  
- **`rustyfx-convert`** – Utilities for currency and crypto conversion.  
- **`rustyfx-crypto`** – Crypto-specific functions and helpers.  

The main `rustyfx` crate serves as a convenient wrapper around all subcrates.

## Installation

Add the wrapper crate to your `Cargo.toml`:

```toml
rustyfx = "0.1.1"
