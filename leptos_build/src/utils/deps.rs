pub fn check_wasm_bindgen_version(manifest_path: &str) {
    let our_version = wasm_bindgen_shared::version();
    let manifest = std::fs::read_to_string(manifest_path).expect("Manifest path to be a readable file.");
    if let Some(your_version) = manifest
    .lines()
    .filter_map(|l| {
        let version = l
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '.')
            .collect::<String>();

        l.split('=')
            .collect::<Vec<&str>>()
            .first()
            .and_then(|crate_name| {
                if crate_name.contains("wasm-bindgen") {
                    let remaining = crate_name
                        .split("wasm-bindgen")
                        .collect::<Vec<&str>>()
                        .join("");
                    if remaining.split_whitespace().collect::<String>().is_empty() {
                        Some(version)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
    }).next() {
        if our_version != your_version {
            panic!("{}",format!("The wasm-bindgen in your Cargo.toml has a version number {your_version} but the cargo-leptos version is {our_version}. If our version is greater than your version, you should update the wasm-bindgen dependency in your app to use {our_version}. If the opposite, check that you're on the latest cargo-leptos version. If you are and you still get this message, then you'll want to pin the wasm-bindgen dependency to {our_version} in your app. Feel free to notify us and we should have a new version of cargo-leptos out promptly."));
        }
    }
}
