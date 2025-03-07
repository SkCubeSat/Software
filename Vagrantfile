# -*- mode: ruby -*-
# vi: set ft=ruby :


Vagrant.configure("2") do |config|

  config.vm.box = "kubos/kubos-dev"
  config.vm.network "private_network", ip: "192.168.33.10"

  config.vm.synced_folder "./kubos", "/home/vagrant/kubos"
  config.vm.synced_folder "./radsat-linux", "/home/vagrant/radsat-linux"
  config.vm.synced_folder "./src", "/home/vagrant/src"

end
