extern crate winreg;

pub fn assets_data_dir() -> PathBuf {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mtga_regkey = hklm.open_subkey_with_flags(r"SOFTWARE\WOW6432Node\Wizards of the Coast\MTGArena",
        KEY_READ).unwrap();
    PathBuf::from(mtga_regkey.get_value("Path").unwrap())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assets_data_dir_windows_works() {
        assert_eq!(assets_data_dir().into_os_string().is_empty(), false);
    }
}