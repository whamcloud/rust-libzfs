#!/bin/bash
set -e

function cleanup {
    vagrant destroy -f
}
trap finish EXIT

vagrant destroy -f
vagrant up
vagrant ssh -c 'sudo -i -- <<EOF
cd /vagrant
cargo test
EOF'
