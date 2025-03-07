# Kubos Development Environment Setup

This Ansible playbook automates the setup of a development environment for Kubos, an operating system designed for cube satellites.

## Prerequisites

- Ansible 2.9 or higher
- For Windows hosts: Windows Subsystem for Linux (WSL) or a Linux control machine to run Ansible

## What This Playbook Does

This playbook will:

1. Detect the operating system of the target machine
2. Install the appropriate package manager if needed:
   - Windows: Chocolatey
   - macOS: Homebrew
   - Linux: Uses the native package manager
3. Install Git
4. Install VirtualBox
5. Add the user to the vboxusers group (Linux only)
6. Install Vagrant

## Usage

### Local Installation

To set up the development environment on your local machine:

```bash
ansible-playbook -i "localhost," -c local main.yml --ask-become-pass
```

### Remote Installation

To set up the development environment on remote machines, create an inventory file and run:

```bash
ansible-playbook -i inventory.ini main.yml --ask-become-pass
```

## Supported Operating Systems

- Windows
- macOS
- Debian/Ubuntu
- RHEL/CentOS/Fedora
- Arch Linux
- openSUSE

## Extending the Playbook

This playbook is designed to be extended with additional tasks for setting up a complete Kubos development environment. You can add more tasks to the main playbook or create additional task files and include them in the main playbook.
