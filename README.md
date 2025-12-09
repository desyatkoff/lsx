# LSX

![](/assets/demo.png)

<p align="center">
    &ensp;<a href="#description"><kbd>&ensp;<br />&ensp;&ensp;Description&ensp;&ensp;<br />&ensp;</kbd></a>&ensp;
    &ensp;<a href="#features"><kbd>&ensp;<br />&ensp;&ensp;Features&ensp;&ensp;<br />&ensp;</kbd></a>&ensp;
    &ensp;<a href="#installation"><kbd>&ensp;<br />&ensp;&ensp;Installation&ensp;&ensp;<br />&ensp;</kbd></a>&ensp;
    &ensp;<a href="#usage"><kbd>&ensp;<br />&ensp;&ensp;Usage&ensp;&ensp;<br />&ensp;</kbd></a>&ensp;
    &ensp;<a href="#feedback"><kbd>&ensp;<br />&ensp;&ensp;Feedback&ensp;&ensp;<br />&ensp;</kbd></a>&ensp;
    &ensp;<a href="#contributing"><kbd>&ensp;<br />&ensp;&ensp;Contributing&ensp;&ensp;<br />&ensp;</kbd></a>&ensp;
    &ensp;<a href="#license"><kbd>&ensp;<br />&ensp;&ensp;License&ensp;&ensp;<br />&ensp;</kbd></a>&ensp;
</p>

## Description

Imagine ls command, but better

## Features

* Lists files and directories
* Option to group directories before or after other files
* Options to show:
    + Permissions
    + Owner
    + Size
    + Date modified
    + Total entries count
* Option to enable table view instead of classic list view
* Option to print colorful output

## Installation

If you use Arch Linux, choose one of these installation methods (note that the AUR package is `ls-x`, not `lsx`):

* Install from AUR using `yay` helper
    ```Shell
    yay -S ls-x
    ```
* Install from AUR without using `yay` helper:
    ```Shell
    git clone https://aur.archlinux.org/ls-x.git && cd ls-x/ && makepkg -si
    ```

If you use some other OS, choose one of these installation methods:

* `git clone` the repository and launch installer script
    ```Shell
    git clone https://github.com/desyatkoff/lsx.git && cd lsx/ && bash ./install.sh
    ```
* `curl` the installer script
    ```Shell
    bash <(curl -fsSL https://raw.githubusercontent.com/desyatkoff/lsx/main/install.sh)
    ```

## Usage

Here is the basic usage guide:

* Do not ignore entries starting with `.`
    ```Shell
    lsx --all
    ```
* List directories before other files
    ```Shell
    lsx --group-directories-first
    ```
* List directories after other files
    ```Shell
    lsx --group-directories-last
    ```
* Enable every `--show-*` option below
    ```Shell
    lsx --show-all-columns
    ```
* Show entry permissions column
    ```Shell
    lsx --show-permissions
    ```
* Show entry owner column
    ```Shell
    lsx --show-owner
    ```
* Show entry size column
    ```Shell
    lsx --show-size
    ```
* Show entry date modified column
    ```Shell
    lsx --show-date-modified
    ```
* Show total entries count
    ```Shell
    lsx --show-total
    ```
* Use table view
    ```Shell
    lsx --table
    ```
* Colorize output
    ```Shell
    lsx --colors
    ```
* Print help
    ```Shell
    lsx --help
    ```
* Print version
    ```Shell
    lsx --version
    ```

It is highly recommended to make an alias for LSX in your shell configuration file so you don't have to type your favorite options every type. For me, it's `alias lsx="lsx --all --group-directories-first --show-all-columns --table --colors"`, with this line in my `~/.zshrc` I can just type `lsx` instead of the long command with all the options!

## Feedback

Found a bug? [Open an issue](https://github.com/desyatkoff/lsx/issues/new)

Want to request a feature? [Start a discussion](https://desyatkoff/lsx/discussions/new?category=ideas)

## Contributing

Refer to [CONTRIBUTING.md](/docs/CONTRIBUTING.md)

## License

Copyright (C) Sergey Desyatkov

LSX is licensed under the GNU General Public License v3.0 or later. See the [LICENSE](/LICENSE) file for more details
