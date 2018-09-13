# CodeChain Cuckoo Miner [![Build Status](https://travis-ci.org/CodeChain-io/codechain-cuckoo-miner.svg?branch=master)](https://travis-ci.org/CodeChain-io/codechain-cuckoo-miner)
Mining worker for Cuckoo algorithms in [CodeChain](https://github.com/CodeChain-io/codechain).

## Build

CodeChain cuckoo miner is written in Rust. We recommend setting up a build environment with [rustup](https://rustup.rs/).

To build an executable in release mode, run the following command.
```
cargo build --release
```

The resulting binary file can be found at `target/release/codechain-cuckoo-miner`.

## Usage

```
codechain-cuckoo-miner [OPTIONS] [SUBCOMMAND]
```

### Usage Examples

* N=0x10, M=0x8, L=6, **1** concurrent jobs :

Using HTTP (listening on port **3333**, submitting on port **8080**)
```
codechain-cuckoo-miner -n 0x10 -m 0x8 -l 6 -j 1 http -p 3333 -s 8080
```

or

Using Stratum
```
codechain-cuckoo-miner -n 0x10 -m 0x8 -l 6 -j 1 stratum
```

## Configuration

### Common options

| Option | Description                    | Default | Required |
| :----: | ------------------------------ |:-------------:|:--------:|
| `-n`   | Number of vertices in graph    | None | Yes |
| `-m`   | Number of edges in graph       | None | Yes |
| `-l`   | Length of cycle to detect      | None | Yes |
| `-j`   | Number of concurrent jobs      |    1 | No |

### RPC Subcommands

* HTTP

| Option | Description                    | Default | Required |
| :----: | ------------------------------ |:-------------:|:--------:|
| `-p` 	 | Port number to receive job 	  | 3333 | No |
| `-s`   | Port number to submit solution | 8080 | No |

* Stratum

| Option | Description                    | Default | Required |
| :----: | ------------------------------ |:-------------:|:--------:|
| `-p`   | Port number to stratum server  | 8008 | No |
| `-i`   | Miner name                     |      | No |
| `-w`   | Miner password                 |      | No |
