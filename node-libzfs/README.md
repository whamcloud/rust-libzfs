# node-libzfs

[![Build Status](https://copr.fedorainfracloud.org/coprs/managerforlustre/manager-for-lustre/package/iml-node-libzfs/status_image/last_build.png)](https://copr.fedorainfracloud.org/coprs/managerforlustre/manager-for-lustre/package/iml-node-libzfs/)

Neon bindings to rust-libzfs

Implements a binding layer from node to [rust-libzfs](https://github.com/whamcloud/rust-libzfs/tree/master/libzfs) via [neon](https://github.com/neon-bindings/neon).

This allows native interop with libzfs. The current API has a small scope, but will expand over time as more use-cases arise.

## Prereqs

Since this is a native module via rust, there are a few build / install time dependencies.

-   Node.js
-   Rust
-   libzfs (installed to a place where `pkg-config` can find it)

This has only been tested with CentOS, but should work ok with other Linux distros.
