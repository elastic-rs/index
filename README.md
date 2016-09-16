# Elastic

Platform  | Channel | Status
------------- | ------------- | -------------
Linux / OSX  | Stable / Nightly | [![Build Status](https://travis-ci.org/KodrAus/elasticsearch-rs.svg?branch=master)](https://travis-ci.org/KodrAus/elasticsearch-rs)
Windows  | Nightly | [![Build status](https://ci.appveyor.com/api/projects/status/s0yo6i7sr4kc5sd5?svg=true)](https://ci.appveyor.com/project/KodrAus/elasticsearch-rs)

`elastic_*` is an ecosystem of crates for interacting with Elasticsearch from Rust.
The API is targeting the `5.x` branch of Elasticsearch, which is currently in alpha.
This means the library is in a fairly inconsistent state, but will be stabilised along with Elasticsearch.

Crate functionality covers:

- [transport](#elastic_hyper)
- [type mapping](#elastic_types)
- [codegen](#elastic_codegen)

Quick reference:

- [crates](#crates)
- [samples](https://github.com/KodrAus/elasticsearch-rs/tree/master/hyper/samples)
- [benchmarks](https://github.com/KodrAus/elasticsearch-rs/tree/master/benches)
- [fluff](#design)

## Crates

### [`elastic_hyper`](http://kodraus.github.io/rustdoc/elastic_hyper/)

[![Latest Version](https://img.shields.io/crates/v/elastic_hyper.svg)](https://crates.io/crates/elastic_hyper)

[Docs](http://kodraus.github.io/rustdoc/elastic_hyper/) |
[Issues](https://github.com/KodrAus/elasticsearch-rs/labels/hyper) |
[Samples](https://github.com/KodrAus/elasticsearch-rs/tree/master/hyper/samples)

Provides a synchronous [`hyper`](https://github.com/hyperium/hyper) implementation of the Elasticsearch REST API. The `hyper` client is simple to use; there's basically no setup needed besides creating a `hyper::Client` object to use for requests. The `hyper` client is general-purpose, and suitable for any scenario where on-demand requests are sufficient.

If you'd prefer to call Elasticsearch using a Query DSL builder, see [`rs-es`](https://github.com/benashford/rs-es).

#### Example

The `elastic_hyper` client is a thin layer over `hyper`; it just maps functions to routes. It's up to the caller to serialise and deserialise HTTP content.
- For query serialisation, the [`json_str`](http://kodraus.github.io/rustdoc/json_str/) crate provides the `json_str!` macro for serialising abitrary rust-like code to json.
- For type serialisation / deserialisation, see [`elastic_types`](http://kodraus.github.io/rustdoc/elastic_types/).

Currently targeting the `master` Elasticsearch branch, aiming for `5.x`.
This will be stabilised through features in the future.

Add `elastic_hyper` and `json_str` to your `Cargo.toml`:

```
[dependencies]
elastic_hyper = "*"
json_str = "*"
```

Ping the availability of your cluster:

```rust
#[macro_use]
extern crate json_str;
extern crate elastic_hyper as elastic;

let (mut client, params) = elastic::default();

elastic::ping::head(&mut client, &params).unwrap();
```

A simple `query_string` query:

```rust
#[macro_use]
extern crate json_str;
extern crate elastic_hyper as elastic;

let (mut client, params) = elastic::default();

let response = elastic::search::post(
	&mut client, &params,
	&json_str!({
		query: {
			query_string: {
				query: "*"
			}
		}
	})
).unwrap();
```

### [`elastic_types`](https://github.com/elastic-rs/elastic-types)

`elastic_types` is a library for building Elasticsearch types in Rust. Define your Elasticsearch types as PORS (Plain Old Rust Structures) and generate an equivalent Elasticsearch mapping from them, where correctness is enforced by Rust's type system.

This crate lives in the [`elastic_types` repo](https://github.com/elastic-rs/elastic-types).

### `elastic_rotor`

[Issues](https://github.com/KodrAus/elasticsearch-rs/labels/rotor)

_In Progress_

Will provide an asynchronous [rotor-http](https://github.com/tailhook/rotor-http) implementation of the Elasticsearch REST API. This client is an active work in progress, as is `rotor` itself so things will change a lot. It'll be best suited to streaming scenarios, or where Elasticsearch connections will be used heavily.

The crate will allow you to use connections in two ways; add connections as state machines to your own `mio` loop, or use an out-of-the-box connection pool.

### `elastic_codegen`

[Docs](http://kodraus.github.io/rustdoc/elastic_codegen/) |
[Issues](https://github.com/KodrAus/elasticsearch-rs/labels/codegen)

Provides code generation for the Elasticsearch REST API from the official [spec](https://github.com/elastic/elasticsearch/tree/master/rest-api-spec) and generic helpers for rust source and integration tests. The goal is to keep this package fairly agnostic, so the same `ast` can be used to generate other kinds of output.

Right now, it's used by `elastic_hyper` to build the client, but could also be used to generate other implementations, like `elastic_rotor` for an asynchronous client.

## Goals

To provide a strongly-typed, full-featured and efficient Elasticsearch client for Rust over (eventually) asynchronous io. Rust gives us a lot of tools for building super-performant but highly accessible libraries, which we aim to continue.

The REST API is provided by an [inline JSON macro](http://kodraus.github.io/rustdoc/json_str/#json-parsing) so it's efficient and always in line with whatever version of Elasticsearch you're targeting.

This means you don't need to learn another API for interacting with Elasticsearch; queries mocked in [Sense](https://www.elastic.co/blog/found-sense-a-cool-json-aware-interface-to-elasticsearch) can literally just be copy+pasted into your Rust code.

The core focus of this project is on strong typing over the core types and responses in Elasticsearch, rather than trying to map the entire Query DSL.

Support for Elastic's plugin products, like `watcher` and `graph` could be added as feature-gated modules in the `elastic_hyper` and `elastic_rotor` clients and `elastic_types` as necessary.

## Roadmap

See [milestones](https://github.com/KodrAus/elasticsearch-rs/milestones).

- [x] Implement core Elasticsearch types
- [x] Implement Elasticsearch response types (interop with `rs-es`)
- [x] Proper type mapping compatibility with ES 5.x. See [here](https://github.com/KodrAus/elasticsearch-rs/issues/121)
- [ ] Rotor Client
- [x] Codegen API endpoints
- [x] Hyper Client
- [x] Doc APIs
- [x] Query DSL proof-of-concept to test design
