# LSX

```
 _     ______  __
| |   / ___\ \/ /
| |   \___ \\  / 
| |___ ___) /  \ 
|_____|____/_/\_\
```

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

## Installation

Choose your preferred installation method:

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
* Print help
    ```Shell
    lsx --help
    ```
* Print version
    ```Shell
    lsx --version
    ```

## Feedback

Found a bug? [Open an issue](https://github.com/desyatkoff/lsx/issues/new)

Want to request a feature? [Start a discussion](https://desyatkoff/lsx/discussions/new?category=ideas)

## Contributing

Refer to [CONTRIBUTING.md](/docs/CONTRIBUTING.md)

## License

Copyright (C) Sergey Desyatkov

LSX is licensed under the GNU General Public License v3.0 or later. See the [LICENSE](/LICENSE) file for more details
