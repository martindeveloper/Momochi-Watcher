use std::vec::Vec;

pub struct CurrentUsage {
    temperatures: vec::Vec<u16>,
    rates: vec::Vec<u32>,
    is_miner_running: bool
}

impl CurrentUsage {
    fn new() {
        CurrentUsage { temperatures: vec::Vec::new(), rates: vec::Vec::new(), is_miner_running: false }
    }
}
