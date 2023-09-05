#![feature(fs_try_exists)]

use std::{ffi::OsString, io::Write};

fn get_install_path() -> impl Iterator<Item = (&'static str, OsString)> {
    get_download_name().iter().map(|download_name| {
        let mut path: std::path::PathBuf = std::env::var("OUT_DIR").unwrap().into();
        path.push(download_name);
        (*download_name, path.into_os_string())
    })
}

fn get_download_name() -> &'static [&'static str] {
    #[cfg(target_family = "unix")]
    {
        return &["libNRD.a"];
    }
    #[cfg(target_family = "windows")]
    {
        return &["NRD.lib", "NRD.dll"];
    }
    #[allow(unreachable_code)]
    {
        panic!()
    }
}

fn main() {
    for (download_name, install_path) in get_install_path() {
        if !std::fs::try_exists(&install_path).expect("Unable to check library file location") {
            let data = sysreq::get(format!(
                "https://github.com/dust-engine/NRD/releases/download/v0.1/{}",
                download_name
            ))
            .expect("Download file error");
            let mut file =
                std::fs::File::create(install_path).expect("Unable to create library file");
            file.write_all(&data).expect("Unable to write library file");
        }
    }

    println!("cargo:rustc-link-lib=NRD");
    println!(
        "cargo:rustc-link-search={}",
        std::env::var("OUT_DIR").unwrap()
    );
}
