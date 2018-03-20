# libzfs-sys

Bindings to libzfs 0.7.7. Uses [bindgen](https://github.com/rust-lang-nursery/rust-bindgen).

## Overview

Bindings created using [rust bindgen](https://github.com/rust-lang-nursery/rust-bindgen) and written
to the src dir. To rebuild bindings run `cargo build`.

## ZFS version

These bindings were compiled against ZFS 0.7.7. As `libzfs` is not a stable interface,
they should only be used against this version.

## OS

These bindings were compiled on Centos 7.4.x. They are likely to work against other
OS, but make sure to test first.
