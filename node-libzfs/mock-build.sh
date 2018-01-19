#!/bin/bash -xe

ed <<"EOF" /etc/mock/default.cfg
$i

[copr-be.cloud.fedoraproject.org_results_managerforlustre_manager-for-lustre_epel-7-x86_64_]
name=added from: https://copr-be.cloud.fedoraproject.org/results/managerforlustre/manager-for-lustre/epel-7-x86_64/
baseurl=https://copr-be.cloud.fedoraproject.org/results/managerforlustre/manager-for-lustre/epel-7-x86_64/
enabled=1

[alonid-llvm-5.0.0]
name=Copr repo for llvm-5.0.0 owned by alonid
baseurl=https://copr-be.cloud.fedoraproject.org/results/alonid/llvm-5.0.0/epel-7-$basearch/
enabled=1

[zfs]
name=ZFS on Linux for EL7 - dkms
baseurl=http://download.zfsonlinux.org/epel/7.4/$basearch/
enabled=1
.
w
q
EOF

groupadd --gid $(stat -c '%g' /builddir/node-libzfs/README.md) -o mocker
useradd --uid $(stat -c '%u' /builddir/node-libzfs/README.md) --gid $(stat -c '%g' /builddir/node-libzfs/README.md) mocker
usermod -a -G mock mocker

cd /builddir/
RELEASE=$(git rev-list HEAD | wc -l)

su - mocker <<EOF
set -xe
cd /builddir/node-libzfs/
rpmbuild -bs --define epel\ 1 --define package_release\ $RELEASE --define _srcrpmdir\ \$PWD --define _sourcedir\ \$PWD *.spec
mock iml-node-libzfs-*.src.rpm --rpmbuild-opts="--define package_release\ $RELEASE" --enable-network
EOF