use std::path::Path;
use std::vec::Vec;

use configuration::config::ApplicationConfig;

pub fn is_application_config_valid(config: &ApplicationConfig) -> (bool, Vec<&'static str>) {
    let config_path = Path::new(&config.path);
    let mut errors = Vec::new();
    let mut is_ok = true;

    if !config_path.exists() {
        is_ok = false;
        errors.push("Config file not found");
    }

    return (is_ok, errors);
}
