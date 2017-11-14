vagrant up
vagrant ssh -c 'sudo -i -- <<EOF
cargo install cargo-test-junit
cd /vagrant
cargo test-junit --name results
EOF'
vagrant destroy -f