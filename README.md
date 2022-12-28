# Runtime Programming in Rust

Let's learn runtime programming with [WebAssembly]
in Rust, by following the wonderful book,
[Programming WebAssembly with Rust], by [Kevin Hoffman].

**You take the red pill -- you stay in Wonderland and I show you
how deep the rabbit-hole goes.** **Morpheus**

## Apps

- [checkers.wat](ch02/checkers.wat), WebAssembly Checkers module
- [checkers.rs](ch03/src/checkers.rs), Rust Checkers module
- [Rogue] [WebAssembly](ch04) with [Rot.js]

## Setup

Install [wabbit], WebAssembly Binary Toolkit:

I'm cheating and installing the binary version, instead.
Please refer the [wabbit] github for building from scratch.

```
$ sudo pacman -S wabt
```

[webassembly]: https://webassembly.github.io/spec/core/intro/index.html
[programming webassembly with rust]: https://pragprog.com/titles/khrust/programming-webassembly-with-rust/
[kevin hoffman]: https://twitter.com/KevinHoffman
[rogue]: https://en.wikipedia.org/wiki/Rogue_(video_game)
[rot.js]: http://roguebasin.com/index.php/Rot.js_tutorial
[build.rs]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
[rustwasm]: https://rustwasm.github.io/docs/book/
[wabbit]: https://github.com/WebAssembly/wabt
