extern crate dirs;

use std::io;
use std::path::PathBuf;

pub fn assets_data_dir() -> io::Result<PathBuf> {
    let home_dir = get_home_dir();
    let game_relative_path: PathBuf = PathBuf::from("Library/Application Support/com.wizards.mtga/Downloads/Data");

    Ok([home_dir, game_relative_path].iter().collect())
}

pub fn logs_dir() -> PathBuf {
    let home_dir = get_home_dir();
    let game_relative_path: PathBuf = PathBuf::from("Library/Logs/Wizards Of The Coast/MTGA");

    [home_dir, game_relative_path].iter().collect()
}

fn get_home_dir() -> PathBuf {
    match dirs::home_dir() {
        Some(dir) => dir,
        None => PathBuf::from("~"),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assets_data_dir_macos_works() {
        let func_output = assets_data_dir();
        assert!(func_output.is_ok());
        assert_eq!(func_output.unwrap().into_os_string().is_empty(), false);
    }

    #[test]
    fn logs_dir_macos_works() {
        assert_eq!(logs_dir().into_os_string().is_empty(), false);
    }
}
