#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

extern crate clap;

#[macro_use]
extern crate log;
extern crate simplelog;

mod application;
mod configuration;

use std::thread;

use simplelog::{Config, TermLogger, LogLevelFilter};

fn main() {
    // TODO(martin.pernica): Move logger init to separated module
    TermLogger::init(LogLevelFilter::Info, Config::default()).unwrap();

    let title : String = String::from("Momochi Tool - Watcher");

    // TODO(martin.pernica): Move argument parsing to separated module
    let arguments = clap::App::new(title.clone())
        .version("0.1.0")
        .about("Watcher for mining rigs")
        .author("Martin Pernica")
        .arg(clap::Arg::with_name("cfg")
            .help("The configuration file to use")
            .long("config")
            .short("c")
            .takes_value(true)
            .required(true))
        .get_matches();

    let config_path : String = String::from(arguments.value_of("cfg").unwrap());

	application::messages::print_welcome(title.clone(), config_path);

	let mut is_main_thread_running : bool = true;
	let mut is_exit_requested : bool = false;

	while is_main_thread_running {
		thread::yield_now();

		if is_exit_requested {
			break;
		}
	}
}
