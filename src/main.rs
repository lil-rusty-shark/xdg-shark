use clap::Parser;
use std::env;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short='v', long, help="display messages for all files checked (verbose)")]
    skip_ok: bool,

    #[arg(short='q', long, help="doesnt display messages for files without fixes (quite)")]
    skip_unsupported: bool,
}


struct EnvVar {
    name: &'static str,
    message: &'static str,
    recommended_value: &'static str,
}


fn check_envrioment_variables () {
        let env_vars = [
        EnvVar {
            name: "XDG_DATA_HOME",
            message: "The $XDG_DATA_HOME environment variable is not set, make sure to add it to your shell's configuration before setting any of the other environment variables!",
            recommended_value: "$HOME/.local/share",
        },
        EnvVar {
            name: "XDG_CONFIG_HOME",
            message: "The $XDG_CONFIG_HOME environment variable is not set, make sure to add it to your shell's configuration before setting any of the other environment variables!",
            recommended_value: "$HOME/.config",
        },
        EnvVar {
            name: "XDG_STATE_HOME",
            message: "The $XDG_STATE_HOME environment variable is not set, make sure to add it to your shell's configuration before setting any of the other environment variables!",
            recommended_value: "$HOME/.local/state",
        },
        EnvVar {
            name: "XDG_CACHE_HOME",
            message: "The $XDG_CACHE_HOME environment variable is not set, make sure to add it to your shell's configuration before setting any of the other environment variables!",
            recommended_value: "$HOME/.cache",
        },
        EnvVar {
            name: "XDG_RUNTIME_DIR",
            message: "The $XDG_RUNTIME_DIR environment variable is not set, make sure to add it to your shell's configuration before setting any of the other environment variables!",
            recommended_value: "/run/user/$UID",
        },
    ];

    for env_var in &env_vars {
        if env::var(env_var.name).is_err() {
            println!("{} {}", env_var.message, env_var.recommended_value);
        }
    }
}


fn main () {
    let cli = Cli::parse();
    check_envrioment_variables()


}

