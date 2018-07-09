glidertracker-status
==============================================================================

[GliderTracker] websocket server status report for the command line

[GliderTracker]: http://glidertracker.org/


Installation
------------------------------------------------------------------------------

Make sure to have [cargo] installed (e.g. via [rustup]), then:

```
cargo install glidertracker-status
```

[cargo]: https://github.com/rust-lang/cargo
[rustup]: https://rustup.rs/


Usage
------------------------------------------------------------------------------

Running `glidertracker-status` will result in something like this:

```json
{
  "connected": true,
  "state": {
    "users": 248,
    "max_users": 500,
    "load": 4.52,
  }
}
```


License
------------------------------------------------------------------------------

This project is licensed under either of

 - Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   <http://www.apache.org/licenses/LICENSE-2.0>)
   
 - MIT license ([LICENSE-MIT](LICENSE-MIT) or
   <http://opensource.org/licenses/MIT>)

at your option.
