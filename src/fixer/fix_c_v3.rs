use crate::{project_operation::Project, CodingStyleAlert};

pub fn fix_c_v3(coding_alert: &CodingStyleAlert, file_content: &mut String, _: &Project) -> String {
    let lines: Vec<&str> = file_content.lines().collect();
    let mut new_content = String::new();

    for (index, line) in lines.iter().enumerate() {
        if index == coding_alert.line - 1 {
            let corrected_line = line.replace("* ", "*").replace(" *", " *");
            new_content.push_str(&corrected_line);
        } else {
            new_content.push_str(line);
        }
        new_content.push('\n');
    }

    new_content
}
