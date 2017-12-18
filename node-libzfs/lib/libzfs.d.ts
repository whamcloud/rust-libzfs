// Copyright (c) 2017 Intel Corporation. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

interface Dataset {
  name: string;
  kind: string;
}

interface File {
  path: string
}

interface Disk {
  path: string;
  phys_path?: string;
  whole_disk?: boolean;
  is_log: boolean;
}

interface Mirror {
  children: VDev[];
  is_log?: boolean;
}

interface RaidZ {
  children: VDev[];
}

interface Replacing {
  children: VDev[];
}

interface Spare {
  children: VDev[];
}

interface Root {
  children: VDev[];
}

type VDev = Mirror | RaidZ | Replacing | Spare | Root | Disk | File;

interface Pool {
  name: string;
  uid: string;
  state: string;
  size: number;
  vdev: VDev;
  datasets: Dataset[];
}

declare function getPoolByName(name: string): Pool | null;

declare function getImportedPools(): Pool[];

declare function getDatasetStringProp(
  name: string,
  prop: string
): string | null;
