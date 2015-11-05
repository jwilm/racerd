racerd
======

JSON/HTTP API for [racer][]. The project's primary goal is supporting Rust in
[YouCompleteMe][].

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
