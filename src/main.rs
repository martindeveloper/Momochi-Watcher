#![allow(dead_code)]
#![allow(unused_variables)]

extern crate clap;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

mod application;
mod configuration;

fn main() {
    let title : String = String::from("Momochi Tool - Watcher");

    let arguments = configuration::parse_arguments(&title);
    let config_path = arguments.get("config_path").unwrap().clone();
    let config : configuration::config::ApplicationConfig = configuration::config::ApplicationConfig { path: config_path };

	let app : application::Application = application::Application::new(&title, config);

	app.init();
    let exit_code = app.run();

    std::process::exit(exit_code);
}
