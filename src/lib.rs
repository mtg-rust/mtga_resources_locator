use std::path::PathBuf;

#[cfg_attr(target_os="macos", path="path_locator/macos.rs")]
#[cfg_attr(target_os="windows", path="path_locator/windows.rs")]
mod path_locator;


pub fn assets_data_dir() -> Option<PathBuf> {
    Some(path_locator::assets_data_dir())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assets_data_dir_works() {
        assert_eq!(assets_data_dir().is_some(), true);
        assert_eq!(assets_data_dir().expect("Value is empty").into_os_string().is_empty(), false);
    }

}
