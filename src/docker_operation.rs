use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn check_docker_socket(docker_socket_path: &Path) -> io::Result<bool> {
    docker_socket_path
        .metadata()
        .map(|m| !m.permissions().readonly())
        .or(Ok(false))
}

pub fn update_docker_image(base_exec_cmd: &str) -> io::Result<()> {
    println!("Downloading new image and cleaning old one...");
    Command::new(base_exec_cmd)
        .args(&["pull", "ghcr.io/epitech/coding-style-checker:latest"])
        .status()?;
    Command::new(base_exec_cmd)
        .args(&["image", "prune", "-f"])
        .status()?;
    println!("Download OK");
    Ok(())
}

pub fn run_coding_style_checker(
    base_exec_cmd: &str,
    delivery_dir: &Path,
    reports_dir: &Path,
) -> io::Result<()> {
    println!("running...");
    Command::new(base_exec_cmd)
        .args(&[
            "run",
            "--rm",
            "--security-opt",
            "label:disable",
            "-i",
            "-v",
            &format!("{}:/mnt/delivery", delivery_dir.display()),
            "-v",
            &format!("{}:/mnt/reports", reports_dir.display()),
            "ghcr.io/epitech/coding-style-checker:latest",
            "/mnt/delivery",
            "/mnt/reports",
        ])
        .status()?;


    Ok(())
}

pub fn init_docker() -> io::Result<&'static str> {
    let docker_socket_path = PathBuf::from("/var/run/docker.sock");
    if check_docker_socket(&docker_socket_path)? {
        return Ok("docker");
    }

    println!("WARNING: Socket access is denied");
    println!("To fix this, add the current user to the docker group with: sudo usermod -a -G docker $USER");
    print!("Do you want to proceed? (yes/no) ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if input.trim().eq_ignore_ascii_case("yes") {
        println!("Proceeding...");
        println!("Reboot your computer for the changes to take effect.");
        Ok("sudo docker")
    } else {
        Ok("Operation cancelled by user.")
    }
}
