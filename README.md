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
codechain-cuckoo-miner [OPTIONS]
```

### Usage Examples
* N=0x10, M=0x8, L=6, listening on port **3333**, submitting on port **8080**, **1** concurrent jobs :
```
codechain-cuckoo-miner -n 0x10 -m 0x8 -l 6 -p 3333 -s 8080 -j 1
```

## Configuration

### Common options

| Option | Description                    | Default | Required |
| :----: | ------------------------------ |:-------------:|:--------:|
| `-p`   | Port number to receive job     | 3333 | No |
| `-s`   | Port number to submit solution | 8080 | No |
| `-j`   | The number of concurrent jobs  |    1 | No |

| Option | Description                    | Default | Required |
| :----: | ------------------------------ |:-------------:|:--------:|
| `-n`   | Number of vertices in graph | None | Yes |
| `-m`   | Number of edges in graph    | None | Yes |
| `-l`   | Length of cycle to detect   | None | Yes |
