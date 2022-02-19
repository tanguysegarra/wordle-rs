# WORDLE-rs

Tired of listening to people talk during daily meetings?
Pain ends now thanks to this bad reimplementation of the trendy WORDLE game.

## Build it and try

```shell
$ git clone https://github.com/tanguysegarra/wordle-rs
$ cd wordle-rs
$ cargo run
```

## Install the game

```shell
$ git clone https://github.com/tanguysegarra/wordle-rs
$ cd wordle-rs
$ cargo install --path .
```

Don't forget to add `$HOME/.cargo/bin/` to your path.

## WHALE-rs

Use the provided docker image to run it without installing the complete rust toolchain!

```shell
$ docker build . -t wordle
$ docker run --rm -it wordle
```
