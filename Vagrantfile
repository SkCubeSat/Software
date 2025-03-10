# -*- mode: ruby -*-
# vi: set ft=ruby :


Vagrant.configure("2") do |config|

  config.vm.box = "kubos/kubos-dev"
  config.vm.network "private_network", ip: "192.168.56.10"

  config.vm.synced_folder ".", "/home/vagrant/Software"
  
  config.ssh.extra_args = ["-t", "cd /home/vagrant/Software; bash --login"]
end
