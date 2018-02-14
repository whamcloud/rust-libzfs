#!/bin/bash -xe

# allow caller to override MAPPED_DIR, but default if they don't
MAPPED_DIR="${MAPPED_DIR:-/build}"

echo 'travis_fold:start:yum'
yum -y install git mock rpm-build ed sudo make rpmdevtools python-setuptools
echo 'travis_fold:end:yum'

# add our repos to the mock configuration
ed <<"EOF" /etc/mock/default.cfg
$i

[copr-be.cloud.fedoraproject.org_results_managerforlustre_manager-for-lustre_epel-7-x86_64_]
name=added from: https://copr-be.cloud.fedoraproject.org/results/managerforlustre/manager-for-lustre/epel-7-x86_64/
baseurl=https://copr-be.cloud.fedoraproject.org/results/managerforlustre/manager-for-lustre/epel-7-x86_64/
enabled=1
.
wq
EOF

eval export "$(grep -e "^TRAVIS=" -e "^TRAVIS_PULL_REQUEST_BRANCH=" "$MAPPED_DIR"/travis_env)"

groupadd --gid "$(stat -c '%g' "$MAPPED_DIR")" mocker
useradd --uid "$(stat -c '%u' "$MAPPED_DIR")" --gid "$(stat -c '%g' "$MAPPED_DIR")" mocker
usermod -a -G mock mocker


if ! su - mocker <<EOF; then
set -xe
cd "$MAPPED_DIR"
make DIST_VERSION="$TRAVIS_PULL_REQUEST_BRANCH" build_test
EOF
    exit "${PIPESTATUS[0]}"
fi

exit 0
