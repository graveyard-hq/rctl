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
            if run_matches.contains_id("image") {
                let run_command_args: Vec<_> = run_matches
                    .get_many::<String>("image")
                    .expect("contains_id")
                    .map(|s| s.as_str())
                    .collect();
                let image = run_command_args.join(", ");
                create_session(&image);
                return;
            }
        }
        Some(("pull", run_matches)) => {
            if run_matches.contains_id("name") {
                let run_command_args: Vec<_> = run_matches
                    .get_many::<String>("name")
                    .expect("contains_id")
                    .map(|s| s.as_str())
                    .collect();
                let image_name = run_command_args.join(", ");
                pull_image(&image_name);
                return;
            }
        }
        Some(("delete", run_matches)) => {
            if run_matches.contains_id("id") {
                let run_command_args: Vec<_> = run_matches
                    .get_many::<String>("id")
                    .expect("contains_id")
                    .map(|s| s.as_str())
                    .collect();
                let container_id = run_command_args.join(", ");
                delete_container(&container_id);
                return;
            }
        }
        _ => unreachable!(),
    }
}

fn create_session(image: &str) {
    let mut cmd = Cmd::new("docker")
        .args(&["run", "-it", &image, "bash"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    println!("[LOG] Connecting to SHELL...");
    cmd.wait().expect("Unexpected Error");
}

fn delete_container(id: &str) {
    let mut cmd = Cmd::new("docker")
        .args(&["rm", &id, "-f"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    cmd.wait().expect("Unexpected Error");
}

fn pull_image(image: &str) {
    let mut cmd = Cmd::new("docker")
        .args(&["pull", &image])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    cmd.wait().expect("Unexpected Error");
}
