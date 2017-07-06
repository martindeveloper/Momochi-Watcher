use configuration::Currency;
use configuration::GpuProvider;

struct Config {
	executable_path: &'static str,
	pooling_timeout: u32,
	currency: Currency,
	gpu: GpuProvider
}

trait SerializableConfig {
	fn save(&self) -> bool;
	fn load(path: String) -> Self;
}

impl SerializableConfig for Config {
	fn save(&self) -> bool {
		return true;
	}

	fn load(path: String) -> Self {
		return Self { 
			executable_path: "",
			pooling_timeout: 5,
			currency: Currency::ZCash,
			gpu: GpuProvider::Nvidia
		};
	}
}
