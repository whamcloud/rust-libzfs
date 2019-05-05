BUILDROOT:=$(shell [ -d "/build" ] && echo "/build" || echo ".")
TMPDIR:=$(shell mktemp -d)

srpm:
	mkdir -p ${TMPDIR}/_topdir/SOURCES/node-libzfs/{lib,native}
	mkdir -p ${TMPDIR}/_topdir/SPECS
	rm -rf ${BUILDROOT}/_topdir
	yum install -y epel-release http://download.zfsonlinux.org/epel/zfs-release.el7_6.noarch.rpm
	yum install -y zfs libzfs2-devel --nogpgcheck
	yum install -y cargo yum-plugin-copr rpm-build nodejs-packaging
	yum copr -y enable alonid/llvm-5.0.0
	yum install -y clang-5.0.0 cargo nodejs
	npm run install
	cp package.json ${TMPDIR}/_topdir/SOURCES/node-libzfs/
	cp lib/index.js ${TMPDIR}/_topdir/SOURCES/node-libzfs/lib/
	cp native/index.node ${TMPDIR}/_topdir/SOURCES/node-libzfs/native
	tar -czvf ${TMPDIR}/_topdir/SOURCES/iml-node-libzfs.tar.gz -C ${TMPDIR}/_topdir/SOURCES/node-libzfs .
	cp iml-node-libzfs.spec ${TMPDIR}/_topdir/SPECS
	rpmbuild -bs -D "_topdir ${TMPDIR}/_topdir" ${TMPDIR}/_topdir/SPECS/iml-node-libzfs.spec
	cp -rf ${TMPDIR}/_topdir ${BUILDROOT}/
	cp -f _topdir/SRPMS/*.rpm $(outdir)




