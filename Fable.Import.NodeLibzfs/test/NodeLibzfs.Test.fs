// Copyright (c) 2018 Intel Corporation. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

module LibzfsTest

open libzfs
open libzfs.Libzfs
open Fable.Import.Jest
open Thot.Json

let private encodePretty =
    Encode.encode 4

test "encode / decode File" <| fun () ->
    let file =
        {
            File =
                {
                    guid = Some "0x28F29B87B841C810";
                    state = "ONLINE";
                    path = "/tmp/file_fs";
                    is_log = Some false;
                }
        }

    let encoded = encodePretty (Libzfs.VDev.encode (File file))

    Matchers.toMatchSnapshot encoded

    VDev.decoder encoded
        |> Matchers.toMatchSnapshot


test "encode / decode Disk" <| fun () ->
    let disk = {
        Disk =
            {
                guid = Some "0x28F29B87B841C810";
                state = "ONLINE";
                path = "/dev/disk/by-id/ata-VBOX_HARDDISK_081118FC1221NCJ6G807-part1";
                dev_id = "ata-VBOX_HARDDISK_081118FC1221NCJ6G807-part1";
                phys_path = Some "pci-0000:00:0d.0-ata-8.0";
                whole_disk = Some true;
                is_log = Some false;
            }
    }

    let encoded = encodePretty (Libzfs.VDev.encode (Disk disk))

    Matchers.toMatchSnapshot encoded

    VDev.decoder encoded
        |> Matchers.toMatchSnapshot


test "decode / encode Mirror" <| fun () ->
    let mirror = """
        {
            "Mirror": {
                "children": [
                    {
                        "Disk": {
                            "guid": "0xC0C12F18FC259D92",
                            "state": "ONLINE",
                            "path": "/dev/disk/by-id/ata-VBOX_HARDDISK_081118FC1221NCJ6G802-part1",
                            "dev_id": "ata-VBOX_HARDDISK_081118FC1221NCJ6G802-part1",
                            "phys_path": "pci-0000:00:0d.0-ata-3.0",
                            "whole_disk": true,
                            "is_log": null
                        }
                    },
                    {
                        "Disk": {
                            "guid": "0x171F9BA74F8A8960",
                            "state": "ONLINE",
                            "path": "/dev/disk/by-id/ata-VBOX_HARDDISK_081118FC1221NCJ6G803-part1",
                            "dev_id": "ata-VBOX_HARDDISK_081118FC1221NCJ6G803-part1",
                            "phys_path": "pci-0000:00:0d.0-ata-4.0",
                            "whole_disk": true,
                            "is_log": null
                        }
                    }
                ],
                "is_log": false
            }
          }
    """

    let decoded = VDev.decoder mirror

    Matchers.toMatchSnapshot decoded

    match decoded with
        | Ok x -> Matchers.toMatchSnapshot (encodePretty (VDev.encode x))
        | Error e -> failwith e


test "decode / encode RaidZ" <| fun () ->
    let raidZ = """
              {
                "RaidZ": {
                  "children": [
                    {
                      "Disk": {
                        "guid": "0xC49B177A2271FE62",
                        "state": "ONLINE",
                        "path": "/dev/disk/by-id/ata-VBOX_HARDDISK_081118FC1221NCJ6G806-part1",
                        "dev_id": "ata-VBOX_HARDDISK_081118FC1221NCJ6G806-part1",
                        "phys_path": "pci-0000:00:0d.0-ata-7.0",
                        "whole_disk": true,
                        "is_log": null
                      }
                    },
                    {
                      "Disk": {
                        "guid": "0x85D9C319913951B6",
                        "state": "ONLINE",
                        "path": "/dev/disk/by-id/ata-VBOX_HARDDISK_081118FC1221NCJ6G808-part1",
                        "dev_id": "ata-VBOX_HARDDISK_081118FC1221NCJ6G808-part1",
                        "phys_path": "pci-0000:00:0d.0-ata-9.0",
                        "whole_disk": true,
                        "is_log": null
                      }
                    },
                    {
                      "Disk": {
                        "guid": "0xA7F3292BAE60BFE1",
                        "state": "ONLINE",
                        "path": "/dev/disk/by-id/ata-VBOX_HARDDISK_081118FC1221NCJ6G809-part1",
                        "dev_id": "ata-VBOX_HARDDISK_081118FC1221NCJ6G809-part1",
                        "phys_path": "pci-0000:00:0d.0-ata-10.0",
                        "whole_disk": true,
                        "is_log": null
                      }
                    }
                  ]
                }
              }
    """

    let decoded = VDev.decoder raidZ

    Matchers.toMatchSnapshot decoded

    match decoded with
        | Ok x -> Matchers.toMatchSnapshot (encodePretty (VDev.encode x))
        | Error e -> failwith e


test "decode / encode whole tree" <| fun () ->
    let root = """
{
  "Root": {
    "children": [
      {
        "Mirror": {
          "children": [
            {
              "Disk": {
                "guid": "0x970502344C66A995",
                "state": "ONLINE",
                "path": "/dev/sdb1",
                "dev_id": "ata-VBOX_HARDDISK_081118FC1221NCJ6G8G1-part1",
                "phys_path": "pci-0000:00:0d.0-ata-2.0",
                "whole_disk": true,
                "is_log": null
              }
            },
            {
              "Disk": {
                "guid": "0xF44F3768513DAD5B",
                "state": "ONLINE",
                "path": "/dev/sdc1",
                "dev_id": "ata-VBOX_HARDDISK_081118FC1221NCJ6G8G2-part1",
                "phys_path": "pci-0000:00:0d.0-ata-3.0",
                "whole_disk": true,
                "is_log": null
              }
            }
          ],
          "is_log": false
        }
      }
    ],
    "spares": [
      {
        "Disk": {
          "guid": "0xBE9F6A598F3A7A4E",
          "state": "ONLINE",
          "path": "/dev/sde1",
          "dev_id": "ata-VBOX_HARDDISK_081118FC1221NCJ6G8G4-part1",
          "phys_path": "pci-0000:00:0d.0-ata-5.0",
          "whole_disk": true,
          "is_log": null
        }
      },
      {
        "Disk": {
          "guid": "0x25F1968AE86CFE17",
          "state": "ONLINE",
          "path": "/dev/sdf1",
          "dev_id": "ata-VBOX_HARDDISK_081118FC1221NCJ6G8G5-part1",
          "phys_path": "pci-0000:00:0d.0-ata-6.0",
          "whole_disk": true,
          "is_log": null
        }
      }
    ],
    "cache": [
      {
        "Disk": {
          "guid": "0xC11810B8A5D140F6",
          "state": "ONLINE",
          "path": "/dev/sdd1",
          "dev_id": "ata-VBOX_HARDDISK_081118FC1221NCJ6G8G3-part1",
          "phys_path": "pci-0000:00:0d.0-ata-4.0",
          "whole_disk": true,
          "is_log": null
        }
      }
    ]
  }
}
    """

    let decoded = VDev.decoder root

    Matchers.toMatchSnapshot decoded

    match decoded with
        | Ok x -> Matchers.toMatchSnapshot (encodePretty (VDev.encode x))
        | Error e -> failwith e

test "decode / encode pool" <| fun () ->
    let pool = """
{
  "name": "test",
  "guid": "14919184393193585238",
  "health": "ONLINE",
  "hostname": "localhost.localdomain",
  "hostid": 3914625515,
  "state": "ACTIVE",
  "readonly": false,
  "size": 83886080,
  "vdev": {
    "Root": {
      "children": [
        {
          "Mirror": {
            "children": [
              {
                "Disk": {
                  "guid": "0xBE4606AF1C39DC3F",
                  "state": "ONLINE",
                  "path": "/dev/sdb1",
                  "dev_id": "ata-VBOX_HARDDISK_081118FC1221NCJ6G8G1-part1",
                  "phys_path": "pci-0000:00:0d.0-ata-2.0",
                  "whole_disk": true,
                  "is_log": null
                }
              },
              {
                "Disk": {
                  "guid": "0xCC43D91716DA2522",
                  "state": "ONLINE",
                  "path": "/dev/sdc1",
                  "dev_id": "ata-VBOX_HARDDISK_081118FC1221NCJ6G8G2-part1",
                  "phys_path": "pci-0000:00:0d.0-ata-3.0",
                  "whole_disk": true,
                  "is_log": null
                }
              }
            ],
            "is_log": false
          }
        }
      ],
      "spares": [
        {
          "Disk": {
            "guid": "0x4DD5D18F6C1F6B34",
            "state": "ONLINE",
            "path": "/dev/sde1",
            "dev_id": "ata-VBOX_HARDDISK_081118FC1221NCJ6G8G4-part1",
            "phys_path": "pci-0000:00:0d.0-ata-5.0",
            "whole_disk": true,
            "is_log": null
          }
        },
        {
          "Disk": {
            "guid": "0x58650FD679A8842D",
            "state": "ONLINE",
            "path": "/dev/sdf1",
            "dev_id": "ata-VBOX_HARDDISK_081118FC1221NCJ6G8G5-part1",
            "phys_path": "pci-0000:00:0d.0-ata-6.0",
            "whole_disk": true,
            "is_log": null
          }
        }
      ],
      "cache": [
        {
          "Disk": {
            "guid": "0x70008C8B94F8AFA8",
            "state": "ONLINE",
            "path": "/dev/sdd1",
            "dev_id": "ata-VBOX_HARDDISK_081118FC1221NCJ6G8G3-part1",
            "phys_path": "pci-0000:00:0d.0-ata-4.0",
            "whole_disk": true,
            "is_log": null
          }
        }
      ]
    }
  },
  "props": [],
  "datasets": [
    {
      "name": "test/ds2",
      "guid": "11387853141151442667",
      "kind": "filesystem",
      "props": [
        {
          "name": "name",
          "value": "test/ds2"
        },
        {
          "name": "type",
          "value": "filesystem"
        },
        {
          "name": "creation",
          "value": "1522333734"
        },
        {
          "name": "used",
          "value": "24576"
        },
        {
          "name": "available",
          "value": "41751040"
        },
        {
          "name": "referenced",
          "value": "24576"
        },
        {
          "name": "compressratio",
          "value": "1.00x"
        },
        {
          "name": "mounted",
          "value": "yes"
        },
        {
          "name": "quota",
          "value": "0"
        },
        {
          "name": "reservation",
          "value": "0"
        },
        {
          "name": "recordsize",
          "value": "131072"
        },
        {
          "name": "mountpoint",
          "value": "/test/ds2"
        },
        {
          "name": "sharenfs",
          "value": "off"
        },
        {
          "name": "checksum",
          "value": "on"
        },
        {
          "name": "compression",
          "value": "off"
        },
        {
          "name": "atime",
          "value": "on"
        },
        {
          "name": "devices",
          "value": "on"
        },
        {
          "name": "exec",
          "value": "on"
        },
        {
          "name": "setuid",
          "value": "on"
        },
        {
          "name": "readonly",
          "value": "off"
        },
        {
          "name": "zoned",
          "value": "off"
        },
        {
          "name": "snapdir",
          "value": "hidden"
        },
        {
          "name": "aclinherit",
          "value": "restricted"
        },
        {
          "name": "createtxg",
          "value": "6638"
        },
        {
          "name": "canmount",
          "value": "on"
        },
        {
          "name": "xattr",
          "value": "on"
        },
        {
          "name": "copies",
          "value": "1"
        },
        {
          "name": "version",
          "value": "5"
        },
        {
          "name": "utf8only",
          "value": "off"
        },
        {
          "name": "normalization",
          "value": "none"
        },
        {
          "name": "casesensitivity",
          "value": "sensitive"
        },
        {
          "name": "vscan",
          "value": "off"
        },
        {
          "name": "nbmand",
          "value": "off"
        },
        {
          "name": "sharesmb",
          "value": "off"
        },
        {
          "name": "refquota",
          "value": "0"
        },
        {
          "name": "refreservation",
          "value": "0"
        },
        {
          "name": "guid",
          "value": "11387853141151442667"
        },
        {
          "name": "primarycache",
          "value": "all"
        },
        {
          "name": "secondarycache",
          "value": "all"
        },
        {
          "name": "usedbysnapshots",
          "value": "0"
        },
        {
          "name": "usedbydataset",
          "value": "24576"
        },
        {
          "name": "usedbychildren",
          "value": "0"
        },
        {
          "name": "usedbyrefreservation",
          "value": "0"
        },
        {
          "name": "logbias",
          "value": "latency"
        },
        {
          "name": "dedup",
          "value": "off"
        },
        {
          "name": "mlslabel",
          "value": "none"
        },
        {
          "name": "sync",
          "value": "standard"
        },
        {
          "name": "dnodesize",
          "value": "legacy"
        },
        {
          "name": "refcompressratio",
          "value": "1.00x"
        },
        {
          "name": "written",
          "value": "24576"
        },
        {
          "name": "logicalused",
          "value": "12288"
        },
        {
          "name": "logicalreferenced",
          "value": "12288"
        },
        {
          "name": "volmode",
          "value": "default"
        },
        {
          "name": "filesystem_limit",
          "value": "18446744073709551615"
        },
        {
          "name": "snapshot_limit",
          "value": "18446744073709551615"
        },
        {
          "name": "filesystem_count",
          "value": "18446744073709551615"
        },
        {
          "name": "snapshot_count",
          "value": "18446744073709551615"
        },
        {
          "name": "snapdev",
          "value": "hidden"
        },
        {
          "name": "acltype",
          "value": "off"
        },
        {
          "name": "context",
          "value": "none"
        },
        {
          "name": "fscontext",
          "value": "none"
        },
        {
          "name": "defcontext",
          "value": "none"
        },
        {
          "name": "rootcontext",
          "value": "none"
        },
        {
          "name": "relatime",
          "value": "off"
        },
        {
          "name": "redundant_metadata",
          "value": "all"
        },
        {
          "name": "overlay",
          "value": "off"
        }
      ]
    },
    {
      "name": "test/ds",
      "guid": "9760213047416233279",
      "kind": "filesystem",
      "props": [
        {
          "name": "name",
          "value": "test/ds"
        },
        {
          "name": "type",
          "value": "filesystem"
        },
        {
          "name": "creation",
          "value": "1521929884"
        },
        {
          "name": "used",
          "value": "24576"
        },
        {
          "name": "available",
          "value": "41751040"
        },
        {
          "name": "referenced",
          "value": "24576"
        },
        {
          "name": "compressratio",
          "value": "1.00x"
        },
        {
          "name": "mounted",
          "value": "yes"
        },
        {
          "name": "quota",
          "value": "0"
        },
        {
          "name": "reservation",
          "value": "0"
        },
        {
          "name": "recordsize",
          "value": "131072"
        },
        {
          "name": "mountpoint",
          "value": "/test/ds"
        },
        {
          "name": "sharenfs",
          "value": "off"
        },
        {
          "name": "checksum",
          "value": "on"
        },
        {
          "name": "compression",
          "value": "off"
        },
        {
          "name": "atime",
          "value": "on"
        },
        {
          "name": "devices",
          "value": "on"
        },
        {
          "name": "exec",
          "value": "on"
        },
        {
          "name": "setuid",
          "value": "on"
        },
        {
          "name": "readonly",
          "value": "off"
        },
        {
          "name": "zoned",
          "value": "off"
        },
        {
          "name": "snapdir",
          "value": "hidden"
        },
        {
          "name": "aclinherit",
          "value": "restricted"
        },
        {
          "name": "createtxg",
          "value": "6"
        },
        {
          "name": "canmount",
          "value": "on"
        },
        {
          "name": "xattr",
          "value": "on"
        },
        {
          "name": "copies",
          "value": "1"
        },
        {
          "name": "version",
          "value": "5"
        },
        {
          "name": "utf8only",
          "value": "off"
        },
        {
          "name": "normalization",
          "value": "none"
        },
        {
          "name": "casesensitivity",
          "value": "sensitive"
        },
        {
          "name": "vscan",
          "value": "off"
        },
        {
          "name": "nbmand",
          "value": "off"
        },
        {
          "name": "sharesmb",
          "value": "off"
        },
        {
          "name": "refquota",
          "value": "0"
        },
        {
          "name": "refreservation",
          "value": "0"
        },
        {
          "name": "guid",
          "value": "9760213047416233279"
        },
        {
          "name": "primarycache",
          "value": "all"
        },
        {
          "name": "secondarycache",
          "value": "all"
        },
        {
          "name": "usedbysnapshots",
          "value": "0"
        },
        {
          "name": "usedbydataset",
          "value": "24576"
        },
        {
          "name": "usedbychildren",
          "value": "0"
        },
        {
          "name": "usedbyrefreservation",
          "value": "0"
        },
        {
          "name": "logbias",
          "value": "latency"
        },
        {
          "name": "dedup",
          "value": "off"
        },
        {
          "name": "mlslabel",
          "value": "none"
        },
        {
          "name": "sync",
          "value": "standard"
        },
        {
          "name": "dnodesize",
          "value": "legacy"
        },
        {
          "name": "refcompressratio",
          "value": "1.00x"
        },
        {
          "name": "written",
          "value": "24576"
        },
        {
          "name": "logicalused",
          "value": "12288"
        },
        {
          "name": "logicalreferenced",
          "value": "12288"
        },
        {
          "name": "volmode",
          "value": "default"
        },
        {
          "name": "filesystem_limit",
          "value": "18446744073709551615"
        },
        {
          "name": "snapshot_limit",
          "value": "18446744073709551615"
        },
        {
          "name": "filesystem_count",
          "value": "18446744073709551615"
        },
        {
          "name": "snapshot_count",
          "value": "18446744073709551615"
        },
        {
          "name": "snapdev",
          "value": "hidden"
        },
        {
          "name": "acltype",
          "value": "off"
        },
        {
          "name": "context",
          "value": "none"
        },
        {
          "name": "fscontext",
          "value": "none"
        },
        {
          "name": "defcontext",
          "value": "none"
        },
        {
          "name": "rootcontext",
          "value": "none"
        },
        {
          "name": "relatime",
          "value": "off"
        },
        {
          "name": "redundant_metadata",
          "value": "all"
        },
        {
          "name": "overlay",
          "value": "off"
        },
        {
          "name": "lustre:mgsnode",
          "value": "10.14.82.0@tcp:10.14.82.1@tcp"
        }
      ]
    }
  ]
}
"""

    let decoded = Pool.decoder pool

    match decoded with
        | Ok x -> Matchers.toMatchSnapshot (encodePretty (Pool.encode x))
        | Error e -> failwith e
