# Rustris

This is my attempt at building a Tetris clone in Rust using the Piston game engine. The goal was to
try to emulate the rules of Standard Tetris as closely as possible. The scoring follows these
[guidelines](http://tetris.wikia.com/wiki/Scoring#Recent_guideline_compatible_games), without the
T-Spin rules. Also used [this](http://www.colinfahey.com/tetris/tetris.html) as a reference.

![Rustris](/assets/images/rustris.png?raw=true)

## Build Instructions
Compiles with Rust 1.8.

To build and run the executable:

    cargo run --release

To build the executable:

    cargo build --release

To run the tests:

    cargo test

## Key Bindings
* Left and Right arrows move the tetromino left and right respectively
* Up rotates the tetromino clockwise
* Down increases the rate of decent of the tetromino (soft-drop)
* Space snaps the piece immediately to the location of the ghost-piece (hard-drop)
* P pauses the game
* Escape quits

