# Mints

Cli utils for the Atari Mint in `no_std` Rust.

This crate defines a custom rust target (`m68k-atari-mintelf.json`) that
allows you to create elf binaries with the crossmint toolchain. You'll
need the `m68k-atari-mintelf` toolchain instead of the old a.out
toolchain (`m68k-atari-mint`).

This crate links to [Mintbox](https://github.com/Takashiidobe/mintbox/),
although you can remove the link lines to link to
[Mintlib](https://github.com/freemint/mintlib) as your libc. This will
make your binaries larger -- by default C and Rust binaries that link to
mintlib are >100KB. I wanted a smaller size so I wrote my own libc with
just enough to bootstrap Rust but still keep small binaries. 

Currently, the allocator uses `libc_alloc` to defer to libc's malloc and
friends -- that means that `memalign` is required to use `alloc` in
rust. [Libcmini](https://github.com/freemint/libcmini) could be used as
a replacement libc for smaller binaries, but it currently doesn't
implement `memalign`, so until then you'll have to use `mintlib` or
`mintbox`.

## Size footguns to keep in mind

- Any language feature that brings in compiler builtins will bloat the
  binary to 300KB. This apparently includes derives (thiserror + noshell
  seems to do this), as well as formatting certain things. If an i32 is
  printed, then it doesn't pull in compiler-builtins, but if you debug
  print it, it pulls in compiler-builtins, and the binary will go up to
  300KB. It also seems like using a `Box<dyn Error>` causes the same
  issue, probably due to accessing formatting code indirectly. An `impl
  Error` doesn't seem to have the same issue.

- To handle printing, The `print` macro calls `format_args.as_str()` 
  which returns a `Some(s)` if the string itself can be folded into a 
  string without requiring trait dispatch, otherwise, just panicking. It
  would be preferable to do this at compile time, so that code like
  `println!("{:?}", vec![1]);` would be compiler errors instead of
  panics at runtime. As is, this code will panic and then sigabort.

## In Progress:

- [x] rm
