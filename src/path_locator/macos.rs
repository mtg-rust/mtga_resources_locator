use std::path::PathBuf;

extern crate dirs;


pub fn assets_data_dir() -> PathBuf {
    let home_dir = get_home_dir();
    let game_relative_path: PathBuf = PathBuf::from("Library/Application Support/com.wizards.mtga/Downloads/Data");

    [home_dir, game_relative_path].iter().collect()
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
        assert_eq!(assets_data_dir().into_os_string().is_empty(), false);
    }

    #[test]
    fn logs_dir_macos_works() {
        assert_eq!(logs_dir().into_os_string().is_empty(), false);
    }
}
