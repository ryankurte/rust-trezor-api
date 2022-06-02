
# Rust Trezor API

A fork of a [fork](https://github.com/joshieDo/rust-trezor-api) of a [fork](https://github.com/romanz/rust-trezor-api) of a [library](https://github.com/stevenroose/rust-trezor-api), which provides a way to communicate with a Trezor device from a rust project. Prior iterations have typically been focused on supporting a specific coin, this is intended to separate coin implementations from client logic to improve maintainability going forward.

## Status

[![GitHub tag](https://img.shields.io/github/tag/ryankurte/rust-trezor-apio.svg)](https://github.com/ryankurte/rust-trezor-api)
![Github Actions](https://github.com/ryankurte/rust-trezor-api/workflows/rust/badge.svg)

**This is a work in progress, expect nothing (yet) but feel free to contribute if you're interested!**


### Components

- [trezor-client](./client) [[docs][client_docs]] - A client library for interacting with trezor devices, supports core interactions and used as a basis for per-coin subcommands.

- [trezor-protos](./proto) [[docs][proto_docs]] - Trezor protocol, generated from [trezor-common/protos](https://github.com/trezor/trezor-common) via `prost`. Shared by client and coin implementations.

- [trezor-cli](./cli) - CLI based on `trezor-client`, provides core trezor device interactions (setting pin, etc.)

- [coins/*](./coins) - Per-coin applets, implemented as standalone subcommands to `trezor-cli`.


## Usage


**TODO**

## Contributing

If you'd like to contribute please feel free to open an [issue][new_issue] or pr.


## Credits
* [TREZOR](https://github.com/trezor/trezor-firmware) 
* [stevenroose](https://github.com/stevenroose)
* [romanz](https://github.com/romanz)
* [joshieDo](https://github.com/joshieDo)


[new_issue]: https://github.com/ryankurte/rust-trezor-api/issues/new
[client_docs]: https://ryan.kurte.nz/rust-trezor-api/trezor_client/
[proto_docs]: https://ryan.kurte.nz/rust-trezor-api/trezor_protos/
