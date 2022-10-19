use clap::{Arg, ArgAction, Command};
use std::process::{Stdio, Command as Cmd};

fn main() {
    let matches = Command::new("rctl")
        .about("Development containers made easy.")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("run")
                .about("Runs your container")
                .arg(
                    Arg::new("image")
                    .help("Select your image <debian, fedora, ubuntu>")
                    .action(ArgAction::Set)
                )
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
        _ => unreachable!(),
    }
}

fn create_session(image: &str) {
    let mut cmd = Cmd::new("docker")
        .args(&["run", "-it", "-m", "128m", "--cpus", "1", &image, "bash"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let status = cmd.wait();
    println!("Exited with status {:?}", status);
}
