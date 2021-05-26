# dagpi-cli

A cool little CLI for managing your dagpi instance!

## Install

### Windows

#### with cargo

```shell
cargo install --git https://github.com/Daggy1234/dagpi-cli --branch main dagpi
```

#### with binary

1) Head to releases and download the latest `dagpi-windows.exe`
2) Add the path of the exe file to windows path [guide here](https://www.mathworks.com/matlabcentral/answers/94933-how-do-i-edit-my-system-path-in-windows)
3) Restart your terminal and the cli should work

### Macos

#### with cargo

```shell
cargo install --git https://github.com/Daggy1234/dagpi-cli --branch main dagpi
```

#### with homebrew

```shell
brew tap daggy1234/tap
brew install dagpi --verbose
```

#### binary

1) Install the latest macos binary `dagpi-macos`
2) Give it executeable permission 
3) Move to path
4) Test it out

Below is a convinence script. Run at your own risk

```shell
wget https://github.com/Daggy1234/dagpi-cli/releases/download/v0.3.0/dagpi-macos
chmod +x ./dagpi-macos
sudo mv ./dagpi-macos /usr/local/bin/dagpi
sudo chown root: /usr/local/bin/dagpi
dagpi -V
```

### Linux

#### with cargo

```shell
cargo install --git https://github.com/Daggy1234/dagpi-cli --branch main dagpi
```

#### Binary

1) Download the binary for your CPU architecture (x86_64, aarch)
2) Add executeable perms to the binary
3) Copy binary to your user's path
4) Test it. Script below

This for x86_64 linux. This will not work on arm. Please Check if your computer is arm (rasberrypi's are arm).

```shell
wget https://github.com/Daggy1234/dagpi-cli/releases/download/v0.3.0/dagpi-linux
chmod +x ./dagpi-linux
sudo install -o root -g root -m 0755 dagpi-linux /usr/local/bin/dagpi
dagpi -V
```

## Useage

Just use 
```shell
dagpi -h
```

to view all available commands and help

## Contributing

This is built with rust, so make sure rustfmt is run and all cargo/clippy checks pass.

