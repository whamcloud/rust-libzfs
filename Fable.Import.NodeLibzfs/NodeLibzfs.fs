// Copyright (c) 2018 Intel Corporation. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

module rec libzfs
open Fable.Core

let [<Import("default","@iml/node-libzfs")>] libzfs: Libzfs.IExports = jsNative

module Libzfs =
    type [<AllowNullLiteral>] IExports =
        [<Emit("$0()")>] abstract Invoke: unit -> NodeLibzfs

    type [<AllowNullLiteral>] RootNode =
        abstract Root: Root with get, set

    type [<AllowNullLiteral>] Root =
        abstract children: VDev list with get, set

    type [<AllowNullLiteral>] FileNode =
        abstract File: File with get, set

    type [<AllowNullLiteral>] File =
        abstract guid: string option with get, set
        abstract state: string with get, set
        abstract path: string with get, set
        abstract is_log: bool option with get, set

    type [<AllowNullLiteral>] DiskNode =
        abstract Disk: Disk with get, set

    type [<AllowNullLiteral>] Disk =
        abstract guid: string option with get, set
        abstract state: string with get, set
        abstract path: string with get, set
        abstract dev_id: string option with get, set
        abstract phys_path: string option with get, set
        abstract whole_disk: bool option with get, set
        abstract is_log: bool option with get, set

    type [<AllowNullLiteral>] MirrorNode =
        abstract Mirror: Mirror with get, set

    type [<AllowNullLiteral>] Mirror =
        abstract children: VDev list with get, set
        abstract is_log: bool option with get, set

    type [<AllowNullLiteral>] RaidZNode =
        abstract RaidZ: RaidZ with get, set

    type [<AllowNullLiteral>] RaidZ =
        abstract children: VDev list with get, set

    type [<AllowNullLiteral>] ReplacingNode =
        abstract Replacing: Replacing with get, set

    type [<AllowNullLiteral>] Replacing =
        abstract children: VDev list with get, set

    type [<AllowNullLiteral>] SpareNode =
        abstract Spare: Spare with get, set

    type [<AllowNullLiteral>] Spare =
        abstract children: VDev list with get, set

    type [<AllowNullLiteral>] CacheNode =
        abstract Cache: VDev list with get, set

    type [<AllowNullLiteral>] Cache =
        abstract children: VDev list with get, set

    [<Erase>]
    type VDev =
        | Root of RootNode
        | File of FileNode
        | Disk of DiskNode
        | Mirror of MirrorNode
        | RaidZ of RaidZNode
        | Replacing of ReplacingNode
        | Spare of SpareNode
        | Cache of CacheNode

    type [<AllowNullLiteral>] Dataset =
        abstract name: string with get, set
        abstract kind: string with get, set

    type [<AllowNullLiteral>] Pool =
        abstract name: string with get, set
        abstract uid: string with get, set
        abstract hostname: string with get, set
        abstract health: string with get, set
        abstract hostid: float option with get, set
        abstract state: string with get, set
        abstract size: float with get, set
        abstract vdev: VDev with get, set
        abstract datasets: Dataset list with get, set

    type [<AllowNullLiteral>] NodeLibzfs =
        abstract getPoolByName: name: string -> Pool option
        abstract getImportedPools: unit -> Pool list
        abstract getDatasetStringProp: name: string * prop: string -> string option
        abstract getDatasetUint64Prop: name: string * prop: string -> float option
