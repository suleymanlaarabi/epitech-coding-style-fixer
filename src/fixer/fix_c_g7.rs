use crate::{project_operation::Project, CodingStyleAlert};

pub fn fix_c_g7(coding_alert: &CodingStyleAlert, file_content: &mut String, _: &Project) -> String {
    let new_content = file_content
        .lines()
        .enumerate()
        .map(|(line_number, line)| {
            if line_number == coding_alert.line - 1 {
                line.trim_end()
            } else {
                line
            }
        })
        .collect::<Vec<&str>>()
        .join("\n");

    new_content
}
