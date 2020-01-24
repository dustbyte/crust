# Crust [![Build Status](https://travis-ci.org/mota/crust.svg?branch=master)](https://travis-ci.org/mota/crust)

Chip-8 suite in Rust

![Crust screenshot](misc/crust_emulator.png "Crust Chip-8 Suite")

## Requirements

* SDL2 must be installed on your system

## Components

* emulator (bin: crust)
* disassembler (bin: disassembler, WIP)
* assembler (bin: assembler, WIP)

## Emulator

### Build

```
$ cargo build --bin crust
```

### Usage

```
$ cargo run --bin crust -- -h
crust 0.1.0
Pierre Wacrenier <pierre@wacrenier.me>
A Chip-8 emulator in Rust

USAGE:
    crust [OPTIONS] <ROM>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --cpu-freq <cpu_freq>    Set the frequency of the CPU clock in Hz
    -i, --io-freq <io_freq>      Set the display and buzzer refresh rate in Hz

ARGS:
    <ROM>    path to the rom file
```

### Controls

```
[1][2][3][C]      [1][2][3][4]
[4][5][6][D]  ==  [q][w][e][r]
[7][8][9][E]  ==  [a][s][d][f]
[A][0][B][F]      [z][x][c][v]
```

## License

MIT License

Copyright (c) 2020 Pierre Wacrenier

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
