use camino::Utf8PathBuf;

// Check if path to Cargo.toml is valid, and find it's parent
pub fn get_current_dir(path: Option<&Utf8PathBuf>) -> Utf8PathBuf {
    // If path to manifest provided, get directory
    if let Some(manifest_path) = path {
        if manifest_path.is_file() {
            manifest_path
                .parent()
                .expect("This path doesn't have a parent and it should")
                .into()
        } else {
            panic!("A path was provided, but it was not a path to a Cargo.toml file")
        }
    }
    // else provide current directory
    else {
        Utf8PathBuf::from("./")
    }
}