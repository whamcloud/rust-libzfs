# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
    config.vm.box = "manager-for-lustre/centos74-1708-base"
    config.vm.synced_folder ".", "/vagrant", type: "virtualbox"
    config.vm.provider "virtualbox" do |vb|
      vb.customize ["modifyvm", :id, "--memory", "512"]
      vb.name = "libzfs"
      
  
      file_to_disk = './tmp/medium_disk.vdi'
  
      unless File.exist?(file_to_disk)
        vb.customize ['createhd', '--filename', file_to_disk, '--size', 500 * 1024]
      end

      vb.customize ['storageattach', :id, '--storagectl', 'SATA Controller', '--port', 1, '--device', 0, '--type', 'hdd', '--medium', file_to_disk]
      vb.customize ['setextradata', :id, 'VBoxInternal/Devices/ahci/0/Config/Port1/SerialNumber', '081118FC1221NCJ6G8GG']
    end
  
    config.vm.boot_timeout = 600
  
    config.vm.provision "shell", inline: <<-SHELL
        yum -y install yum-plugin-copr epel-release
        yum-config-manager --add-repo https://downloads.hpdd.intel.com/public/lustre/lustre-2.10.1/el7/server
        yum -y copr enable alonid/llvm-5.0.0
        yum -y install clang-5.0.0 zfs libzfs2-devel --nogpgcheck
        modprobe zfs
        genhostid
        zpool create test -o cachefile=none -o multihost=on /dev/sdb
        zpool export test
        curl https://sh.rustup.rs -sSf > /home/vagrant/rustup.sh
        chmod 777 rustup.sh
        su vagrant
        ./rustup.sh -y
        source $HOME/.cargo/env
        exit
    SHELL
  end
  