#!/usr/bin/env bash


set -euo pipefail

IFS=$'\n\t'

if ! command -v sudo &>/dev/null; then
    if command -v doas &>/dev/null; then
        alias sudo="doas"
    else
        exit 1
    fi
fi

if ! command -v rustup &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

    export PATH="$PATH:$HOME/.cargo/bin"
fi

if [[ ! -d ".git" && ! -f "install.sh" ]]; then
    if [[ ! -d "lsx" ]]; then
        if ! command -v git &>/dev/null; then
            if command -v pacman &>/dev/null; then
                sudo pacman -S --noconfirm git
            elif command -v apt &>/dev/null; then
                sudo apt update
                sudo apt install -y git
            elif command -v dnf &>/dev/null; then
                sudo dnf install -y git
            elif command -v zypper &>/dev/null; then
                sudo zypper install -y git
            elif command -v xbps-install &>/dev/null; then
                sudo xbps-install -Sy git
            elif command -v eopkg &>/dev/null; then
                sudo eopkg install -y git
            elif command -v apk &>/dev/null; then
                sudo apk add git
            elif command -v nix-env &>/dev/null; then
                nix-env -iA nixpkgs.git
            else
                exit 1
            fi
        fi

        git clone https://github.com/desyatkoff/lsx.git
    fi

    cd lsx/
fi

if ! command -v make &>/dev/null; then
    if command -v pacman &>/dev/null; then
        sudo pacman -S --noconfirm make
    elif command -v apt &>/dev/null; then
        sudo apt update
        sudo apt install -y make
    elif command -v dnf &>/dev/null; then
        sudo dnf install -y make
    elif command -v zypper &>/dev/null; then
        sudo zypper install -y make
    elif command -v xbps-install &>/dev/null; then
        sudo xbps-install -Sy make
    elif command -v eopkg &>/dev/null; then
        sudo eopkg install -y make
    elif command -v apk &>/dev/null; then
        sudo apk add make
    elif command -v nix-env &>/dev/null; then
        nix-env -iA nixpkgs.make
    else
        exit 1
    fi
fi

sudo make clean
make build
sudo make install
