#[cfg(feature = "gen")]
extern crate bindgen;

#[cfg(not(feature = "gen"))]
fn main() {}

#[cfg(feature = "gen")]
fn main() {
    use std::env;
    use std::path::Path;

    // Setup bindings builder
    let generated = bindgen::builder()
        .header("include/gbm.h")
        .ctypes_prefix("libc")
        .whitelist_type(r"^gbm_.*$")
        .whitelist_function(r"^gbm_.*$")
        .constified_enum("gbm_bo_flags")
        .constified_enum_module("gbm_bo_transfer_flags")
        .generate()
        .unwrap();

    println!("cargo:rerun-if-changed=include/gbm.h");

    // Generate the bindings
    let out_dir = env::var("OUT_DIR").unwrap();
    let bind_name = "gen.rs";
    let dest_path = Path::new(&out_dir).join(bind_name);

    generated.write_to_file(dest_path).unwrap();

    #[cfg(feature = "update_bindings")]
    {
        use std::{fs, io::Write};

        let bind_file = Path::new(&out_dir).join(bind_name);
        let dest_dir = Path::new("src")
            .join("platforms")
            .join(env::var("CARGO_CFG_TARGET_OS").unwrap())
            .join(env::var("CARGO_CFG_TARGET_ARCH").unwrap());
        let dest_file = dest_dir.join(bind_name);

        fs::create_dir_all(&dest_dir).unwrap();
        fs::copy(&bind_file, &dest_file).unwrap();

        if let Ok(github_env) = env::var("GITHUB_ENV") {
            let mut env_file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(github_env)
                .unwrap();
            write!(env_file, "GBM_SYS_BINDINGS_FILE={}", dest_file.display()).unwrap();
        }
    }
}
