# vetis-wt

[![Crates.io downloads](https://img.shields.io/crates/d/vetis-wt)](https://crates.io/crates/vetis-wt) [![crates.io](https://img.shields.io/crates/v/vetis-wt?style=flat-square)](https://crates.io/crates/vetis-wt) [![Build Status](https://github.com/vetis-server/vetis-wt/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/vetis-server/vetis-wt/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/vetis-wt) [![Documentation](https://docs.rs/vetis-wt/badge.svg)](https://docs.rs/vetis-wt/latest/vetis-wt) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/vetis-server/vetis-wt/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/vetis-server/vetis-wt/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/vetis-server/vetis-wt)

vetis-wt adds webtransport support to your vetis server!

## Features

- Protocol version 13

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
vetis-wt = "0.1.0"
```

## Configuration

Below you can find instruction on how to configure many logging options availble
on vetis.

### stdout

#### Using API

```rust,ignore

```

#### vetis.yaml

There's no plans to provide this kind of configuration method.

#### Configuration Options

- **log_level**: Log level: TRACE, WARN, DEBUG, INFO, ERROR and FATAL

### Runtime Features

Choose the appropriate runtime feature for your project:

- `runtime-tokio` (default): Use with Tokio runtime

## License

Licensed under either of

- Apache License, Version 2.0
  (LICENSE-APACHE or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  (LICENSE-MIT or <https://opensource.org/licenses/MIT>)

at your option.

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
