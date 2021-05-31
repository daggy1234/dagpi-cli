# dagpi-cli

A cool little CLI for managing your dagpi instance!

## Install

### Windows

#### with cargo

```shell
cargo install --git https://github.com/Daggy1234/dagpi-cli --branch main dagpi
```

#### with binary

1) Head to releases and download the latest `x86_64-pc-windows-msvc.zip`, extract it and find the `dagpi.exe` binary!
2) Add the path of the exe file to windows path [guide here](https://www.mathworks.com/matlabcentral/answers/94933-how-do-i-edit-my-system-path-in-windows)
3) Restart your terminal and the cli should work
4) CLI can be updated with a simple `dagpi update`

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

Below is a convinence script. For user installation

```shell
curl -s -L https://github.com/Daggy1234/dagpi-cli/releases/download/v0.4.2/dagpi-0.4.2-x86_64-apple-darwin.tar.gz | tar -xvz -O > ~/.local/bin/dagpi
chmod +x ~/.local/bin/dagpi
```

You can auto update this binary also with `dagpi update`.

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
curl -s -L https://github.com/Daggy1234/dagpi-cli/releases/download/v0.4.2/dagpi-0.4.2-x86_64-unknown-linux-gnu.tar.gz | tar -xvz -O > ~/.local/bin/dagpi
chmod +x ~/.local/bin/dagpi
```

## Useage

Just use 
```shell
dagpi -h
```

to view all available commands and help

## Updates

### Installed by cargo

You need to rebuild. Use 

```shell
cargo install --git https://github.com/Daggy1234/dagpi-cli --branch main dagpi
```

### Installed binary

Dagpi binary updates are pretty nifty.  Just do 
```cargo
dagpi update
```
and it should handle the rest!

### Package Managers

Use their update system. Dagpi doesn't attempt to update when installed with package managers.

## Contributing

This is built with rust, so make sure rustfmt is run and all cargo/clippy checks pass.

