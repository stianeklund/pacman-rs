# pacman-rs

[![Build Status](https://travis-ci.com/stianeklund/pacman-rs.svg?branch-master)](https://travis-ci.com/stianeklund/pacman-rs)

A WIP Zilog Z80 CPU Emulator & Pacman arcade emulator written in Rust

Compatible with Windows, Linux, & Mac OS

## Emulator compatibility

* This is a work in progress project ported from [eighty-eighty](https://github.com/stianeklund/eighty-eighty) and does not run any games, yet.
* Interrupts not implemented.
* Passes the preliminary z80 tests & CPUTEST by SuperSoft Associates.



### CPU Tests

#### Diagnostics II v1.2 by by Supersoft Associates (1981):

```
Test loaded: "CPUTEST.COM" Bytes: 19200

DIAGNOSTICS II V1.2 - CPU TEST
COPYRIGHT (C) 1981 - SUPERSOFT ASSOCIATES

ABCDEFGHIJKLMNOPQRSTUVWXYZ
CPU IS Z80
BEGIN TIMING TEST
END TIMING TEST
CPU TESTS OK
```

#### Preliminary z80 Exerciser (by Frank D. Cringle):

```
Test loaded: "tests/prelim.com" Bytes: 1280
Preliminary tests complete Jump to 0 from 0447
```

#### Preliminary 8080 / z80 Exerciser (by Frank D. Cringle, modified by Ian Bartholemew for the 8080*):
``` 
Test loaded: "8080PRE.COM" Bytes: 1024
8080 Preliminary tests complete
Jump to 0 from 032F
```

#### Zexall

```
Not compatible yet.
```
#### Zexdoc

```
Not compatible yet.
```
--- 

### Pacman 

* Rendering not implemented yet.


---

## How to build & run Pacman-rs

#### Running CPU tests:

With Rust & cargo installed:

Run tests from the terminal you can use `cargo test` or, for `stdout` output:
Run all tests: `cargo test -- --nocapture`

#### Running Pacman:
Please make sure you build the project as `release`, otherwise it will run at slow speeds.
You will have to source the rom files on your own.

`cargo run --release pacman.rom` or `cargo run --release pacman.6e pacman.6f pacman.6h pacman.6j pacman.5e pacman.5f`

The emulator supports loading split files or single binaries.

TODO: *SHA /MD5 here.*


If you have multiple files you can merge them with `cat` or `copy \b` for convenience.

##### Merging the rom files into one file:

Linux, Unix, MacOS: 

* `cat pacman.6e pacman.6f pacman.6h pacman.6j pacman.5e pacman.5f > pacman.rom`

Windows:
* `copy /b pacman.6e+pacman.6f+pacman.6h+pacman.6j+pacman.5e+pacman.5f pacman.rom`


---

### References used:

* https://z80.info
* http://www.z80.info/#BASICS_INST
* http://z80.info/zip/z80-documented.pdf
* https://www.lomont.org/software/games/pacman/PacmanEmulation.pdf
* [Z80 test roms](http://mdfs.net/Software/Z80/Exerciser/)
* https://old.reddit.com/r/emudev & the emudev community on Discord.
