Getting Started
=================

.. _contribute:

Setup
------------

To use get started, first clone the repository using ``git``:

.. code-block:: console

   $ git clone "https://github.com/SkCubeSat/Software"

or:

.. code-block:: console

   $ git clone "git@github.com:SkCubeSat/Software.git"


Using SSH with Github
------------

As you may already know, Github stopped allowing password authentication for git operations. Instead, you can use SSH keys to authenticate. Here are the steps to set up an SSH key for your Github account. this instructions will mimic the way how you would do on the radsat-server computer. Please tailor the following instructions to your specific needs if performing on your own personal computer:

first, generate a pair of SSH keys. You can skip this step if you want to use an existing key pair. Past the text bellow, replacing the email address in the example with the email address associated with your account on GitHub.:

.. code-block::

   ssh-keygen -t ed25519 -C "your_email@example.com"

Following the prompts, you can choose to add a passphrase to your key pair. This is optional, but it is recommended to add a passphrase for added security on the linuxbox.

Now we will add the ssh key to the ssh-agent. Start the ssh-agent in the background:

.. code-block::

   eval "$(ssh-agent -s)"

Add your SSH private key to the ssh-agent:

.. code-block:: 

   ssh-add ~/.ssh/id_ed25519

Finally, add the SSH key to your GitHub account. Copy the SSH key to your clipboard:

.. code-block:: 

   cat ~/.ssh/id_ed25519.pub

and paste in in Github ssh keys settings. For more info, see `Github's documentation <https://docs.github.com/en/authentication/connecting-to-github-with-ssh/adding-a-new-ssh-key-to-your-github-account>`_.

Contribute
----------

We are always looking for people to help us develop our
``SkCubeSat``. If you would like to help, you can always create a pull request!
If you want to get in touch with us, you can contact our current Software team leads at ``oren.rotaru@usask.ca`` and ``alvan.alam@usask.ca``. 

