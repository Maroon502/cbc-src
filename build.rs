use std::env;
use std::path::{Path, PathBuf};

use coin_build_tools::{utils, link, coinbuilder};

const LIB_NAME: &str = "Cbc";

fn main() {
    println!("cargo:rerun-if-changed={}_lib_sources.txt", LIB_NAME.to_ascii_lowercase());
    println!("cargo:rerun-if-env-changed=CARGO_{}_STATIC", LIB_NAME.to_ascii_uppercase());
    println!("cargo:rerun-if-env-changed=CARGO_{}_SYSTEM", LIB_NAME.to_ascii_uppercase());

    link::link_lib_system_if_defined(LIB_NAME);

    if !Path::new(&format!("{}/AUTHORS", LIB_NAME)).exists() {
        utils::update_submodules(env::var("CARGO_MANIFEST_DIR").unwrap());
    }
    build_lib_and_link();
}

fn build_lib_and_link() {
    let mut config = coinbuilder::init_builder();

    let src_dir = format!(
        "{}",
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join(LIB_NAME)
            .join(LIB_NAME)
            .join("src")
            .display()
    );

    let mut includes_dir = vec![
        format!("{}", src_dir),
    ];

    let mut lib_sources = include_str!("cbc_lib_sources.txt")
        .trim()
        .split('\n')
        .map(|file| format!("{}/{}", src_dir, file.trim()))
        .collect::<Vec<String>>();

    let mut coinflags = vec!["CBC".to_string()];

    {
        lib_sources.push(format!("{}/OsiCbc/OsiCbcSolverInterface.cpp", src_dir));
        includes_dir.push(format!("{}/OsiCbc", src_dir));
        coinflags.push("OSICBC".to_string());
    }

    if cfg!(feature = "cbcsolver") {
        let lib_sources_cbcsolver = include_str!("cbcsolver_lib_sources.txt")
        .trim()
        .split('\n')
        .map(|file| format!("{}/{}", src_dir, file.trim()))
        .collect::<Vec<String>>();

        lib_sources.extend(lib_sources_cbcsolver);
    }

    coinbuilder::print_metedata(includes_dir.clone(), coinflags.clone());

    let (include_other, coinflags_other) = coinbuilder::get_metedata_from("CoinUtils");
    includes_dir.extend(include_other);
    coinflags.extend(coinflags_other);

    let (include_other, coinflags_other) = coinbuilder::get_metedata_from("Osi");
    includes_dir.extend(include_other);
    coinflags.extend(coinflags_other);

    let (include_other, coinflags_other) = coinbuilder::get_metedata_from("Clp");
    includes_dir.extend(include_other);
    coinflags.extend(coinflags_other);

    let (include_other, coinflags_other) = coinbuilder::get_metedata_from("Cgl");
    includes_dir.extend(include_other);
    coinflags.extend(coinflags_other);

    coinflags.iter().for_each(|flag| {
        config.define(&format!("COIN_HAS_{}", flag), None);
    });
    config.includes(includes_dir);
    config.files(lib_sources);

    config.compile("Cbc");
}
