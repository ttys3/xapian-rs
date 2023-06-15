# xapian-rs

[![crates.io](https://badgen.net/crates/v/xapian)](https://crates.io/crates/xapian)
[![docs](https://badgen.net/static/docs/passing/green)](https://ttys3.github.io/xapian-rs/xapian/)
![downloads](https://badgen.net/crates/d/xapian)
![latest version downloads](https://badgen.net/crates/dl/xapian)

xapian bind for rust

## TODO list

ref file:///usr/share/doc/xapian-core-devel/apidoc/html/inherits.html

- [x] Database
- [x] WritableDatabase
- [x] Document
- [x] Enquire
- [ ] ESet
- [ ] ESetIterator
- [x] MSet
- [x] MSetIterator
- [x] MatchSpy
- [x] ValueCountMatchSpy
- [ ] PositionIterator
- [ ] PostingIterator
- [x] Query
- [x] QueryParser
- [ ] RSet
- [ ] RangeProcessor
- [ ] DateRangeProcessor
- [x] NumberRangeProcessor
- [ ] UnitRangeProcessor
- [x] Stem
- [ ] Stopper
- [ ] SimpleStopper
- [ ] StemStopper
- [ ] TermGenerator
- [x] TermIterator
- [ ] Utf8Iterator
- [ ] ValueIterator
- [ ] Weight

## honey backend status

> The remaining blockers for this are:

* adding update support to the new honey backend (to replace glass)
* adding support for RAM storage to honey (to replace inmemory)
* moving some remote client and server code out of libxapian (or
  replacing it)

## crate maintainers docs

xapian github repo: https://github.com/xapian/xapian

User guide online: Getting Started with Xapian https://getting-started-with-xapian.readthedocs.io/

the Xapian developer guide https://xapian-developer-guide.readthedocs.io/

https://xapian.org/docs/

new version news: https://lists.xapian.org/pipermail/xapian-discuss/2023-March/009961.html

xapian api docs: https://xapian.org/docs/apidoc/

## rust binding

cxx tuts https://cxx.rs/tutorial.html

autocxx book https://google.github.io/autocxx/


c ffi https://doc.rust-lang.org/nomicon/ffi.html

with cmake crate example https://eshard.com/posts/Rust-Cxx-interop

Rust and C++ interoperability https://sites.google.com/a/chromium.org/dev/Home/chromium-security/memory-safety/rust-and-c-interoperability

## performance

32bit docid

```
doc count: 31944, index doc took: 6251ms
```

build with `--enable-log` will result in performance drop, **about 3x slower**.

check log enabled or not: `grep XAPIAN_DEBUG_LOG xapian-core/config.h`

```
doc count: 31944, index doc took: 18154ms
```

## troubleshooting

```
terminate called without an active exception

Process finished with exit code 134 (interrupted by signal 6: SIGABRT)
```

solution:

do not use  `Error e` in `rust::behavior::trycatch` C++ code, use `const Xapian::Error& e` instead.

reason:

`Error` is defined in generated `target/cxxbridge/rust/cxx.h` file under `rust` namespace.

use `Xapian::Error` will fix the problem.

## xapian debug

HACKING: `XAPIAN_DEBUG_LOG=-` send output to stderr, not stdout.

`XAPIAN_DEBUG_FLAGS`

`XAPIAN_FLUSH_THRESHOLD`

## Intellij IDE

intellij-rust false positives with cxx

see https://github.com/intellij-rust/intellij-rust/issues/8369#issuecomment-1362698416

> `cxx::bridge` is [attribute](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros) procedural macro. And since the plugin doesn't expand them by default at this moment, code is highlighted with error annotations in places where syntax is not allowed from the point of view of common Rust. For example, you cannot write type aliases inside extern blocks. But you can enable expansion of attributes macros with `org.rust.macros.proc.attr` [experimental feature](https://plugins.jetbrains.com/plugin/8182-rust/docs/rust-faq.html#experimental-features). After this, almost everything will work as expected (except annotation of `unsafe` before extern block).
>
> <img alt="image" width="645" src="https://user-images.githubusercontent.com/2539310/209118936-3788dcd6-daae-4b85-8108-d3c0d537c139.png">
>
> Don't forget to [reload](https://plugins.jetbrains.com/plugin/8182-rust/docs/rust-cargo-tool-window.html#cargo-load) the project model after enabling the experimental feature.
>
> See [#6908](https://github.com/intellij-rust/intellij-rust/issues/6908) for details



## docs

why not use docs.rs to host docs?

we need external libs to build this crate: `libxapian` and `libclang`

docs.rs currently is not so friendly to such kind of need.

ref https://docs.rs/about/builds#missing-dependencies

> Docs.rs dependencies are managed through [crates-build-env](https://github.com/rust-lang/crates-build-env).
> See [Forge](https://forge.rust-lang.org/docs-rs/add-dependencies.html) for how to add a dependency.


