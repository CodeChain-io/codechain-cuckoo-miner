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

use codechain_miner::{Config, RpcConfig, Worker};

use super::worker::CuckooWorker;

#[derive(Clone)]
pub struct CuckooConfig {
    pub max_vertex: usize,
    pub max_edge: usize,
    pub cycle_length: usize,
    pub rpc_config: RpcConfig,
    pub concurrent_jobs: u16,
}

impl Config for CuckooConfig {
    fn rpc_config(&self) -> RpcConfig {
        self.rpc_config.clone()
    }

    fn jobs(&self) -> usize {
        self.concurrent_jobs as usize
    }

    fn worker(&self) -> Box<dyn Worker> {
        Box::new(CuckooWorker::new(self.clone()))
    }
}
