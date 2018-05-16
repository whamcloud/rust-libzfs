%define base_name node-libzfs
Name:       iml-%{base_name}
Version:    0.1.18
Release:    1%{?dist}
Summary:    Implements a binding layer from node to rust-libzfs
License:    MIT
Group:      System Environment/Libraries
URL:        https://github.com/intel-hpdd/rust-libzfs/tree/master/%{base_name}
# Forcing local source because rpkg in copr does not seem to have a way
# to build source in the same way a package manager would.
Source0:    %{name}-%{version}.tgz

ExclusiveArch: %{nodejs_arches}

BuildRequires: nodejs-packaging
BuildRequires: nodejs
BuildRequires: npm
BuildRequires: cargo
BuildRequires: clang-5.0.0
BuildRequires: libzfs2-devel
BuildRequires: zfs

Requires: nodejs

%description
Implements a binding layer from node to rust-libzfs.

%prep
%setup
npm i neon-cli@0.1.22
%nodejs_fixdep -r neon-cli

%build
npm run install

%install
mkdir -p %{buildroot}%{nodejs_sitearch}/@iml/node-libzfs/lib/
mkdir -p %{buildroot}%{nodejs_sitearch}/@iml/node-libzfs/native/
cp -p package.json %{buildroot}%{nodejs_sitearch}/@iml/node-libzfs/
cp -p lib/index.js %{buildroot}%{nodejs_sitearch}/@iml/node-libzfs/lib/
cp -p native/index.node %{buildroot}%{nodejs_sitearch}/@iml/node-libzfs/native/

%clean
rm -rf %{buildroot}

%check
%{__nodejs} -e 'require("./")'

%files
%{nodejs_sitearch}/@iml/node-libzfs/lib/index.js
%{nodejs_sitearch}/@iml/node-libzfs/native/index.node
%{nodejs_sitearch}/@iml/node-libzfs/package.json

%changelog
* Tue May 15 2018 Joe Grund <joe.grund@intel.com> - 0.1.18-1
- Bump to ZFS 0.7.9

* Thu Apr 19 2018 Joe Grund <joe.grund@intel.com> - 0.1.17-1
- Change size parsing to string.
- Bump deps.

* Thu Mar 29 2018 Joe Grund <joe.grund@intel.com> - 0.1.15.-1
- Add support for zfs props.

* Fri Feb 09 2018 Joe Grund <joe.grund@intel.com> - 0.1.13-1
- initial package
