%{?!package_release: %define package_release 1}

%define base_name node-libzfs
Name:       iml-%{base_name}
Version:    0.1.13
Release:    %{package_release}%{?dist}
Summary:    Implements a binding layer from node to rust-libzfs
License:    MIT
Group:      System Environment/Libraries
URL:        https://github.com/intel-hpdd/rust-libzfs/tree/master/%{base_name}
Source0:    http://registry.npmjs.org/@iml/%{base_name}/-/%{base_name}-%{version}.tgz

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
%setup -q -n package
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
* Fri Feb 09 2018 Joe Grund <joe.grund@intel.com> - 0.1.13-1
- initial package