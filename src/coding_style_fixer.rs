use std::{fs::read_to_string, path::PathBuf, process::Command};

use crate::{fixer::get_rules, project_operation::Project, CodingStyleAlert};

fn fix_clang() {
    Command::new("sh")
        .arg("-c")
        .arg("find . -name '*.c' -exec clang-format -i {} \\;")
        .output()
        .expect("Failed to execute command");
}

pub fn get_alerts(
    report_file_path: &PathBuf,
    delivery_dir: &PathBuf
) -> Vec<CodingStyleAlert> {
    let coding_report_content =
        read_to_string(report_file_path).expect("Something went wrong reading the file");

    CodingStyleAlert::alerts_from_report(&coding_report_content, &get_rules(), delivery_dir)
}

pub fn fix_coding_style_issues(
    alerts: &mut Vec<CodingStyleAlert>,
    delivery_dir: &PathBuf,
    project: &Project,
) {
    fix_clang();
    
    for alert in alerts {
        alert.fix_issue(delivery_dir, project);
    }
}
