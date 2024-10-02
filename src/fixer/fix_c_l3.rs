use crate::{project_operation::Project, CodingStyleAlert};
pub fn fix_c_l3(coding_alert: &CodingStyleAlert, file_content: &mut String, _: &Project) -> String {
    let mut new_content = String::new();
    let lines: Vec<&str> = file_content.lines().collect();

    for (index, line) in lines.iter().enumerate() {
        if index == coding_alert.line - 1 {
            let mut corrected_line = line.to_string();
            println!("line: {}", corrected_line);
            corrected_line = corrected_line.replace("if(", "if (");
            corrected_line = corrected_line.replace("){", ") {");

           
            new_content.push_str(&corrected_line);
        } else {
            new_content.push_str(line);
        }
        new_content.push('\n');
    }

    new_content
}
