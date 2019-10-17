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

use codechain_miner::{run, HttpConfig, RpcConfig, StratumConfig};

use self::config::CuckooConfig;

macro_rules! hex_value_t {
    ($m:ident, $v:expr) => {{
        let val = value_t!($m, $v, String).map_err(|_| format!("Invalid value"))?;
        if val.len() > 2 && &val[0..2] == "0x" {
            usize::from_str_radix(&val[2..], 16).map_err(|_| format!("'{}' isn't parsed", val))
        } else {
            Err(format!("'{}' isn't a hexadecimal number", val))
        }
    }};
}

fn get_options() -> Result<CuckooConfig, String> {
    let yaml = load_yaml!("./cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();

    let max_vertex = hex_value_t!(matches, "max vertex").map_err(|e| format!("Invalid max vertex: {}", e))?;
    let max_edge = hex_value_t!(matches, "max edge").map_err(|e| format!("Invalid max edge: {}", e))?;
    let cycle_length = value_t!(matches, "cycle length", usize).map_err(|_| "Invalid cycle length")?;
    let concurrent_jobs = value_t!(matches, "concurrent jobs", u16).map_err(|_| "Invalid concurrent jobs")?;

    let rpc_config: RpcConfig = if let Some(ref matches) = matches.subcommand_matches("http") {
        let listening_port = value_t!(matches, "listening port", u16).map_err(|_| "Invalid listening port")?;
        let submitting_port = value_t!(matches, "submitting port", u16).map_err(|_| "Invalid submitting port")?;

        RpcConfig::Http(HttpConfig {
            listen_port: listening_port,
            submitting_port,
        })
    } else if let Some(ref matches) = matches.subcommand_matches("stratum") {
        let server_port = value_t!(matches, "server port", u16).map_err(|_| "Invalid server port")?;
        let miner_name = value_t!(matches, "miner name", String).map_err(|_| "Invalid miner name")?;
        let miner_pwd = value_t!(matches, "miner pwd", String).map_err(|_| "Invalid miner password")?;

        RpcConfig::Stratum(StratumConfig {
            port: server_port,
            id: miner_name,
            pwd: miner_pwd,
        })
    } else {
        return Err("Invalid RPC Config".to_string())
    };

    Ok(CuckooConfig {
        max_vertex,
        max_edge,
        cycle_length,
        rpc_config,
        concurrent_jobs,
    })
}

fn main() -> Result<(), String> {
    env_logger::init();
    let worker_config = get_options()?;

    run(worker_config);
    Ok(())
}
