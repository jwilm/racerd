racerd
======

JSON/HTTP API for [racer][]. The project's primary goal is supporting Rust in
[YouCompleteMe][].

[Documentation][]

## Status

```rust
fn is_it_ready() -> bool {
    false
}
```

## ycmd protocol notes

_ycmd_ has an example client which logs HTTP requests and responses. The
(slightly annotated with markdown fences) output of the client is in
[YCMPROTO.md][]. The completions and events APIs should be instructive for
designing a racer service.

[YouCompleteMe]: https://github.com/Valloric/YouCompleteMe
[YCMPROTO.md]: YCMPROTO.md
[racer]: https://github.com/phildawes/racer

## Building

Requires nightly rust to satisfy racer

### OSX Issues

If errors are encountered building openssl or openssl-sys on MacOSX, that
may be because you are running version 10.11 which dropped the system
provided OpenSSL. The following commands should fix that issue.

```sh
# Will update to latest openssl available through homebrew
brew install openssl

# Set up symlinks for binaries, includes
brew link --force openssl
```

[Documentation]: http://jwilm.github.io/racerd/libracerd/
