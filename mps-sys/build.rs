extern crate bindgen;
extern crate cc;
extern crate regex;

use std::env;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt::Write;
use std::error::Error;

use regex::Regex;

fn generate_mps_args<P: AsRef<Path>>(header: P) -> Result<String, Box<Error>> {
    let mut out = String::new();
    let re = Regex::new(r"^#define[\t ]+MPS_KEY_(?P<name>[A-Z_]+)_FIELD[\t ]+(?P<field>[a-z_]+)")?;
    let source_code = BufReader::new(File::open(header)?);

    writeln!(&mut out, "#[macro_export] macro_rules! mps_arg_s {{")?;
    for line in source_code.lines() {
        let l = line?;
        if let Some(c) = re.captures(&l) {
            writeln!(
                &mut out,
                r"(MPS_KEY_{0}, $value:expr) => {{ unsafe {{
                let mut _arg: $crate::mps_arg_s = ::std::mem::zeroed();
                _arg.key = &$crate::_mps_key_{0};
                _arg.val.{1} = $value;
                _arg
            }} }};",
                &c["name"],
                &c["field"]
            )?;
        }
    }

    writeln!(
        &mut out,
        r"(MPS_KEY_ARGS_END) => {{ unsafe {{
        let mut _arg: $crate::mps_arg_s = ::std::mem::zeroed();
        _arg.key = &$crate::_mps_key_ARGS_END;
        _arg
    }} }};"
    )?;

    writeln!(&mut out, "}}")?;

    Ok(out)
}

fn main() {
    // use cool variety if debug mode enabled
    let variety = if env::var("DEBUG").map(|val| val == "true").unwrap_or(false) {
        "CONFIG_VAR_COOL"
    } else {
        "CONFIG_VAR_HOT"
    };

    cc::Build::new()
        .file("mps-kit/code/mps.c")
        .define(variety, None)
        .flag("-std=c11")
        .flag_if_supported("-Wimplicit-fallthrough=2")
        .include("mps-kit/code")
        .compile("libmps.a");

    let mps_h = "mps-kit/code/mps.h";
    let mps_arg_macro = generate_mps_args(mps_h).expect("failed to generate args macro");

    let bindings = bindgen::Builder::default()
        .header("mps-kit/code/mps.h")
        .header("mps-kit/code/mpsavm.h")
        .raw_line(mps_arg_macro)
        .clang_arg("-Imps-kit/code")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
