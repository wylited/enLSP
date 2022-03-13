pub mod server;

// The main entry point to en_LSP. Parses the CLI arguments, then runs the server.
#[tokio::main]
async fn main() {
    let exit = parse_main();
    std::process::exit(exit.await);
}

async fn parse_main() -> i32 {
    env_logger::init();

    if let Some(first_arg) = std::env::args().nth(1) {
        return match first_arg.as_str() {
            "--version" | "-V" => {
                println!("{}", version());
                0
            }
            "--help" | "-h" => {
                println!("{}", help());
                0
            }
            "--cli" => {
                0
            }
            unknown => {
                dbg!(unknown);
                println!("Unknown argument '{}'. Supported arguments:\n{}", unknown, help());
                101
            }
        };
    }
    server::run_server().await
}

fn help() -> &'static str {
    r#"
    --version or -V to print the version and commit info
    --help or -h for this message
    --cli starts the enLSP in command line mode
    No input starts the enLSP as a language server
    "#
}

fn version() -> String {
    use rustc_tools_util::VersionInfo;

    rustc_tools_util::get_version_info!().to_string()
}