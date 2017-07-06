use std;

pub fn print_welcome(title : String, config_path : String) {
	let title_length : usize = title.len();
	let title_underline : String = std::iter::repeat("-").take(title_length).collect::<String>();

	info!("{0}", title);
    info!("{0}", title_underline);

    info!("Using config at path: {0}", config_path);
}
