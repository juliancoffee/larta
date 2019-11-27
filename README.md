# Larta

A simple utility to get the prompt.

## Install
Installing to your `$CARGO_HOME/bin` (in Linux by default is is `$HOME/.cargo/bin`)
```shell
$ cargo install --git https://github.com/juliancoffee/larta
```
If you want to look at sources or modify it, run:
```shell
$ git clone https://github.com/juliancoffee/larta
$ cargo build
```
Or after cloning run this command from the root directory of a project to install it to `$CARGO_HOME/bin`.
```shell
$ cargo install --path .
```

## How to use
For fish, for example, add to your config.fish
```lua
function fish_prompt
    larta
end
```
If your screen is small, you can use `larta -s`, then path will looks like `~/w/p/src` instead of `~/Workspace/project/src`
