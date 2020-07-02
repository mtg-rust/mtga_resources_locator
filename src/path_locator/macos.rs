use std::path::PathBuf;

extern crate dirs;


pub fn assets_data_dir() -> PathBuf {
    let home_dir = match dirs::home_dir() {
        Some(dir) => dir,
        None => PathBuf::from("~"),
    };
    let game_relative_path: PathBuf = PathBuf::from("Library/Application Support/com.wizards.mtga/Downloads/Data");

    [home_dir, game_relative_path].iter().collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assets_data_dir_macos_works() {
        assert_eq!(assets_data_dir().into_os_string().is_empty(), false);
    }
}
