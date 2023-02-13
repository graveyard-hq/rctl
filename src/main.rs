use clap::{Arg, ArgAction, Command};
use std::process::{Command as Cmd, Stdio};

fn main() {
    let matches = Command::new("rctl")
        .about("Development containers made easy.")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("run").about("Runs your container").arg(
                Arg::new("image")
                    .help("Select your image <debian, fedora, ubuntu>")
                    .action(ArgAction::Set),
            ),
        )
        .subcommand(
            Command::new("delete").about("Delete your container").arg(
                Arg::new("id")
                    .help("Container ID to delete.")
                    .action(ArgAction::Set),
            ),
        )
        .subcommand(
            Command::new("pull")
                .about("Pull a image from docker registry")
                .arg(
                    Arg::new("name")
                        .help("Name of image to use Eg. <debian:latest>")
                        .action(ArgAction::Set),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("run", run_matches)) => {
            let image = get_args(run_matches, "image").unwrap_or_else(|| "".to_owned());
            execute("run", &image);
        }
        Some(("delete", run_matches)) => {
            let id = get_args(run_matches, "id").unwrap_or_else(|| "".to_owned());
            execute("delete", &id);
        }
        Some(("pull", run_matches)) => {
            let name = get_args(run_matches, "name").unwrap_or_else(|| "".to_owned());
            execute("pull", &name);
        }
        _ => unreachable!(),
    }
}

fn get_args(matches: &clap::ArgMatches, name: &str) -> Option<String> {
    if matches.contains_id(name) {
        let run_command_args: Vec<_> = matches
            .get_many::<String>(name)
            .expect("contains_id")
            .map(|s| s.as_str())
            .collect();
        Some(run_command_args.join(", "))
    } else {
        None
    }
}

fn execute(command: &str, args: &str) {
    let mut cmd = match command {
        "run" => Cmd::new("docker").args(&["run", "-it", args, "bash"]),
        "delete" => Cmd::new("docker").args(&["rm", args, "-f"]),
        "pull" => Cmd::new("docker").args(&["pull", args]),
        _ => panic!("Invalid command"),
    };

    cmd.stdout(Stdio::inherit());
    cmd.stderr(Stdio::inherit());

    match cmd.spawn() {
        Ok(child) => {
            match child.wait() {
                Ok(status) => {
                    if !status.success() {
                        println!("Command exited with non-zero status: {}", status);
                    }
                }
                Err(e) => {
                    println!("Failed to wait on child: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to spawn command: {}", e);
        }
    }
}
