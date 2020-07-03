extern crate dirs;
extern crate winreg;

use std::io;
use std::path::PathBuf;
use winreg::enums::*;
use winreg::RegKey;

pub fn assets_data_dir() -> io::Result<PathBuf> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mtga_reg_subkey = hklm.open_subkey(r"SOFTWARE\WOW6432Node\Wizards of the Coast\MTGArena");
    let path_reg_key: String = mtga_reg_subkey.get_value("Path")?;

    Ok(PathBuf::from(path_reg_key))
}

pub fn logs_dir() -> PathBuf {
    let home_dir = match dirs::home_dir() {
        Some(dir) => dir,
        None => PathBuf::from(r"C:\"),
    };
    let logs_relative_path: PathBuf = PathBuf::from(r"AppData\LocalLow\Wizards Of The Coast\MTGA");

    [home_dir, logs_relative_path].iter().collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assets_data_dir_windows_works() {
        let func_output = assets_data_dir();
        assert!(func_output.is_ok());
        assert_eq!(func_output.unwrap().into_os_string().is_empty(), false);
    }

    #[test]
    fn logs_dir_windows_works() {
        assert_eq!(logs_dir().into_os_string().is_empty(), false);
    }
}
