use bindgen;
use cc;
use std::env;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Copy, Clone)]
struct BindgenError;

impl fmt::Display for BindgenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BindgenError")
    }
}

impl Error for BindgenError {}

fn main() -> Result<(), Box<dyn Error>> {
    cc::Build::new()
        .include("c_src")
        .define("gcc", "")
        .flag("-Wno-unused-parameter")
        .file("c_src/driverlib/emac.c")
        .file("c_src/driverlib/interrupt.c")
        .file("c_src/driverlib/sysctl.c")
        .file("c_src/driverlib/cpu.c")
        .compile("tivaware");

    let bindings = bindgen::Builder::default()
        .clang_arg("-target")
        .clang_arg(env::var("TARGET")?)
        .use_core()
        .header("stdint.h")
        .header("stdbool.h")
        .header("c_src/driverlib/emac.h")
        .header("c_src/inc/hw_emac.h")
        .ctypes_prefix("::cty")
        .constified_enum(".*")
        .prepend_enum_name(false)
        .generate_comments(false)
        .generate()
        .map_err(|_| BindgenError)?;

    let out_path = PathBuf::from(env::var("OUT_DIR")?);
    bindings.write_to_file(out_path.join("bindings.rs"))?;
    Ok(())
}
