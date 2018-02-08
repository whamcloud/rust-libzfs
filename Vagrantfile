# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
    config.vm.box = "manager-for-lustre/centos74-1708-base"
    config.vm.synced_folder ".", "/vagrant", type: "virtualbox"
    config.vm.provider "virtualbox" do |vb|
      vb.customize ["modifyvm", :id, "--memory", "512"]
      vb.name = "libzfs"
      
      for i in 1..5 do
        disk = "./tmp/disk#{i}.vdi"

        unless File.exist?(disk)
          vb.customize ["createmedium", "disk",
            "--filename", disk,
            "--size", "100",
            "--format", "VDI",
            "--variant", "fixed"
          ]
        end

        vb.customize ['storageattach', :id, '--storagectl', 'SATA Controller', '--port', i, '--type', 'hdd', '--medium', disk]
        vb.customize ['setextradata', :id, "VBoxInternal/Devices/ahci/0/Config/Port#{i}/SerialNumber", "081118FC1221NCJ6G8G#{i}"]
      end
    end
  
    config.vm.boot_timeout = 600
  
    config.vm.provision "shell", inline: <<-SHELL
        yum -y install yum-plugin-copr epel-release http://download.zfsonlinux.org/epel/zfs-release.el7_4.noarch.rpm
        yum -y copr enable alonid/llvm-5.0.0
        yum -y install clang-5.0.0 zfs libzfs2-devel --nogpgcheck
        modprobe zfs
        genhostid
        zpool create test -o cachefile=none -o multihost=on /dev/sdb
        zfs create test/ds
        zpool export test
        curl https://sh.rustup.rs -sSf > /home/vagrant/rustup.sh
        chmod 755 rustup.sh
        ./rustup.sh -y
        source $HOME/.cargo/env
    SHELL
  end
  
