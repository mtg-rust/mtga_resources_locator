use std::io;
use std::path::PathBuf;

#[cfg_attr(target_os="macos", path="path_locator/macos.rs")]
#[cfg_attr(target_os="windows", path="path_locator/windows.rs")]
mod path_locator;


pub fn assets_data_dir() -> io::Result<Option<PathBuf>> {
    let data_dir_result = path_locator::assets_data_dir()?;
    Ok(Some(data_dir_result))
}

pub fn logs_dir() -> Option<PathBuf> {
    Some(path_locator::logs_dir())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assets_data_dir_works() {
        let func_output = assets_data_dir();
        assert!(func_output.is_ok());
        let data_dir = func_output.unwrap();
        assert_eq!(data_dir.is_some(), true);
        assert_eq!(data_dir.expect("Value is empty").into_os_string().is_empty(), false);
    }


    #[test]
    fn logs_dir_works() {
        assert_eq!(logs_dir().is_some(), true);
        assert_eq!(logs_dir().expect("Value is empty").into_os_string().is_empty(), false);
    }
}
