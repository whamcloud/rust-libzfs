// Copyright (c) 2018 Intel Corporation. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

module rec libzfs
open Fable.Core
open Thot.Json

let [<Import("default","@iml/node-libzfs")>] libzfs: Libzfs.IExports = jsNative

module Libzfs =
    type [<AllowNullLiteral>] IExports =
        [<Emit("$0()")>] abstract Invoke: unit -> NodeLibzfs

    type Root =
        {
            children: VDev array;
            spares: VDev array;
            cache: VDev array;
        }

        static member Decode =
            Decode.map3
                (fun children spares cache ->
                    {
                        children = children;
                        spares = spares;
                        cache = cache;
                    }
                )
                (Decode.field "children" VDev.DecodeArray)
                (Decode.field "spares" VDev.DecodeArray)
                (Decode.field "cache" VDev.DecodeArray)

        static member Encode
            ({
                children = children;
                spares = spares;
                cache = cache;
            }) =
                Encode.object [
                    ("children", (Encode.array (VDev.EncodeList children)));
                    ("spares", (Encode.array (VDev.EncodeList spares)));
                    ("cache", (Encode.array (VDev.EncodeList cache)));
                ]



    type RootNode =
        {
            Root: Root;
        }

        static member Key = "Root"

        static member Encode x =
            Encode.object [
                (RootNode.Key, Root.Encode x.Root)
            ]

        static member Decode =
            Decode.map
                (fun x ->
                    {
                        Root = x;
                    }
                )
                (Decode.field RootNode.Key Root.Decode)


    type File =
        {
            guid: string option;
            state: string;
            path: string;
            is_log: bool option;
        }

        static member Encode
            ({
                guid = guid;
                state = state;
                path = path;
                is_log = isLog;
            }: File) =
                Encode.object [
                    ("guid", (Encode.option Encode.string guid));
                    ("state", Encode.string state);
                    ("path", Encode.string path);
                    ("is_log", (Encode.option Encode.bool isLog));
                ]

        static member Decode =
            Decode.map4
                (fun guid state path is_log ->
                    {
                        guid = guid;
                        state = state;
                        path = path;
                        is_log = is_log;
                    }
                )
                (Decode.field "guid" (Decode.option Decode.string))
                (Decode.field "state" Decode.string)
                (Decode.field "path" Decode.string)
                (Decode.field "is_log" (Decode.option Decode.bool))


    type FileNode =
        {
            File: File;
        }

        static member Key = "File"

        static member Encode x =
            Encode.object [
                (FileNode.Key, File.Encode x.File)
            ]

        static member Decode =
            Decode.map
                (fun x ->
                    {
                        File = x;
                    }
                )
                (Decode.field FileNode.Key File.Decode)

    type Disk =
        {
            guid: string option;
            state: string;
            path: string;
            dev_id: string;
            phys_path: string option;
            whole_disk: bool option;
            is_log: bool option;
        }

        static member Encode
            ({
                guid = guid;
                state = state;
                path = path;
                dev_id = devId;
                phys_path = physPath;
                whole_disk = wholeDisk;
                is_log = isLog;
            }) =
                Encode.object [
                    ("guid", (Encode.option Encode.string guid));
                    ("state", Encode.string state);
                    ("path", Encode.string path);
                    ("dev_id", Encode.string devId);
                    ("phys_path", Encode.option Encode.string physPath);
                    ("whole_disk", (Encode.option Encode.bool wholeDisk));
                    ("is_log", (Encode.option Encode.bool isLog));
                ]

        static member Decode =
            Decode.map7
                (fun guid state path devId physPath wholeDisk isLog ->
                    {
                        guid = guid;
                        state = state;
                        path = path;
                        dev_id = devId;
                        phys_path = physPath;
                        whole_disk = wholeDisk;
                        is_log = isLog;
                    }
                )
                (Decode.field "guid" (Decode.option Decode.string))
                (Decode.field "state" Decode.string)
                (Decode.field "path" Decode.string)
                (Decode.field "dev_id" Decode.string)
                (Decode.field "phys_path" (Decode.option Decode.string))
                (Decode.field "whole_disk" (Decode.option Decode.bool))
                (Decode.field "is_log" (Decode.option Decode.bool))


    type DiskNode =
        {
            Disk: Disk;
        }

        static member Key = "Disk"

        static member Encode x =
            Encode.object [
                (DiskNode.Key, Disk.Encode x.Disk)
            ]

        static member Decode =
            Decode.map
                (fun x ->
                    {
                        Disk = x;
                    }
                )
                (Decode.field DiskNode.Key Disk.Decode)

    type Mirror =
        {
            children: VDev array;
            is_log: bool option;
        }


        static member Encode
            ({
                children = children;
                is_log = isLog;
            }) =
                Encode.object [
                    ("children", Encode.array (VDev.EncodeList children));
                    ("is_log", Encode.option Encode.bool isLog)
                ]

        static member Decode =
            Decode.map2
                (fun children isLog ->
                    {
                        children = children;
                        is_log = isLog;
                    }
                )
                (Decode.field "children" VDev.DecodeArray)
                (Decode.field "is_log" (Decode.option Decode.bool))

    type MirrorNode =
        {
            Mirror: Mirror;
        }

        static member Key = "Mirror"

        static member Encode x =
            Encode.object [
                (MirrorNode.Key, Mirror.Encode x.Mirror)
            ]

        static member Decode =
            Decode.map
                (fun x ->
                    {
                        Mirror = x;
                    }
                )
                (Decode.field MirrorNode.Key Mirror.Decode)

    type RaidZ =
        {
            children: VDev array
        }

        static member Encode
            ({
                children = children;
            }:RaidZ) =
                Encode.object [
                    ("children", Encode.array (VDev.EncodeList children));
                ]

        static member Decode =
            Decode.map
                (fun children ->
                    {
                        children = children;
                    }
                )
                (Decode.field "children" VDev.DecodeArray)

    type  RaidZNode =
        {
            RaidZ: RaidZ
        }

        static member Key = "RaidZ"

        static member Encode x =
            Encode.object [
                (RaidZNode.Key, RaidZ.Encode x.RaidZ)
            ]

        static member Decode =
            Decode.map
                (fun x ->
                    {
                        RaidZ = x;
                    }
                )
                (Decode.field RaidZNode.Key RaidZ.Decode)

    type Replacing =
        {
            children: VDev array;
        }

        static member Encode
            ({
                children = children;
            }:Replacing) =
                Encode.object [
                    ("children", Encode.array (VDev.EncodeList children));
                ]

        static member Decode =
            Decode.map
                (fun children ->
                    {
                        children = children;
                    }
                )
                (Decode.field "children" VDev.DecodeArray)

    type ReplacingNode =
        {
            Replacing: Replacing;
        }

        static member Key = "Replacing"

        static member Encode x =
            Encode.object [
                (ReplacingNode.Key, Replacing.Encode x.Replacing)
            ]

        static member Decode =
            Decode.map
                (fun x ->
                    {
                        Replacing = x;
                    }
                )
                (Decode.field ReplacingNode.Key Replacing.Decode)

    type VDev =
        | Root of RootNode
        | File of FileNode
        | Disk of DiskNode
        | Mirror of MirrorNode
        | RaidZ of RaidZNode
        | Replacing of ReplacingNode

        static member Encode =
            function
                | Root x ->
                    RootNode.Encode x
                | File x ->
                    FileNode.Encode x
                | Disk x ->
                    DiskNode.Encode x
                | Mirror x ->
                    MirrorNode.Encode x
                | RaidZ x ->
                    RaidZNode.Encode x
                | Replacing x ->
                    ReplacingNode.Encode x

        static member EncodeList xs =
            xs
                |> Array.map (VDev.Encode)

        static member Encoder =
            VDev.Encode
                >> Encode.encode 0

        static member Decode =
            Decode.oneOf [
                RootNode.Decode >> Result.map VDev.Root;
                FileNode.Decode >> Result.map VDev.File;
                DiskNode.Decode >> Result.map VDev.Disk;
                MirrorNode.Decode >> Result.map VDev.Mirror;
                RaidZNode.Decode >> Result.map VDev.RaidZ;
                ReplacingNode.Decode >> Result.map VDev.Replacing;
            ]

        static member DecodeArray =
            Decode.array VDev.Decode

        static member Decoder =
            Decode.decodeString VDev.Decode


    type [<AllowNullLiteral>] Dataset =
        abstract name: string with get, set
        abstract kind: string with get, set

    type [<AllowNullLiteral>] Pool =
        abstract name: string with get, set
        abstract uid: string with get, set
        abstract hostname: string with get, set
        abstract health: string with get, set
        abstract hostid: int option with get, set
        abstract state: string with get, set
        abstract size: int with get, set
        abstract vdev: VDev with get, set
        abstract datasets: Dataset array with get, set

    type [<AllowNullLiteral>] NodeLibzfs =
        abstract getPoolByName: name: string -> Pool option
        abstract getImportedPools: unit -> Pool list
        abstract getDatasetStringProp: name: string * prop: string -> string option
        abstract getDatasetUint64Prop: name: string * prop: string -> int option
