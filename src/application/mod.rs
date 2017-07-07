pub mod log;

use std;
use std::thread;
use configuration::config::ApplicationConfig;
use configuration::config::FileConfig;
use configuration::validator::is_application_config_valid;

pub struct Application<'a> {
    name: &'a String,
    application_config: ApplicationConfig,
    watcher_config: Option<FileConfig>,
    is_exit_requested: bool
}

impl<'a> Application<'a> {
    pub fn new(name: &'a String, config: ApplicationConfig) -> Application<'a> {
        Application { name: name, application_config: config, is_exit_requested: false, watcher_config: None }
    }

    pub fn init(&self) {
        self.configure_log();
        self.print_welcome();
    }

    pub fn run(&self) -> i32 {
        let (is_config_valid, config) = self.load_config();

        if !is_config_valid || config.is_none() {
            return -1;
        }

        let config = config.unwrap();

        info!("Executable: {0} | Pooling time: {1} | Currency: {2:?} | GPU: {3:?}", config.executable_path, config.pooling_timeout, config.currency, config.gpu);

        self.spawn_api_thread();
        self.spawn_watcher_thread();

        let is_main_thread_running : bool = true;

        while is_main_thread_running {
            thread::yield_now();

            if self.is_exit_requested {
                break;
            }
        }

        return 0;
    }

    pub fn terminate(&mut self) {
        self.is_exit_requested = true;
    }

    fn load_config(&self) -> (bool, Option<FileConfig>) {
        let (is_config_valid, config_errors) = is_application_config_valid(&self.application_config);

        if !is_config_valid {
            error!("Config file is not valid, following problems were found:");

            for error_message in config_errors {
                error!(" - {}", error_message);
            }

            return (false, None);
        } else {
            info!("Config file is valid");
        }

        let config = FileConfig::new_from_file(&self.application_config.path);

        return (true, Some(config));
    }

    fn spawn_watcher_thread(&self) {
        info!("Spawning watcher thread");
    }

    fn spawn_api_thread(&self) {
        info!("Spawning remote API thread");
    }

    fn configure_log(&self) {
        self::log::init_logger();
    }

    fn print_welcome(&self) {
        let title_length : usize = self.name.len();
        let title_underline : String = std::iter::repeat("-").take(title_length).collect::<String>();

        info!("{0}", self.name);
        info!("{0}", title_underline);

        info!("Using config at path: {0}", self.application_config.path);

        log::flush_logger();
    }
}
