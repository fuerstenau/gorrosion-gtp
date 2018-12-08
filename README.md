# Gorrosion

[![Build status][travis-badge]][travis-link]
[![Coverage status][coveralls-badge]][coveralls-link]
[![Crates.io][crates.io-badge]][crates.io-link]
[![Documentation][docs.rs-badge]][docs.rs-link]

## What is this?

An interface between the [Go Text Protocoll (GTP)][gtp-link]
and the language Rust,
in particular its type system.
This library tries its best to offer a low-effort way
of interacting with other GTP speakers through Rust
without having to worry too much about types
while still getting as much support from the type system as possible.

## How does it work?

I'm not sure it *does*.
Trying to make up for the lack of higher types with enums
is an interesting experience,
to say the least.

## What are the alternatives?

Searching for “Go” on crates.io gives roughly a bazillion hits,
most of them not about the game.
So far, I was able to unearth only one crate with GTP support:
* [libgo](https://crates.io/crates/libgo)

Its last update is over a year old
and I fail to understand the philosophy behind the interface.
This crate aims to surpass this competion
in terms of both maintainedness and documentation quality.

## Anything else you want to say?

* This is a spin-off of [Gorrosion][gorrosion], another Go themed crate.
* One day in the future I ~might~ will write some useful documentation.
Until then, feel free to e-mail me with questions.
(Well, after then, too, but not before consulting the docs.)

[travis-link]: https://travis-ci.org/fuerstenau/gorrosion-gtp
[travis-badge]: https://api.travis-ci.org/fuerstenau/gorrosion-gtp.svg?branch=master
[coveralls-link]: https://coveralls.io/github/fuerstenau/gorrosion-gtp?branch=master
[coveralls-badge]: https://coveralls.io/repos/github/fuerstenau/gorrosion-gtp/badge.svg?branch=master
[crates.io-link]: https://crates.io/crates/gorrosion-gtp
[crates.io-badge]: https://img.shields.io/crates/v/gorrosion-gtp.svg
[docs.rs-link]: https://docs.rs/gorrosion-gtp
[docs.rs-badge]: https://docs.rs/gorrosion/badge-gtp.svg
[gtp-link]: http://www.lysator.liu.se/~gunnar/gtp/
[gorrosion]: https://crates.ion/crates/gorrosion
