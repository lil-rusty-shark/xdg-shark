use async_std::{
    fs,
    path::Path as AsyncPath,
};

use clap::Parser;

use glob::glob;

use serde_json::Value;

use std::{
    env::var,
    path::PathBuf,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CommandLineArguments {
    #[arg(
        short = 'v',
        long,
        help = "display messages for all files checked (verbose)"
    )]
    skip_ok: bool,
    #[arg(
        short = 'q',
        long,
        help = "doesnt display messages for files without fixes (quite)"
    )]
    skip_unsupported: bool,
}

struct EnvVar {
    name: &'static str,
    recommended_value: &'static str,
}

fn check_environment_variables() {
    let env_vars = [
        EnvVar {
            name: "XDG_DATA_HOME",
            recommended_value: "$HOME/.local/share",
        },
        EnvVar {
            name: "XDG_CONFIG_HOME",
            recommended_value: "$HOME/.config",
        },
        EnvVar {
            name: "XDG_STATE_HOME",
            recommended_value: "$HOME/.local/state",
        },
        EnvVar {
            name: "XDG_CACHE_HOME",
            recommended_value: "$HOME/.cache",
        },
        EnvVar {
            name: "XDG_RUNTIME_DIR",
            recommended_value: "/run/user/$UID",
        },
    ];
    env_vars.iter().for_each(|env_var| {
        if var(env_var.name).is_err() {
            println!("The ${} environment variable is not set!\nmake sure to add it to your shell's configuration before setting any of the other environment variables!\nRecommended value: {}\n",
                env_var.name, env_var.recommended_value
            );
        }
    });
}

async fn parse_json_files(json_files: &[PathBuf]) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let mut parsed_jsons = Vec::new();
    for json_file in json_files {
        let file_contents = fs::read_to_string(json_file).await?;
        let json: Value = serde_json::from_str(&file_contents)?;
        parsed_jsons.push(json);
    }
    Ok(parsed_jsons)
}

async fn process_file_object(
    file_obj: &Value,
    command_line_arguments: &CommandLineArguments,
) {
    if let Some(path) = file_obj.get("path") {
        if let Some(path_str) = path.as_str() {
            let expanded_path = match shellexpand::full(path_str) {
                Ok(expanded) => expanded.into_owned(),
                Err(e) => {
                    eprintln!("Error expanding path {}: {}", path_str, e);
                    return;
                }
            };
            let path_exists = AsyncPath::new(&expanded_path).exists().await;
            let supported = file_obj
                .get("movable")
                .map_or(false, |v| v.as_bool().unwrap_or_default());
            let file_name = file_obj
                .get("name")
                .map_or("Unknown", |v| v.as_str().unwrap_or("Unknown"));
            let help_msg = file_obj.get("help").map_or("No help available", |v| {
                v.as_str().unwrap_or("No help available")
            });

            let help_msg = help_msg.replace("```bash\n", "");
            let help_msg = help_msg.replace("```\n", "");

            if command_line_arguments.skip_ok & !path_exists {
                println!("[{}] {}\n\n{}\n\n", file_name, expanded_path, help_msg)
            } else if !command_line_arguments.skip_unsupported & !supported & path_exists {
                println!("[{}] {}\n\n{}\n\n", file_name, expanded_path, help_msg)
            } else if path_exists & supported {
                println!("[{}] {}\n\n{}\n\n", file_name, expanded_path, help_msg)
            }
        }
    }
}

async fn process_parsed_jsons(
    parsed_jsons: Vec<Value>,
    command_line_arguments: &CommandLineArguments,
) {
    for json in parsed_jsons {
        if let Some(files) = json.get("files") {
            if let Some(file_array) = files.as_array() {
                for file_obj in file_array {
                    process_file_object(file_obj, command_line_arguments).await;
                }
            }
        }
    }
}

fn get_json_files(dir: &str) -> std::result::Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let pattern = format!("{}/{}", dir, "*.json");
    let mut json_files = Vec::new();
    for entry in glob(&pattern)? {
        match entry { 
            Ok(path) => {
                if path.is_file() {
                    json_files.push(path);
                } else {
                    eprintln!("Warning: {} is not a file", path.display());
                }
            },
            Err(e) => eprintln!("Error: {:?}", e), 
        }
    }
    Ok(json_files)
}

#[async_std::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let command_line_arguments = CommandLineArguments::parse();
    check_environment_variables();
    let json_files = get_json_files("./programs")?;

    let parsed_jsons = parse_json_files(&json_files).await?;



    process_parsed_jsons(parsed_jsons, &command_line_arguments).await;

    Ok(())
}

