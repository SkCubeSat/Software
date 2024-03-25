Getting Started
===============

This section will guide you through the initial setup required to start contributing to the `radsatsk2` project documentation. Before you begin, ensure you have a recent version of Python and pip installed on your computer.

Prerequisites
-------------

Before you can start contributing to the documentation, you need to have the following prerequisites installed:

- **Python**: A recent version of Python must be installed on your computer. You can download Python from a package manager or from `The Python official website <https://www.python.org/downloads/>`__

- **pip**: The Python package installer, pip, should be installed along with Python. You can check if pip is installed by running `pip --version` in your command line or terminal.

Setting Up Your Environment
---------------------------

Once you have the prerequisites, you need to install a few Python packages that are essential for working with Sphinx documentation, specifically `sphinx-autobuild` for hosting a local live server, `lumache`, and `sphinx_rtd_theme` for the theme.

1. Open your terminal or command line.

2. Ensure you are in your project directory or navigate to it using:

.. code-block:: console

   cd path/to/your/project

3. Install the necessary packages using pip:

.. code-block::

   pip install sphinx sphinx-autobuild lumache sphinx_rtd_theme

This command installs Sphinx, the sphinx-autobuild package for live reloading your documentation as you make changes, the `lumache` package , and the `sphinx_rtd_theme`.

Starting a Local Live Server
----------------------------

To preview your changes live as you edit the documentation:

1. Within `docs` directory, run the following command:

.. code-block:: 

   cd docs
   sphinx-autobuild source source/_build/html

2. Open your browser and navigate to :code:`http://127.0.0.1:8000/` to view the documentation.

3. Alternatively, if you want to serve the documentation from the `radsat-server` computer. You can simply specify the host of the tailscale ip address. You can verify the ip address of the device you are on by running ``ifconfig`` and looking for ``tailscale0``. The ip address will be listed under ``inet``. Once you've determined the ip you can run the command below:

.. code-block::

   sphinx-autobuild source source/_build/html --host 100.91.27.26

Replace the ip address with the one you found from the ``ifconfig`` command.
This way you dont even have to run the server on your local machine and instead you can use the linuxbox to serve the documentation while you are working in it.


