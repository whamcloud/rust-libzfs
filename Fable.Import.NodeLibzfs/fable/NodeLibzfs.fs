// Copyright (c) 2018 Intel Corporation. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

module rec libzfs
open Fable.Core
open Thot.Json
open Libzfs

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

    module Root =
        let decode x =
            x
                |> Decode.map3
                    (fun children spares cache ->
                        {
                            children = children;
                            spares = spares;
                            cache = cache;
                        }
                    )
                    (Decode.field "children" VDev.decodeArray)
                    (Decode.field "spares" VDev.decodeArray)
                    (Decode.field "cache" VDev.decodeArray)

        let encode
            ({
                children = children;
                spares = spares;
                cache = cache;
            }) =
                Encode.object [
                    ("children", (Encode.array (VDev.encodeList children)));
                    ("spares", (Encode.array (VDev.encodeList spares)));
                    ("cache", (Encode.array (VDev.encodeList cache)));
                ]



    type RootNode =
        {
            Root: Root;
        }

    module RootNode =
        let private key = "Root"

        let encode x =
            Encode.object [
                (key, Root.encode x.Root)
            ]

        let decode =
            Decode.map
                (fun x ->
                    {
                        Root = x;
                    }
                )
                (Decode.field key Root.decode)


    type File =
        {
            guid: string option;
            state: string;
            path: string;
            is_log: bool option;
        }

    module File =
        let encode
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

        let decode =
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

    module FileNode =
        let key = "File"

        let encode x =
            Encode.object [
                (key, File.encode x.File)
            ]

        let decode =
            Decode.map
                (fun x ->
                    {
                        File = x;
                    }
                )
                (Decode.field key File.decode)

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

    module Disk =

        let encode
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

        let decode =
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

    module DiskNode =
        let key = "Disk"

        let encode x =
            Encode.object [
                (key, Disk.encode x.Disk)
            ]

        let decode =
            Decode.map
                (fun x ->
                    {
                        Disk = x;
                    }
                )
                (Decode.field key Disk.decode)

    type Mirror =
        {
            children: VDev array;
            is_log: bool option;
        }

    module Mirror =
        let encode
            ({
                children = children;
                is_log = isLog;
            }) =
                Encode.object [
                    ("children", Encode.array (VDev.encodeList children));
                    ("is_log", Encode.option Encode.bool isLog)
                ]

        let decode x =
            x
                |> Decode.map2
                    (fun children isLog ->
                        {
                            children = children;
                            is_log = isLog;
                        }
                    )
                    (Decode.field "children" VDev.decodeArray)
                    (Decode.field "is_log" (Decode.option Decode.bool))

    type MirrorNode =
        {
            Mirror: Mirror;
        }

    module MirrorNode =
        let private key = "Mirror"

        let encode x =
            Encode.object [
                (key, Mirror.encode x.Mirror)
            ]

        let decode =
            Decode.map
                (fun x ->
                    {
                        Mirror = x;
                    }
                )
                (Decode.field key Mirror.decode)

    type RaidZ =
        {
            children: VDev array
        }

    module RaidZ =
        let encode
            ({
                children = children;
            }:RaidZ) =
                Encode.object [
                    ("children", Encode.array (VDev.encodeList children));
                ]

        let decode x =
            x
                |> Decode.map
                    (fun children ->
                        ({
                            children = children;
                        }:RaidZ)
                    )
                    (Decode.field "children" VDev.decodeArray)

    type  RaidZNode =
        {
            RaidZ: RaidZ
        }

    module RaidZNode =
        let key = "RaidZ"

        let encode x =
            Encode.object [
                (key, RaidZ.encode x.RaidZ)
            ]

        let decode =
            Decode.map
                (fun x ->
                    {
                        RaidZ = x;
                    }
                )
                (Decode.field RaidZNode.key RaidZ.decode)

    type Replacing =
        {
            children: VDev array;
        }

    module Replacing =
        let encode
            ({
                children = children;
            }:Replacing) =
                Encode.object [
                    ("children", Encode.array (VDev.encodeList children));
                ]

        let decode x =
            x
                |> Decode.map
                    (fun children ->
                        {
                            children = children;
                        }
                    )
                    (Decode.field "children" VDev.decodeArray)

    type ReplacingNode =
        {
            Replacing: Replacing;
        }

    module ReplacingNode =
        let private key = "Replacing"

        let encode x =
            Encode.object [
                (key, Replacing.encode x.Replacing)
            ]

        let decode =
            Decode.map
                (fun x ->
                    {
                        Replacing = x;
                    }
                )
                (Decode.field key Replacing.decode)

    type VDev =
        | Root of RootNode
        | File of FileNode
        | Disk of DiskNode
        | Mirror of MirrorNode
        | RaidZ of RaidZNode
        | Replacing of ReplacingNode

    module VDev =
        let encode =
            function
                | Root x ->
                    RootNode.encode x
                | File x ->
                    FileNode.encode x
                | Disk x ->
                    DiskNode.encode x
                | Mirror x ->
                    MirrorNode.encode x
                | RaidZ x ->
                    RaidZNode.encode x
                | Replacing x ->
                    ReplacingNode.encode x

        let encodeList xs =
            xs
                |> Array.map (encode)

        let encoder =
            encode
                >> Encode.encode 0

        let decode =
            Decode.oneOf [
                RootNode.decode >> Result.map VDev.Root;
                FileNode.decode >> Result.map VDev.File;
                DiskNode.decode >> Result.map VDev.Disk;
                MirrorNode.decode >> Result.map VDev.Mirror;
                RaidZNode.decode >> Result.map VDev.RaidZ;
                ReplacingNode.decode >> Result.map VDev.Replacing;
            ]

        let decodeArray =
            Decode.array decode

        let decoder =
            Decode.decodeString decode


    type ZProp =
        {
            name: string;
            value: string;
        }

    module ZProp =
        let encode
            {
                name = name;
                value = value;
            } =
                Encode.object [
                    ("name", Encode.string name);
                    ("value", Encode.string value);
                ]

        let decode =
            Decode.map2
                (fun name value ->
                    {
                        name = name;
                        value = value;
                    }
                )
                (Decode.field "name" Decode.string)
                (Decode.field "value" Decode.string)

    type Dataset =
        {
            name: string;
            guid: string;
            kind: string;
            props: ZProp array;
        }

    module Dataset =
        let encode
            {
              name = name;
              guid = guid;
              kind = kind;
              props = props;
            } =
              Encode.object [
                ("name", Encode.string name);
                ("guid", Encode.string guid);
                ("kind", Encode.string kind);
                ("props", Encode.array (Array.map ZProp.encode props));
              ]

        let decode =
            Decode.map4
                (fun name guid kind props ->
                    {
                        name = name;
                        guid = guid;
                        kind = kind;
                        props = props;
                    }
                )
                (Decode.field "name" Decode.string)
                (Decode.field "guid" Decode.string)
                (Decode.field "kind" Decode.string)
                (Decode.field "props" (Decode.array ZProp.decode))

    type Pool =
        {
            name: string;
            guid: string;
            health: string;
            hostname: string;
            hostid: int option;
            state: string;
            readonly: bool;
            size: int;
            vdev: VDev;
            props: ZProp array;
            datasets: Dataset array;
        }

    module Pool =
        let encode
            {
                name = name;
                guid = guid;
                health = health;
                hostname = hostname;
                hostid = hostid;
                state = state;
                readonly = readonly;
                size = size;
                vdev = vdev;
                props = props;
                datasets = datasets;
            } =
                Encode.object [
                    ("name", Encode.string name);
                    ("guid", Encode.string guid);
                    ("health", Encode.string health);
                    ("hostname", Encode.string hostname);
                    ("hostid", Encode.option Encode.int hostid);
                    ("state", Encode.string state);
                    ("readonly", Encode.bool readonly);
                    ("size", Encode.int size);
                    ("vdev", VDev.encode vdev);
                    ("props", Encode.array (Array.map ZProp.encode props));
                    ("datasets", Encode.array (Array.map Dataset.encode datasets));
                ]

        let decode =
            Decode.decode
                (fun name guid health hostname hostid
                     state readonly size vdev props datasets ->
                        {
                            name = name;
                            guid = guid;
                            health = health;
                            hostname = hostname;
                            hostid = hostid;
                            state = state;
                            readonly = readonly;
                            size = size;
                            vdev = vdev;
                            props = props;
                            datasets = datasets;
                        }
                )
                |> (Decode.required "name" Decode.string)
                |> (Decode.required "guid" Decode.string)
                |> (Decode.required "health" Decode.string)
                |> (Decode.required "hostname" Decode.string)
                |> (Decode.required "hostid" (Decode.option Decode.int))
                |> (Decode.required "state" Decode.string)
                |> (Decode.required "readonly" Decode.bool)
                |> (Decode.required "size" Decode.int)
                |> (Decode.required "vdev" VDev.decode)
                |> (Decode.required "props" (Decode.array ZProp.decode))
                |> (Decode.required "datasets" (Decode.array Dataset.decode))

        let decoder =
            Decode.decodeString decode

    type [<AllowNullLiteral>] NodeLibzfs =
        abstract getPoolByName: name: string -> Pool option
        abstract getDatasetByName: name: string -> Dataset option
        abstract getImportedPools: unit -> Pool list
