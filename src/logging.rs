use std::env;

use log::LevelFilter;

pub fn setup() {
    let mut builder = &mut env_logger::builder();

    // check if user has set -v
    let mut pargs = pico_args::Arguments::from_env();
    let verbose = pargs.contains(["-v", "--verbose"]);

    // do something only if env var has not been set explicitely by user
    if env::var("RUST_LOG").is_err() {
        let wanted_level = if verbose {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        };

        // set level **only for our own package**
        let package_mod = module_path!().split("::").next().unwrap_or_default();
        builder = builder.filter_module(package_mod, wanted_level);
    } else {
        if verbose {
            eprintln!("RUST_LOG env var has been set by user, ignoring -v");
        }
    }

    builder.init();
}
