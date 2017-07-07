extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::Read;

use configuration::Currency;
use configuration::GpuProvider;

#[derive(Default)]
pub struct ApplicationConfig {
	pub path: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileConfig {
	pub executable_path: String,
	pub pooling_timeout: u32,
	pub currency: Currency,
	pub gpu: GpuProvider
}

impl FileConfig {
    pub fn new_from_file(path: &String) -> FileConfig {
        let mut file = File::open(path).expect("Unable to open the config file");
        let mut contents = String::new();

        file.read_to_string(&mut contents).expect("Unable to read the config file");

        let deserialized: FileConfig = serde_json::from_str(&contents).unwrap();

        return deserialized;
    }
}
