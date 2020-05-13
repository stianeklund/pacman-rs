# pacman-rs

[![Build Status](https://travis-ci.com/stianeklund/pacman-rs.svg?branch-master)](https://travis-ci.com/stianeklund/pacman-rs)

A WIP Zilog Z80 CPU Emulator & Pacman arcade emulator written in Rust

Compatible with Windows, Linux & Mac OS


## Emulator compatibility

* This is a work in progress project and does not run any games or pass major CPU tests, yet.



### CPU Tests

#### Diagnostics II v1.2 by by Supersoft Associates (1981):

* Not passing yet due to R register issues.
```
Test loaded: "CPUTEST.COM" Bytes: 19200

DIAGNOSTICS II V1.2 - CPU TEST
COPYRIGHT (C) 1981 - SUPERSOFT ASSOCIATES

ABCDEFGHIJKLMNOPQRSTUVWXYZ
CPU IS Z80
BEGIN TIMING TEST
END TIMING TEST
```


#### Microcosm Associates 8080/8085 CPU Diagnostics v1.0:
```
Test loaded: "TEST.COM" Bytes: 1793
MICROCOSM ASSOCIATES 8080/8085 CPU DIAGNOSTIC VERSION 1.0  (C) 1980

CPU IS OPERATIONAL
Jump to 0 from 014F
test test::tests::test_com ... ok
```

#### Preliminary 8080 / z80 Exerciser (by by Frank D. Cringle, modified by Ian Bartholemew for the 8080*):
``` 
Test loaded: "8080PRE.COM" Bytes: 1024
8080 Preliminary tests complete
Jump to 0 from 032F
test test::tests::preliminary ... ok
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

* Rendering not implemented.


---

## How to build & run Pacman-rs

#### Running CPU tests:

With Rust & cargo installed:

Run tests from the terminal you can use `cargo test` or, for `stdout` output:

Test names:
* `test::tests::preliminary`
* `test::tests::cpu_exer`
* `test::tests::cpu_test`
* `test::tests::test_com`


Run all tests: `cargo test -- --nocapture`
Specific tests: `cargo test --package eighty-eighty --bin pacman-rs test::tests::preliminary -- --nocapture --exact`

#### Running Pacman:
Please make sure you build the project as `release`, otherwise it will run at slow speeds.
You will have to source the rom files on your own.

`cargo run --release pacman.rom` or `cargo run --release pacman placeholder...`
The emulator supports loading split files or single binaries.

*SHA /MD5 here.*


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
* https://www.lomont.org/software/games/pacman/PacmanEmulation.pdf
* [Z80 test roms](http://mdfs.net/Software/Z80/Exerciser/)
