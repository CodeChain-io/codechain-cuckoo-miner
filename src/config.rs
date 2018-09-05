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

use codechain_miner::{Config, HttpConfig, RpcConfig, Worker};

use super::worker::CuckooWorker;

#[derive(Clone, Copy)]
pub struct CuckooConfig {
    pub max_vertex: usize,
    pub max_edge: usize,
    pub cycle_length: usize,

    pub listening_port: u16,
    pub submitting_port: u16,

    pub concurrent_jobs: u16,
}

impl Config for CuckooConfig {
    fn rpc_config(&self) -> RpcConfig {
        RpcConfig::Http(HttpConfig {
            listen_port: self.listening_port,
            submitting_port: self.submitting_port,
        })
    }

    fn jobs(&self) -> usize {
        self.concurrent_jobs as usize
    }

    fn worker(&self) -> Box<Worker> {
        Box::new(CuckooWorker::new(*self))
    }
}
