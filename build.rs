use built::Options;
use std::env;
use std::path::Path;

fn main() {
    let src = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dst = Path::new(&env::var("OUT_DIR").unwrap()).join("built.rs");
    let mut opts = Options::default();
    opts.set_dependencies(true).set_compiler(true).set_env(true);
    built::write_built_file_with_opts(&opts, src.as_ref(), &dst)
        .expect("Failed to acquire build-time information");
}
