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

## In Progress:

- [x] rm
