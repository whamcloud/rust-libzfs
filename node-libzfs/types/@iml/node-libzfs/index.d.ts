// Copyright (c) 2017 Intel Corporation. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

declare module 'libzfs' {
  export interface Dataset {
    name: string;
    kind: string;
  }

  export interface File {
    path: string;
  }

  export interface Disk {
    path: string;
    dev_id?: string;
    phys_path?: string;
    whole_disk?: boolean;
    is_log: boolean;
  }

  export interface Mirror {
    children: VDev[];
    is_log?: boolean;
  }

  export interface RaidZ {
    children: VDev[];
  }

  export interface Replacing {
    children: VDev[];
  }

  export interface Spare {
    children: VDev[];
  }

  export interface Root {
    children: VDev[];
  }

  export type VDev = Mirror | RaidZ | Replacing | Spare | Root | Disk | File;

  export interface Pool {
    name: string;
    uid: string;
    hostname: string;
    hostid: number | null;
    state: string;
    size: number;
    vdev: VDev;
    datasets: Dataset[];
  }

  export function getPoolByName(name: string): Pool | null;

  export function getImportedPools(): Pool[];

  export function getDatasetStringProp(
    name: string,
    prop: string
  ): string | null;

  export function getDatasetUint64Prop(
    name: string,
    prop: string
  ): number | null;
}
