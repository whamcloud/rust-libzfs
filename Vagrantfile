# -*- mode: ruby -*-
# vi: set ft=ruby :

require 'open3'

Vagrant.configure('2') do |config|
  config.vm.box = 'centos/7'

  config.vm.provider 'virtualbox' do |v|
    v.linked_clone = true
  end

  config.vm.synced_folder '.', '/vagrant',
                          type: 'rsync',
                          rsync__exclude: [
                            '.git/',
                            'target/',
                            'include/',
                            'node-libzfs/target',
                            'node-libzfs/node_modules'
                          ]

  config.vm.provider 'virtualbox' do |vb|
    vb.name = 'libzfs'
    vb.memory = 512
    vb.cpus = 4

    unless controller_exists('libzfs', 'SATA Controller')
      vb.customize ['storagectl', :id,
                    '--name', 'SATA Controller',
                    '--add', 'sata']
    end

    (1..9).each do |i|
      disk = "./tmp/disk#{i}.vdi"

      unless File.exist?(disk)
        vb.customize ['createmedium', 'disk',
                      '--filename', disk,
                      '--size', '100',
                      '--format', 'VDI',
                      '--variant', 'fixed']
      end

      vb.customize ['storageattach', :id, 
                    '--storagectl', 'SATA Controller',
                    '--port', i,
                    '--type', 'hdd',
                    '--medium', disk]

      vb.customize ['setextradata', :id,
                    "VBoxInternal/Devices/ahci/0/Config/Port#{i}/SerialNumber",
                    "081118FC1221NCJ6G8G#{i}"]
    end
  end

  config.vm.provision 'shell', inline: <<-SHELL
    yum -y install yum-plugin-copr epel-release http://download.zfsonlinux.org/epel/zfs-release.el7_5.noarch.rpm
    yum -y copr enable alonid/llvm-5.0.0
    yum -y install clang-5.0.0 zfs libzfs2-devel --nogpgcheck
    modprobe zfs
    genhostid
    zpool create test mirror sdb sdc cache sdd spare sde sdf
    zfs create test/ds
    zfs set lustre:mgsnode="10.14.82.0@tcp:10.14.82.1@tcp" test/ds
    zpool export test
    curl https://sh.rustup.rs -sSf > /home/vagrant/rustup.sh
    chmod 755 rustup.sh
    ./rustup.sh -y
    source $HOME/.cargo/env
    rustup component add rustfmt-preview
  SHELL
end

# Checks if a scsi controller exists.
# This is used as a predicate to create controllers,
# as vagrant does not provide this functionality by default.
def controller_exists(name, controller_name)
  out, err = Open3.capture2e("VBoxManage showvminfo #{name}")

  return false if err.exitstatus != 0

  out.split(/\n/)
     .select { |x| x.start_with? 'Storage Controller Name' }
     .map { |x| x.split(':')[1].strip }
     .any? { |x| x == controller_name }
end
