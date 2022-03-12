pub mod lib;

// The main entry point to en_LSP. Parses the CLI arguments, then runs the server.

#[tokio::main]
async fn main() {
    let exit = parse_main();
    std::process::exit(exit.await);
}

async fn parse_main() -> i32 {
    env_logger::init();
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    if let Some(first_arg) = std::env::args().nth(1) {
        return match first_arg.as_str() {
            "--version" | "-V" => {
                println!("{}", lib::version());
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

    0
}

fn help() -> &'static str {
    r#"
    --version or -V to print the version and commit info
    --help or -h for this message
    --cli starts the enLSP in command line mode
    No input starts the enLSP as a language server
    "#
}