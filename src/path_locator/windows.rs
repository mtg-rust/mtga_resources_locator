use std::path::PathBuf;

extern crate dirs;
extern crate winreg;

pub fn assets_data_dir() -> PathBuf {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mtga_regkey = hklm.open_subkey_with_flags(r"SOFTWARE\WOW6432Node\Wizards of the Coast\MTGArena",
        KEY_READ).unwrap();
    PathBuf::from(mtga_regkey.get_value("Path").unwrap())
}

pub fn logs_dir() -> PathBuf {
    let home_dir = Some(dirs::home_dir());
    let logs_relative_path: PathBuf = PathBuf::from(r"AppData\LocalLow\Wizards Of The Coast\MTGA");

    [home_dir, logs_relative_path].iter().collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assets_data_dir_windows_works() {
        assert_eq!(assets_data_dir().into_os_string().is_empty(), false);
    }

    #[test]
    fn logs_dir_windows_works() {
        assert_eq!(logs_dir().into_os_string().is_empty(), false);
    }
}
