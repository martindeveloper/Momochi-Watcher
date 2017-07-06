pub mod configs;

#[derive(Debug)]
pub enum Currency {
    Ethereum,
    ZCash
}

#[derive(Debug)]
pub enum GpuProvider {
    Nvidia,
    Amd
}
