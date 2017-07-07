pub mod config;
pub mod validator;

use std::collections::HashMap;

extern crate clap;

#[derive(Serialize, Deserialize, Debug)]
pub enum Currency {
    Ethereum,
    ZCash
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GpuProvider {
    Nvidia,
    Amd
}

pub const CONFIG_PATH_KEY : &'static str = "config_path";

pub fn parse_arguments(name: &String) -> HashMap<String, String> {
    let matches = clap::App::new(name.clone())
        .version("0.1.0")
        .about("Watcher for mining rigs")
        .author("Martin Pernica")
        .arg(clap::Arg::with_name(CONFIG_PATH_KEY)
            .default_value("config.yml")
            .help("The configuration file path to load")
            .long("config")
            .short("c")
            .takes_value(true)
            .required(true)
        ).get_matches();

    let mut arguments_map = HashMap::new();

    let config_path : String = matches.value_of(CONFIG_PATH_KEY).unwrap().to_owned();
    arguments_map.insert(CONFIG_PATH_KEY.to_string(), config_path);

    return arguments_map;
}
