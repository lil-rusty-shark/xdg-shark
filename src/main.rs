use clap::Parser;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short='v', long, help="display messages for all files checked (verbose)")]
    skip_ok: bool,

    #[arg(short='q', long, help="doesnt display messages for files without fixes (quite)")]
    skip_unsupported: bool,
}


fn main () {
    let cli = Cli::parse();
}

