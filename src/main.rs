use clap::{App, Arg, SubCommand};
use std::process::{Command, Stdio};

fn main() {
    let matches = App::new("rctl")
        .about("Development containers made easy.")
        .version("0.1.0")
        .subcommand(
            SubCommand::with_name("run")
                .about("Runs your container")
                .arg(
                    Arg::with_name("image")
                        .help("Select your image <debian, fedora, ubuntu>")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Delete your container")
                .arg(Arg::with_name("id").help("Container ID to delete.").required(true)),
        )
        .subcommand(
            SubCommand::with_name("pull")
                .about("Pull an image from Docker registry")
                .arg(
                    Arg::with_name("name")
                        .help("Name of image to use Eg. <debian:latest>")
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("run", Some(run_matches)) => {
            let image = run_matches.value_of("image").unwrap();
            execute("run", &[image, "bash"]);
        }
        ("delete", Some(delete_matches)) => {
            let id = delete_matches.value_of("id").unwrap();
            execute("rm", &[id, "-f"]);
        }
        ("pull", Some(pull_matches)) => {
            let name = pull_matches.value_of("name").unwrap();
            execute("pull", &[name]);
        }
        _ => unreachable!(),
    }
}

fn execute(command: &str, args: &[&str]) {
    let mut cmd = match command {
        "run" => Command::new("docker").args(&["run", "-it"]),
        "rm" => Command::new("docker").arg("rm"),
        "pull" => Command::new("docker").arg("pull"),
        _ => panic!("Invalid command"),
    };

    cmd.args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    match cmd.spawn() {
        Ok(child) => match child.wait() {
            Ok(status) => {
                if !status.success() {
                    println!("Command exited with non-zero status: {}", status);
                }
            }
            Err(e) => println!("Failed to wait on child: {}", e),
        },
        Err(e) => println!("Failed to spawn command: {}", e),
    }
}
