use std::env;
use std::path::{Path, PathBuf};

use coin_build_tools::{coinbuilder, link, utils};

const LIB_NAME: &str = "Cbc";

fn main() {
    println!(
        "cargo:rerun-if-changed={}_lib_sources.txt",
        LIB_NAME.to_ascii_lowercase()
    );
    println!(
        "cargo:rerun-if-env-changed=CARGO_{}_STATIC",
        LIB_NAME.to_ascii_uppercase()
    );
    println!(
        "cargo:rerun-if-env-changed=CARGO_{}_SYSTEM",
        LIB_NAME.to_ascii_uppercase()
    );

    let want_system = utils::want_system(LIB_NAME);

    if want_system && link::link_lib_system_if_supported(LIB_NAME) {
        let mut coinflags = vec!["CBC".to_string()];

        let link_type = if utils::want_static(LIB_NAME) {
            "static".to_string()
        } else {
            "dylib".to_string()
        };

        if cfg!(feature = "osicbc") {
            println!("cargo:rustc-link-lib={}=OsiCbc", link_type);
            coinflags.push("OSICBC".to_string());
        }

        if cfg!(feature = "cbcsolver") {
            println!("cargo:rustc-link-lib={}=CbcSolver", link_type);
            coinflags.push("CBCSOLVER".to_string());
        }

        coinbuilder::print_metedata(Vec::new(), coinflags);
        return;
    }

    if !Path::new(&format!("{}/LICENSE", LIB_NAME)).exists() {
        utils::update_submodules(env::var("CARGO_MANIFEST_DIR").unwrap());
    }
    build_lib_and_link();
}

fn build_lib_and_link() {
    let src_dir = format!(
        "{}",
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join(LIB_NAME)
            .join(LIB_NAME)
            .join("src")
            .display()
    );

    let mut includes_dir = vec![format!("{}", src_dir)];

    let mut lib_sources = include_str!("cbc_lib_sources.txt")
        .trim()
        .split('\n')
        .map(|file| format!("{}/{}", src_dir, file.trim()))
        .collect::<Vec<String>>();

    let mut coinflags = vec!["CBC".to_string()];

    if cfg!(feature = "osicbc") {
        lib_sources.push(format!("{}/OsiCbc/OsiCbcSolverInterface.cpp", src_dir));
        includes_dir.push(format!("{}/OsiCbc", src_dir));
        coinflags.push("OSICBC".to_string());
    }

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

    coinbuilder::print_metedata(includes_dir.clone(), coinflags.clone());

    let mut config = coinbuilder::init_builder();
    if cfg!(feature = "parallel") {
        config.define("CBC_THREAD", None);
    }
    coinflags.iter().for_each(|flag| {
        config.define(&format!("COIN_HAS_{}", flag), None);
    });
    config.includes(includes_dir.clone());
    config.files(lib_sources);

    config.compile("Cbc");

    if cfg!(feature = "cbcsolver") {
        let lib_sources = include_str!("cbcsolver_lib_sources.txt")
            .trim()
            .split('\n')
            .map(|file| format!("{}/{}", src_dir, file.trim()))
            .collect::<Vec<String>>();

        let mut config = coinbuilder::init_builder();
        coinflags.iter().for_each(|flag| {
            config.define(&format!("COIN_HAS_{}", flag), None);
        });
        config.define("CBC_THREAD_SAFE", None);
        if cfg!(feature = "parallel") {
            config.define("CBC_THREAD", None);
        }
        config.includes(includes_dir);
        config.files(lib_sources);
        config.compile("CbcSolver");
    }
}
