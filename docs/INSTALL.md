# Installation

## Using a package manager
One day.. Just not today

## Using the release page
Press the releases button on the github repository, and download the latest release.
Untar it, and you're done, you can now run lymap! Warning that if you do not use the makefile you'll need to make your own desktop file. <br>

But you can also use the Makefile, Which is located in the directory you just downloaded. <br>
Make sure to install [make](https://www.gnu.org/software/make/manual/make.html) on your system and simply do:
```bash
cd path/to/lymap
sudo make binary_install
``` 
And it's done! This will create a global lymap executable and install the assets in a much more convinient place.

## Compiling from source
Lymap only has a few dependencies: rustup, make, git and the X11 build dependencies.

You can install rustup by going [here](https://rustup.rs/). <br>
This boils down to opening a terminal and running
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Most linux distributions already come with make and git preinstalled. But you can install them using your package manager. Below are a few examples.
```bash
# Debian/Ubuntu
sudo apt install make git libX11-dev
```

```bash
# Fedora
sudo dnf install make git libX11-devel
```

Now that our dependencies are installed, we can proceed to the installation.

```bash
git clone https://github.com/its-Lyn/lymap
cd lymap

cargo build --release
sudo make install
```

Now, you could also `cargo clean` after running the install make script. This will remove the `target/` directory and will save a lot of space.

## Uninstalling lymap
Simply run the following command in the folder you downloaded.
```bash
sudo make uninstall
```
