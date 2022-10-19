use std::process::{Stdio, Command as Cmd};

fn main() {
	println!("rctl")
	create_session("debian")
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
