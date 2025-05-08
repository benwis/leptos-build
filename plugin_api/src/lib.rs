pub mod bindings {

    use wit_bindgen::generate;
    generate!({path: "./wit/leptos-build-plugin.wit", pub_export_macro: true,export_macro_name: "export", });
}

pub use crate::bindings::{export, Guest as LeptosBuildPlugin};
