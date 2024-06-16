use std::{fs::read_to_string, path::PathBuf};

use crate::{
    all_coding_style_fixe::get_rules, prelude::CodingStyleAlert, project_operation::Project,
};

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
