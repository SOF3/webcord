# webcord
[![Travis-CI](https://travis-ci.com/SOF3/webcord.svg?branch=master)](https://travis-ci.om/SOF3/webcord)
[![crates.io](https://img.shields.io/crates/v/webcord.svg)](https://crates.io/crates/webcord)
[![crates.io](https://img.shields.io/crates/d/webcord.svg)](https://crates.io/crates/webcord)
[![docs.rs](https://docs.rs/webcord/badge.svg)](https://docs.rs/webcord)
[![GitHub](https://img.shields.io/github/stars/SOF3/webcord?style=social)](https://github.com/SOF3/webcord)

Mirrors Discord chat logs on webpages in a searchable fashion

## Deployment
Docker is used for deployment. First build the docker image:

```
docker build -t webcord .
```

Then setup the config file based on the schema defined in [src/secrets.rs](src/secrets.rs) in JSON, YAML or TOML format.

Then the docker image can be run using this config file:

```
docker run -it -v $(pwd)/config.toml:/webcord/config.toml -e RUST_LOG=info -p 0.0.0.0:80:$PORT/tcp webcord
```

where `$PORT` is the port you set up in `config.toml`.

## Development
NodeJS and Rust are used for development.

For developing the client, node 10.17.x is used, along with tools as dev-dependencies in `client/package.json`.

To compile the source TypeScript into output (stored in `build/main.js`), run `just js`.

To compile the source SASS into output (stored in `build/style.css`), run `just css`.
Since CSS minification is sensitive to classes used in the script, the CSS build depends on the JS build.

For developing the server, rust nightly 1.41+ is required. Run `just rust`.
Note that the `CARGO_FLAGS` environment variable must be set (can be an empty string) to specify extra flags (such as `--release`) to cargo.
