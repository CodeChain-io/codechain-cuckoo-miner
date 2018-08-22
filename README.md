# CodeChain Cuckoo Miner
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
* N=16, M=8, L=6, **listening to port **3333**, submitting to port **8080** :
```
codechain-cuckoo-miner -n 16 -m 8 -l 6
```

## Configuration

### Common options

| Option | Description                    | Default | Required |
| :----: | ------------------------------ |:-------------:|:--------:|
| `-p`   | Port number to receive job     | 3333 | No |
| `-s`   | Port number to submit solution | 8080 | No |

| Option | Description                    | Default | Required |
| :----: | ------------------------------ |:-------------:|:--------:|
| `-n`   | Number of vertices in graph | None | Yes |
| `-m`   | Number of edges in graph    | None | Yes |
| `-l`   | Length of cycle to detect   | None | Yes |
