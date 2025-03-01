# Lambda Function News Project

[![Lint](https://github.com/actualwitch/news/actions/workflows/lint.yml/badge.svg)](https://github.com/actualwitch/news/actions/workflows/lint.yml)

This is a WIP source code for a collaborative link aggregator similar to [lobste.rs](https://lobste.rs/), but actually written in Rust.

## Development

This project includes [Tilt](https://tilt.dev/) configuration for local development with auxillary services run in kubernetes, however you will need to have Rust and `cargo-leptos` installed locally. To start the development environment, run:

```sh
tilt up
```

## Roadmap

- [ ] Read-only links with imaginary discussions
- [x] Postgres based data store
- [ ] Kubernetes deployment
- [ ] ActivityPub/RSS read-only feed
- [ ] ActivityPub interaction support

