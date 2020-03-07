# wcurses

<a href="https://docs.rs/wcurses"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

![wcurses example](loop.gif)

```toml
[dependencies]
wcurses = "0"
```

Want to make your wasi terminal apps look a bit cooler? Trying to move the terminal cursor around for a text game? want to clear the screen? This lib might be for you.  It was made with [http://webassembly.sh/](http://webassembly.sh/) in mind, but it should work with wasmer with a comand like the following.

```bash
wasmer my_app.wasm --env LINES=20 --env COLUMNS=80
```

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `wcurses` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
