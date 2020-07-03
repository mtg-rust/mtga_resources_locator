![](https://github.com/mtg-rust/mtga_resources_locator/workflows/Tests/badge.svg)


# MTGA Resources Locator
Utility that returns where MTGA data is stored.

Disclaimer: this project started as a way to learn Rust, so this IS full of bugs and anit-patterns and stupid code.

The library exposes two functions:
* `assets_data_dir()`: returns the `PathBuf` relative to the folder where MTGA JSON files are stored;
* `logs_dir()`: returns the `PathBuf` relative to the folder where logs are stored.

The method should work on both Windows and macOS. Actually only macOS has been tested.
