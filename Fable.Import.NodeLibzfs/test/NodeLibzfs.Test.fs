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

    let encoded = encodePretty (Libzfs.VDev.Encode (File file))

    Matchers.toMatchSnapshot encoded

    VDev.Decoder encoded
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

    let encoded = encodePretty (Libzfs.VDev.Encode (Disk disk))

    Matchers.toMatchSnapshot encoded

    VDev.Decoder encoded
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

    let decoded = VDev.Decoder mirror

    Matchers.toMatchSnapshot decoded

    match decoded with
        | Ok x -> Matchers.toMatchSnapshot (encodePretty (VDev.Encode x))
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

    let decoded = VDev.Decoder raidZ

    Matchers.toMatchSnapshot decoded

    match decoded with
        | Ok x -> Matchers.toMatchSnapshot (encodePretty (VDev.Encode x))
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

    let decoded = VDev.Decoder root

    Matchers.toMatchSnapshot decoded

    match decoded with
        | Ok x -> Matchers.toMatchSnapshot (encodePretty (VDev.Encode x))
        | Error e -> failwith e
