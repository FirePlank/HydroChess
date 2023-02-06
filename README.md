# HydroChess

HydroChess is a chess engine written in Rust that is capable of beating most intermediate to advanced players.

For variant support I have made a modified version of HydroChess called [Fairy-HydroChess](https://github.com/FirePlank/Fairy-HydroChess). It supports a lot of variants so I suggest you check it out if you are interested!

## Installation

To install HydroChess, you will need to have Rust and Cargo installed on your machine. You can find instructions for installing Rust [here](https://www.rust-lang.org/tools/install).

Once you have Rust and Cargo installed, you can install HydroChess by cloning this repository and running the following command in the root directory:

`cargo build --release`

This will compile the HydroChess code and create an executable in the `target/release` directory.

## Running HydroChess

To run HydroChess, you will need to use a chess interface that is compatible with the [Universal Chess Interface (UCI)](https://en.wikipedia.org/wiki/Universal_Chess_Interface) protocol.

Once you have a UCI-compatible chess interface installed, you can start a game by selecting HydroChess as the engine. The specific steps for doing this will depend on the interface you are using.

## Contributing
I made this project as more of a hobby and more for myself as a challenge, but feel free to suggest any changes or improvements if you so please.
