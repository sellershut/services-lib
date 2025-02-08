<div align="center">
  <img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/sellershut/services-lib/check.yaml?label=build">
  
  <a href="https://codecov.io/gh/sellershut/services-lib" > 
 <img src="https://codecov.io/gh/sellershut/services-lib/graph/badge.svg?token=AxLifNA07h"/> 
 </a>
</div>
<h1 align="center">sellershut-infrastructure</h1>
<p align="center">
Configure the infrastructure used by the sellershut platform
<br />

## Features

- `tracing` - Structured, context-aware logging using [tracing](https://docs.rs/tracing)
- `postgres` - Configure a postgres database with [sqlx](https://docs.rs/sqlx)

## Examples

Check out the [examples](./examples/) directory for how to setup the services in this crate

```sh
cargo run --example tracing -F tracing
```
> [!NOTE]
> You need to activate each example's required features
