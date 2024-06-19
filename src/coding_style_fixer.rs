use std::{fs::read_to_string, path::PathBuf};

use crate::{fixer::get_rules, project_operation::Project, CodingStyleAlert};

pub fn fix_coding_style_issues(
    report_file_path: &PathBuf,
    delivery_dir: &PathBuf,
    project: &Project,
) {
    let rules = get_rules();

    let coding_report_content =
        read_to_string(report_file_path).expect("Something went wrong reading the file");

    let alerts = CodingStyleAlert::alerts_from_report(&coding_report_content, &rules, delivery_dir);
    for alert in alerts {
        alert.fix_issue(delivery_dir, project);
    }
}
