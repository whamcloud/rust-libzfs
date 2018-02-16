# rust-libzfs

[![Build Status](https://travis-ci.org/intel-hpdd/rust-libzfs.svg?branch=master)](https://travis-ci.org/intel-hpdd/rust-libzfs)

[![Build Status](https://copr.fedorainfracloud.org/coprs/managerforlustre/manager-for-lustre/package/iml-node-libzfs/status_image/last_build.png)](https://copr.fedorainfracloud.org/coprs/managerforlustre/manager-for-lustre/package/iml-node-libzfs/)

This repo provides [bindings](libzfs-sys) from libzfs to rust using bindgen.
It also provides a [wrapper](libzfs) around those bindings for idiomatic use.
Additionally, it provides [node bindings](node-libzfs) around the rust wrapper.
