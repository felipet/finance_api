# Finance Library API definitions

> [!Warning]
> This library is in an early stage. The APIs are not frozen yet, and they might
> suffer considerable changes.

This Rust library crate defines several `traits` that describe the expected features
of the finance objects included in the Finance library. Most of the code within this
repository is included as a facade that other modules must implement to provide the
features described in the documentation.

## Current implementations of the Finance Library

- [Ibex Finance][ibexfinance] is an implementation of the Finance Library for Ibex
  indexes and companies.

[ibexfinance]: https://github.com/felipet/finance_ibex