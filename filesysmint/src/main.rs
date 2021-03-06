use clap::{App, Arg};
use env_logger::Builder;
use std::path::PathBuf;

use filesysmint::FileSysMint;
use node_runtime::chain_spec::ChainSpec;

const DEFAULT_BASE_PATH: &str = "";
//
const DEFAULT_FILE_LOG: &str = "info";
const DEFAULT_FILE_PATH: &str = ".filesys";
const DEFAULT_XDG_APP_NAME: &str = ".filesys";
const DEFAULT_CONFIG_FILE: &str = "config.json";



fn main() {
    // Parse command line arguments.
    let matches = App::new("Filesysmint")
        .args(&[
            Arg::with_name("base_path")
                .short("d")
                .long("base-path")
                .value_name("PATH")
                .help("Specify a base path for persisted files.")
                .default_value(DEFAULT_BASE_PATH)
                .takes_value(true),
            Arg::with_name("chain_spec_file")
                .short("c")
                .long("chain-spec-file")
                .value_name("CHAIN_SPEC")
                .help("Specify a file location to read a custom chain spec.")
                .takes_value(true),
            Arg::with_name("devnet")
                .long("devnet")
                .help("Run with DevNet validator configuration (single alice.near validator)")
                .takes_value(false),
            Arg::with_name("abci_address")
                .short("a")
                .long("abci-address")
                .value_name("ABCI_Address")
                .help("Specify ip address and port for Tendermint ABCI")
                .default_value("127.0.0.1:26658")
                .takes_value(true),
            Arg::with_name("file_path")
                .short("f")
                .long("file_path")
                .value_name("FILE_PATH")
                .help("Set the repo directory, defaults to ~/.filesys/repo")
                .default_value(DEFAULT_FILE_PATH)
                .takes_value(true),
        ])
        .get_matches();
    let base_path = matches.value_of("base_path").map(PathBuf::from).unwrap();
    let chain_spec = if matches.is_present("devnet") {
        ChainSpec::default_devnet()
    } else {
        let chain_spec_path = matches.value_of("chain_spec_file").map(PathBuf::from);
        ChainSpec::from_file_or_default(&chain_spec_path, ChainSpec::default_poa())
    };
    let addr = matches.value_of("abci_address").map(|address| address.parse().unwrap()).unwrap();
    let file_path = matches.value_of("file_path").map(PathBuf::from).unwrap();

    // Setup logging.
    let mut builder = Builder::from_default_env();
    builder.default_format_timestamp_nanos(true);
    builder.filter(None, log::LevelFilter::Info);
    builder.try_init().unwrap();

    // Fire it up!
    abci::run(addr, FileSysMint::new(&base_path, chain_spec));
}
