// ts2fable 0.1.9
module rec libzfs
open Fable.Core

let [<Import("default","@iml/node-libzfs")>] libzfs: Libzfs.IExports = jsNative

module Libzfs =

    type [<AllowNullLiteral>] IExports =
        [<Emit("$0()")>] abstract Invoke: unit -> NodeLibzfs

    type [<AllowNullLiteral>] Dataset =
        abstract name: string with get, set
        abstract kind: string with get, set

    type [<AllowNullLiteral>] File =
        abstract path: string with get, set

    type [<AllowNullLiteral>] Disk =
        abstract path: string with get, set
        abstract dev_id: string option with get, set
        abstract phys_path: string option with get, set
        abstract whole_disk: bool option with get, set
        abstract is_log: bool with get, set

    type [<AllowNullLiteral>] Mirror =
        abstract children: ResizeArray<VDev> with get, set
        abstract is_log: bool option with get, set

    type [<AllowNullLiteral>] RaidZ =
        abstract children: ResizeArray<VDev> with get, set

    type [<AllowNullLiteral>] Replacing =
        abstract children: ResizeArray<VDev> with get, set

    type [<AllowNullLiteral>] Spare =
        abstract children: ResizeArray<VDev> with get, set

    type [<AllowNullLiteral>] Root =
        abstract children: ResizeArray<VDev> with get, set

    type VDev =
        U7<Mirror, RaidZ, Replacing, Spare, Root, Disk, File>

    [<RequireQualifiedAccess; CompilationRepresentation(CompilationRepresentationFlags.ModuleSuffix)>]
    module VDev =
        let ofMirror v: VDev = v |> U7.Case1
        let isMirror (v: VDev) = match v with U7.Case1 _ -> true | _ -> false
        let asMirror (v: VDev) = match v with U7.Case1 o -> Some o | _ -> None
        let ofRaidZ v: VDev = v |> U7.Case2
        let isRaidZ (v: VDev) = match v with U7.Case2 _ -> true | _ -> false
        let asRaidZ (v: VDev) = match v with U7.Case2 o -> Some o | _ -> None
        let ofReplacing v: VDev = v |> U7.Case3
        let isReplacing (v: VDev) = match v with U7.Case3 _ -> true | _ -> false
        let asReplacing (v: VDev) = match v with U7.Case3 o -> Some o | _ -> None
        let ofSpare v: VDev = v |> U7.Case4
        let isSpare (v: VDev) = match v with U7.Case4 _ -> true | _ -> false
        let asSpare (v: VDev) = match v with U7.Case4 o -> Some o | _ -> None
        let ofRoot v: VDev = v |> U7.Case5
        let isRoot (v: VDev) = match v with U7.Case5 _ -> true | _ -> false
        let asRoot (v: VDev) = match v with U7.Case5 o -> Some o | _ -> None
        let ofDisk v: VDev = v |> U7.Case6
        let isDisk (v: VDev) = match v with U7.Case6 _ -> true | _ -> false
        let asDisk (v: VDev) = match v with U7.Case6 o -> Some o | _ -> None
        let ofFile v: VDev = v |> U7.Case7
        let isFile (v: VDev) = match v with U7.Case7 _ -> true | _ -> false
        let asFile (v: VDev) = match v with U7.Case7 o -> Some o | _ -> None

    type [<AllowNullLiteral>] Pool =
        abstract name: string with get, set
        abstract uid: string with get, set
        abstract hostname: string with get, set
        abstract hostid: float option with get, set
        abstract state: string with get, set
        abstract size: float with get, set
        abstract vdev: VDev with get, set
        abstract datasets: ResizeArray<Dataset> with get, set

    type [<AllowNullLiteral>] NodeLibzfs =
        abstract getPoolByName: name: string -> Pool option
        abstract getImportedPools: unit -> ResizeArray<Pool>
        abstract getDatasetStringProp: name: string * prop: string -> string option
        abstract getDatasetUint64Prop: name: string * prop: string -> float option
