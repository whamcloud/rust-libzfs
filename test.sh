#!/bin/bash
set -e

vagrant destroy -f
vagrant up
vagrant ssh -c 'sudo -i -- <<EOF
cd /vagrant
cargo test
EOF'
vagrant destroy -f
