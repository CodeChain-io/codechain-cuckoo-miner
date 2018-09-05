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
#[macro_use]
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

use codechain_miner::run;

use self::config::CuckooConfig;

fn get_options() -> Result<CuckooConfig, String> {
    let yaml = load_yaml!("./cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();

    let listening_port = value_t!(matches, "listening port", u16).map_err(|_| "Invalid listening port")?;
    let submitting_port = value_t!(matches, "submitting port", u16).map_err(|_| "Invalid submitting port")?;
    let max_vertex = value_t!(matches, "max vertex", usize).map_err(|_| "Invalid max vertex")?;
    let max_edge = value_t!(matches, "max edge", usize).map_err(|_| "Invalid max edge")?;
    let cycle_length = value_t!(matches, "cycle length", usize).map_err(|_| "Invalid cycle length")?;
    let concurrent_jobs = value_t!(matches, "concurrent jobs", u16).map_err(|_| "Invalid concurrent jobs")?;

    Ok(CuckooConfig {
        max_vertex,
        max_edge,
        cycle_length,
        listening_port,
        submitting_port,
        concurrent_jobs,
    })
}

fn main() -> Result<(), String> {
    env_logger::init();
    let worker_config = get_options()?;

    run(worker_config);
    Ok(())
}
