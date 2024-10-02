use std::fs;
use std::io::{self};
use std::path::PathBuf;

use crate::docker_operation::{init_docker, run_coding_style_checker, update_docker_image};

pub fn coding_checker(
    delivery_dir: &PathBuf,
    reports_dir: &PathBuf,
    update_image: bool,
) -> io::Result<PathBuf> {
    let export_file = reports_dir.join("coding-style-reports.log");

    fs::remove_file(&export_file).ok();

    let base_exec_cmd = init_docker()?;

    if update_image {
        update_docker_image(base_exec_cmd)?;
    }

    run_coding_style_checker(base_exec_cmd, &delivery_dir, &reports_dir)?;

    if export_file.exists() {
        let report_file_path = reports_dir.join("coding-style-reports.log");
        return Ok(report_file_path);
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "No coding style report found",
    ))
}
