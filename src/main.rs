// Copyright 2018 Kodebox, Inc.
// This file is part of CodeChain.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

extern crate byteorder;
extern crate clap;
extern crate codechain_miner;
extern crate crypto;
extern crate cuckoo;
extern crate env_logger;
extern crate ethereum_types;
#[macro_use]
extern crate log;
extern crate rlp;
extern crate rustc_hex;

mod config;
mod worker;

use clap::{App, Arg};
use codechain_miner::run;

use self::config::CuckooConfig;

fn get_options() -> Result<CuckooConfig, String> {
    let matches = App::new("codechain-cuckoo-miner")
        .args(&[
            Arg::with_name("max vertex")
                .short("n")
                .takes_value(true)
                .required(true),
            Arg::with_name("max edge")
                .short("m")
                .takes_value(true)
                .required(true),
            Arg::with_name("cycle length")
                .short("l")
                .takes_value(true)
                .required(true),
            Arg::with_name("listening port")
                .short("p")
                .global(true)
                .takes_value(true)
                .default_value("3333"),
            Arg::with_name("submitting port")
                .short("s")
                .global(true)
                .takes_value(true)
                .default_value("8080"),
        ])
        .get_matches();

    let listening_port: u16 = matches
        .value_of("listening port")
        .unwrap()
        .parse()
        .map_err(|_| "Invalid listening port")?;
    let submitting_port: u16 = matches
        .value_of("submitting port")
        .unwrap()
        .parse()
        .map_err(|_| "Invalid submitting port")?;
    let max_vertex = matches
        .value_of("max vertex")
        .unwrap()
        .parse()
        .map_err(|_| "Invalid max vertex")?;
    let max_edge = matches
        .value_of("max edge")
        .unwrap()
        .parse()
        .map_err(|_| "Invalid max edge")?;
    let cycle_length = matches
        .value_of("cycle length")
        .unwrap()
        .parse()
        .map_err(|_| "Invalid cycle length")?;

    Ok(CuckooConfig {
        max_vertex,
        max_edge,
        cycle_length,
        listening_port,
        submitting_port,
    })
}

fn main() -> Result<(), String> {
    env_logger::init();
    let worker_config = get_options()?;

    run(worker_config);
    Ok(())
}
