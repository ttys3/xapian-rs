# xapian-rs
xapian bind for rust

## honey backend status

> The remaining blockers for this are:

* adding update support to the new honey backend (to replace glass)
* adding support for RAM storage to honey (to replace inmemory)
* moving some remote client and server code out of libxapian (or
  replacing it)

## crate maintainers docs

c ffi https://doc.rust-lang.org/nomicon/ffi.html

with cmake crate example https://eshard.com/posts/Rust-Cxx-interop

Rust and C++ interoperability https://sites.google.com/a/chromium.org/dev/Home/chromium-security/memory-safety/rust-and-c-interoperability

xapian github repo: https://github.com/xapian/xapian

User guide online: Getting Started with Xapian https://getting-started-with-xapian.readthedocs.io/

the Xapian developer guide https://xapian-developer-guide.readthedocs.io/

https://xapian.org/docs/

cxx tuts https://cxx.rs/tutorial.html

new version news: https://lists.xapian.org/pipermail/xapian-discuss/2023-March/009961.html

xapian api docs: https://xapian.org/docs/apidoc/

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