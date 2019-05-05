%define base_name node-libzfs

Name:       iml-%{base_name}
Version:    0.1.20
# Release Start
Release:    0.1557028367%{?dist}
# Release End

Summary:    Implements a binding layer from node to rust-libzfs
License:    MIT
Group:      System Environment/Libraries
URL:        https://github.com/whamcloud/rust-libzfs/tree/master/%{base_name}

Source0:    %{name}.tar.gz

ExclusiveArch: %{nodejs_arches}

BuildRequires: nodejs-packaging

Requires: nodejs

%description
%{summary}

%prep
%setup -c
%nodejs_fixdep -r neon-cli

%build

%install
mkdir -p %{buildroot}%{nodejs_sitearch}/@iml/node-libzfs/{lib,native}
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
* Sat May 04 2019 Joe Grund <jgrund@whamcloud.com> - 0.1.20-1
- Bump to ZFS 0.7.13

* Thu Nov 01 2018 Joe Grund <jgrund@whamcloud.com> - 0.1.19-1
- Bump to ZFS 0.7.11

* Tue May 15 2018 Joe Grund <joe.grund@intel.com> - 0.1.18-1
- Bump to ZFS 0.7.9

* Thu Apr 19 2018 Joe Grund <joe.grund@intel.com> - 0.1.17-1
- Change size parsing to string.
- Bump deps.

* Thu Mar 29 2018 Joe Grund <joe.grund@intel.com> - 0.1.15.-1
- Add support for zfs props.

* Fri Feb 09 2018 Joe Grund <joe.grund@intel.com> - 0.1.13-1
- initial package
